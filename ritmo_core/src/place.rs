use crate::CoreContext;
use ritmo_domain::{Place, PlaceType};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    PlaceRepository, PlaceTypeRepository, XPersonPlacesRepository, XPublisherPlacesRepository,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceOwner {
    Person(i64),
    Publisher(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedPlace {
    pub place_id: i64,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub circa: bool,
    pub disputed: bool,
    pub place_type_key: String,
}

pub async fn list_types(ctx: &CoreContext) -> RitmoResult<Vec<PlaceType>> {
    PlaceTypeRepository::new(&ctx.ctx).list_all().await
}

pub async fn search(ctx: &CoreContext, query: &str) -> RitmoResult<Vec<Place>> {
    PlaceRepository::new(&ctx.ctx).search(query.trim()).await
}

pub async fn get(ctx: &CoreContext, place_id: i64) -> RitmoResult<Place> {
    PlaceRepository::new(&ctx.ctx).get(place_id).await
}

pub async fn create(ctx: &CoreContext, place: &Place) -> RitmoResult<i64> {
    let place = normalize_place(place)?;
    PlaceRepository::new(&ctx.ctx)
        .save(
            place.continent,
            place.country,
            place.city,
            place.circa,
            place.disputed,
        )
        .await
}

pub async fn update(ctx: &CoreContext, place: &Place) -> RitmoResult<()> {
    let place = normalize_place(place)?;
    PlaceRepository::new(&ctx.ctx)
        .update(
            place.id,
            place.continent,
            place.country,
            place.city,
            place.circa,
            place.disputed,
        )
        .await
}

pub async fn list_linked(ctx: &CoreContext, owner: PlaceOwner) -> RitmoResult<Vec<LinkedPlace>> {
    let relations = match owner {
        PlaceOwner::Person(person_id) => XPersonPlacesRepository::new(&ctx.ctx)
            .list_by_person(person_id)
            .await?,
        PlaceOwner::Publisher(publisher_id) => XPublisherPlacesRepository::new(&ctx.ctx)
            .list_by_publisher(publisher_id)
            .await?,
    };

    let place_repo = PlaceRepository::new(&ctx.ctx);
    let place_types = list_types(ctx).await?;

    let mut items = Vec::with_capacity(relations.len());
    for (place_id, place_type_id) in relations {
        let place = place_repo.get(place_id).await?;
        let place_type_key = place_types
            .iter()
            .find(|place_type| place_type.id == place_type_id)
            .map(|place_type| place_type.name.clone())
            .ok_or_else(|| {
                RitmoErr::DataIntegrity(format!("missing place type for relation {place_type_id}"))
            })?;

        items.push(LinkedPlace {
            place_id: place.id,
            continent: place.continent,
            country: place.country,
            city: place.city,
            circa: place.circa,
            disputed: place.disputed,
            place_type_key,
        });
    }

    Ok(items)
}

pub async fn link(
    ctx: &CoreContext,
    owner: PlaceOwner,
    place_id: i64,
    place_type_key: &str,
) -> RitmoResult<LinkedPlace> {
    let place_type = find_place_type(ctx, place_type_key).await?;

    match owner {
        PlaceOwner::Person(person_id) => {
            XPersonPlacesRepository::new(&ctx.ctx)
                .create(person_id, place_id, place_type.id)
                .await?;
        }
        PlaceOwner::Publisher(publisher_id) => {
            XPublisherPlacesRepository::new(&ctx.ctx)
                .create(publisher_id, place_id, place_type.id)
                .await?;
        }
    }

    let place = get(ctx, place_id).await?;
    Ok(LinkedPlace {
        place_id: place.id,
        continent: place.continent,
        country: place.country,
        city: place.city,
        circa: place.circa,
        disputed: place.disputed,
        place_type_key: place_type.name,
    })
}

pub async fn replace_link_type(
    ctx: &CoreContext,
    owner: PlaceOwner,
    place_id: i64,
    current_place_type_key: &str,
    new_place_type_key: &str,
) -> RitmoResult<()> {
    if current_place_type_key == new_place_type_key {
        return Ok(());
    }

    let current = find_place_type(ctx, current_place_type_key).await?;
    let next = find_place_type(ctx, new_place_type_key).await?;

    match owner {
        PlaceOwner::Person(person_id) => {
            let repo = XPersonPlacesRepository::new(&ctx.ctx);
            repo.delete(person_id, place_id, current.id).await?;
            repo.create(person_id, place_id, next.id).await?;
        }
        PlaceOwner::Publisher(publisher_id) => {
            let repo = XPublisherPlacesRepository::new(&ctx.ctx);
            repo.delete(publisher_id, place_id, current.id).await?;
            repo.create(publisher_id, place_id, next.id).await?;
        }
    }

    Ok(())
}

pub async fn unlink(ctx: &CoreContext, owner: PlaceOwner, place_id: i64) -> RitmoResult<()> {
    match owner {
        PlaceOwner::Person(person_id) => {
            let repo = XPersonPlacesRepository::new(&ctx.ctx);
            for (linked_place_id, place_type_id) in repo.list_by_person(person_id).await? {
                if linked_place_id == place_id {
                    repo.delete(person_id, place_id, place_type_id).await?;
                }
            }
        }
        PlaceOwner::Publisher(publisher_id) => {
            let repo = XPublisherPlacesRepository::new(&ctx.ctx);
            for (linked_place_id, place_type_id) in repo.list_by_publisher(publisher_id).await? {
                if linked_place_id == place_id {
                    repo.delete(publisher_id, place_id, place_type_id).await?;
                }
            }
        }
    }

    if XPersonPlacesRepository::new(&ctx.ctx)
        .list_by_place(place_id)
        .await?
        .is_empty()
        && XPublisherPlacesRepository::new(&ctx.ctx)
            .list_by_place(place_id)
            .await?
            .is_empty()
    {
        PlaceRepository::new(&ctx.ctx).delete(place_id).await?;
    }

    Ok(())
}

fn normalize_place(place: &Place) -> RitmoResult<Place> {
    let continent = normalize_optional(&place.continent);
    let country = normalize_optional(&place.country);
    let city = normalize_optional(&place.city);

    if continent.is_none() && country.is_none() && city.is_none() {
        return Err(RitmoErr::InvalidInput(
            "place must define continent, country, or city".to_string(),
        ));
    }

    Ok(Place {
        id: place.id,
        continent,
        country,
        city,
        circa: place.circa,
        disputed: place.disputed,
    })
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value.as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

async fn find_place_type(ctx: &CoreContext, place_type_key: &str) -> RitmoResult<PlaceType> {
    PlaceTypeRepository::new(&ctx.ctx)
        .list_all()
        .await?
        .into_iter()
        .find(|place_type| place_type.name == place_type_key)
        .ok_or_else(|| RitmoErr::InvalidInput(format!("unknown place type: {place_type_key}")))
}

#[cfg(test)]
mod tests {
    use super::{create, link, list_linked, list_types, search, unlink, update, PlaceOwner};
    use crate::CoreContext;
    use ritmo_domain::{Person, Place, Publisher};
    use ritmo_repository::{PersonRepository, PublisherRepository};

    fn sample_place() -> Place {
        Place {
            id: 0,
            continent: Some(" Europa ".to_owned()),
            country: Some(" Italia ".to_owned()),
            city: Some(" Roma ".to_owned()),
            circa: false,
            disputed: false,
        }
    }

    #[tokio::test]
    async fn create_link_update_and_unlink_place() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = ritmo_repository::RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx.clone());

        let person_id = PersonRepository::new(&repo_ctx)
            .save(&Person {
                id: 0,
                name: "Persona".to_owned(),
                display_name: None,
                given_name: None,
                surname: None,
                middle_names: None,
                title: None,
                suffix: None,
                birth_date: None,
                death_date: None,
                biography: None,
                verified: false,
            })
            .await
            .unwrap();

        let place_id = create(&core, &sample_place()).await.unwrap();
        let linked = link(&core, PlaceOwner::Person(person_id), place_id, "birth")
            .await
            .unwrap();
        assert_eq!(linked.place_type_key, "birth");

        let places = list_linked(&core, PlaceOwner::Person(person_id)).await.unwrap();
        assert_eq!(places.len(), 1);
        assert_eq!(places[0].country.as_deref(), Some("Italia"));

        update(
            &core,
            &Place {
                id: place_id,
                continent: Some("Europa".to_owned()),
                country: Some("Italia".to_owned()),
                city: Some("Milano".to_owned()),
                circa: true,
                disputed: false,
            },
        )
        .await
        .unwrap();

        let matches = search(&core, "mila").await.unwrap();
        assert_eq!(matches.len(), 1);
        assert!(matches[0].circa);

        unlink(&core, PlaceOwner::Person(person_id), place_id)
            .await
            .unwrap();
        assert!(search(&core, "mila").await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn unlink_keeps_shared_places() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = ritmo_repository::RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx.clone());

        let person_id = PersonRepository::new(&repo_ctx)
            .save(&Person {
                id: 0,
                name: "Persona".to_owned(),
                display_name: None,
                given_name: None,
                surname: None,
                middle_names: None,
                title: None,
                suffix: None,
                birth_date: None,
                death_date: None,
                biography: None,
                verified: false,
            })
            .await
            .unwrap();
        let publisher_id = PublisherRepository::new(&repo_ctx)
            .save(&Publisher {
                id: 0,
                name: "Editore".to_owned(),
            })
            .await
            .unwrap();

        let place_id = create(&core, &sample_place()).await.unwrap();
        link(&core, PlaceOwner::Person(person_id), place_id, "birth")
            .await
            .unwrap();
        link(&core, PlaceOwner::Publisher(publisher_id), place_id, "activity")
            .await
            .unwrap();

        unlink(&core, PlaceOwner::Person(person_id), place_id)
            .await
            .unwrap();

        assert_eq!(search(&core, "roma").await.unwrap().len(), 1);
        assert_eq!(list_types(&core).await.unwrap().len(), 5);
    }
}
