use tauri::State;
use sqlx::PgPool;
use uuid::Uuid;
use grc_shared::*;
use crate::models::{evidence, audit};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct PickedFileInfo {
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
}

fn mime_from_ext(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "pdf" => "application/pdf",
        "doc" | "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" | "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "csv" => "text/csv",
        "txt" | "md" => "text/plain",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "html" | "htm" => "text/html",
        "json" => "application/json",
        "xml" => "application/xml",
        "zip" => "application/zip",
        _ => "application/octet-stream",
    }
}

#[tauri::command]
pub fn pick_evidence_file(app: tauri::AppHandle) -> Result<Option<PickedFileInfo>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_resp = app.dialog()
        .file()
        .add_filter("Documents", &["pdf", "doc", "docx", "xlsx", "csv", "txt", "md", "html"])
        .add_filter("Images", &["png", "jpg", "jpeg", "gif"])
        .add_filter("All Files", &["*"])
        .blocking_pick_file();

    match file_resp {
        Some(file_path) => {
            let path_buf: PathBuf = file_path.as_path()
                .ok_or("Could not resolve file path")?
                .to_path_buf();

            let file_name = path_buf.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let metadata = std::fs::metadata(&path_buf).map_err(|e| e.to_string())?;
            let size = metadata.len() as i64;

            let ext = path_buf.extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();
            let mime = mime_from_ext(&ext);

            Ok(Some(PickedFileInfo {
                file_name,
                file_path: path_buf.to_string_lossy().to_string(),
                file_size_bytes: size,
                mime_type: mime.to_string(),
            }))
        }
        None => Ok(None),
    }
}

/// Helper: read evidence_storage_path from app_config
async fn get_storage_path(pool: &PgPool) -> Result<String, String> {
    let row = sqlx::query_scalar::<_, String>(
        "SELECT evidence_storage_path FROM app_config LIMIT 1"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row)
}

#[tauri::command]
pub async fn upload_evidence(
    pool: State<'_, PgPool>,
    dto: UploadEvidenceDto,
) -> Result<Evidence, String> {
    // Copy file to evidence storage directory if a storage path is configured
    let storage_path = get_storage_path(&pool).await?;
    let mut dest_file_path = dto.file_path.clone();

    if !storage_path.is_empty() {
        let source = Path::new(&dto.file_path);
        if source.exists() {
            let dest_dir = PathBuf::from(&storage_path)
                .join(dto.engagement_id.to_string());
            std::fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

            let dest = dest_dir.join(&dto.file_name);
            std::fs::copy(source, &dest).map_err(|e| e.to_string())?;
            dest_file_path = dest.to_string_lossy().to_string();
        }
    }

    let mut dto_with_dest = dto;
    dto_with_dest.file_path = dest_file_path;

    let result = evidence::upload(&pool, dto_with_dest).await?;
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

#[tauri::command]
pub async fn list_evidence_links(
    pool: State<'_, PgPool>,
    evidence_id: Uuid,
) -> Result<Vec<EvidenceLink>, String> {
    evidence::list_links(&pool, evidence_id).await
}
