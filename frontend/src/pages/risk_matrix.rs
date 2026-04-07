use leptos::*;
use leptos_router::*;
use grc_shared::models::RiskEntry;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::api::invoke;
use crate::components::risk_heatmap::RiskHeatmap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RiskMatrixArg {
    engagement_id: Option<String>,
    ai_system_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct RiskMatrixData {
    entries: Vec<RiskEntry>,
    matrix: [[Vec<Uuid>; 5]; 5],
}

#[component]
pub fn RiskMatrixPage() -> impl IntoView {
    let params = use_params_map();
    let sys_id = move || params.with(|p| p.get("sys_id").cloned());

    let matrix_data = create_resource(
        sys_id,
        |sid| async move {
            invoke::call::<_, RiskMatrixData>(
                "get_risk_matrix_data",
                &RiskMatrixArg { engagement_id: None, ai_system_id: sid },
            ).await
        },
    );

    view! {
        <div class="page risk-matrix-page">
            <h1>"Risk Matrix"</h1>
            <Suspense fallback=move || view! { <p>"Loading risk data…"</p> }>
                {move || matrix_data.get().map(|result| match result {
                    Ok(data) => {
                        let severity_counts = |min: usize, max: usize| {
                            data.entries.iter().filter(|e| {
                                let s = e.inherent_score as usize;
                                s >= min && s <= max
                            }).count()
                        };
                        view! {
                            <div class="risk-summary">
                                <span class="risk-low">{format!("Low: {}", severity_counts(1, 4))}</span>
                                <span class="risk-medium">{format!("Medium: {}", severity_counts(5, 9))}</span>
                                <span class="risk-high">{format!("High: {}", severity_counts(10, 14))}</span>
                                <span class="risk-very-high">{format!("Very High: {}", severity_counts(15, 19))}</span>
                                <span class="risk-critical">{format!("Critical: {}", severity_counts(20, 25))}</span>
                            </div>
                            <RiskHeatmap matrix=data.matrix.clone() />
                            <section class="risk-entries">
                                <h2>"Risk Register"</h2>
                                <table role="grid">
                                    <thead><tr>
                                        <th>"Title"</th><th>"Score"</th><th>"Status"</th><th>"Priority"</th>
                                    </tr></thead>
                                    <tbody>
                                        {data.entries.iter().map(|r| {
                                            let severity = match r.inherent_score {
                                                1..=4 => "risk-low",
                                                5..=9 => "risk-medium",
                                                10..=14 => "risk-high",
                                                15..=19 => "risk-very-high",
                                                _ => "risk-critical",
                                            };
                                            view! {
                                                <tr>
                                                    <td>{&r.title}</td>
                                                    <td class=severity>{r.inherent_score.to_string()}</td>
                                                    <td>{format!("{:?}", r.status)}</td>
                                                    <td>{format!("{:?}", r.priority)}</td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </section>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
