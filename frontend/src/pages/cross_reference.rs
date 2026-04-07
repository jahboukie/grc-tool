use leptos::*;
use grc_shared::models::{FrameworkRequirement, CrossReferenceExpanded};
use serde::Serialize;
use std::collections::HashMap;

use crate::api::invoke;
use crate::components::framework_pill::FrameworkPill;
use crate::components::status_badge::StatusBadge;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CrossRefArg { requirement_id: String }

#[derive(Serialize)]
struct SearchArg { query: String }

const FRAMEWORKS: &[&str] = &[
    "eu_ai_act", "iso_42001", "iso_23894", "nist_ai_rmf", "oecd_ai",
];

const FRAMEWORK_LABELS: &[&str] = &[
    "EU AI Act", "ISO 42001", "ISO 23894", "NIST AI RMF", "OECD AI",
];

fn framework_str(req: &FrameworkRequirement) -> String {
    serde_json::to_value(&req.framework)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default()
}

#[component]
pub fn CrossReferencePage() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_req, set_selected_req) = create_signal::<Option<FrameworkRequirement>>(None);

    // Matrix overview data
    let all_refs = create_resource(|| (), |_| async {
        invoke::call_no_args::<Vec<CrossReferenceExpanded>>("list_cross_references").await
    });

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

            // Matrix overview
            <Suspense fallback=move || view! { <p>"Loading matrix…"</p> }>
                {move || all_refs.get().map(|result| match result {
                    Ok(refs) => {
                        // Build counts: (source_fw, target_fw) -> count
                        let mut counts: HashMap<(String, String), u32> = HashMap::new();
                        for cr in &refs {
                            let s = framework_str(&cr.source);
                            let t = framework_str(&cr.target);
                            *counts.entry((s.clone(), t.clone())).or_default() += 1;
                            *counts.entry((t, s)).or_default() += 1; // bidirectional display
                        }
                        view! {
                            <div class="cross-ref-matrix" style="overflow-x:auto;margin-bottom:1.5rem;">
                                <table role="grid" style="font-size:0.85rem;">
                                    <thead><tr>
                                        <th></th>
                                        {FRAMEWORK_LABELS.iter().map(|l| view! { <th style="text-align:center">{*l}</th> }).collect_view()}
                                    </tr></thead>
                                    <tbody>
                                        {FRAMEWORKS.iter().zip(FRAMEWORK_LABELS.iter()).map(|(fw, label)| {
                                            view! {
                                                <tr>
                                                    <th>{*label}</th>
                                                    {FRAMEWORKS.iter().map(|fw2| {
                                                        let c = counts.get(&(fw.to_string(), fw2.to_string())).copied().unwrap_or(0);
                                                        let style = if *fw == *fw2 {
                                                            "text-align:center;background:var(--pico-muted-border-color)"
                                                        } else if c > 0 {
                                                            "text-align:center;font-weight:bold"
                                                        } else {
                                                            "text-align:center;opacity:0.3"
                                                        };
                                                        view! { <td style=style>{if *fw == *fw2 { "—".into() } else { c.to_string() }}</td> }
                                                    }).collect_view()}
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                                <p style="margin-top:0.5rem;font-size:0.8rem;opacity:0.7">{format!("{} cross-references across {} frameworks", refs.len(), FRAMEWORKS.len())}</p>
                            </div>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>

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
