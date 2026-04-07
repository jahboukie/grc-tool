use tauri::State;
use sqlx::PgPool;
use grc_shared::*;
use crate::models::audit;

#[tauri::command]
pub async fn list_audit_log(
    pool: State<'_, PgPool>,
    filter: AuditFilterDto,
) -> Result<Vec<AuditLog>, String> {
    audit::list(&pool, filter).await
}
