use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{assessment, audit};
use crate::db::enum_to_str;

#[tauri::command]
pub async fn upsert_assessment(
    pool: State<'_, PgPool>,
    dto: UpsertAssessmentDto,
) -> Result<RequirementAssessment, String> {
    let status_str = enum_to_str(&dto.status);
    let result = assessment::upsert(&pool, dto).await?;
    audit::log(
        &pool, "requirement_assessment", result.id, AuditAction::AssessmentRecorded,
        Some("status"), None, Some(&status_str),
        &format!("Assessment recorded for requirement {}", result.requirement_id),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_assessments(
    pool: State<'_, PgPool>,
    ai_system_id: Uuid,
    framework: Option<String>,
) -> Result<Vec<RequirementAssessment>, String> {
    assessment::list(&pool, ai_system_id, framework).await
}

#[tauri::command]
pub async fn get_assessment(
    pool: State<'_, PgPool>,
    ai_system_id: Uuid,
    requirement_id: Uuid,
) -> Result<RequirementAssessment, String> {
    assessment::get(&pool, ai_system_id, requirement_id).await
}
