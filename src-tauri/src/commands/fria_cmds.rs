use sqlx::PgPool;
use tauri::State;
use uuid::Uuid;

use crate::db::enum_to_str;
use crate::models::{audit, fria};
use grc_shared::*;

#[tauri::command]
pub async fn get_fria_assessment(
    pool: State<'_, PgPool>,
    ai_system_id: Uuid,
) -> Result<Option<FriaAssessment>, String> {
    fria::get(&pool, ai_system_id).await
}

#[tauri::command]
pub async fn list_fria_assessments(
    pool: State<'_, PgPool>,
    engagement_id: Option<Uuid>,
) -> Result<Vec<FriaAssessment>, String> {
    fria::list(&pool, engagement_id).await
}

#[tauri::command]
pub async fn upsert_fria_assessment(
    pool: State<'_, PgPool>,
    dto: UpsertFriaAssessmentDto,
) -> Result<FriaAssessment, String> {
    let ai_system_id = dto.ai_system_id;
    let existed = fria::get(&pool, ai_system_id).await?.is_some();
    let result = fria::upsert(&pool, dto).await?;
    let status_str = enum_to_str(&result.status);

    audit::log(
        &pool,
        "fria_assessment",
        result.id,
        if existed {
            AuditAction::Updated
        } else {
            AuditAction::Created
        },
        Some("status"),
        None,
        Some(&status_str),
        &format!(
            "{} FRIA for AI system {}",
            if existed { "Updated" } else { "Created" },
            ai_system_id
        ),
    )
    .await?;

    Ok(result)
}