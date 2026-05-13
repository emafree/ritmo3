use crate::support::{map_query, partial_date_from_parts};
use ritmo_domain::filter::{
    Filter, FilterField, FilterOperator, FilterSet, FilterValue, LogicalOperator,
};
use ritmo_domain::{Book, PartialDate};
use ritmo_errors::RitmoResult;
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};

pub async fn search_books(pool: &SqlitePool, filter_sets: &[FilterSet]) -> RitmoResult<Vec<Book>> {
    let active_sets: Vec<&FilterSet> = filter_sets
        .iter()
        .filter(|filter_set| filter_set.active)
        .collect();

    let mut builder = QueryBuilder::<Sqlite>::new(
        "SELECT books.id, books.name, books.isbn, books.publication_date_year, books.publication_date_month, books.publication_date_day, books.publication_date_circa, books.notes FROM books",
    );

    if !active_sets.is_empty() {
        builder.push(" WHERE ");
        for (set_index, filter_set) in active_sets.iter().enumerate() {
            if set_index > 0 {
                builder.push(" AND ");
            }
            push_filter_set_condition(&mut builder, filter_set);
        }
    }

    builder.push(" ORDER BY books.name");

    let rows = builder.build().fetch_all(pool).await.map_err(map_query)?;

    Ok(rows
        .into_iter()
        .map(|row| Book {
            id: row.get("id"),
            title: row.get("name"),
            isbn: row.get("isbn"),
            publication_year: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            notes: row.get("notes"),
        })
        .collect())
}

fn push_filter_set_condition(builder: &mut QueryBuilder<'_, Sqlite>, filter_set: &FilterSet) {
    if filter_set.filters.is_empty() {
        builder.push("(1=1)");
        return;
    }

    builder.push("(");
    for (index, filter) in filter_set.filters.iter().enumerate() {
        if index > 0 {
            match filter_set.operator {
                LogicalOperator::And => builder.push(" AND "),
                LogicalOperator::Or => builder.push(" OR "),
            };
        }
        push_filter_condition(builder, filter);
    }
    builder.push(")");
}

fn push_filter_condition(builder: &mut QueryBuilder<'_, Sqlite>, filter: &Filter) {
    match &filter.field {
        FilterField::BookTitle => push_text_filter(builder, "books.name", filter),
        FilterField::BookIsbn => push_text_filter(builder, "books.isbn", filter),
        FilterField::BookFormat => {
            push_relation_filter(builder, "books.format_id", "formats", "id", "key", filter)
        }
        FilterField::BookSeries => {
            push_relation_filter(builder, "books.series_id", "series", "id", "name", filter)
        }
        FilterField::BookPublisher => push_relation_filter(
            builder,
            "books.publisher_id",
            "publishers",
            "id",
            "name",
            filter,
        ),
        FilterField::BookPublicationDate => push_date_filter(
            builder,
            "books.publication_date_year",
            "books.publication_date_month",
            "books.publication_date_day",
            filter,
        ),
        FilterField::BookTag => push_exists_relation_filter(
            builder,
            "x_books_tags",
            "book_id",
            "tag_id",
            Some(("tags", "id", "name")),
            filter,
            None,
        ),
        FilterField::BookLanguage { role_id } => push_exists_relation_filter(
            builder,
            "book_languages",
            "book_id",
            "language_id",
            Some(("languages", "id", "official_name")),
            filter,
            role_id.map(|id| ("role_id", id)),
        ),
        FilterField::ContentTitle => push_content_text_filter(builder, "contents.name", filter),
        FilterField::ContentGenre => push_content_relation_filter(
            builder,
            "contents.genre_id",
            "genres",
            "id",
            "key",
            filter,
        ),
        FilterField::ContentPublicationDate => push_content_date_filter(
            builder,
            "contents.publication_date_year",
            "contents.publication_date_month",
            "contents.publication_date_day",
            filter,
        ),
        FilterField::ContentTag => push_content_exists_relation_filter(
            builder,
            "x_contents_tags",
            "content_id",
            "tag_id",
            Some(("tags", "id", "name")),
            filter,
            None,
        ),
        FilterField::ContentLanguage { role_id } => push_content_exists_relation_filter(
            builder,
            "content_languages",
            "content_id",
            "language_id",
            Some(("languages", "id", "official_name")),
            filter,
            role_id.map(|id| ("role_id", id)),
        ),
        FilterField::PersonName => push_person_name_filter(builder, filter),
        FilterField::PersonRole { role_id } => push_person_role_filter(builder, *role_id, filter),
        FilterField::PersonLanguage { role_id } => {
            push_person_language_filter(builder, *role_id, filter)
        }
        FilterField::PersonPlace { place_type_id } => {
            push_person_place_filter(builder, *place_type_id, filter)
        }
        FilterField::PersonBirthDate => push_person_date_filter(builder, "birth", filter),
        FilterField::PersonDeathDate => push_person_date_filter(builder, "death", filter),
    }
}

