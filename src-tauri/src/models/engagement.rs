use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_engagement(row: &PgRow) -> Engagement {
    Engagement {
        id: row.get("id"),
        name: row.get("name"),
        client_name: row.get("client_name"),
        description: row.get("description"),
        status: parse_enum(row.get::<String, _>("status").as_str()),
        primary_role: parse_enum(row.get::<String, _>("primary_role").as_str()),
        industry_sector: parse_enum(row.get::<String, _>("industry_sector").as_str()),
        jurisdictions: parse_enum_vec(&row.get::<Vec<String>, _>("jurisdictions")),
        assurance_objective: parse_enum(row.get::<String, _>("assurance_objective").as_str()),
        ai_use_case: parse_enum(row.get::<String, _>("ai_use_case").as_str()),
        personal_data_profile: parse_enum(row.get::<String, _>("personal_data_profile").as_str()),
        involves_vulnerable_groups: row.get("involves_vulnerable_groups"),
        is_public_facing: row.get("is_public_facing"),
        frameworks: parse_enum_vec(&row.get::<Vec<String>, _>("frameworks")),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create(pool: &PgPool, dto: CreateEngagementDto) -> Result<Engagement, String> {
    let role_str = enum_to_str(&dto.primary_role);
    let industry_str = enum_to_str(&dto.industry_sector);
    let jurisdictions = enum_vec_to_strs(&dto.jurisdictions);
    let objective_str = enum_to_str(&dto.assurance_objective);
    let use_case_str = enum_to_str(&dto.ai_use_case);
    let personal_data_str = enum_to_str(&dto.personal_data_profile);
    let fw_strs = enum_vec_to_strs(&dto.frameworks);

    let row = sqlx::query(
        "INSERT INTO engagements (
            name, client_name, description, primary_role, industry_sector,
            jurisdictions, assurance_objective, ai_use_case, personal_data_profile,
            involves_vulnerable_groups, is_public_facing, frameworks
         )
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
         RETURNING *"
    )
    .bind(&dto.name)
    .bind(&dto.client_name)
    .bind(&dto.description)
    .bind(&role_str)
    .bind(&industry_str)
    .bind(&jurisdictions)
    .bind(&objective_str)
    .bind(&use_case_str)
    .bind(&personal_data_str)
    .bind(dto.involves_vulnerable_groups)
    .bind(dto.is_public_facing)
    .bind(&fw_strs)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_engagement(&row))
}

pub async fn list(pool: &PgPool, status_filter: Option<String>) -> Result<Vec<Engagement>, String> {
    let rows = if let Some(status) = status_filter {
        sqlx::query("SELECT * FROM engagements WHERE status = $1 ORDER BY updated_at DESC")
            .bind(&status)
            .fetch_all(pool)
            .await
    } else {
        sqlx::query("SELECT * FROM engagements ORDER BY updated_at DESC")
            .fetch_all(pool)
            .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_engagement).collect())
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<Engagement, String> {
    let row = sqlx::query("SELECT * FROM engagements WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row_to_engagement(&row))
}

pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateEngagementDto) -> Result<Engagement, String> {
    let status_str = dto.status.map(|s| enum_to_str(&s));
    let role_str = dto.primary_role.map(|r| enum_to_str(&r));
    let industry_str = dto.industry_sector.map(|v| enum_to_str(&v));
    let jurisdictions = dto.jurisdictions.map(|items| enum_vec_to_strs(&items));
    let objective_str = dto.assurance_objective.map(|v| enum_to_str(&v));
    let use_case_str = dto.ai_use_case.map(|v| enum_to_str(&v));
    let personal_data_str = dto.personal_data_profile.map(|v| enum_to_str(&v));
    let fw_strs = dto.frameworks.map(|f| enum_vec_to_strs(&f));

    let row = sqlx::query(
        "UPDATE engagements SET
            name = COALESCE($2, name),
            client_name = COALESCE($3, client_name),
            description = COALESCE($4, description),
            status = COALESCE($5, status),
            primary_role = COALESCE($6, primary_role),
            industry_sector = COALESCE($7, industry_sector),
            jurisdictions = COALESCE($8, jurisdictions),
            assurance_objective = COALESCE($9, assurance_objective),
            ai_use_case = COALESCE($10, ai_use_case),
            personal_data_profile = COALESCE($11, personal_data_profile),
            involves_vulnerable_groups = COALESCE($12, involves_vulnerable_groups),
            is_public_facing = COALESCE($13, is_public_facing),
            frameworks = COALESCE($14, frameworks)
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(&dto.name)
    .bind(&dto.client_name)
    .bind(&dto.description)
    .bind(&status_str)
    .bind(&role_str)
    .bind(&industry_str)
    .bind(&jurisdictions)
    .bind(&objective_str)
    .bind(&use_case_str)
    .bind(&personal_data_str)
    .bind(dto.involves_vulnerable_groups)
    .bind(dto.is_public_facing)
    .bind(&fw_strs)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_engagement(&row))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM engagements WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
