use leptos::*;
use grc_shared::models::{FrameworkRequirement, CrossReferenceExpanded};
use serde::Serialize;

use crate::api::invoke;
use crate::components::framework_pill::FrameworkPill;
use crate::components::status_badge::StatusBadge;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CrossRefArg { requirement_id: String }

#[derive(Serialize)]
struct SearchArg { query: String }

#[component]
pub fn CrossReferencePage() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_req, set_selected_req) = create_signal::<Option<FrameworkRequirement>>(None);

    let search_results = create_resource(
        move || search_query.get(),
        |query| async move {
            if query.len() < 2 { return Ok(vec![]); }
            invoke::call::<_, Vec<FrameworkRequirement>>("search_requirements", &SearchArg { query }).await
        },
    );

    let cross_refs = create_resource(
        move || selected_req.get().map(|r| r.id.to_string()),
        |maybe_id| async move {
            match maybe_id {
                Some(id) => invoke::call::<_, Vec<CrossReferenceExpanded>>(
                    "get_cross_references",
                    &CrossRefArg { requirement_id: id },
                ).await,
                None => Ok(vec![]),
            }
        },
    );

    view! {
        <div class="page cross-reference-page">
            <h1>"Cross-Reference Map"</h1>
            <div class="search-bar">
                <input
                    type="search"
                    placeholder="Search for a requirement (e.g. Art. 9, risk management)…"
                    prop:value=search_query
                    on:input=move |e| set_search_query.set(event_target_value(&e))
                />
            </div>

            <Suspense fallback=move || view! { <span /> }>
                {move || search_results.get().map(|result| match result {
                    Ok(reqs) if !reqs.is_empty() && selected_req.get().is_none() => view! {
                        <ul class="search-results">
                            {reqs.into_iter().map(|r| {
                                let r2 = r.clone();
                                view! {
                                    <li on:click=move |_| set_selected_req.set(Some(r2.clone()))>
                                        <strong>{&r.reference_id}</strong>" — "{&r.title}
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_view(),
                    _ => view! { <span /> }.into_view(),
                })}
            </Suspense>

            {move || selected_req.get().map(|req| {
                let fw = serde_json::to_value(&req.framework)
                    .ok()
                    .and_then(|v| v.as_str().map(String::from))
                    .unwrap_or_default();
                view! {
                    <article class="source-requirement">
                        <header>
                            <strong>{&req.reference_id}</strong>" — "{&req.title}
                        </header>
                        <div>
                            <FrameworkPill framework=fw />
                            " | "{&req.article_clause}
                        </div>
                        <button on:click=move |_| set_selected_req.set(None)>"Clear"</button>
                    </article>
                }
            })}

            <Suspense fallback=move || view! { <span /> }>
                {move || cross_refs.get().map(|result| match result {
                    Ok(refs) if !refs.is_empty() => view! {
                        <h2>"Cross-Referenced Requirements"</h2>
                        <div class="cross-ref-list">
                            {refs.into_iter().map(|cr| {
                                let rel_str = serde_json::to_value(&cr.relationship)
                                    .ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default();
                                let fw_str = serde_json::to_value(&cr.target.framework)
                                    .ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default();
                                view! {
                                <article class=format!("cross-ref-card relationship-{}", rel_str)>
                                    <header>
                                        <StatusBadge status=rel_str.clone() />
                                        <strong>{&cr.target.reference_id}</strong>" — "{&cr.target.title}
                                    </header>
                                    <div>
                                        <FrameworkPill framework=fw_str />
                                        " | "{&cr.target.article_clause}
                                    </div>
                                    <p class="cross-ref-notes">{&cr.notes}</p>
                                </article>
                            }}).collect_view()}
                        </div>
                    }.into_view(),
                    Ok(_) if selected_req.get().is_some() => view! {
                        <p>"No cross-references found for this requirement."</p>
                    }.into_view(),
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                    _ => view! { <span /> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