fn push_text_filter(builder: &mut QueryBuilder<'_, Sqlite>, column: &str, filter: &Filter) {
    match filter.operator {
        FilterOperator::Contains => {
            let values = filter_text_values(&filter.values);
            if values.is_empty() {
                push_false(builder);
                return;
            }

            builder.push("(");
            for (index, value) in values.into_iter().enumerate() {
                if index > 0 {
                    builder.push(" OR ");
                }
                builder
                    .push(column)
                    .push(" LIKE ")
                    .push_bind(format!("%{value}%"));
                builder.push(" COLLATE NOCASE");
            }
            builder.push(")");
        }
        FilterOperator::Equals => {
            let values = filter_text_values(&filter.values);
            if values.is_empty() {
                push_false(builder);
                return;
            }

            if values.len() == 1 {
                builder
                    .push(column)
                    .push(" = ")
                    .push_bind(values[0].clone());
                builder.push(" COLLATE NOCASE");
            } else {
                builder.push("(");
                for (index, value) in values.into_iter().enumerate() {
                    if index > 0 {
                        builder.push(" OR ");
                    }
                    builder.push(column).push(" = ").push_bind(value);
                    builder.push(" COLLATE NOCASE");
                }
                builder.push(")");
            }
        }
        _ => push_false(builder),
    }
}

fn push_relation_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    scalar_column: &str,
    relation_table: &str,
    relation_id_column: &str,
    relation_text_column: &str,
    filter: &Filter,
) {
    let ids = filter_id_values(&filter.values);
    let texts = filter_text_values(&filter.values);

    match filter.operator {
        FilterOperator::Equals => {
            if !ids.is_empty() {
                push_id_list_or_single(builder, scalar_column, &ids);
            } else if !texts.is_empty() {
                builder.push("EXISTS (SELECT 1 FROM ");
                builder.push(relation_table);
                builder.push(" WHERE ");
                builder.push(relation_table);
                builder.push(".");
                builder.push(relation_id_column);
                builder.push(" = ");
                builder.push(scalar_column);
                builder.push(" AND (");
                for (index, text) in texts.into_iter().enumerate() {
                    if index > 0 {
                        builder.push(" OR ");
                    }
                    builder.push(relation_table);
                    builder.push(".");
                    builder.push(relation_text_column);
                    builder.push(" = ");
                    builder.push_bind(text);
                    builder.push(" COLLATE NOCASE");
                }
                builder.push("))");
            } else {
                push_false(builder);
            }
        }
        FilterOperator::Contains => {
            if texts.is_empty() {
                push_false(builder);
                return;
            }

            builder.push("EXISTS (SELECT 1 FROM ");
            builder.push(relation_table);
            builder.push(" WHERE ");
            builder.push(relation_table);
            builder.push(".");
            builder.push(relation_id_column);
            builder.push(" = ");
            builder.push(scalar_column);
            builder.push(" AND (");
            for (index, text) in texts.into_iter().enumerate() {
                if index > 0 {
                    builder.push(" OR ");
                }
                builder.push(relation_table);
                builder.push(".");
                builder.push(relation_text_column);
                builder.push(" LIKE ");
                builder.push_bind(format!("%{text}%"));
                builder.push(" COLLATE NOCASE");
            }
            builder.push("))");
        }
        _ => push_false(builder),
    }
}

