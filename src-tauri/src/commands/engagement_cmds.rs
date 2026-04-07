use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{engagement, audit};

#[tauri::command]
pub async fn create_engagement(
    pool: State<'_, PgPool>,
    dto: CreateEngagementDto,
) -> Result<Engagement, String> {
    let result = engagement::create(&pool, dto).await?;
    audit::log(
        &pool, "engagement", result.id, AuditAction::Created,
        None, None, None, &format!("Created engagement: {}", result.name),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_engagements(
    pool: State<'_, PgPool>,
    status_filter: Option<String>,
) -> Result<Vec<Engagement>, String> {
    engagement::list(&pool, status_filter).await
}

#[tauri::command]
pub async fn get_engagement(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<Engagement, String> {
    engagement::get(&pool, id).await
}

#[tauri::command]
pub async fn update_engagement(
    pool: State<'_, PgPool>,
    id: Uuid,
    dto: UpdateEngagementDto,
) -> Result<Engagement, String> {
    let result = engagement::update(&pool, id, dto).await?;
    audit::log(
        &pool, "engagement", id, AuditAction::Updated,
        None, None, None, &format!("Updated engagement: {}", result.name),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_engagement(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<(), String> {
    let eng = engagement::get(&pool, id).await?;
    engagement::delete(&pool, id).await?;
    audit::log(
        &pool, "engagement", id, AuditAction::Deleted,
        None, None, None, &format!("Deleted engagement: {}", eng.name),
    ).await?;
    Ok(())
}
