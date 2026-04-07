use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;

use grc_shared::*;
use crate::db::*;

fn row_to_audit(row: &PgRow) -> AuditLog {
    AuditLog {
        id: row.get("id"),
        entity_type: row.get("entity_type"),
        entity_id: row.get("entity_id"),
        action: parse_enum(row.get::<String, _>("action").as_str()),
        field_changed: row.get("field_changed"),
        old_value: row.get("old_value"),
        new_value: row.get("new_value"),
        details: row.get("details"),
        timestamp: row.get("timestamp"),
    }
}

/// Append an entry to the immutable audit log.
pub async fn log(
    pool: &PgPool,
    entity_type: &str,
    entity_id: Uuid,
    action: AuditAction,
    field_changed: Option<&str>,
    old_value: Option<&str>,
    new_value: Option<&str>,
    details: &str,
) -> Result<(), String> {
    let action_str = enum_to_str(&action);

    sqlx::query(
        "INSERT INTO audit_log (entity_type, entity_id, action, field_changed, old_value, new_value, details)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(entity_type)
    .bind(entity_id)
    .bind(&action_str)
    .bind(field_changed)
    .bind(old_value)
    .bind(new_value)
    .bind(details)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn list(pool: &PgPool, filter: AuditFilterDto) -> Result<Vec<AuditLog>, String> {
    let action_str = filter.action.as_ref().map(|a| enum_to_str(a));
    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or(0);

    let rows = sqlx::query(
        "SELECT * FROM audit_log
         WHERE ($1::text IS NULL OR entity_type = $1)
           AND ($2::uuid IS NULL OR entity_id = $2)
           AND ($3::text IS NULL OR action = $3)
           AND ($4::timestamptz IS NULL OR timestamp >= $4)
           AND ($5::timestamptz IS NULL OR timestamp <= $5)
         ORDER BY timestamp DESC
         LIMIT $6 OFFSET $7"
    )
    .bind(&filter.entity_type)
    .bind(filter.entity_id)
    .bind(&action_str)
    .bind(filter.from_date)
    .bind(filter.to_date)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_audit).collect())
}