fn push_exists_relation_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    join_table: &str,
    owner_column: &str,
    foreign_key_column: &str,
    linked_table: Option<(&str, &str, &str)>,
    filter: &Filter,
    extra_condition: Option<(&str, i64)>,
) {
    let ids = filter_id_values(&filter.values);
    let texts = filter_text_values(&filter.values);

    builder.push("EXISTS (SELECT 1 FROM ");
    builder.push(join_table);

    if let Some((linked_table_name, linked_id_column, _)) = linked_table {
        builder.push(" JOIN ");
        builder.push(linked_table_name);
        builder.push(" ON ");
        builder.push(linked_table_name);
        builder.push(".");
        builder.push(linked_id_column);
        builder.push(" = ");
        builder.push(join_table);
        builder.push(".");
        builder.push(foreign_key_column);
    }

    builder.push(" WHERE ");
    builder.push(join_table);
    builder.push(".");
    builder.push(owner_column);
    builder.push(" = books.id");

    if let Some((extra_column, extra_value)) = extra_condition {
        builder.push(" AND ");
        builder.push(join_table);
        builder.push(".");
        builder.push(extra_column);
        builder.push(" = ");
        builder.push_bind(extra_value);
    }

    match filter.operator {
        FilterOperator::Equals => {
            if !ids.is_empty() {
                builder.push(" AND ");
                push_id_list_or_single_prefixed(builder, join_table, foreign_key_column, &ids);
            } else if !texts.is_empty() {
                if let Some((linked_table_name, _, linked_text_column)) = linked_table {
                    builder.push(" AND (");
                    for (index, text) in texts.into_iter().enumerate() {
                        if index > 0 {
                            builder.push(" OR ");
                        }
                        builder.push(linked_table_name);
                        builder.push(".");
                        builder.push(linked_text_column);
                        builder.push(" = ");
                        builder.push_bind(text);
                        builder.push(" COLLATE NOCASE");
                    }
                    builder.push(")");
                } else {
                    builder.push(" AND 1=0");
                }
            } else {
                builder.push(" AND 1=0");
            }
        }
        FilterOperator::Contains => {
            if texts.is_empty() {
                builder.push(" AND 1=0");
            } else if let Some((linked_table_name, _, linked_text_column)) = linked_table {
                builder.push(" AND (");
                for (index, text) in texts.into_iter().enumerate() {
                    if index > 0 {
                        builder.push(" OR ");
                    }
                    builder.push(linked_table_name);
                    builder.push(".");
                    builder.push(linked_text_column);
                    builder.push(" LIKE ");
                    builder.push_bind(format!("%{text}%"));
                    builder.push(" COLLATE NOCASE");
                }
                builder.push(")");
            } else {
                builder.push(" AND 1=0");
            }
        }
        _ => {
            builder.push(" AND 1=0");
        }
    }

    builder.push(")");
}

fn push_content_text_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    content_column: &str,
    filter: &Filter,
) {
    builder.push("EXISTS (SELECT 1 FROM x_books_contents book_content JOIN contents ON contents.id = book_content.content_id WHERE book_content.book_id = books.id AND ");
    push_text_filter(builder, content_column, filter);
    builder.push(")");
}

fn push_content_relation_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    scalar_column: &str,
    relation_table: &str,
    relation_id_column: &str,
    relation_text_column: &str,
    filter: &Filter,
) {
    builder.push("EXISTS (SELECT 1 FROM x_books_contents book_content JOIN contents ON contents.id = book_content.content_id WHERE book_content.book_id = books.id AND ");
    push_relation_filter(
        builder,
        scalar_column,
        relation_table,
        relation_id_column,
        relation_text_column,
        filter,
    );
    builder.push(")");
}

fn push_content_date_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    year_column: &str,
    month_column: &str,
    day_column: &str,
    filter: &Filter,
) {
    builder.push("EXISTS (SELECT 1 FROM x_books_contents book_content JOIN contents ON contents.id = book_content.content_id WHERE book_content.book_id = books.id AND ");
    push_date_filter(builder, year_column, month_column, day_column, filter);
    builder.push(")");
}

