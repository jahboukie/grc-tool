use tauri::State;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use grc_shared::*;
use crate::models::audit;
use crate::llm;

#[tauri::command]
pub async fn query_llm(
    pool: State<'_, PgPool>,
    dto: LlmQueryDto,
) -> Result<LlmConversation, String> {
    // Get config for LLM provider settings
    let config_row = sqlx::query("SELECT * FROM app_config LIMIT 1")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let provider: String = config_row.get("llm_provider");
    let api_key: String = config_row.get("llm_api_key_encrypted");
    let model: String = config_row.get("llm_model");

    // Build context from engagement/system if provided
    let context = llm::client::build_context(&pool, dto.engagement_id, dto.ai_system_id).await?;
    let system_prompt = llm::client::build_system_prompt(&context);

    // Fetch recent conversation history for multi-turn context (last 10 turns)
    let history_rows = if let Some(eid) = dto.engagement_id {
        sqlx::query(
            "SELECT query, response FROM llm_conversations
             WHERE engagement_id = $1
             ORDER BY created_at DESC LIMIT 10"
        )
        .bind(eid)
        .fetch_all(&*pool)
        .await
    } else {
        sqlx::query(
            "SELECT query, response FROM llm_conversations
             ORDER BY created_at DESC LIMIT 10"
        )
        .fetch_all(&*pool)
        .await
    }
    .map_err(|e| e.to_string())?;

    let mut history: Vec<(String, String)> = history_rows.iter().map(|r| {
        (r.get::<String, _>("query"), r.get::<String, _>("response"))
    }).collect();
    history.reverse(); // oldest first for chronological order

    // Call LLM
    let response = llm::client::call_llm(&provider, &api_key, &model, &system_prompt, &dto.query, &history).await?;

    // Store conversation
    let row = sqlx::query(
        "INSERT INTO llm_conversations (engagement_id, ai_system_id, query, response, model_used)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(dto.engagement_id)
    .bind(dto.ai_system_id)
    .bind(&dto.query)
    .bind(&response)
    .bind(&model)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let conversation = LlmConversation {
        id: row.get("id"),
        engagement_id: row.get("engagement_id"),
        ai_system_id: row.get("ai_system_id"),
        query: row.get("query"),
        response: row.get("response"),
        model_used: row.get("model_used"),
        created_at: row.get("created_at"),
    };

    audit::log(
        &pool, "llm_conversation", conversation.id, AuditAction::LlmQueried,
        None, None, None, &format!("LLM query: {}...", &dto.query.chars().take(50).collect::<String>()),
    ).await?;

    Ok(conversation)
}

#[tauri::command]
pub async fn list_conversations(
    pool: State<'_, PgPool>,
    engagement_id: Option<Uuid>,
) -> Result<Vec<LlmConversation>, String> {
    let rows = if let Some(eid) = engagement_id {
        sqlx::query("SELECT * FROM llm_conversations WHERE engagement_id = $1 ORDER BY created_at DESC")
            .bind(eid)
            .fetch_all(&*pool)
            .await
    } else {
        sqlx::query("SELECT * FROM llm_conversations ORDER BY created_at DESC")
            .fetch_all(&*pool)
            .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(|row| LlmConversation {
        id: row.get("id"),
        engagement_id: row.get("engagement_id"),
        ai_system_id: row.get("ai_system_id"),
        query: row.get("query"),
        response: row.get("response"),
        model_used: row.get("model_used"),
        created_at: row.get("created_at"),
    }).collect())
}
