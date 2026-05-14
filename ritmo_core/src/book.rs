use crate::CoreContext;
use ritmo_domain::{Book, Person, Role, Tag};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    BookRepository, PersonRepository, RoleRepository, TagRepository, XBookLanguagesRepository,
    XBooksContentsRepository, XBooksPeopleRolesRepository, XBooksTagsRepository,
};

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

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let contents_repo = XBooksContentsRepository::new(&ctx.ctx);
    for content_id in contents_repo.list_by_book(id).await? {
        contents_repo.delete(id, content_id).await?;
    }

    let people_roles_repo = XBooksPeopleRolesRepository::new(&ctx.ctx);
    for (person_id, role_id) in people_roles_repo.list_by_book(id).await? {
        people_roles_repo.delete(id, person_id, role_id).await?;
    }

    let tags_repo = XBooksTagsRepository::new(&ctx.ctx);
    for tag_id in tags_repo.list_by_book(id).await? {
        tags_repo.delete(id, tag_id).await?;
    }

    let langs_repo = XBookLanguagesRepository::new(&ctx.ctx);
    for (language_id, role_id) in langs_repo.list_by_book(id).await? {
        langs_repo.delete(id, language_id, role_id).await?;
    }

    let repo = BookRepository::new(&ctx.ctx);
    repo.delete(id).await
}
