use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_assessment(row: &PgRow) -> RequirementAssessment {
    RequirementAssessment {
        id: row.get("id"),
        ai_system_id: row.get("ai_system_id"),
        requirement_id: row.get("requirement_id"),
        status: parse_enum(row.get::<String, _>("status").as_str()),
        assessor_notes: row.get("assessor_notes"),
        remediation_plan: row.get("remediation_plan"),
        target_date: row.get("target_date"),
        assessed_at: row.get("assessed_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn upsert(pool: &PgPool, dto: UpsertAssessmentDto) -> Result<RequirementAssessment, String> {
    let status_str = enum_to_str(&dto.status);

    let row = sqlx::query(
        "INSERT INTO requirement_assessments (ai_system_id, requirement_id, status, assessor_notes, remediation_plan, target_date)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (ai_system_id, requirement_id)
         DO UPDATE SET
            status = EXCLUDED.status,
            assessor_notes = EXCLUDED.assessor_notes,
            remediation_plan = EXCLUDED.remediation_plan,
            target_date = EXCLUDED.target_date,
            assessed_at = NOW()
         RETURNING *"
    )
    .bind(dto.ai_system_id)
    .bind(dto.requirement_id)
    .bind(&status_str)
    .bind(&dto.assessor_notes)
    .bind(&dto.remediation_plan)
    .bind(dto.target_date)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_assessment(&row))
}

pub async fn list(
    pool: &PgPool,
    ai_system_id: Uuid,
    framework: Option<String>,
) -> Result<Vec<RequirementAssessment>, String> {
    let rows = if let Some(fw) = framework {
        sqlx::query(
            "SELECT ra.* FROM requirement_assessments ra
             JOIN framework_requirements fr ON ra.requirement_id = fr.id
             WHERE ra.ai_system_id = $1 AND fr.framework = $2
             ORDER BY fr.sort_order"
        )
        .bind(ai_system_id)
        .bind(&fw)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query(
            "SELECT * FROM requirement_assessments WHERE ai_system_id = $1 ORDER BY assessed_at DESC"
        )
        .bind(ai_system_id)
        .fetch_all(pool)
        .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_assessment).collect())
}

pub async fn get(
    pool: &PgPool,
    ai_system_id: Uuid,
    requirement_id: Uuid,
) -> Result<RequirementAssessment, String> {
    let row = sqlx::query(
        "SELECT * FROM requirement_assessments WHERE ai_system_id = $1 AND requirement_id = $2"
    )
    .bind(ai_system_id)
    .bind(requirement_id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_assessment(&row))
}
