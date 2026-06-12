use crate::CoreContext;
use ritmo_domain::{Book, Person, Role, Tag};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    BookRepository, PersonRepository, RoleRepository, TagRepository, XBooksPeopleRolesRepository,
    XBooksTagsRepository,
};
use std::collections::HashSet;

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Book>> {
    BookRepository::new(&ctx.ctx).list_all().await
}

pub async fn get(ctx: &CoreContext, id: i64) -> RitmoResult<Book> {
    BookRepository::new(&ctx.ctx).get(id).await
}

pub async fn list_people_with_roles(
    ctx: &CoreContext,
    book_id: i64,
) -> RitmoResult<Vec<(Person, Role)>> {
    let pr_repo = XBooksPeopleRolesRepository::new(&ctx.ctx);
    let person_repo = PersonRepository::new(&ctx.ctx);
    let role_repo = RoleRepository::new(&ctx.ctx);

    let pairs = pr_repo.list_by_book(book_id).await?;
    let mut result = Vec::new();
    for (person_id, role_id) in pairs {
        let person = person_repo.get(person_id).await?;
        let role = role_repo.get(role_id).await?;
        result.push((person, role));
    }
    Ok(result)
}

pub async fn list_tags(ctx: &CoreContext, book_id: i64) -> RitmoResult<Vec<Tag>> {
    let tags_rel_repo = XBooksTagsRepository::new(&ctx.ctx);
    let tag_repo = TagRepository::new(&ctx.ctx);

    let tag_ids = tags_rel_repo.list_by_book(book_id).await?;
    let mut tags = Vec::new();
    for tag_id in tag_ids {
        tags.push(tag_repo.get(tag_id).await?);
    }
    Ok(tags)
}

pub async fn get_format_name(ctx: &CoreContext, book_id: i64) -> RitmoResult<Option<String>> {
    BookRepository::new(&ctx.ctx).get_format_name(book_id).await
}

pub async fn get_series_name(ctx: &CoreContext, book_id: i64) -> RitmoResult<Option<String>> {
    BookRepository::new(&ctx.ctx).get_series_name(book_id).await
}

pub async fn create(ctx: &CoreContext, item: &Book) -> RitmoResult<i64> {
    if item.title.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("title cannot be empty".to_string()));
    }
    let repo = BookRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Book) -> RitmoResult<()> {
    if item.title.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("title cannot be empty".to_string()));
    }
    let repo = BookRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn replace_tags_from_names(
    ctx: &CoreContext,
    book_id: i64,
    tag_names: &[String],
) -> RitmoResult<()> {
    let tags_rel_repo = XBooksTagsRepository::new(&ctx.ctx);
    let tag_repo = TagRepository::new(&ctx.ctx);

    let mut target_tag_ids = HashSet::new();
    for tag_name in tag_names {
        let tag_name = tag_name.trim();
        if tag_name.is_empty() {
            continue;
        }

        let tag_id = if let Some(existing_tag) = tag_repo
            .search(tag_name)
            .await?
            .into_iter()
            .find(|tag| tag.name.eq_ignore_ascii_case(tag_name))
        {
            existing_tag.id
        } else {
            tag_repo
                .save(&Tag {
                    id: 0,
                    name: tag_name.to_owned(),
                    tag_type: "personal".to_owned(),
                })
                .await?;
            tag_repo
                .search(tag_name)
                .await?
                .into_iter()
                .find(|tag| tag.name.eq_ignore_ascii_case(tag_name))
                .ok_or_else(|| RitmoErr::RecordNotFound)?
                .id
        };
        target_tag_ids.insert(tag_id);
    }

    let current_tag_ids: HashSet<i64> = tags_rel_repo.list_by_book(book_id).await?.into_iter().collect();

    for tag_id in current_tag_ids.difference(&target_tag_ids) {
        tags_rel_repo.delete(book_id, *tag_id).await?;
    }
    for tag_id in target_tag_ids.difference(&current_tag_ids) {
        tags_rel_repo.create(book_id, *tag_id).await?;
    }

    Ok(())
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    BookRepository::new(&ctx.ctx).delete(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ritmo_domain::Content;
    use ritmo_repository::{ContentRepository, RepositoryContext, XBooksContentsRepository};

    fn sample_book() -> Book {
        Book {
            id: 0,
            title: "Libro".to_owned(),
            original_title: None,
            publisher_id: None,
            format_id: None,
            series_id: None,
            series_index: None,
            isbn: None,
            publication_year: None,
            notes: None,
            has_cover: false,
            has_paper: false,
        }
    }

    #[tokio::test]
    async fn delete_relies_on_database_cascade() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx.clone());

        let book_id = BookRepository::new(&repo_ctx).save(&sample_book()).await.unwrap();
        let content_id = ContentRepository::new(&repo_ctx)
            .save(&Content {
                id: 0,
                title: "Contenuto".to_owned(),
                original_title: None,
                type_id: None,
                publication_year: None,
                notes: None,
            })
            .await
            .unwrap();
        XBooksContentsRepository::new(&repo_ctx)
            .create(book_id, content_id)
            .await
            .unwrap();

        delete(&core, book_id).await.unwrap();

        assert!(BookRepository::new(&repo_ctx).get(book_id).await.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM x_books_contents WHERE book_id = ?",
            )
            .bind(book_id)
            .fetch_one(repo_ctx.pool())
            .await
            .unwrap(),
            0
        );
        assert!(ContentRepository::new(&repo_ctx).get(content_id).await.is_ok());
    }

    #[tokio::test]
    async fn replace_tags_from_names_uses_existing_case_insensitive_tags() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx.clone());

        let book_id = BookRepository::new(&repo_ctx).save(&sample_book()).await.unwrap();
        let existing_tag_id = sqlx::query("INSERT INTO d_tags(name, tag_type) VALUES (?, ?)")
            .bind("Noir")
            .bind("genre")
            .execute(repo_ctx.pool())
            .await
            .unwrap()
            .last_insert_rowid();
        let second_tag_id = sqlx::query("INSERT INTO d_tags(name, tag_type) VALUES (?, ?)")
            .bind("Fantasy")
            .bind("setting")
            .execute(repo_ctx.pool())
            .await
            .unwrap()
            .last_insert_rowid();
        XBooksTagsRepository::new(&repo_ctx)
            .create(book_id, existing_tag_id)
            .await
            .unwrap();

        replace_tags_from_names(&core, book_id, &[" noir ".to_owned(), "fantasy".to_owned()])
            .await
            .unwrap();

        let linked = XBooksTagsRepository::new(&repo_ctx)
            .list_by_book(book_id)
            .await
            .unwrap();
        assert_eq!(linked.len(), 2);
        assert!(linked.contains(&existing_tag_id));
        assert!(linked.contains(&second_tag_id));
    }
}
