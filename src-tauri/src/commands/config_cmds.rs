use tauri::State;
use sqlx::{PgPool, Row};
use grc_shared::*;

#[tauri::command]
pub async fn get_config(
    pool: State<'_, PgPool>,
) -> Result<AppConfig, String> {
    let row = sqlx::query("SELECT * FROM app_config LIMIT 1")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(AppConfig {
        id: row.get("id"),
        llm_provider: row.get("llm_provider"),
        llm_api_key_encrypted: row.get("llm_api_key_encrypted"),
        llm_model: row.get("llm_model"),
        evidence_storage_path: row.get("evidence_storage_path"),
        db_host: row.get("db_host"),
        db_port: row.get("db_port"),
        db_name: row.get("db_name"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn update_config(
    pool: State<'_, PgPool>,
    dto: UpdateConfigDto,
) -> Result<AppConfig, String> {
    let row = sqlx::query(
        "UPDATE app_config SET
            llm_provider = COALESCE($1, llm_provider),
            llm_api_key_encrypted = COALESCE($2, llm_api_key_encrypted),
            llm_model = COALESCE($3, llm_model),
            evidence_storage_path = COALESCE($4, evidence_storage_path),
            updated_at = NOW()
         RETURNING *"
    )
    .bind(&dto.llm_provider)
    .bind(&dto.llm_api_key)
    .bind(&dto.llm_model)
    .bind(&dto.evidence_storage_path)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(AppConfig {
        id: row.get("id"),
        llm_provider: row.get("llm_provider"),
        llm_api_key_encrypted: row.get("llm_api_key_encrypted"),
        llm_model: row.get("llm_model"),
        evidence_storage_path: row.get("evidence_storage_path"),
        db_host: row.get("db_host"),
        db_port: row.get("db_port"),
        db_name: row.get("db_name"),
        updated_at: row.get("updated_at"),
    })
}
