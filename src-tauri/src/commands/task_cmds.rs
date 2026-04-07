use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{task, audit};

#[tauri::command]
pub async fn create_task(
    pool: State<'_, PgPool>,
    dto: CreateTaskDto,
) -> Result<Task, String> {
    let result = task::create(&pool, dto).await?;
    audit::log(
        &pool, "task", result.id, AuditAction::Created,
        None, None, None, &format!("Created task: {}", result.title),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_tasks(
    pool: State<'_, PgPool>,
    engagement_id: Option<Uuid>,
    status: Option<String>,
    priority: Option<String>,
) -> Result<Vec<Task>, String> {
    task::list(&pool, engagement_id, status, priority).await
}

#[tauri::command]
pub async fn update_task(
    pool: State<'_, PgPool>,
    id: Uuid,
    dto: UpdateTaskDto,
) -> Result<Task, String> {
    let result = task::update(&pool, id, dto).await?;
    audit::log(
        &pool, "task", id, AuditAction::Updated,
        None, None, None, &format!("Updated task: {}", result.title),
    ).await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_task(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<(), String> {
    task::delete(&pool, id).await?;
    audit::log(
        &pool, "task", id, AuditAction::Deleted,
        None, None, None, "Deleted task",
    ).await?;
    Ok(())
}
