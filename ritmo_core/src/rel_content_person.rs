use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XContentsPeopleRolesRepository;

pub async fn link(
    ctx: &CoreContext,
    content_id: i64,
    person_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XContentsPeopleRolesRepository::new(&ctx.ctx);
    repo.create(content_id, person_id, role_id).await
}

pub async fn unlink(
    ctx: &CoreContext,
    content_id: i64,
    person_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XContentsPeopleRolesRepository::new(&ctx.ctx);
    repo.delete(content_id, person_id, role_id).await
}
