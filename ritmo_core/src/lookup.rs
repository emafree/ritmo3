use crate::CoreContext;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    BookRepository, ContentRepository, ContentTypeRepository, FormatRepository,
    PublisherRepository, SeriesRepository,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupEntityType {
    Books,
    Contents,
}

impl LookupEntityType {
    pub fn parse(value: &str) -> Result<Self, RitmoErr> {
        match value {
            "books" => Ok(Self::Books),
            "contents" => Ok(Self::Contents),
            _ => Err(RitmoErr::InvalidInput(format!(
                "unknown entity_type: {value}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupKind {
    Publisher,
    Series,
    Format,
    Type,
}

impl LookupKind {
    pub fn parse(value: &str) -> Result<Self, RitmoErr> {
        match value {
            "publisher" => Ok(Self::Publisher),
            "series" => Ok(Self::Series),
            "format" => Ok(Self::Format),
            "type" => Ok(Self::Type),
            _ => Err(RitmoErr::InvalidInput(format!(
                "unknown lookup_kind: {value}"
            ))),
        }
    }

    fn validate_entity(self, entity_type: LookupEntityType) -> Result<(), RitmoErr> {
        match (self, entity_type) {
            (Self::Publisher | Self::Series | Self::Format, LookupEntityType::Books)
            | (Self::Type, LookupEntityType::Contents) => Ok(()),
            _ => Err(RitmoErr::InvalidInput(format!(
                "invalid entity_type for lookup_kind: {:?} on {:?}",
                self, entity_type
            ))),
        }
    }
}

pub async fn set_lookup_by_id(
    ctx: &CoreContext,
    entity_type: &str,
    entity_id: i64,
    lookup_kind: &str,
    lookup_id: i64,
) -> RitmoResult<()> {
    let entity_type = LookupEntityType::parse(entity_type)?;
    let lookup_kind = LookupKind::parse(lookup_kind)?;
    lookup_kind.validate_entity(entity_type)?;

    match lookup_kind {
        LookupKind::Publisher => {
            BookRepository::new(&ctx.ctx)
                .set_publisher_id(entity_id, Some(lookup_id))
                .await
        }
        LookupKind::Series => {
            BookRepository::new(&ctx.ctx)
                .set_series_id(entity_id, Some(lookup_id))
                .await
        }
        LookupKind::Format => {
            BookRepository::new(&ctx.ctx)
                .set_format_id(entity_id, Some(lookup_id))
                .await
        }
        LookupKind::Type => {
            ContentRepository::new(&ctx.ctx)
                .set_type_id(entity_id, Some(lookup_id))
                .await
        }
    }
}

pub async fn set_lookup_by_value(
    ctx: &CoreContext,
    entity_type: &str,
    entity_id: i64,
    lookup_kind: &str,
    lookup_value: &str,
) -> RitmoResult<()> {
    let lookup_value = lookup_value.trim();
    if lookup_value.is_empty() {
        return Err(RitmoErr::InvalidInput("lookup_value required".to_owned()));
    }

    let entity_type = LookupEntityType::parse(entity_type)?;
    let lookup_kind = LookupKind::parse(lookup_kind)?;
    lookup_kind.validate_entity(entity_type)?;

    let lookup_id = match lookup_kind {
        LookupKind::Publisher => {
            PublisherRepository::new(&ctx.ctx)
                .get_or_create(lookup_value)
                .await?
                .id
        }
        LookupKind::Series => {
            SeriesRepository::new(&ctx.ctx)
                .get_or_create(lookup_value)
                .await?
                .id
        }
        LookupKind::Format => {
            FormatRepository::new(&ctx.ctx)
                .get_or_create(lookup_value)
                .await?
                .id
        }
        LookupKind::Type => {
            ContentTypeRepository::new(&ctx.ctx)
                .get_or_create(lookup_value)
                .await?
                .id
        }
    };

    set_lookup_by_id(ctx, entity_type_str(entity_type), entity_id, lookup_kind_str(lookup_kind), lookup_id)
        .await
}

pub async fn clear_lookup(
    ctx: &CoreContext,
    entity_type: &str,
    entity_id: i64,
    lookup_kind: &str,
) -> RitmoResult<()> {
    let entity_type = LookupEntityType::parse(entity_type)?;
    let lookup_kind = LookupKind::parse(lookup_kind)?;
    lookup_kind.validate_entity(entity_type)?;

    match lookup_kind {
        LookupKind::Publisher => {
            BookRepository::new(&ctx.ctx)
                .set_publisher_id(entity_id, None)
                .await
        }
        LookupKind::Series => {
            BookRepository::new(&ctx.ctx)
                .set_series_id(entity_id, None)
                .await
        }
        LookupKind::Format => {
            BookRepository::new(&ctx.ctx)
                .set_format_id(entity_id, None)
                .await
        }
        LookupKind::Type => {
            ContentRepository::new(&ctx.ctx)
                .set_type_id(entity_id, None)
                .await
        }
    }
}

fn entity_type_str(entity_type: LookupEntityType) -> &'static str {
    match entity_type {
        LookupEntityType::Books => "books",
        LookupEntityType::Contents => "contents",
    }
}

fn lookup_kind_str(lookup_kind: LookupKind) -> &'static str {
    match lookup_kind {
        LookupKind::Publisher => "publisher",
        LookupKind::Series => "series",
        LookupKind::Format => "format",
        LookupKind::Type => "type",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{book, content};
    use ritmo_domain::{Book, Content};
    use ritmo_repository::RepositoryContext;

    #[tokio::test]
    async fn set_lookup_by_value_and_clear_update_single_foreign_keys() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx);

        let book_id = book::create(
            &core,
            &Book {
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
            },
        )
        .await
        .unwrap();
        let content_id = content::create(
            &core,
            &Content {
                id: 0,
                title: "Contenuto".to_owned(),
                original_title: None,
                type_id: None,
                publication_year: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        set_lookup_by_value(&core, "books", book_id, "publisher", "  Einaudi ")
            .await
            .unwrap();
        set_lookup_by_value(&core, "contents", content_id, "type", " inline_type ")
            .await
            .unwrap();

        assert_eq!(book::get(&core, book_id).await.unwrap().publisher_id.is_some(), true);
        assert_eq!(content::get(&core, content_id).await.unwrap().type_id.is_some(), true);

        clear_lookup(&core, "books", book_id, "publisher")
            .await
            .unwrap();
        clear_lookup(&core, "contents", content_id, "type")
            .await
            .unwrap();

        assert_eq!(book::get(&core, book_id).await.unwrap().publisher_id, None);
        assert_eq!(content::get(&core, content_id).await.unwrap().type_id, None);
    }

    #[test]
    fn lookup_kind_rejects_invalid_entity_combinations() {
        assert!(LookupKind::Publisher
            .validate_entity(LookupEntityType::Books)
            .is_ok());
        assert!(LookupKind::Type
            .validate_entity(LookupEntityType::Contents)
            .is_ok());
        assert!(LookupKind::Publisher
            .validate_entity(LookupEntityType::Contents)
            .is_err());
        assert!(LookupKind::Type
            .validate_entity(LookupEntityType::Books)
            .is_err());
    }
}
