use crate::CoreContext;
use ritmo_domain::Book;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    BookRepository, XBookLanguagesRepository, XBooksContentsRepository,
    XBooksPeopleRolesRepository, XBooksTagsRepository,
};

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
