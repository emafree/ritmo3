use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XBooksPeopleRolesRepository;

pub async fn link(
    ctx: &CoreContext,
    book_id: i64,
    person_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XBooksPeopleRolesRepository::new(&ctx.ctx);
    repo.create(book_id, person_id, role_id).await
}

pub async fn unlink(
    ctx: &CoreContext,
    book_id: i64,
    person_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XBooksPeopleRolesRepository::new(&ctx.ctx);
    repo.delete(book_id, person_id, role_id).await
}
