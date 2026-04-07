use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_evidence(row: &PgRow) -> Evidence {
    Evidence {
        id: row.get("id"),
        engagement_id: row.get("engagement_id"),
        file_name: row.get("file_name"),
        file_path: row.get("file_path"),
        file_size_bytes: row.get("file_size_bytes"),
        mime_type: row.get("mime_type"),
        evidence_type: parse_enum(row.get::<String, _>("evidence_type").as_str()),
        description: row.get("description"),
        tags: row.get("tags"),
        uploaded_at: row.get("uploaded_at"),
    }
}

fn row_to_evidence_link(row: &PgRow) -> EvidenceLink {
    EvidenceLink {
        id: row.get("id"),
        evidence_id: row.get("evidence_id"),
        requirement_assessment_id: row.get("requirement_assessment_id"),
        risk_entry_id: row.get("risk_entry_id"),
        task_id: row.get("task_id"),
    }
}

pub async fn upload(pool: &PgPool, dto: UploadEvidenceDto) -> Result<Evidence, String> {
    let type_str = enum_to_str(&dto.evidence_type);

    let row = sqlx::query(
        "INSERT INTO evidence (engagement_id, file_name, file_path, file_size_bytes,
            mime_type, evidence_type, description, tags)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
    )
    .bind(dto.engagement_id)
    .bind(&dto.file_name)
    .bind(&dto.file_path)
    .bind(dto.file_size_bytes)
    .bind(&dto.mime_type)
    .bind(&type_str)
    .bind(&dto.description)
    .bind(&dto.tags)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_evidence(&row))
}

pub async fn list(
    pool: &PgPool,
    engagement_id: Uuid,
    type_filter: Option<String>,
) -> Result<Vec<Evidence>, String> {
    let rows = if let Some(t) = type_filter {
        sqlx::query(
            "SELECT * FROM evidence WHERE engagement_id = $1 AND evidence_type = $2 ORDER BY uploaded_at DESC"
        )
        .bind(engagement_id)
        .bind(&t)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query(
            "SELECT * FROM evidence WHERE engagement_id = $1 ORDER BY uploaded_at DESC"
        )
        .bind(engagement_id)
        .fetch_all(pool)
        .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_evidence).collect())
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM evidence WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn link(pool: &PgPool, dto: LinkEvidenceDto) -> Result<EvidenceLink, String> {
    let row = sqlx::query(
        "INSERT INTO evidence_links (evidence_id, requirement_assessment_id, risk_entry_id, task_id)
         VALUES ($1, $2, $3, $4)
         RETURNING *"
    )
    .bind(dto.evidence_id)
    .bind(dto.requirement_assessment_id)
    .bind(dto.risk_entry_id)
    .bind(dto.task_id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_evidence_link(&row))
}

pub async fn unlink(pool: &PgPool, link_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM evidence_links WHERE id = $1")
        .bind(link_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn list_links(pool: &PgPool, evidence_id: Uuid) -> Result<Vec<EvidenceLink>, String> {
    let rows = sqlx::query(
        "SELECT * FROM evidence_links WHERE evidence_id = $1"
    )
    .bind(evidence_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_evidence_link).collect())
}
