use askama::Template;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::Utc;
use grc_shared::*;
use crate::db::*;
use crate::models::{engagement, ai_system, requirement, risk};

#[derive(Template)]
#[template(path = "compliance_report.html")]
struct ComplianceReportTemplate {
    engagement_name: String,
    client_name: String,
    generated_at: String,
    primary_role: String,
    systems: Vec<SystemComplianceView>,
}

struct SystemComplianceView {
    name: String,
    risk_category: String,
    domain: String,
    framework_sections: Vec<FrameworkSection>,
}

struct FrameworkSection {
    framework_name: String,
    met: i64,
    total: i64,
    pct: String,
    requirements: Vec<RequirementView>,
}

struct RequirementView {
    reference_id: String,
    title: String,
    status: String,
    status_css: String,
    notes: String,
}

#[derive(Template)]
#[template(path = "gap_analysis.html")]
struct GapAnalysisTemplate {
    engagement_name: String,
    generated_at: String,
    total_gaps: i64,
    total_partial: i64,
    frameworks: Vec<GapFrameworkView>,
}

struct GapFrameworkView {
    framework_name: String,
    gaps: Vec<GapView>,
}

struct GapView {
    reference_id: String,
    title: String,
    status: String,
    notes: String,
    remediation: String,
    target_date: String,
}

#[derive(Template)]
#[template(path = "risk_report.html")]
struct RiskReportTemplate {
    engagement_name: String,
    system_name: String,
    generated_at: String,
    risks: Vec<RiskView>,
}

struct RiskView {
    title: String,
    description: String,
    likelihood: String,
    impact: String,
    score: i32,
    severity: String,
    severity_color: String,
    mitigation: String,
    status: String,
}

pub async fn generate(pool: &PgPool, request: &ReportRequest) -> Result<String, String> {
    let eng = engagement::get(pool, request.engagement_id).await?;
    let timestamp = Utc::now().format("%Y-%m-%d_%H%M%S").to_string();

    let (html, filename) = match request.report_type {
        ReportType::FullCompliance => {
            let html = generate_compliance(pool, &eng, request.ai_system_id).await?;
            (html, format!("compliance_report_{}.html", timestamp))
        }
        ReportType::GapAnalysis => {
            let html = generate_gap_analysis(pool, &eng, request.ai_system_id).await?;
            (html, format!("gap_analysis_{}.html", timestamp))
        }
        ReportType::RiskAssessment => {
            let sys_id = request.ai_system_id.ok_or("AI system ID required for risk report")?;
            let html = generate_risk(pool, &eng, sys_id).await?;
            (html, format!("risk_report_{}.html", timestamp))
        }
    };

    // Write to reports directory
    let reports_dir = std::env::var("GRC_REPORTS_PATH")
        .unwrap_or_else(|_| {
            let home = std::env::var("USERPROFILE")
                .or_else(|_| std::env::var("HOME"))
                .unwrap_or_else(|_| ".".to_string());
            format!("{}/grc-reports", home)
        });

    std::fs::create_dir_all(&reports_dir).map_err(|e| e.to_string())?;
    let file_path = format!("{}/{}", reports_dir, filename);
    std::fs::write(&file_path, &html).map_err(|e| e.to_string())?;

    Ok(file_path)
}

async fn generate_compliance(
    pool: &PgPool,
    eng: &Engagement,
    ai_system_id: Option<Uuid>,
) -> Result<String, String> {
    let systems = if let Some(sid) = ai_system_id {
        vec![ai_system::get(pool, sid).await?]
    } else {
        ai_system::list(pool, eng.id).await?
    };

    let mut system_views = Vec::new();
    for sys in &systems {
        let mut framework_sections = Vec::new();
        for fw in Framework::all() {
            let fw_str = enum_to_str(fw);
            let reqs = requirement::list(pool, Some(fw_str.clone()), None).await?;
            let mut req_views = Vec::new();
            let mut met = 0i64;
            let mut total = 0i64;

            for req in &reqs {
                let (status, notes) = match sqlx::query(
                    "SELECT status, assessor_notes FROM requirement_assessments WHERE ai_system_id = $1 AND requirement_id = $2"
                ).bind(sys.id).bind(req.id).fetch_optional(pool).await {
                    Ok(Some(row)) => {
                        let s: String = row.get("status");
                        let n: String = row.get("assessor_notes");
                        (s, n)
                    }
                    _ => ("not_assessed".to_string(), String::new()),
                };

                if status != "not_applicable" { total += 1; }
                if status == "met" { met += 1; }

                let cs: ComplianceStatus = parse_enum(&status);
                req_views.push(RequirementView {
                    reference_id: req.reference_id.clone(),
                    title: req.title.clone(),
                    status: cs.display_name().to_string(),
                    status_css: cs.css_class().to_string(),
                    notes,
                });
            }

            let pct = if total > 0 { met as f64 / total as f64 * 100.0 } else { 0.0 };
            framework_sections.push(FrameworkSection {
                framework_name: fw.display_name().to_string(),
                met, total, pct: format!("{:.1}", pct),
                requirements: req_views,
            });
        }

        system_views.push(SystemComplianceView {
            name: sys.name.clone(),
            risk_category: sys.risk_category.display_name().to_string(),
            domain: sys.domain.clone(),
            framework_sections,
        });
    }

    let tmpl = ComplianceReportTemplate {
        engagement_name: eng.name.clone(),
        client_name: eng.client_name.clone(),
        generated_at: Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
        primary_role: eng.primary_role.display_name().to_string(),
        systems: system_views,
    };

    tmpl.render().map_err(|e| e.to_string())
}

