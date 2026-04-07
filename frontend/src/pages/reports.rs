use leptos::*;
use grc_shared::models::{AiSystem, Engagement, ReportRequest, ReportResult, ReportType};
use serde::Serialize;

use crate::api::invoke;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementFilter {
    status_filter: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementIdArg {
    engagement_id: String,
}

#[component]
pub fn ReportsPage() -> impl IntoView {
    let (message, set_message) = create_signal::<Option<String>>(None);
    let (generating, set_generating) = create_signal(false);
    let (selected_engagement_id, set_selected_engagement_id) = create_signal(Option::<String>::None);
    let (selected_system_id, set_selected_system_id) = create_signal(Option::<String>::None);

    let engagements = create_resource(|| (), |_| async {
        invoke::call::<_, Vec<Engagement>>(
            "list_engagements",
            &EngagementFilter { status_filter: None },
        ).await
    });

    let ai_systems = create_resource(
        move || selected_engagement_id.get(),
        |eid| async move {
            match eid {
                Some(id) => invoke::call::<_, Vec<AiSystem>>(
                    "list_ai_systems",
                    &EngagementIdArg { engagement_id: id },
                ).await,
                None => Ok(vec![]),
            }
        },
    );

    // Auto-select first engagement
    create_effect(move |_| {
        if selected_engagement_id.get().is_none() {
            if let Some(Ok(list)) = engagements.get() {
                if let Some(first) = list.first() {
                    set_selected_engagement_id.set(Some(first.id.to_string()));
                }
            }
        }
    });

    let generate = move |rt: ReportType| {
        let Some(eid_str) = selected_engagement_id.get() else {
            set_message.set(Some("Select an engagement first.".into()));
            return;
        };
        let Ok(eid) = uuid::Uuid::parse_str(&eid_str) else {
            set_message.set(Some("Invalid engagement ID.".into()));
            return;
        };
        let sid = selected_system_id.get().and_then(|s| uuid::Uuid::parse_str(&s).ok());

        // Risk report requires an AI system
        if matches!(rt, ReportType::RiskAssessment) && sid.is_none() {
            set_message.set(Some("Risk report requires an AI system to be selected.".into()));
            return;
        }

        set_generating.set(true);
        set_message.set(None);
        spawn_local(async move {
            let req = ReportRequest {
                report_type: rt,
                engagement_id: eid,
                ai_system_id: sid,
            };
            match invoke::call_named::<_, ReportResult>("generate_report", "request", &req).await {
                Ok(result) => set_message.set(Some(format!("Report opened: {}", result.file_path))),
                Err(e) => set_message.set(Some(format!("Error: {}", e))),
            }
            set_generating.set(false);
        });
    };

    view! {
        <div class="page reports-page">
            <h1>"Reports"</h1>
            <p>"Generate HTML compliance reports that open in your default browser."</p>

            <div class="scope-selectors" style="display:flex;gap:1rem;align-items:end;margin-bottom:1.5rem;">
                <label style="flex:1">"Engagement"
                    <Suspense fallback=move || view! { <select disabled><option>"Loading…"</option></select> }>
                        {move || engagements.get().map(|result| match result {
                            Ok(engs) => view! {
                                <select on:change=move |e| {
                                    let v = event_target_value(&e);
                                    if v.is_empty() { set_selected_engagement_id.set(None); set_selected_system_id.set(None); }
                                    else { set_selected_engagement_id.set(Some(v)); set_selected_system_id.set(None); }
                                }>
                                    <option value="">"Select engagement…"</option>
                                    {engs.into_iter().map(|eng| {
                                        let id = eng.id.to_string();
                                        let selected = selected_engagement_id.get().as_deref() == Some(id.as_str());
                                        view! { <option value=id.clone() selected=selected>{eng.name}</option> }
                                    }).collect_view()}
                                </select>
                            }.into_view(),
                            Err(_) => view! { <select disabled><option>"Error"</option></select> }.into_view(),
                        })}
                    </Suspense>
                </label>
                <label style="flex:1">"AI System (optional, required for Risk)"
                    <Suspense fallback=move || view! { <select disabled><option>"—"</option></select> }>
                        {move || ai_systems.get().map(|result| match result {
                            Ok(systems) => view! {
                                <select on:change=move |e| {
                                    let v = event_target_value(&e);
                                    if v.is_empty() { set_selected_system_id.set(None); }
                                    else { set_selected_system_id.set(Some(v)); }
                                }>
                                    <option value="">"All systems"</option>
                                    {systems.into_iter().map(|s| {
                                        let id = s.id.to_string();
                                        view! { <option value=id.clone()>{s.name}</option> }
                                    }).collect_view()}
                                </select>
                            }.into_view(),
                            Err(_) => view! { <select disabled><option>"—"</option></select> }.into_view(),
                        })}
                    </Suspense>
                </label>
            </div>

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
                    <button disabled=move || generating.get() || selected_system_id.get().is_none() on:click=move |_| generate(ReportType::RiskAssessment)>"Generate"</button>
                </article>
            </div>
            {move || message.get().map(|m| view! { <p class="report-message">{m}</p> })}
        </div>
    }
}
