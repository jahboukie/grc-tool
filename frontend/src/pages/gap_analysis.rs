use leptos::*;
use serde::Serialize;
use grc_shared::models::{AiSystem, Engagement, GapAnalysisData, ReportRequest, ReportResult, ReportType};

use crate::api::invoke;
use crate::components::framework_pill::FrameworkPill;
use crate::components::status_badge::StatusBadge;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GapArg {
    engagement_id: Option<String>,
    ai_system_id: Option<String>,
}

#[derive(Serialize)]
struct EngIdArg { engagement_id: String }

#[component]
pub fn GapAnalysisPage() -> impl IntoView {
    let (sel_eng, set_sel_eng) = create_signal(String::new());
    let (sel_sys, set_sel_sys) = create_signal(String::new());
    let (status_filter, set_status_filter) = create_signal(String::from("all"));

    let engagements = create_resource(|| (), |_| async {
        invoke::call_no_args::<Vec<Engagement>>("list_engagements").await
    });

    let systems = create_resource(
        move || sel_eng.get(),
        move |eid: String| async move {
            if eid.is_empty() { return Ok(vec![] as Vec<AiSystem>); }
            invoke::call::<_, Vec<AiSystem>>("list_ai_systems", &EngIdArg { engagement_id: eid }).await
        },
    );

    let gap_data = create_resource(
        move || (sel_eng.get(), sel_sys.get()),
        |(eid, sid)| async move {
            invoke::call::<_, GapAnalysisData>(
                "get_gap_analysis",
                &GapArg {
                    engagement_id: if eid.is_empty() { None } else { Some(eid) },
                    ai_system_id: if sid.is_empty() { None } else { Some(sid) },
                },
            ).await
        },
    );

    let (report_msg, set_report_msg) = create_signal(Option::<String>::None);

    view! {
        <div class="page gap-analysis-page">
            <div style="display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;">
                <h1>"Gap Analysis"</h1>
                <button class="outline" style="white-space:nowrap;" on:click=move |_| {
                    let eid = sel_eng.get();
                    if eid.is_empty() {
                        set_report_msg.set(Some("Select an engagement first.".into()));
                        return;
                    }
                    let sid = sel_sys.get();
                    set_report_msg.set(Some("Generating…".into()));
                    spawn_local(async move {
                        let req = ReportRequest {
                            report_type: ReportType::GapAnalysis,
                            engagement_id: uuid::Uuid::parse_str(&eid).unwrap_or_default(),
                            ai_system_id: if sid.is_empty() { None } else { uuid::Uuid::parse_str(&sid).ok() },
                        };
                        match invoke::call_named::<_, ReportResult>("generate_report", "request", &req).await {
                            Ok(r) => set_report_msg.set(Some(format!("Report opened: {}", r.file_path))),
                            Err(e) => set_report_msg.set(Some(format!("Error: {}", e))),
                        }
                    });
                }>"Generate Gap Report"</button>
            </div>
            {move || report_msg.get().map(|m| view! { <p style="font-size:0.85rem;">{m}</p> })}
            <div style="display:flex;gap:1rem;align-items:end;flex-wrap:wrap;margin-bottom:1rem;">
                <label style="flex:1;min-width:200px;">"Engagement"
                    <Suspense fallback=move || view! { <select disabled><option>"Loading…"</option></select> }>
                        {move || engagements.get().map(|r| {
                            let list = r.unwrap_or_default();
                            view! {
                                <select prop:value=sel_eng on:change=move |e| {
                                    let v = event_target_value(&e);
                                    set_sel_eng.set(v);
                                    set_sel_sys.set(String::new());
                                }>
                                    <option value="">"All Engagements"</option>
                                    {list.into_iter().map(|eng| {
                                        let id = eng.id.to_string();
                                        view! { <option value=id.clone()>{eng.client_name}</option> }
                                    }).collect_view()}
                                </select>
                            }
                        })}
                    </Suspense>
                </label>
                <label style="flex:1;min-width:200px;">"AI System"
                    <Suspense fallback=move || view! { <select disabled><option>"—"</option></select> }>
                        {move || systems.get().map(|r: Result<Vec<AiSystem>, String>| {
                            let list = r.unwrap_or_default();
                            view! {
                                <select prop:value=sel_sys on:change=move |e| set_sel_sys.set(event_target_value(&e))>
                                    <option value="">"All Systems"</option>
                                    {list.into_iter().map(|sys| {
                                        let id = sys.id.to_string();
                                        view! { <option value=id.clone()>{sys.name}</option> }
                                    }).collect_view()}
                                </select>
                            }
                        })}
                    </Suspense>
                </label>
                <label style="min-width:150px;">"Status"
                    <select prop:value=status_filter on:change=move |e| set_status_filter.set(event_target_value(&e))>
                        <option value="all">"All"</option>
                        <option value="gap">"Gap"</option>
                        <option value="partial">"Partial"</option>
                        <option value="not_assessed">"Not Assessed"</option>
                    </select>
                </label>
            </div>
            <Suspense fallback=move || view! { <p>"Loading gap analysis…"</p> }>
                {move || gap_data.get().map(|result| match result {
                    Ok(data) => {
                        let sf = status_filter.get();
                        let total_by_fw: Vec<_> = data.frameworks.iter().map(|fwg| {
                            let gap_count = fwg.gaps.iter().filter(|e| {
                                serde_json::to_value(&e.assessment.status).ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .map(|s| s == "gap")
                                    .unwrap_or(false)
                            }).count();
                            (fwg.framework_name.clone(), fwg.gaps.len(), gap_count)
                        }).collect();
                        view! {
                        <div class="gap-summary" style="margin-bottom:1rem;">
                            <strong>{format!("{} gaps | {} partial", data.total_gaps, data.total_partial)}</strong>
                            <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.5rem;font-size:0.85rem;">
                                {total_by_fw.into_iter().map(|(name, total, gaps)| {
                                    let pct = if total > 0 { (gaps as f64 / total as f64 * 100.0) as u32 } else { 0 };
                                    view! { <span>{format!("{}: {} gaps ({}%)", name, gaps, pct)}</span> }
                                }).collect_view()}
                            </div>
                        </div>
                        {data.frameworks.into_iter().map(|fwg| {
                            let sf2 = sf.clone();
                            let fw_str = serde_json::to_value(&fwg.framework)
                                .ok()
                                .and_then(|v| v.as_str().map(String::from))
                                .unwrap_or_default();
                            let filtered: Vec<_> = fwg.gaps.into_iter().filter(|entry| {
                                if sf2 == "all" { return true; }
                                let st = serde_json::to_value(&entry.assessment.status)
                                    .ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default();
                                st == sf2
                            }).collect();
                            if filtered.is_empty() { return view! {}.into_view(); }
                            view! {
                            <details open>
                                <summary>
                                    <FrameworkPill framework=fw_str />
                                    {format!(" — {} items", filtered.len())}
                                </summary>
                                <div class="gap-entries">
                                    {filtered.into_iter().map(|entry| {
                                        let status_str = serde_json::to_value(&entry.assessment.status)
                                            .ok()
                                            .and_then(|v| v.as_str().map(String::from))
                                            .unwrap_or_default();
                                        let ref_id = entry.requirement.reference_id.clone();
                                        let title = entry.requirement.title.clone();
                                        let notes = entry.assessment.assessor_notes.clone();
                                        let remediation = entry.assessment.remediation_plan.clone();
                                        let target = entry.assessment.target_date.map(|d| d.to_string());
                                        let xrefs = entry.cross_references;
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
                                            {(!xrefs.is_empty()).then(|| {
                                                let items: Vec<_> = xrefs.into_iter().map(|cr| {
                                                    let rel = serde_json::to_value(&cr.relationship).ok()
                                                        .and_then(|v| v.as_str().map(String::from))
                                                        .unwrap_or_default();
                                                    let target_id = cr.target_requirement_id.to_string();
                                                    (rel, target_id, cr.notes)
                                                }).collect();
                                                view! {
                                                    <div style="font-size:0.85rem;margin-top:0.5rem;color:var(--pico-muted-color);">
                                                        <strong>"Cross-refs: "</strong>
                                                        {items.into_iter().map(|(rel, tid, notes)| {
                                                            let label = if notes.is_empty() {
                                                                format!("{} ({})", tid, rel)
                                                            } else {
                                                                format!("{} ({}) — {}", tid, rel, notes)
                                                            };
                                                            view! { <span style="margin-right:1rem;">{label}</span> }
                                                        }).collect_view()}
                                                    </div>
                                                }
                                            })}
                                        </article>
                                    }}).collect_view()}
                                </div>
                            </details>
                        }.into_view()}).collect_view()}
                    }.into_view()},
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
