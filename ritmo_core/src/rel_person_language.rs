use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XPersonLanguagesRepository;

pub async fn link(
    ctx: &CoreContext,
    person_id: i64,
    language_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XPersonLanguagesRepository::new(&ctx.ctx);
    repo.create(person_id, language_id, role_id).await
}

pub async fn unlink(
    ctx: &CoreContext,
    person_id: i64,
    language_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XPersonLanguagesRepository::new(&ctx.ctx);
    repo.delete(person_id, language_id, role_id).await
}
