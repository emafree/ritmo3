use crate::support::{map_query, partial_date_from_parts};
use ritmo_domain::filter::{
    Filter, FilterField, FilterOperator, FilterSet, FilterValue, LogicalOperator,
};
use ritmo_domain::{Content, PartialDate};
use ritmo_errors::RitmoResult;
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};

pub async fn search_contents(
    pool: &SqlitePool,
    filter_sets: &[FilterSet],
) -> RitmoResult<Vec<Content>> {
    let active_sets: Vec<&FilterSet> = filter_sets.iter().filter(|set| set.active).collect();

    let mut builder = QueryBuilder::<Sqlite>::new(
        "SELECT contents.id, contents.name, contents.publication_date_year, contents.publication_date_month, contents.publication_date_day, contents.publication_date_circa, contents.notes FROM contents",
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

    builder.push(" ORDER BY contents.name");

    let rows = builder.build().fetch_all(pool).await.map_err(map_query)?;

    Ok(rows
        .into_iter()
        .map(|row| Content {
            id: row.get("id"),
            title: row.get("name"),
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
        FilterField::ContentTitle => push_text_filter(builder, "contents.name", filter),
        FilterField::ContentGenre => {
            push_relation_filter(builder, "contents.genre_id", "genres", "id", "key", filter)
        }
        FilterField::ContentPublicationDate => push_date_filter(
            builder,
            "contents.publication_date_year",
            "contents.publication_date_month",
            "contents.publication_date_day",
            filter,
        ),
        FilterField::ContentTag => push_exists_relation_filter(
            builder,
            "x_contents_tags",
            "content_id",
            "tag_id",
            Some(("tags", "id", "name")),
            filter,
            None,
        ),
        FilterField::ContentLanguage { role_id } => push_exists_relation_filter(
            builder,
            "content_languages",
            "content_id",
            "language_id",
            Some(("languages", "id", "official_name")),
            filter,
            role_id.map(|id| ("role_id", id)),
        ),
        _ => push_false(builder),
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
    builder.push(" = contents.id");

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
