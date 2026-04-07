use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::Uuid;

use crate::db::*;
use grc_shared::*;

fn row_to_fria(row: &PgRow) -> FriaAssessment {
    FriaAssessment {
        id: row.get("id"),
        engagement_id: row.get("engagement_id"),
        ai_system_id: row.get("ai_system_id"),
        status: parse_enum(row.get::<String, _>("status").as_str()),
        scope_summary: row.get("scope_summary"),
        deployer_context: row.get("deployer_context"),
        affected_persons_and_groups: row.get("affected_persons_and_groups"),
        vulnerable_groups: row.get("vulnerable_groups"),
        fundamental_rights_risks: row.get("fundamental_rights_risks"),
        human_oversight_measures: row.get("human_oversight_measures"),
        mitigation_measures: row.get("mitigation_measures"),
        consultation_summary: row.get("consultation_summary"),
        conclusion: row.get("conclusion"),
        authority_notification_status: parse_enum(
            row.get::<String, _>("authority_notification_status").as_str(),
        ),
        review_date: row.get("review_date"),
        related_risk_ids: row.get("related_risk_ids"),
        related_task_ids: row.get("related_task_ids"),
        related_evidence_ids: row.get("related_evidence_ids"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn get(pool: &PgPool, ai_system_id: Uuid) -> Result<Option<FriaAssessment>, String> {
    let row = sqlx::query("SELECT * FROM fria_assessments WHERE ai_system_id = $1")
        .bind(ai_system_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row.as_ref().map(row_to_fria))
}

pub async fn list(pool: &PgPool, engagement_id: Option<Uuid>) -> Result<Vec<FriaAssessment>, String> {
    let rows = if let Some(engagement_id) = engagement_id {
        sqlx::query(
            "SELECT * FROM fria_assessments WHERE engagement_id = $1 ORDER BY updated_at DESC"
        )
        .bind(engagement_id)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query("SELECT * FROM fria_assessments ORDER BY updated_at DESC")
            .fetch_all(pool)
            .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_fria).collect())
}

pub async fn upsert(pool: &PgPool, dto: UpsertFriaAssessmentDto) -> Result<FriaAssessment, String> {
    let status_str = enum_to_str(&dto.status);
    let notification_str = enum_to_str(&dto.authority_notification_status);

    let row = sqlx::query(
        "INSERT INTO fria_assessments (
            engagement_id, ai_system_id, status, scope_summary, deployer_context,
            affected_persons_and_groups, vulnerable_groups, fundamental_rights_risks,
            human_oversight_measures, mitigation_measures, consultation_summary,
            conclusion, authority_notification_status, review_date, related_risk_ids,
            related_task_ids, related_evidence_ids
         ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8,
            $9, $10, $11,
            $12, $13, $14, $15,
            $16, $17
         )
         ON CONFLICT (ai_system_id)
         DO UPDATE SET
            engagement_id = EXCLUDED.engagement_id,
            status = EXCLUDED.status,
            scope_summary = EXCLUDED.scope_summary,
            deployer_context = EXCLUDED.deployer_context,
            affected_persons_and_groups = EXCLUDED.affected_persons_and_groups,
            vulnerable_groups = EXCLUDED.vulnerable_groups,
            fundamental_rights_risks = EXCLUDED.fundamental_rights_risks,
            human_oversight_measures = EXCLUDED.human_oversight_measures,
            mitigation_measures = EXCLUDED.mitigation_measures,
            consultation_summary = EXCLUDED.consultation_summary,
            conclusion = EXCLUDED.conclusion,
            authority_notification_status = EXCLUDED.authority_notification_status,
            review_date = EXCLUDED.review_date,
            related_risk_ids = EXCLUDED.related_risk_ids,
            related_task_ids = EXCLUDED.related_task_ids,
            related_evidence_ids = EXCLUDED.related_evidence_ids
         RETURNING *"
    )
    .bind(dto.engagement_id)
    .bind(dto.ai_system_id)
    .bind(&status_str)
    .bind(&dto.scope_summary)
    .bind(&dto.deployer_context)
    .bind(&dto.affected_persons_and_groups)
    .bind(&dto.vulnerable_groups)
    .bind(&dto.fundamental_rights_risks)
    .bind(&dto.human_oversight_measures)
    .bind(&dto.mitigation_measures)
    .bind(&dto.consultation_summary)
    .bind(&dto.conclusion)
    .bind(&notification_str)
    .bind(dto.review_date)
    .bind(&dto.related_risk_ids)
    .bind(&dto.related_task_ids)
    .bind(&dto.related_evidence_ids)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_fria(&row))
}