use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{evidence, audit};

#[tauri::command]
pub async fn upload_evidence(
    pool: State<'_, PgPool>,
    dto: UploadEvidenceDto,
) -> Result<Evidence, String> {
    let result = evidence::upload(&pool, dto).await?;
    audit::log(
        &pool, "evidence", result.id, AuditAction::Created,
        None, None, None, &format!("Uploaded evidence: {}", result.file_name),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_evidence(
    pool: State<'_, PgPool>,
    engagement_id: Uuid,
    type_filter: Option<String>,
) -> Result<Vec<Evidence>, String> {
    evidence::list(&pool, engagement_id, type_filter).await
}

#[tauri::command]
pub async fn delete_evidence(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<(), String> {
    evidence::delete(&pool, id).await?;
    audit::log(
        &pool, "evidence", id, AuditAction::Deleted,
        None, None, None, "Deleted evidence",
    ).await?;
    Ok(())
}

#[tauri::command]
pub async fn link_evidence(
    pool: State<'_, PgPool>,
    dto: LinkEvidenceDto,
) -> Result<EvidenceLink, String> {
    let result = evidence::link(&pool, dto).await?;
    audit::log(
        &pool, "evidence", result.evidence_id, AuditAction::EvidenceAttached,
        None, None, None, &format!("Evidence linked (link_id: {})", result.id),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn unlink_evidence(
    pool: State<'_, PgPool>,
    link_id: Uuid,
) -> Result<(), String> {
    evidence::unlink(&pool, link_id).await?;
    audit::log(
        &pool, "evidence_link", link_id, AuditAction::EvidenceDetached,
        None, None, None, "Evidence link removed",
    ).await?;
    Ok(())
}
