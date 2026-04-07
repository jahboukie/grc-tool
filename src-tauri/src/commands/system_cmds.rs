use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{ai_system, audit};

#[tauri::command]
pub async fn create_ai_system(
    pool: State<'_, PgPool>,
    dto: CreateAiSystemDto,
) -> Result<AiSystem, String> {
    let result = ai_system::create(&pool, dto).await?;
    audit::log(
        &pool, "ai_system", result.id, AuditAction::Created,
        None, None, None, &format!("Created AI system: {}", result.name),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_ai_systems(
    pool: State<'_, PgPool>,
    engagement_id: Uuid,
) -> Result<Vec<AiSystem>, String> {
    ai_system::list(&pool, engagement_id).await
}

#[tauri::command]
pub async fn get_ai_system(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<AiSystem, String> {
    ai_system::get(&pool, id).await
}

#[tauri::command]
pub async fn update_ai_system(
    pool: State<'_, PgPool>,
    id: Uuid,
    dto: UpdateAiSystemDto,
) -> Result<AiSystem, String> {
    let result = ai_system::update(&pool, id, dto).await?;
    audit::log(
        &pool, "ai_system", id, AuditAction::Updated,
        None, None, None, &format!("Updated AI system: {}", result.name),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_ai_system(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<(), String> {
    let sys = ai_system::get(&pool, id).await?;
    ai_system::delete(&pool, id).await?;
    audit::log(
        &pool, "ai_system", id, AuditAction::Deleted,
        None, None, None, &format!("Deleted AI system: {}", sys.name),
    ).await?;
    Ok(())
}
