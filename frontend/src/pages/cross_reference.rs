use leptos::*;
use grc_shared::models::{AiSystem, Engagement, FrameworkRequirement, CrossReferenceExpanded, RequirementAssessment};
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngIdArg { engagement_id: String }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssessmentListArg { ai_system_id: String, framework: Option<String> }

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
    let (selected_eng_id, set_selected_eng_id) = create_signal(String::new());
    let (selected_sys_id, set_selected_sys_id) = create_signal(String::new());

    // Load engagements and systems for assessment context
    let engagements = create_resource(|| (), |_| async {
        invoke::call_no_args::<Vec<Engagement>>("list_engagements").await
    });

    let ai_systems = create_resource(
        move || selected_eng_id.get(),
        |eid| async move {
            if eid.is_empty() { return Ok(vec![] as Vec<AiSystem>); }
            invoke::call::<_, Vec<AiSystem>>("list_ai_systems", &EngIdArg { engagement_id: eid }).await
        },
    );

    let assessments = create_resource(
        move || selected_sys_id.get(),
        |sid| async move {
            if sid.is_empty() { return Ok(vec![] as Vec<RequirementAssessment>); }
            invoke::call::<_, Vec<RequirementAssessment>>(
                "list_assessments",
                &AssessmentListArg { ai_system_id: sid, framework: None },
            ).await
        },
    );

    // Build a lookup: requirement_id → status string
    let assessment_map = create_memo(move |_| {
        let mut map = HashMap::<String, String>::new();
        if let Some(Ok(list)) = assessments.get() {
            for a in list {
                let st = serde_json::to_value(&a.status).ok()
                    .and_then(|v| v.as_str().map(String::from))
                    .unwrap_or_else(|| "not_assessed".to_string());
                map.insert(a.requirement_id.to_string(), st);
            }
        }
        map
    });

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

            // AI System selector for assessment context
            <div style="display:flex;gap:1rem;margin-bottom:1rem;flex-wrap:wrap;">
                <label style="flex:1;min-width:200px;">"Engagement"
                    <Suspense fallback=move || view! { <select disabled><option>"Loading…"</option></select> }>
                        {move || engagements.get().map(|r| {
                            let list = r.unwrap_or_default();
                            view! {
                                <select prop:value=selected_eng_id on:change=move |e| {
                                    set_selected_eng_id.set(event_target_value(&e));
                                    set_selected_sys_id.set(String::new());
                                }>
                                    <option value="">"— Optional: select for status —"</option>
                                    {list.into_iter().map(|eng| {
                                        let id = eng.id.to_string();
                                        view! { <option value=id.clone()>{eng.name}</option> }
                                    }).collect_view()}
                                </select>
                            }
                        })}
                    </Suspense>
                </label>
                <label style="flex:1;min-width:200px;">"AI System"
                    <Suspense fallback=move || view! { <select disabled><option>"—"</option></select> }>
                        {move || ai_systems.get().map(|r| {
                            let list = r.unwrap_or_default();
                            view! {
                                <select prop:value=selected_sys_id on:change=move |e| set_selected_sys_id.set(event_target_value(&e))>
                                    <option value="">"— Select —"</option>
                                    {list.into_iter().map(|sys| {
                                        let id = sys.id.to_string();
                                        view! { <option value=id.clone()>{sys.name}</option> }
                                    }).collect_view()}
                                </select>
                            }
                        })}
                    </Suspense>
                </label>
            </div>

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
                                let amap = assessment_map.get();
                                let rel_str = serde_json::to_value(&cr.relationship)
                                    .ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default();
                                let fw_str = serde_json::to_value(&cr.target.framework)
                                    .ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default();
                                let target_status = amap.get(&cr.target.id.to_string()).cloned();
                                view! {
                                <article class=format!("cross-ref-card relationship-{}", rel_str)>
                                    <header>
                                        <StatusBadge status=rel_str.clone() />
                                        <strong>{&cr.target.reference_id}</strong>" — "{&cr.target.title}
                                        {target_status.map(|st| view! {
                                            <span style="margin-left:auto;"><StatusBadge status=st /></span>
                                        })}
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
