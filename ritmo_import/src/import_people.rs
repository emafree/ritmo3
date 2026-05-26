use ritmo_domain::{Alias, PartialDate, Place, PlaceType};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    AliasRepository, LanguageRepository, PersonRepository, PlaceRepository, PlaceTypeRepository,
    RepositoryContext, RoleRepository, XPersonLanguagesRepository, XPersonPlacesRepository,
};

use crate::model::{PartialDateInput, PersonFile, PersonInput};
use crate::reporter::CliReporter;
use ritmo_errors::reporter::RitmoReporter;

fn to_partial_date(input: &PartialDateInput) -> PartialDate {
    PartialDate {
        year: input.year,
        month: input.month,
        day: input.day,
        circa: input.circa,
    }
}

pub async fn import_people_file(
    repo_ctx: &RepositoryContext,
    file: &PersonFile,
    reporter: &mut CliReporter,
) -> RitmoResult<()> {
    for person_input in &file.person {
        import_person(repo_ctx, person_input, reporter).await?;
    }
    Ok(())
}

async fn import_person(
    repo_ctx: &RepositoryContext,
    input: &PersonInput,
    reporter: &mut CliReporter,
) -> RitmoResult<()> {
    let person_repo = PersonRepository::new(repo_ctx);

    // 1. get_or_create the person (key: name)
    let mut person = person_repo.get_or_create(&input.name).await?;

    // 2. Update empty biographical fields if the person already existed with empty fields.
    // We only fill in fields that are currently None, never overwriting existing data.
    let mut needs_update = false;

    if person.display_name.is_none() {
        if let Some(v) = &input.display_name {
            person.display_name = Some(v.clone());
            needs_update = true;
        }
    }
    if person.given_name.is_none() {
        if let Some(v) = &input.given_name {
            person.given_name = Some(v.clone());
            needs_update = true;
        }
    }
    if person.surname.is_none() {
        if let Some(v) = &input.surname {
            person.surname = Some(v.clone());
            needs_update = true;
        }
    }
    if person.middle_names.is_none() {
        if let Some(v) = &input.middle_names {
            person.middle_names = Some(v.clone());
            needs_update = true;
        }
    }
    if person.title.is_none() {
        if let Some(v) = &input.title {
            person.title = Some(v.clone());
            needs_update = true;
        }
    }
    if person.suffix.is_none() {
        if let Some(v) = &input.suffix {
            person.suffix = Some(v.clone());
            needs_update = true;
        }
    }
    if person.birth_date.is_none() {
        if let Some(v) = &input.birth_date {
            person.birth_date = Some(to_partial_date(v));
            needs_update = true;
        }
    }
    if person.death_date.is_none() {
        if let Some(v) = &input.death_date {
            person.death_date = Some(to_partial_date(v));
            needs_update = true;
        }
    }
    if person.biography.is_none() {
        if let Some(v) = &input.biography {
            person.biography = Some(v.clone());
            needs_update = true;
        }
    }

    if needs_update {
        person_repo.update(&person).await?;
    }

    // 3. Add aliases (idempotent via INSERT OR IGNORE in the repository)
    let alias_repo = AliasRepository::new(repo_ctx);
    for alias_name in &input.aliases {
        let alias = Alias {
            id: 0,
            alternative_name: alias_name.clone(),
            person_id: person.id,
        };
        // Ignore duplicate errors silently
        let _ = alias_repo.save(&alias).await;
    }

    // 4. Add languages, resolving role by name
    let lang_repo = LanguageRepository::new(repo_ctx);
    let role_repo = RoleRepository::new(repo_ctx);
    let lang_link_repo = XPersonLanguagesRepository::new(repo_ctx);
    for lang_input in &input.language {
        let (field, value) = if let Some(v) = &lang_input.iso2 {
            ("iso_code_2char", v.as_str())
        } else if let Some(v) = &lang_input.iso3 {
            ("iso_code_3char", v.as_str())
        } else if let Some(v) = &lang_input.name {
            ("official_name", v.as_str())
        } else {
            return Err(RitmoErr::InvalidInput(
                "language: nessun campo specificato".into(),
            ));
        };
        let language = lang_repo.get_or_create_by_field(field, value).await?;
        let role = role_repo.get_or_create(&lang_input.role).await?;
        // Ignore duplicate relation errors silently
        let _ = lang_link_repo
            .create(person.id, language.id, role.id)
            .await;
    }

    // 5. Add places, resolving place type by name.
    // ritmo_core does not expose a person-place linking function, so we use the
    // repository layer directly here.
    let place_repo = PlaceRepository::new(repo_ctx);
    let place_type_repo = PlaceTypeRepository::new(repo_ctx);
    let person_places_repo = XPersonPlacesRepository::new(repo_ctx);
    for place_input in &input.place {
        // Resolve or create the place type by name
        let all_types = place_type_repo.list_all().await?;
        let place_type = if let Some(pt) = all_types
            .into_iter()
            .find(|pt| pt.name == place_input.place_type)
        {
            pt
        } else {
            let new_pt = PlaceType {
                id: 0,
                name: place_input.place_type.clone(),
            };
            let id = place_type_repo.save(&new_pt).await?;
            place_type_repo.get(id).await?
        };

        // Create the place record
        let place = Place {
            id: 0,
            continent: place_input.continent.clone(),
            country: place_input.country.clone(),
            city: place_input.city.clone(),
            circa: place_input.circa,
            disputed: place_input.disputed,
        };
        let place_id = place_repo.save(&place).await?;

        // Link the place to the person (idempotent via INSERT OR IGNORE)
        let _ = person_places_repo
            .create(person.id, place_id, place_type.id)
            .await;
    }

    reporter.progress(&format!("→ {} ... ok", input.name));
    Ok(())
}