fn push_content_exists_relation_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    relation_table: &str,
    owner_column: &str,
    foreign_key_column: &str,
    linked_table: Option<(&str, &str, &str)>,
    filter: &Filter,
    extra_condition: Option<(&str, i64)>,
) {
    let ids = filter_id_values(&filter.values);
    let texts = filter_text_values(&filter.values);

    builder.push("EXISTS (SELECT 1 FROM x_books_contents book_content WHERE book_content.book_id = books.id AND EXISTS (SELECT 1 FROM ");
    builder.push(relation_table);

    if let Some((linked_table_name, linked_id_column, _)) = linked_table {
        builder.push(" JOIN ");
        builder.push(linked_table_name);
        builder.push(" ON ");
        builder.push(linked_table_name);
        builder.push(".");
        builder.push(linked_id_column);
        builder.push(" = ");
        builder.push(relation_table);
        builder.push(".");
        builder.push(foreign_key_column);
    }

    builder.push(" WHERE ");
    builder.push(relation_table);
    builder.push(".");
    builder.push(owner_column);
    builder.push(" = book_content.content_id");

    if let Some((extra_column, extra_value)) = extra_condition {
        builder.push(" AND ");
        builder.push(relation_table);
        builder.push(".");
        builder.push(extra_column);
        builder.push(" = ");
        builder.push_bind(extra_value);
    }

    match filter.operator {
        FilterOperator::Equals => {
            if !ids.is_empty() {
                builder.push(" AND ");
                push_id_list_or_single_prefixed(builder, relation_table, foreign_key_column, &ids);
            } else if !texts.is_empty() {
                if let Some((linked_table_name, _, linked_text_column)) = linked_table {
                    builder.push(" AND (");
                    for (index, text) in texts.into_iter().enumerate() {
                        if index > 0 {
                            builder.push(" OR ");
                        }
                        builder.push(linked_table_name);
                        builder.push(".");
                        builder.push(linked_text_column);
                        builder.push(" = ");
                        builder.push_bind(text);
                        builder.push(" COLLATE NOCASE");
                    }
                    builder.push(")");
                } else {
                    builder.push(" AND 1=0");
                }
            } else {
                builder.push(" AND 1=0");
            }
        }
        FilterOperator::Contains => {
            if texts.is_empty() {
                builder.push(" AND 1=0");
            } else if let Some((linked_table_name, _, linked_text_column)) = linked_table {
                builder.push(" AND (");
                for (index, text) in texts.into_iter().enumerate() {
                    if index > 0 {
                        builder.push(" OR ");
                    }
                    builder.push(linked_table_name);
                    builder.push(".");
                    builder.push(linked_text_column);
                    builder.push(" LIKE ");
                    builder.push_bind(format!("%{text}%"));
                    builder.push(" COLLATE NOCASE");
                }
                builder.push(")");
            } else {
                builder.push(" AND 1=0");
            }
        }
        _ => {
            builder.push(" AND 1=0");
        }
    }

    builder.push(")))");
}

fn push_person_name_filter(builder: &mut QueryBuilder<'_, Sqlite>, filter: &Filter) {
    let texts = filter_text_values(&filter.values);
    match filter.operator {
        FilterOperator::Contains | FilterOperator::Equals => {
            if texts.is_empty() {
                push_false(builder);
                return;
            }

            builder.push("EXISTS (SELECT 1 FROM x_books_people_roles bpr JOIN people p ON p.id = bpr.person_id WHERE bpr.book_id = books.id AND (");
            for (index, text) in texts.iter().enumerate() {
                if index > 0 {
                    builder.push(" OR ");
                }
                builder.push("p.name");
                if matches!(filter.operator, FilterOperator::Contains) {
                    builder.push(" LIKE ");
                    builder.push_bind(format!("%{text}%"));
                } else {
                    builder.push(" = ");
                    builder.push_bind(text.clone());
                }
                builder.push(" COLLATE NOCASE");
            }
            builder.push(")) OR EXISTS (SELECT 1 FROM x_books_contents bc JOIN x_contents_people_roles cpr ON cpr.content_id = bc.content_id JOIN people p ON p.id = cpr.person_id WHERE bc.book_id = books.id AND (");
            for (index, text) in texts.into_iter().enumerate() {
                if index > 0 {
                    builder.push(" OR ");
                }
                builder.push("p.name");
                if matches!(filter.operator, FilterOperator::Contains) {
                    builder.push(" LIKE ");
                    builder.push_bind(format!("%{text}%"));
                } else {
                    builder.push(" = ");
                    builder.push_bind(text);
                }
                builder.push(" COLLATE NOCASE");
            }
            builder.push(")))");
        }
        _ => push_false(builder),
    }
}

fn push_person_role_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    role_id: Option<i64>,
    filter: &Filter,
) {
    let role_ids = if let Some(single_role) = role_id {
        vec![single_role]
    } else {
        filter_id_values(&filter.values)
    };

    if role_ids.is_empty() {
        push_false(builder);
        return;
    }

    builder
        .push("(EXISTS (SELECT 1 FROM x_books_people_roles bpr WHERE bpr.book_id = books.id AND ");
    push_id_list_or_single_prefixed(builder, "bpr", "role_id", &role_ids);
    builder.push(") OR EXISTS (SELECT 1 FROM x_books_contents bc JOIN x_contents_people_roles cpr ON cpr.content_id = bc.content_id WHERE bc.book_id = books.id AND ");
    push_id_list_or_single_prefixed(builder, "cpr", "role_id", &role_ids);
    builder.push("))");
}

