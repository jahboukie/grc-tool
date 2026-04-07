use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;
use crate::models::requirement;

fn row_to_cross_ref(row: &PgRow) -> CrossReference {
    CrossReference {
        id: row.get("id"),
        source_requirement_id: row.get("source_requirement_id"),
        target_requirement_id: row.get("target_requirement_id"),
        relationship: parse_enum(row.get::<String, _>("relationship").as_str()),
        notes: row.get("notes"),
    }
}

pub async fn get_for_requirement(
    pool: &PgPool,
    requirement_id: Uuid,
) -> Result<Vec<CrossReferenceExpanded>, String> {
    let rows = sqlx::query(
        "SELECT * FROM cross_references
         WHERE source_requirement_id = $1 OR target_requirement_id = $1"
    )
    .bind(requirement_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for row in &rows {
        let cr = row_to_cross_ref(row);
        let source = requirement::get(pool, cr.source_requirement_id).await?;
        let target = requirement::get(pool, cr.target_requirement_id).await?;
        results.push(CrossReferenceExpanded {
            id: cr.id,
            source,
            target,
            relationship: cr.relationship,
            notes: cr.notes,
        });
    }

    Ok(results)
}

pub async fn get_for_gap(
    pool: &PgPool,
    requirement_id: Uuid,
) -> Result<Vec<CrossReference>, String> {
    let rows = sqlx::query(
        "SELECT * FROM cross_references
         WHERE source_requirement_id = $1 OR target_requirement_id = $1"
    )
    .bind(requirement_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_cross_ref).collect())
}
