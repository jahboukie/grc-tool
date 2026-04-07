use leptos::*;
use serde::Serialize;
use grc_shared::models::GapAnalysisData;

use crate::api::invoke;
use crate::components::framework_pill::FrameworkPill;
use crate::components::status_badge::StatusBadge;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GapArg {
    engagement_id: Option<String>,
    ai_system_id: Option<String>,
}

#[component]
pub fn GapAnalysisPage() -> impl IntoView {
    let gap_data = create_resource(|| (), |_| async {
        invoke::call::<_, GapAnalysisData>(
            "get_gap_analysis",
            &GapArg { engagement_id: None, ai_system_id: None },
        ).await
    });

    view! {
        <div class="page gap-analysis-page">
            <h1>"Gap Analysis"</h1>
            <Suspense fallback=move || view! { <p>"Loading gap analysis…"</p> }>
                {move || gap_data.get().map(|result| match result {
                    Ok(data) => view! {
                        <div class="gap-summary">
                            <strong>{format!("{} gaps | {} partial", data.total_gaps, data.total_partial)}</strong>
                        </div>
                        {data.frameworks.into_iter().map(|fwg| {
                            let fw_str = serde_json::to_value(&fwg.framework)
                                .ok()
                                .and_then(|v| v.as_str().map(String::from))
                                .unwrap_or_default();
                            view! {
                            <details open>
                                <summary>
                                    <FrameworkPill framework=fw_str />
                                    {format!(" — {} items", fwg.gaps.len())}
                                </summary>
                                <div class="gap-entries">
                                    {fwg.gaps.into_iter().map(|entry| {
                                        let status_str = serde_json::to_value(&entry.assessment.status)
                                            .ok()
                                            .and_then(|v| v.as_str().map(String::from))
                                            .unwrap_or_default();
                                        let ref_id = entry.requirement.reference_id.clone();
                                        let title = entry.requirement.title.clone();
                                        let notes = entry.assessment.assessor_notes.clone();
                                        let remediation = entry.assessment.remediation_plan.clone();
                                        let target = entry.assessment.target_date.map(|d| d.to_string());
                                        view! {
                                        <article class="gap-entry">
                                            <header>
                                                <StatusBadge status=status_str />
                                                <strong>{ref_id}</strong>
                                                " "{title}
                                            </header>
                                            {(!notes.is_empty()).then(|| view! {
                                                <p class="gap-notes">"Notes: "{notes}</p>
                                            })}
                                            {(!remediation.is_empty()).then(|| view! {
                                                <p class="gap-remediation">"Remediation: "{remediation}</p>
                                            })}
                                            {target.map(|d| view! {
                                                <p class="gap-target">"Target: "{d}</p>
                                            })}
                                        </article>
                                    }}).collect_view()}
                                </div>
                            </details>
                        }}).collect_view()}
                    }.into_view(),
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
