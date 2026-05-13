use crate::support::{map_delete, map_insert, map_query, not_found};
use ritmo_domain::filter::{Filter, FilterSet, FilterValue, LogicalOperator};
use ritmo_errors::{RitmoErr, RitmoResult};
use sqlx::{Row, SqlitePool};

pub async fn save_filter_set(pool: &SqlitePool, filter_set: &FilterSet) -> RitmoResult<i64> {
    let mut tx = pool.begin().await.map_err(map_query)?;

    let operator = logical_operator_to_db(&filter_set.operator);
    let result = sqlx::query("INSERT INTO s_filter_sets(name, active, operator) VALUES (?, ?, ?)")
        .bind(&filter_set.name)
        .bind(i64::from(filter_set.active))
        .bind(operator)
        .execute(&mut *tx)
        .await
        .map_err(map_insert)?;

    let filter_set_id = result.last_insert_rowid();

    for filter in &filter_set.filters {
        sqlx::query(
            "INSERT INTO s_filter_conditions(filter_set_id, field, operator, values) VALUES (?, ?, ?, ?)",
        )
        .bind(filter_set_id)
        .bind(serialize_json(&filter.field)?)
        .bind(serialize_json(&filter.operator)?)
        .bind(serialize_json(&filter.values)?)
        .execute(&mut *tx)
        .await
        .map_err(map_insert)?;
    }

    tx.commit().await.map_err(map_query)?;
    Ok(filter_set_id)
}

pub async fn get_filter_set(pool: &SqlitePool, id: i64) -> RitmoResult<FilterSet> {
    let set_row = sqlx::query("SELECT id, name, active, operator FROM s_filter_sets WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

    let filters = load_filters(pool, id).await?;

    Ok(FilterSet {
        id: set_row.get("id"),
        name: set_row.get("name"),
        active: set_row.get::<i64, _>("active") != 0,
        operator: logical_operator_from_db(set_row.get::<String, _>("operator").as_str()),
        filters,
    })
}

pub async fn update_filter_set(pool: &SqlitePool, filter_set: &FilterSet) -> RitmoResult<()> {
    let mut tx = pool.begin().await.map_err(map_query)?;

    sqlx::query("UPDATE s_filter_sets SET name = ?, active = ?, operator = ?, updated_at = strftime('%s', 'now') WHERE id = ?")
        .bind(&filter_set.name)
        .bind(i64::from(filter_set.active))
        .bind(logical_operator_to_db(&filter_set.operator))
        .bind(filter_set.id)
        .execute(&mut *tx)
        .await
        .map_err(map_query)?;

    sqlx::query("DELETE FROM s_filter_conditions WHERE filter_set_id = ?")
        .bind(filter_set.id)
        .execute(&mut *tx)
        .await
        .map_err(map_delete)?;

    for filter in &filter_set.filters {
        sqlx::query(
            "INSERT INTO s_filter_conditions(filter_set_id, field, operator, values) VALUES (?, ?, ?, ?)",
        )
        .bind(filter_set.id)
        .bind(serialize_json(&filter.field)?)
        .bind(serialize_json(&filter.operator)?)
        .bind(serialize_json(&filter.values)?)
        .execute(&mut *tx)
        .await
        .map_err(map_insert)?;
    }

    tx.commit().await.map_err(map_query)?;
    Ok(())
}

pub async fn delete_filter_set(pool: &SqlitePool, id: i64) -> RitmoResult<()> {
    let mut tx = pool.begin().await.map_err(map_query)?;

    sqlx::query("DELETE FROM s_filter_conditions WHERE filter_set_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_delete)?;

    sqlx::query("DELETE FROM s_filter_sets WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_delete)?;

    tx.commit().await.map_err(map_query)?;
    Ok(())
}

pub async fn list_filter_sets(pool: &SqlitePool) -> RitmoResult<Vec<FilterSet>> {
    let rows = sqlx::query("SELECT id, name, active, operator FROM s_filter_sets ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(map_query)?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let id: i64 = row.get("id");
        result.push(FilterSet {
            id,
            name: row.get("name"),
            active: row.get::<i64, _>("active") != 0,
            operator: logical_operator_from_db(row.get::<String, _>("operator").as_str()),
            filters: load_filters(pool, id).await?,
        });
    }

    Ok(result)
}

async fn load_filters(pool: &SqlitePool, filter_set_id: i64) -> RitmoResult<Vec<Filter>> {
    let rows = sqlx::query(
        "SELECT field, operator, values FROM s_filter_conditions WHERE filter_set_id = ? ORDER BY id",
    )
    .bind(filter_set_id)
    .fetch_all(pool)
    .await
    .map_err(map_query)?;

    rows.into_iter()
        .map(|row| {
            let field_json: String = row.get("field");
            let operator_json: String = row.get("operator");
            let values_json: String = row.get("values");

            Ok(Filter {
                field: deserialize_json(&field_json)?,
                operator: deserialize_json(&operator_json)?,
                values: deserialize_json::<Vec<FilterValue>>(&values_json)?,
            })
        })
        .collect()
}

fn logical_operator_to_db(operator: &LogicalOperator) -> &'static str {
    match operator {
        LogicalOperator::And => "AND",
        LogicalOperator::Or => "OR",
    }
}

fn logical_operator_from_db(value: &str) -> LogicalOperator {
    if value.eq_ignore_ascii_case("OR") {
        LogicalOperator::Or
    } else {
        LogicalOperator::And
    }
}

fn serialize_json<T: serde::Serialize>(value: &T) -> RitmoResult<String> {
    serde_json::to_string(value).map_err(|err| RitmoErr::SerializationError(err.to_string()))
}

fn deserialize_json<T: serde::de::DeserializeOwned>(value: &str) -> RitmoResult<T> {
    serde_json::from_str(value).map_err(|err| RitmoErr::SerializationError(err.to_string()))
}
