pub mod client;

use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct LlmContext {
    pub engagement_name: String,
    pub primary_role: String,
    pub system_name: Option<String>,
    pub risk_category: Option<String>,
    pub domain: Option<String>,
    pub frameworks: Vec<String>,
    pub met_count: i64,
    pub total_count: i64,
    pub gap_titles: Vec<String>,
}

pub async fn build_context_from_db(
    pool: &PgPool,
    engagement_id: Option<Uuid>,
    ai_system_id: Option<Uuid>,
) -> Result<Option<LlmContext>, String> {
    let Some(eid) = engagement_id else { return Ok(None) };

    let eng_row = sqlx::query("SELECT * FROM engagements WHERE id = $1")
        .bind(eid)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    let Some(eng) = eng_row else { return Ok(None) };

    let engagement_name: String = eng.get("name");
    let primary_role: String = eng.get("primary_role");
    let frameworks: Vec<String> = eng.get("frameworks");

    let (system_name, risk_category, domain) = if let Some(sid) = ai_system_id {
        let sys = sqlx::query("SELECT name, risk_category, domain FROM ai_systems WHERE id = $1")
            .bind(sid)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;
        match sys {
            Some(row) => (Some(row.get("name")), Some(row.get("risk_category")), Some(row.get("domain"))),
            None => (None, None, None),
        }
    } else {
        (None, None, None)
    };

    let met_count: i64 = if let Some(sid) = ai_system_id {
        sqlx::query("SELECT COUNT(*) as cnt FROM requirement_assessments WHERE ai_system_id = $1 AND status = 'met'")
            .bind(sid).fetch_one(pool).await.map_err(|e| e.to_string())?.get("cnt")
    } else { 0 };

    let total_count: i64 = if let Some(sid) = ai_system_id {
        sqlx::query("SELECT COUNT(*) as cnt FROM requirement_assessments WHERE ai_system_id = $1 AND status != 'not_applicable'")
            .bind(sid).fetch_one(pool).await.map_err(|e| e.to_string())?.get("cnt")
    } else { 0 };

    let gap_rows = if let Some(sid) = ai_system_id {
        sqlx::query(
            "SELECT fr.title FROM requirement_assessments ra
             JOIN framework_requirements fr ON ra.requirement_id = fr.id
             WHERE ra.ai_system_id = $1 AND ra.status = 'gap'
             LIMIT 10"
        ).bind(sid).fetch_all(pool).await.map_err(|e| e.to_string())?
    } else { vec![] };

    let gap_titles: Vec<String> = gap_rows.iter().map(|r| r.get("title")).collect();

    Ok(Some(LlmContext {
        engagement_name,
        primary_role,
        system_name,
        risk_category,
        domain,
        frameworks,
        met_count,
        total_count,
        gap_titles,
    }))
}