fn push_person_language_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    role_id: Option<i64>,
    filter: &Filter,
) {
    let language_ids = filter_id_values(&filter.values);
    if language_ids.is_empty() {
        push_false(builder);
        return;
    }

    builder.push("(EXISTS (SELECT 1 FROM x_books_people_roles bpr JOIN person_languages pl ON pl.person_id = bpr.person_id WHERE bpr.book_id = books.id");
    if let Some(value) = role_id {
        builder.push(" AND pl.role_id = ");
        builder.push_bind(value);
    }
    builder.push(" AND ");
    push_id_list_or_single_prefixed(builder, "pl", "language_id", &language_ids);
    builder.push(") OR EXISTS (SELECT 1 FROM x_books_contents bc JOIN x_contents_people_roles cpr ON cpr.content_id = bc.content_id JOIN person_languages pl ON pl.person_id = cpr.person_id WHERE bc.book_id = books.id");
    if let Some(value) = role_id {
        builder.push(" AND pl.role_id = ");
        builder.push_bind(value);
    }
    builder.push(" AND ");
    push_id_list_or_single_prefixed(builder, "pl", "language_id", &language_ids);
    builder.push("))");
}

fn push_person_place_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    place_type_id: Option<i64>,
    filter: &Filter,
) {
    let place_ids = filter_id_values(&filter.values);
    let place_texts = filter_text_values(&filter.values);

    builder.push("(EXISTS (SELECT 1 FROM x_books_people_roles bpr JOIN x_person_places pp ON pp.person_id = bpr.person_id JOIN d_places dp ON dp.id = pp.place_id WHERE bpr.book_id = books.id");
    if let Some(value) = place_type_id {
        builder.push(" AND pp.place_type_id = ");
        builder.push_bind(value);
    }
    push_person_place_value_condition(builder, filter.operator.clone(), &place_ids, &place_texts);
    builder.push(") OR EXISTS (SELECT 1 FROM x_books_contents bc JOIN x_contents_people_roles cpr ON cpr.content_id = bc.content_id JOIN x_person_places pp ON pp.person_id = cpr.person_id JOIN d_places dp ON dp.id = pp.place_id WHERE bc.book_id = books.id");
    if let Some(value) = place_type_id {
        builder.push(" AND pp.place_type_id = ");
        builder.push_bind(value);
    }
    push_person_place_value_condition(builder, filter.operator.clone(), &place_ids, &place_texts);
    builder.push("))");
}

fn push_person_place_value_condition(
    builder: &mut QueryBuilder<'_, Sqlite>,
    operator: FilterOperator,
    ids: &[i64],
    texts: &[String],
) {
    match operator {
        FilterOperator::Equals => {
            if !ids.is_empty() {
                builder.push(" AND ");
                push_id_list_or_single_prefixed(builder, "dp", "id", ids);
            } else if !texts.is_empty() {
                builder.push(" AND (");
                for (index, text) in texts.iter().enumerate() {
                    if index > 0 {
                        builder.push(" OR ");
                    }
                    builder.push("dp.city = ");
                    builder.push_bind(text.clone());
                    builder.push(" COLLATE NOCASE OR dp.country = ");
                    builder.push_bind(text.clone());
                    builder.push(" COLLATE NOCASE");
                }
                builder.push(")");
            } else {
                builder.push(" AND 1=0");
            }
        }
        FilterOperator::Contains => {
            if texts.is_empty() {
                builder.push(" AND 1=0");
            } else {
                builder.push(" AND (");
                for (index, text) in texts.iter().enumerate() {
                    if index > 0 {
                        builder.push(" OR ");
                    }
                    builder.push("dp.city LIKE ");
                    builder.push_bind(format!("%{text}%"));
                    builder.push(" COLLATE NOCASE OR dp.country LIKE ");
                    builder.push_bind(format!("%{text}%"));
                    builder.push(" COLLATE NOCASE");
                }
                builder.push(")");
            }
        }
        _ => {
            builder.push(" AND 1=0");
        }
    }
}

