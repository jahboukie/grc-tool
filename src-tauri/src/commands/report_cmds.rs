use tauri::State;
use sqlx::PgPool;
use chrono::Utc;
use grc_shared::*;
use crate::models::audit;
use crate::reports;

#[tauri::command]
pub async fn generate_report(
    pool: State<'_, PgPool>,
    request: ReportRequest,
) -> Result<ReportResult, String> {
    let file_path = reports::generate(&pool, &request).await?;

    audit::log(
        &pool, "report", request.engagement_id, AuditAction::ReportGenerated,
        None, None, None,
        &format!("Generated {:?} report", request.report_type),
    ).await?;

    Ok(ReportResult {
        file_path,
        report_type: request.report_type,
        generated_at: Utc::now(),
    })
}
