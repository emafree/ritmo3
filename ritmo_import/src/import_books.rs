use ritmo_domain::PartialDate;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    BookRepository, ContentRepository, FormatRepository, GenreRepository, LanguageRepository,
    PersonRepository, PublisherRepository, RepositoryContext, RoleRepository, SeriesRepository,
    TagRepository, XBookLanguagesRepository, XBooksContentsRepository, XBooksPeopleRolesRepository,
    XBooksTagsRepository, XContentLanguagesRepository, XContentsPeopleRolesRepository,
};

use crate::model::{BookFile, BookInput, ContentInput, PartialDateInput};
use crate::reporter::CliReporter;
use ritmo_errors::reporter::RitmoReporter;

fn to_partial_date(input: &PartialDateInput) -> PartialDate {
    PartialDate {
        year: input.year,
        month: input.month,
        day: input.day,
        circa: input.circa,
    }
}

fn resolve_language_field<'a>(
    iso2: &'a Option<String>,
    iso3: &'a Option<String>,
    name: &'a Option<String>,
) -> RitmoResult<(&'static str, &'a str)> {
    if let Some(v) = iso2 {
        Ok(("iso_code_2char", v.as_str()))
    } else if let Some(v) = iso3 {
        Ok(("iso_code_3char", v.as_str()))
    } else if let Some(v) = name {
        Ok(("official_name", v.as_str()))
    } else {
        Err(RitmoErr::InvalidInput(
            "language: nessun campo specificato".into(),
        ))
    }
}

pub async fn import_books_file(
    repo_ctx: &RepositoryContext,
    file: &BookFile,
    reporter: &mut CliReporter,
) -> RitmoResult<()> {
    for book_input in &file.book {
        import_book(repo_ctx, book_input, reporter).await?;
    }
    Ok(())
}

async fn import_book(
    repo_ctx: &RepositoryContext,
    input: &BookInput,
    reporter: &mut CliReporter,
) -> RitmoResult<()> {
    // 1. Resolve format, publisher, series via get_or_create.
    // Note: the Book domain struct does not include format_id, publisher_id, series_id or
    // series_index, so these resolved ids cannot be stored on the book record through the
    // existing BookRepository API.
    // TODO: link format/publisher/series to the book once ritmo_core/ritmo_repository exposes
    //       the corresponding update operations on d_books.
    if let Some(format_name) = &input.format {
        let _format = FormatRepository::new(repo_ctx)
            .get_or_create(format_name)
            .await?;
    }
    if let Some(publisher_name) = &input.publisher {
        let _publisher = PublisherRepository::new(repo_ctx)
            .get_or_create(publisher_name)
            .await?;
    }
    if let Some(series_name) = &input.series {
        let _series = SeriesRepository::new(repo_ctx)
            .get_or_create(series_name)
            .await?;
    }

    // 2. get_or_create the book
    let book_repo = BookRepository::new(repo_ctx);
    let mut book = book_repo.get_or_create(&input.name).await?;

    // Update basic book fields if they are currently empty
    let mut needs_update = false;
    if book.isbn.is_none() {
        if let Some(v) = &input.isbn {
            book.isbn = Some(v.clone());
            needs_update = true;
        }
    }
    if book.notes.is_none() {
        if let Some(v) = &input.notes {
            book.notes = Some(v.clone());
            needs_update = true;
        }
    }
    if book.publication_year.is_none() {
        if let Some(v) = &input.publication_date {
            book.publication_year = Some(to_partial_date(v));
            needs_update = true;
        }
    }
    if needs_update {
        book_repo.update(&book).await?;
    }

    // 3. Add tags via get_or_create + link to book
    let tag_repo = TagRepository::new(repo_ctx);
    let book_tags_repo = XBooksTagsRepository::new(repo_ctx);
    for tag_name in &input.tags {
        let tag = tag_repo.get_or_create(tag_name).await?;
        // Ignore duplicate relation errors silently
        let _ = book_tags_repo.create(book.id, tag.id).await;
    }

    // 4. Add book languages, resolving role by name
    let lang_repo = LanguageRepository::new(repo_ctx);
    let role_repo = RoleRepository::new(repo_ctx);
    let book_langs_repo = XBookLanguagesRepository::new(repo_ctx);
    for lang_input in &input.language {
        let (field, value) =
            resolve_language_field(&lang_input.iso2, &lang_input.iso3, &lang_input.name)?;
        let language = lang_repo.get_or_create_by_field(field, value).await?;
        let role = role_repo.get_or_create(&lang_input.role).await?;
        let _ = book_langs_repo
            .create(book.id, language.id, role.id)
            .await;
    }

    // 5. Add book persons, resolving role by name
    let book_people_repo = XBooksPeopleRolesRepository::new(repo_ctx);
    let person_repo = PersonRepository::new(repo_ctx);
    for person_input in &input.person {
        let person = person_repo.get_or_create(&person_input.name).await?;
        let role = role_repo.get_or_create(&person_input.role).await?;
        let _ = book_people_repo
            .create(book.id, person.id, role.id)
            .await;
        reporter.progress(&format!("  ✓ persona: {} ({})", person_input.name, person_input.role));
    }

    // 6. Process each content
    let book_contents_repo = XBooksContentsRepository::new(repo_ctx);
    for content_input in &input.content {
        import_content(
            repo_ctx,
            book.id,
            content_input,
            &book_contents_repo,
            &lang_repo,
            &role_repo,
            reporter,
        )
        .await?;
    }

    reporter.progress(&format!("→ {} ... ok", input.name));
    Ok(())
}