fn push_person_date_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    date_prefix: &str,
    filter: &Filter,
) {
    builder.push("(EXISTS (SELECT 1 FROM x_books_people_roles bpr JOIN people p ON p.id = bpr.person_id WHERE bpr.book_id = books.id AND ");
    push_date_filter(
        builder,
        &format!("p.{date_prefix}_date_year"),
        &format!("p.{date_prefix}_date_month"),
        &format!("p.{date_prefix}_date_day"),
        filter,
    );
    builder.push(") OR EXISTS (SELECT 1 FROM x_books_contents bc JOIN x_contents_people_roles cpr ON cpr.content_id = bc.content_id JOIN people p ON p.id = cpr.person_id WHERE bc.book_id = books.id AND ");
    push_date_filter(
        builder,
        &format!("p.{date_prefix}_date_year"),
        &format!("p.{date_prefix}_date_month"),
        &format!("p.{date_prefix}_date_day"),
        filter,
    );
    builder.push("))");
}

fn push_date_filter(
    builder: &mut QueryBuilder<'_, Sqlite>,
    year_column: &str,
    month_column: &str,
    day_column: &str,
    filter: &Filter,
) {
    let date_expr = format!(
        "(COALESCE({year_column}, 0) * 10000 + COALESCE({month_column}, 0) * 100 + COALESCE({day_column}, 0))"
    );

    match filter.operator {
        FilterOperator::Equals => {
            if let Some(date) = first_date_value(&filter.values) {
                builder
                    .push(&date_expr)
                    .push(" = ")
                    .push_bind(partial_date_to_sortable(date));
            } else {
                push_false(builder);
            }
        }
        FilterOperator::Before => {
            if let Some(date) = first_date_value(&filter.values) {
                builder
                    .push(&date_expr)
                    .push(" < ")
                    .push_bind(partial_date_to_sortable(date));
            } else {
                push_false(builder);
            }
        }
        FilterOperator::After => {
            if let Some(date) = first_date_value(&filter.values) {
                builder
                    .push(&date_expr)
                    .push(" > ")
                    .push_bind(partial_date_to_sortable(date));
            } else {
                push_false(builder);
            }
        }
        FilterOperator::Between => {
            if let Some((start, end)) = first_date_range_value(&filter.values) {
                let start_value = partial_date_to_sortable(start);
                let end_value = partial_date_to_sortable(end);
                builder
                    .push(&date_expr)
                    .push(" BETWEEN ")
                    .push_bind(start_value)
                    .push(" AND ")
                    .push_bind(end_value);
            } else {
                push_false(builder);
            }
        }
        FilterOperator::Contains => push_false(builder),
    }
}

fn filter_text_values(values: &[FilterValue]) -> Vec<String> {
    values
        .iter()
        .filter_map(|value| match value {
            FilterValue::Text(text) => Some(text.clone()),
            _ => None,
        })
        .collect()
}

fn filter_id_values(values: &[FilterValue]) -> Vec<i64> {
    values
        .iter()
        .filter_map(|value| match value {
            FilterValue::Id(id) => Some(*id),
            _ => None,
        })
        .collect()
}

fn first_date_value(values: &[FilterValue]) -> Option<&PartialDate> {
    values.iter().find_map(|value| match value {
        FilterValue::Date(date) => Some(date),
        _ => None,
    })
}

fn first_date_range_value(values: &[FilterValue]) -> Option<(&PartialDate, &PartialDate)> {
    values.iter().find_map(|value| match value {
        FilterValue::DateRange(start, end) => Some((start, end)),
        _ => None,
    })
}

fn partial_date_to_sortable(date: &PartialDate) -> i32 {
    let year = date.year.unwrap_or_default();
    let month = i32::from(date.month.unwrap_or_default());
    let day = i32::from(date.day.unwrap_or_default());
    (year * 10000) + (month * 100) + day
}

fn push_id_list_or_single(builder: &mut QueryBuilder<'_, Sqlite>, column: &str, ids: &[i64]) {
    if ids.len() == 1 {
        builder.push(column).push(" = ").push_bind(ids[0]);
        return;
    }

    builder.push(column).push(" IN (");
    let mut separated = builder.separated(", ");
    for id in ids {
        separated.push_bind(*id);
    }
    separated.push_unseparated(")");
}

fn push_id_list_or_single_prefixed(
    builder: &mut QueryBuilder<'_, Sqlite>,
    table_prefix: &str,
    column: &str,
    ids: &[i64],
) {
    let prefixed_column = format!("{table_prefix}.{column}");
    push_id_list_or_single(builder, &prefixed_column, ids);
}

fn push_false(builder: &mut QueryBuilder<'_, Sqlite>) {
    builder.push("1=0");
}
