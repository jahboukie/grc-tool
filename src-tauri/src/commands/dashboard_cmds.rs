use tauri::State;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;
use crate::models::{task, cross_reference};

#[tauri::command]
pub async fn get_dashboard_stats(
    pool: State<'_, PgPool>,
) -> Result<DashboardStats, String> {
    let active_engagements: i64 = sqlx::query("SELECT COUNT(*) as cnt FROM engagements WHERE status = 'active'")
        .fetch_one(&*pool).await.map_err(|e| e.to_string())?
        .get("cnt");

    let total_ai_systems: i64 = sqlx::query("SELECT COUNT(*) as cnt FROM ai_systems")
        .fetch_one(&*pool).await.map_err(|e| e.to_string())?
        .get("cnt");

    let open_tasks: i64 = sqlx::query("SELECT COUNT(*) as cnt FROM tasks WHERE status IN ('open','in_progress','blocked')")
        .fetch_one(&*pool).await.map_err(|e| e.to_string())?
        .get("cnt");

    let open_risks: i64 = sqlx::query("SELECT COUNT(*) as cnt FROM risk_entries WHERE status IN ('open','in_progress','blocked')")
        .fetch_one(&*pool).await.map_err(|e| e.to_string())?
        .get("cnt");

    let total_gaps: i64 = sqlx::query("SELECT COUNT(*) as cnt FROM requirement_assessments WHERE status = 'gap'")
        .fetch_one(&*pool).await.map_err(|e| e.to_string())?
        .get("cnt");

    let total_evidence: i64 = sqlx::query("SELECT COUNT(*) as cnt FROM evidence")
        .fetch_one(&*pool).await.map_err(|e| e.to_string())?
        .get("cnt");

    let fria_row = sqlx::query(
        "SELECT
            COUNT(*) FILTER (WHERE a.risk_category = 'high') as in_scope,
            COUNT(*) FILTER (WHERE f.status = 'completed') as completed
         FROM ai_systems a
         LEFT JOIN fria_assessments f ON a.id = f.ai_system_id"
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let fria_in_scope: i64 = fria_row.get("in_scope");
    let fria_completed: i64 = fria_row.get("completed");

    // Compliance by framework
    let fw_rows = sqlx::query(
        "SELECT fr.framework,
                COUNT(*) FILTER (WHERE ra.status IS NOT NULL) as total,
                COUNT(*) FILTER (WHERE ra.status = 'met') as met,
                COUNT(*) FILTER (WHERE ra.status = 'partial') as partial,
                COUNT(*) FILTER (WHERE ra.status = 'gap') as gap,
                COUNT(*) FILTER (WHERE ra.status = 'not_assessed' OR ra.status IS NULL) as not_assessed
         FROM framework_requirements fr
         LEFT JOIN requirement_assessments ra ON fr.id = ra.requirement_id
         GROUP BY fr.framework"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let compliance_by_framework: Vec<FrameworkCompliance> = fw_rows.iter().map(|row| {
        let total: i64 = row.get("total");
        let met: i64 = row.get("met");
        let not_assessed: i64 = row.get("not_assessed");
        let applicable = total - not_assessed;
        let pct = if applicable > 0 { (met as f64 / applicable as f64) * 100.0 } else { 0.0 };
        FrameworkCompliance {
            framework: parse_enum(row.get::<String, _>("framework").as_str()),
            total_applicable: applicable,
            met,
            partial: row.get("partial"),
            gap: row.get("gap"),
            not_assessed,
            pct,
        }
    }).collect();

    let priority_tasks = task::list(&pool, None, None, None).await?
        .into_iter().take(10).collect();

    Ok(DashboardStats {
        active_engagements,
        total_ai_systems,
        compliance_by_framework,
        open_tasks,
        open_risks,
        total_gaps,
        total_evidence,
        fria_in_scope,
        fria_completed,
        priority_tasks,
    })
}

#[tauri::command]
pub async fn get_gap_analysis(
    pool: State<'_, PgPool>,
    engagement_id: Option<Uuid>,
    ai_system_id: Option<Uuid>,
) -> Result<GapAnalysisData, String> {
    let rows = sqlx::query(
        "SELECT ra.*, fr.framework, fr.reference_id, fr.title as req_title,
                fr.description as req_description, fr.article_clause, fr.category,
                fr.subcategory, fr.applicable_risk_categories, fr.applicable_roles,
                fr.is_mandatory, fr.guidance_text, fr.implementation_notes, fr.sort_order
         FROM requirement_assessments ra
         JOIN framework_requirements fr ON ra.requirement_id = fr.id
         JOIN ai_systems ais ON ra.ai_system_id = ais.id
         WHERE ra.status IN ('gap', 'partial')
           AND ($1::uuid IS NULL OR ais.engagement_id = $1)
           AND ($2::uuid IS NULL OR ra.ai_system_id = $2)
         ORDER BY fr.framework, fr.sort_order"
    )
    .bind(engagement_id)
    .bind(ai_system_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut framework_map: std::collections::HashMap<String, Vec<GapEntry>> = std::collections::HashMap::new();
    let mut total_gaps: i64 = 0;
    let mut total_partial: i64 = 0;

    for row in &rows {
        let status: String = row.get("status");
        if status == "gap" { total_gaps += 1; }
        if status == "partial" { total_partial += 1; }

        let fw_str: String = row.get("framework");
        let req = FrameworkRequirement {
            id: row.get("requirement_id"),
            framework: parse_enum(&fw_str),
            reference_id: row.get("reference_id"),
            title: row.get("req_title"),
            description: row.get("req_description"),
            article_clause: row.get("article_clause"),
            category: row.get("category"),
            subcategory: row.get("subcategory"),
            applicable_risk_categories: parse_enum_vec(&row.get::<Vec<String>, _>("applicable_risk_categories")),
            applicable_roles: parse_enum_vec(&row.get::<Vec<String>, _>("applicable_roles")),
            is_mandatory: row.get("is_mandatory"),
            guidance_text: row.get("guidance_text"),
            implementation_notes: row.get("implementation_notes"),
            sort_order: row.get("sort_order"),
        };

        let assessment = RequirementAssessment {
            id: row.get("id"),
            ai_system_id: row.get("ai_system_id"),
            requirement_id: row.get("requirement_id"),
            status: parse_enum(&status),
            assessor_notes: row.get("assessor_notes"),
            remediation_plan: row.get("remediation_plan"),
            target_date: row.get("target_date"),
            assessed_at: row.get("assessed_at"),
            updated_at: row.get("updated_at"),
        };

        let cross_refs = cross_reference::get_for_gap(&pool, req.id).await.unwrap_or_default();

        framework_map.entry(fw_str).or_default().push(GapEntry {
            requirement: req,
            assessment,
            cross_references: cross_refs,
        });
    }

    let frameworks: Vec<FrameworkGaps> = framework_map.into_iter().map(|(fw_str, gaps)| {
        let fw: Framework = parse_enum(&fw_str);
        let name = fw.display_name().to_string();
        FrameworkGaps { framework: fw, framework_name: name, gaps }
    }).collect();

    Ok(GapAnalysisData { frameworks, total_gaps, total_partial })
}