async fn import_content(
    repo_ctx: &RepositoryContext,
    book_id: i64,
    input: &ContentInput,
    book_contents_repo: &XBooksContentsRepository,
    lang_repo: &LanguageRepository,
    role_repo: &RoleRepository,
    reporter: &mut CliReporter,
) -> RitmoResult<()> {
    // Resolve content type and genre via get_or_create.
    // Note: the Content domain struct does not include type_id or genre_id, so these
    // resolved ids cannot be stored on the content record through the existing
    // ContentRepository API.
    // TODO: link type/genre to the content once ritmo_core/ritmo_repository exposes
    //       the corresponding update operations on d_contents.
    if let Some(type_name) = &input.content_type {
        let _genre = GenreRepository::new(repo_ctx)
            .get_or_create(type_name)
            .await?;
    }
    if let Some(genre_name) = &input.genre {
        let _genre = GenreRepository::new(repo_ctx)
            .get_or_create(genre_name)
            .await?;
    }

    // get_or_create the content
    let content_repo = ContentRepository::new(repo_ctx);
    let mut content = content_repo.get_or_create(&input.name).await?;

    // Update basic content fields if empty
    let mut needs_update = false;
    if content.notes.is_none() {
        if let Some(v) = &input.notes {
            content.notes = Some(v.clone());
            needs_update = true;
        }
    }
    if content.publication_year.is_none() {
        if let Some(v) = &input.publication_date {
            content.publication_year = Some(to_partial_date(v));
            needs_update = true;
        }
    }
    if needs_update {
        content_repo.update(&content).await?;
    }

    // Link content to book (idempotent)
    let _ = book_contents_repo.create(book_id, content.id).await;

    // Add content languages, resolving role by name
    let content_langs_repo = XContentLanguagesRepository::new(repo_ctx);
    for lang_input in &input.language {
        let (field, value) =
            resolve_language_field(&lang_input.iso2, &lang_input.iso3, &lang_input.name)?;
        let language = lang_repo.get_or_create_by_field(field, value).await?;
        let role = role_repo.get_or_create(&lang_input.role).await?;
        let _ = content_langs_repo
            .create(content.id, language.id, role.id)
            .await;
    }

    // Add content persons, resolving role by name
    let person_repo = PersonRepository::new(repo_ctx);
    let content_people_repo = XContentsPeopleRolesRepository::new(repo_ctx);
    for person_input in &input.person {
        let person = person_repo.get_or_create(&person_input.name).await?;
        let role = role_repo.get_or_create(&person_input.role).await?;
        let _ = content_people_repo
            .create(content.id, person.id, role.id)
            .await;
    }

    reporter.progress(&format!("  ✓ contenuto: {}", input.name));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::import_books_file;
    use crate::model::{
        BookFile, BookInput, BookLanguageInput, ContentInput, ContentLanguageInput,
    };
    use crate::reporter::CliReporter;
    use ritmo_errors::RitmoErr;
    use ritmo_repository::RepositoryContext;

    #[tokio::test]
    async fn import_books_file_fails_when_language_field_is_missing() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .expect("in-memory db");
        let repo_ctx = RepositoryContext::new(pool);
        let mut reporter = CliReporter;
        let input = BookFile {
            book: vec![BookInput {
                name: "Libro".into(),
                original_title: None,
                format: None,
                publisher: None,
                series: None,
                series_index: None,
                publication_date: None,
                isbn: None,
                notes: None,
                has_cover: false,
                has_paper: false,
                file_link: None,
                tags: vec![],
                language: vec![BookLanguageInput {
                    iso2: None,
                    iso3: None,
                    name: None,
                    role: "actual".into(),
                }],
                person: vec![],
                content: vec![],
            }],
        };

        let err = import_books_file(&repo_ctx, &input, &mut reporter)
            .await
            .expect_err("missing language selector should fail");
        match err {
            RitmoErr::InvalidInput(msg) => {
                assert_eq!(msg, "language: nessun campo specificato");
            }
            other => panic!("expected InvalidInput, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn import_books_file_accepts_iso2_iso3_and_name_language_fields() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .expect("in-memory db");
        let repo_ctx = RepositoryContext::new(pool);
        let mut reporter = CliReporter;
        let input = BookFile {
            book: vec![BookInput {
                name: "Libro".into(),
                original_title: None,
                format: None,
                publisher: None,
                series: None,
                series_index: None,
                publication_date: None,
                isbn: None,
                notes: None,
                has_cover: false,
                has_paper: false,
                file_link: None,
                tags: vec![],
                language: vec![BookLanguageInput {
                    iso2: Some("it".into()),
                    iso3: None,
                    name: None,
                    role: "actual".into(),
                }],
                person: vec![],
                content: vec![ContentInput {
                    name: "Contenuto".into(),
                    original_title: None,
                    content_type: None,
                    genre: None,
                    publication_date: None,
                    notes: None,
                    language: vec![
                        ContentLanguageInput {
                            iso2: None,
                            iso3: Some("grc".into()),
                            name: None,
                            role: "original".into(),
                        },
                        ContentLanguageInput {
                            iso2: None,
                            iso3: None,
                            name: Some("English".into()),
                            role: "source".into(),
                        },
                    ],
                    person: vec![],
                }],
            }],
        };

        import_books_file(&repo_ctx, &input, &mut reporter)
            .await
            .expect("valid language selectors should import");
    }
}
