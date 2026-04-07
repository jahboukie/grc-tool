use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{risk, audit};

#[tauri::command]
pub async fn create_risk_entry(
    pool: State<'_, PgPool>,
    dto: CreateRiskDto,
) -> Result<RiskEntry, String> {
    let result = risk::create(&pool, dto).await?;
    audit::log(
        &pool, "risk_entry", result.id, AuditAction::Created,
        None, None, None, &format!("Created risk: {}", result.title),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_risk_entries(
    pool: State<'_, PgPool>,
    ai_system_id: Option<Uuid>,
) -> Result<Vec<RiskEntry>, String> {
    risk::list(&pool, ai_system_id).await
}

#[tauri::command]
pub async fn update_risk_entry(
    pool: State<'_, PgPool>,
    id: Uuid,
    dto: UpdateRiskDto,
) -> Result<RiskEntry, String> {
    let result = risk::update(&pool, id, dto).await?;
    audit::log(
        &pool, "risk_entry", id, AuditAction::Updated,
        None, None, None, &format!("Updated risk: {}", result.title),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_risk_entry(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<(), String> {
    risk::delete(&pool, id).await?;
    audit::log(
        &pool, "risk_entry", id, AuditAction::Deleted,
        None, None, None, "Deleted risk entry",
    ).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_risk_matrix_data(
    pool: State<'_, PgPool>,
    engagement_id: Option<Uuid>,
    ai_system_id: Option<Uuid>,
) -> Result<RiskMatrixData, String> {
    risk::get_matrix_data(&pool, engagement_id, ai_system_id).await
}
