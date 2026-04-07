use leptos::*;
use grc_shared::models::{ReportRequest, ReportResult, ReportType};

use crate::api::invoke;

#[component]
pub fn ReportsPage() -> impl IntoView {
    let (message, set_message) = create_signal::<Option<String>>(None);
    let (generating, set_generating) = create_signal(false);

    let generate = move |rt: ReportType| {
        set_generating.set(true);
        set_message.set(None);
        spawn_local(async move {
            let req = ReportRequest {
                report_type: rt,
                engagement_id: uuid::Uuid::nil(),
                ai_system_id: None,
            };
            match invoke::call_named::<_, ReportResult>("generate_report", "request", &req).await {
                Ok(result) => set_message.set(Some(format!("Report generated: {}", result.file_path))),
                Err(e) => set_message.set(Some(format!("Error: {}", e))),
            }
            set_generating.set(false);
        });
    };

    view! {
        <div class="page reports-page">
            <h1>"Reports"</h1>
            <p>"Generate HTML compliance reports that open in your default browser."</p>
            <div class="report-actions">
                <article class="report-option">
                    <h3>"Compliance Report"</h3>
                    <p>"Full compliance status across all frameworks for an engagement."</p>
                    <button disabled=generating on:click=move |_| generate(ReportType::FullCompliance)>"Generate"</button>
                </article>
                <article class="report-option">
                    <h3>"Gap Analysis Report"</h3>
                    <p>"All gaps and partial assessments with remediation plans."</p>
                    <button disabled=generating on:click=move |_| generate(ReportType::GapAnalysis)>"Generate"</button>
                </article>
                <article class="report-option">
                    <h3>"Risk Report"</h3>
                    <p>"Risk register with heat map and mitigation status."</p>
                    <button disabled=generating on:click=move |_| generate(ReportType::RiskAssessment)>"Generate"</button>
                </article>
            </div>
            {move || message.get().map(|m| view! { <p class="report-message">{m}</p> })}
        </div>
    }
}
