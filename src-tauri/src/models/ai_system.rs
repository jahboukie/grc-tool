use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_ai_system(row: &PgRow) -> AiSystem {
    AiSystem {
        id: row.get("id"),
        engagement_id: row.get("engagement_id"),
        name: row.get("name"),
        description: row.get("description"),
        intended_purpose: row.get("intended_purpose"),
        risk_category: parse_enum(row.get::<String, _>("risk_category").as_str()),
        domain: row.get("domain"),
        is_gpai: row.get("is_gpai"),
        is_high_risk_listed: row.get("is_high_risk_listed"),
        is_safety_component: row.get("is_safety_component"),
        deployment_context: row.get("deployment_context"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create(pool: &PgPool, dto: CreateAiSystemDto) -> Result<AiSystem, String> {
    let risk_str = enum_to_str(&dto.risk_category);

    let row = sqlx::query(
        "INSERT INTO ai_systems (engagement_id, name, description, intended_purpose,
            risk_category, domain, is_gpai, is_high_risk_listed, is_safety_component,
            deployment_context)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING *"
    )
    .bind(dto.engagement_id)
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(&dto.intended_purpose)
    .bind(&risk_str)
    .bind(&dto.domain)
    .bind(dto.is_gpai)
    .bind(dto.is_high_risk_listed)
    .bind(dto.is_safety_component)
    .bind(&dto.deployment_context)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_ai_system(&row))
}

pub async fn list(pool: &PgPool, engagement_id: Uuid) -> Result<Vec<AiSystem>, String> {
    let rows = sqlx::query(
        "SELECT * FROM ai_systems WHERE engagement_id = $1 ORDER BY created_at ASC"
    )
    .bind(engagement_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_ai_system).collect())
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<AiSystem, String> {
    let row = sqlx::query("SELECT * FROM ai_systems WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row_to_ai_system(&row))
}

pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateAiSystemDto) -> Result<AiSystem, String> {
    let risk_str = dto.risk_category.map(|r| enum_to_str(&r));

    let row = sqlx::query(
        "UPDATE ai_systems SET
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            intended_purpose = COALESCE($4, intended_purpose),
            risk_category = COALESCE($5, risk_category),
            domain = COALESCE($6, domain),
            is_gpai = COALESCE($7, is_gpai),
            is_high_risk_listed = COALESCE($8, is_high_risk_listed),
            is_safety_component = COALESCE($9, is_safety_component),
            deployment_context = COALESCE($10, deployment_context)
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(&dto.intended_purpose)
    .bind(&risk_str)
    .bind(&dto.domain)
    .bind(dto.is_gpai)
    .bind(dto.is_high_risk_listed)
    .bind(dto.is_safety_component)
    .bind(&dto.deployment_context)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_ai_system(&row))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM ai_systems WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
