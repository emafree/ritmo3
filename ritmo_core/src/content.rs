use crate::CoreContext;
use ritmo_domain::Content;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    ContentRepository, XBooksContentsRepository, XContentLanguagesRepository,
    XContentsPeopleRolesRepository, XContentsTagsRepository,
};

pub async fn create(ctx: &CoreContext, item: &Content) -> RitmoResult<i64> {
    if item.title.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("title cannot be empty".to_string()));
    }
    let repo = ContentRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Content) -> RitmoResult<()> {
    if item.title.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("title cannot be empty".to_string()));
    }
    let repo = ContentRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let books_repo = XBooksContentsRepository::new(&ctx.ctx);
    for book_id in books_repo.list_by_content(id).await? {
        books_repo.delete(book_id, id).await?;
    }

    let people_roles_repo = XContentsPeopleRolesRepository::new(&ctx.ctx);
    for (person_id, role_id) in people_roles_repo.list_by_content(id).await? {
        people_roles_repo.delete(id, person_id, role_id).await?;
    }

    let tags_repo = XContentsTagsRepository::new(&ctx.ctx);
    for tag_id in tags_repo.list_by_content(id).await? {
        tags_repo.delete(id, tag_id).await?;
    }

    let langs_repo = XContentLanguagesRepository::new(&ctx.ctx);
    for (language_id, role_id) in langs_repo.list_by_content(id).await? {
        langs_repo.delete(id, language_id, role_id).await?;
    }

    let repo = ContentRepository::new(&ctx.ctx);
    repo.delete(id).await
}
