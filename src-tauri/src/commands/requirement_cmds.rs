use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{requirement, cross_reference};

#[tauri::command]
pub async fn get_cross_references(
    pool: State<'_, PgPool>,
    requirement_id: Uuid,
) -> Result<Vec<CrossReferenceExpanded>, String> {
    cross_reference::get_for_requirement(&pool, requirement_id).await
}

#[tauri::command]
pub async fn list_cross_references(
    pool: State<'_, PgPool>,
) -> Result<Vec<CrossReferenceExpanded>, String> {
    cross_reference::list_all(&pool).await
}

#[tauri::command]
pub async fn list_requirements(
    pool: State<'_, PgPool>,
    framework: Option<String>,
    category: Option<String>,
) -> Result<Vec<FrameworkRequirement>, String> {
    requirement::list(&pool, framework, category).await
}

#[tauri::command]
pub async fn get_requirement(
    pool: State<'_, PgPool>,
    id: Uuid,
) -> Result<FrameworkRequirement, String> {
    requirement::get(&pool, id).await
}

#[tauri::command]
pub async fn search_requirements(
    pool: State<'_, PgPool>,
    query: String,
) -> Result<Vec<FrameworkRequirement>, String> {
    requirement::search(&pool, &query).await
}