async fn generate_gap_analysis(
    pool: &PgPool,
    eng: &Engagement,
    ai_system_id: Option<Uuid>,
) -> Result<String, String> {
    let rows = sqlx::query(
        "SELECT ra.status, ra.assessor_notes, ra.remediation_plan, ra.target_date,
                fr.framework, fr.reference_id, fr.title
         FROM requirement_assessments ra
         JOIN framework_requirements fr ON ra.requirement_id = fr.id
         JOIN ai_systems ais ON ra.ai_system_id = ais.id
         WHERE ra.status IN ('gap', 'partial')
           AND ais.engagement_id = $1
           AND ($2::uuid IS NULL OR ra.ai_system_id = $2)
         ORDER BY fr.framework, fr.sort_order"
    )
    .bind(eng.id)
    .bind(ai_system_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut total_gaps = 0i64;
    let mut total_partial = 0i64;
    let mut fw_map: std::collections::HashMap<String, Vec<GapView>> = std::collections::HashMap::new();

    for row in &rows {
        let status: String = row.get("status");
        if status == "gap" { total_gaps += 1; } else { total_partial += 1; }
        let fw: String = row.get("framework");
        let target: Option<chrono::NaiveDate> = row.get("target_date");

        fw_map.entry(fw).or_default().push(GapView {
            reference_id: row.get("reference_id"),
            title: row.get("title"),
            status,
            notes: row.get("assessor_notes"),
            remediation: row.get("remediation_plan"),
            target_date: target.map(|d| d.to_string()).unwrap_or_default(),
        });
    }

    let frameworks: Vec<GapFrameworkView> = fw_map.into_iter().map(|(fw_str, gaps)| {
        let fw: Framework = parse_enum(&fw_str);
        GapFrameworkView { framework_name: fw.display_name().to_string(), gaps }
    }).collect();

    let tmpl = GapAnalysisTemplate {
        engagement_name: eng.name.clone(),
        generated_at: Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
        total_gaps,
        total_partial,
        frameworks,
    };

    tmpl.render().map_err(|e| e.to_string())
}

async fn generate_risk(
    pool: &PgPool,
    eng: &Engagement,
    ai_system_id: Uuid,
) -> Result<String, String> {
    let sys = ai_system::get(pool, ai_system_id).await?;
    let entries = risk::list(pool, Some(ai_system_id)).await?;

    let risks: Vec<RiskView> = entries.iter().map(|r| {
        let score = r.inherent_score;
        let severity = risk_severity(score).to_string();
        let color = match score {
            1..=4 => "#16a34a",
            5..=9 => "#ca8a04",
            10..=14 => "#ea580c",
            15..=19 => "#dc2626",
            _ => "#991b1b",
        };
        RiskView {
            title: r.title.clone(),
            description: r.description.clone(),
            likelihood: format!("{:?}", r.likelihood),
            impact: format!("{:?}", r.impact),
            score,
            severity,
            severity_color: color.to_string(),
            mitigation: r.mitigation_measures.clone(),
            status: format!("{:?}", r.status),
        }
    }).collect();

    let tmpl = RiskReportTemplate {
        engagement_name: eng.name.clone(),
        system_name: sys.name,
        generated_at: Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
        risks,
    };

    tmpl.render().map_err(|e| e.to_string())
}
