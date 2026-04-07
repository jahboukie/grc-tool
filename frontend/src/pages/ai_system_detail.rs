use leptos::*;
use leptos_router::*;
use grc_shared::models::AiSystem;
use serde::Serialize;

use crate::api::invoke;
use crate::components::status_badge::StatusBadge;

#[derive(Serialize)]
struct IdArg { id: String }

#[component]
pub fn AiSystemDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let system = create_resource(id, |id| async move {
        invoke::call::<_, AiSystem>("get_ai_system", &IdArg { id }).await
    });

    view! {
        <div class="page ai-system-detail-page">
            <Suspense fallback=move || view! { <p>"Loading system…"</p> }>
                {move || system.get().map(|result| match result {
                    Ok(s) => {
                        let risk_str = serde_json::to_value(&s.risk_category)
                            .ok()
                            .and_then(|v| v.as_str().map(String::from))
                            .unwrap_or_default();
                        view! {
                            <header class="detail-header">
                                <h1>{&s.name}</h1>
                                <StatusBadge status=risk_str.clone() />
                            </header>
                            <section class="classification-panel">
                                <h2>"Classification"</h2>
                                <dl>
                                    <dt>"Risk Category"</dt><dd><StatusBadge status=risk_str /></dd>
                                    <dt>"Domain"</dt><dd>{&s.domain}</dd>
                                    <dt>"Intended Purpose"</dt><dd>{&s.intended_purpose}</dd>
                                    <dt>"GPAI"</dt><dd>{if s.is_gpai { "Yes" } else { "No" }}</dd>
                                    <dt>"Annex III Listed"</dt><dd>{if s.is_high_risk_listed { "Yes" } else { "No" }}</dd>
                                    <dt>"Safety Component"</dt><dd>{if s.is_safety_component { "Yes" } else { "No" }}</dd>
                                </dl>
                            </section>
                            <section class="deployment-section">
                                <h2>"Deployment Context"</h2>
                                <p>{&s.deployment_context}</p>
                            </section>
                            <section class="detail-actions">
                                <h2>"Workflow"</h2>
                                <p>
                                    <A href=format!("/fria/{}", s.id)>"Open FRIA workspace"</A>
                                </p>
                            </section>
                            <section class="description-section">
                                <h2>"Description"</h2>
                                <p>{&s.description}</p>
                            </section>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
