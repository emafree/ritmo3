use crate::CoreContext;
use ritmo_domain::{Content, Language, Person, Role, Tag};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    ContentRepository, LanguageRepository, PersonRepository, RoleRepository, TagRepository,
    XBooksContentsRepository, XContentLanguagesRepository, XContentsPeopleRolesRepository,
    XContentsTagsRepository,
};

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Content>> {
    ContentRepository::new(&ctx.ctx).list_all().await
}

pub async fn get(ctx: &CoreContext, id: i64) -> RitmoResult<Content> {
    ContentRepository::new(&ctx.ctx).get(id).await
}

pub async fn list_people_with_roles(
    ctx: &CoreContext,
    content_id: i64,
) -> RitmoResult<Vec<(Person, Role)>> {
    let pr_repo = XContentsPeopleRolesRepository::new(&ctx.ctx);
    let person_repo = PersonRepository::new(&ctx.ctx);
    let role_repo = RoleRepository::new(&ctx.ctx);

    let pairs = pr_repo.list_by_content(content_id).await?;
    let mut result = Vec::new();
    for (person_id, role_id) in pairs {
        let person = person_repo.get(person_id).await?;
        let role = role_repo.get(role_id).await?;
        result.push((person, role));
    }
    Ok(result)
}

pub async fn list_tags(ctx: &CoreContext, content_id: i64) -> RitmoResult<Vec<Tag>> {
    let tags_rel_repo = XContentsTagsRepository::new(&ctx.ctx);
    let tag_repo = TagRepository::new(&ctx.ctx);

    let tag_ids = tags_rel_repo.list_by_content(content_id).await?;
    let mut tags = Vec::new();
    for tag_id in tag_ids {
        tags.push(tag_repo.get(tag_id).await?);
    }
    Ok(tags)
}

pub async fn list_languages(
    ctx: &CoreContext,
    content_id: i64,
) -> RitmoResult<Vec<Language>> {
    let lang_rel_repo = XContentLanguagesRepository::new(&ctx.ctx);
    let lang_repo = LanguageRepository::new(&ctx.ctx);

    let pairs = lang_rel_repo.list_by_content(content_id).await?;
    let mut languages = Vec::new();
    for (language_id, _role_id) in pairs {
        languages.push(lang_repo.get(language_id).await?);
    }
    Ok(languages)
}

pub async fn get_genre_name(ctx: &CoreContext, content_id: i64) -> RitmoResult<Option<String>> {
    ContentRepository::new(&ctx.ctx).get_genre_name(content_id).await
}

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
