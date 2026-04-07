use leptos::*;
use leptos_router::*;
use grc_shared::models::{
    AiSystem, CrossReferenceExpanded, Engagement, FrameworkRequirement,
    RequirementAssessment, UpsertAssessmentDto,
};
use grc_shared::enums::ComplianceStatus;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::api::invoke;
use crate::components::framework_pill::FrameworkPill;
use crate::components::requirement_row::RequirementRow;

#[derive(Serialize)]
struct RequirementFilter {
    framework: Option<String>,
    category: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementIdArg {
    engagement_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListAssessmentsArg {
    ai_system_id: String,
    framework: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CrossRefArg {
    requirement_id: String,
}

const STATUS_OPTIONS: &[(&str, &str)] = &[
    ("not_assessed", "Not Assessed"),
    ("met", "Met"),
    ("partial", "Partial"),
    ("gap", "Gap"),
    ("not_applicable", "N/A"),
];

fn status_from_str(s: &str) -> ComplianceStatus {
    match s {
        "met" => ComplianceStatus::Met,
        "partial" => ComplianceStatus::Partial,
        "gap" => ComplianceStatus::Gap,
        "not_applicable" => ComplianceStatus::NotApplicable,
        _ => ComplianceStatus::NotAssessed,
    }
}

fn status_to_str(s: &ComplianceStatus) -> &'static str {
    match s {
        ComplianceStatus::Met => "met",
        ComplianceStatus::Partial => "partial",
        ComplianceStatus::Gap => "gap",
        ComplianceStatus::NotApplicable => "not_applicable",
        ComplianceStatus::NotAssessed => "not_assessed",
    }
}

#[component]
pub fn FrameworkNavigatorPage() -> impl IntoView {
    let params = use_params_map();
    let fw_param = move || params.with(|p| p.get("fw").cloned());

    let (selected_fw, set_selected_fw) = create_signal(fw_param());
    let (search_query, set_search_query) = create_signal(String::new());
    let (risk_filter, set_risk_filter) = create_signal(String::new());

    // AI system selection for assessments
    let (selected_engagement_id, set_selected_engagement_id) = create_signal(Option::<String>::None);
    let (selected_system_id, set_selected_system_id) = create_signal(Option::<String>::None);

    // Selected requirement for detail panel
    let (selected_req_id, set_selected_req_id) = create_signal(Option::<Uuid>::None);

    // Assessment form state
    let (form_status, set_form_status) = create_signal("not_assessed".to_string());
    let (form_notes, set_form_notes) = create_signal(String::new());
    let (form_remediation, set_form_remediation) = create_signal(String::new());
    let (form_target_date, set_form_target_date) = create_signal(String::new());
    let (save_message, set_save_message) = create_signal(Option::<String>::None);
    let (saving, set_saving) = create_signal(false);

    // Version counter to trigger assessment reload
    let (assessments_version, set_assessments_version) = create_signal(0u32);

    let frameworks = vec![
        ("eu_ai_act", "EU AI Act"),
        ("iso_42001", "ISO 42001"),
        ("iso_23894", "ISO 23894"),
        ("nist_ai_rmf", "NIST AI RMF"),
        ("oecd_ai_principles", "OECD AI"),
    ];

    // Load engagements for the system selector
    let engagements = create_resource(
        || (),
        |_| async {
            invoke::call::<_, Vec<Engagement>>(
                "list_engagements",
                &serde_json::json!({ "statusFilter": null }),
            ).await
        },
    );

    // Load AI systems for selected engagement
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

    // Load assessments for the selected system
    let assessments = create_resource(
        move || (selected_system_id.get(), assessments_version.get()),
        |(sid, _version)| async move {
            match sid {
                Some(id) => invoke::call::<_, Vec<RequirementAssessment>>(
                    "list_assessments",
                    &ListAssessmentsArg { ai_system_id: id, framework: None },
                ).await,
                None => Ok(vec![]),
            }
        },
    );

    // Build a lookup map: requirement_id → assessment status string
    let assessment_map = create_memo(move |_| {
        let mut map = HashMap::<String, String>::new();
        if let Some(Ok(list)) = assessments.get() {
            for a in list {
                map.insert(a.requirement_id.to_string(), status_to_str(&a.status).to_string());
            }
        }
        map
    });

    let requirements = create_resource(
        move || selected_fw.get(),
        |fw| async move {
            invoke::call::<_, Vec<FrameworkRequirement>>(
                "list_requirements",
                &RequirementFilter { framework: fw, category: None },
            ).await
        },
    );

    // Load cross-references for selected requirement
    let cross_refs = create_resource(
        move || selected_req_id.get(),
        |req_id| async move {
            match req_id {
                Some(id) => invoke::call::<_, Vec<CrossReferenceExpanded>>(
                    "get_cross_references",
                    &CrossRefArg { requirement_id: id.to_string() },
                ).await.unwrap_or_default(),
                None => vec![],
            }
        },
    );

    // Store all requirements for detail panel lookup
    let (all_reqs, set_all_reqs) = create_signal(Vec::<FrameworkRequirement>::new());

    // Selected requirement for detail panel (derived signal)
    let (selected_requirement, set_selected_requirement) = create_signal(Option::<FrameworkRequirement>::None);

    // Update selected requirement when req_id or all_reqs changes
    create_effect(move |_| {
        let req_id = selected_req_id.get();
        match req_id {
            Some(id) => {
                let found = all_reqs.get().into_iter().find(|r| r.id == id);
                set_selected_requirement.set(found);
            }
            None => set_selected_requirement.set(None),
        }
    });

    // When a requirement is selected, populate form from existing assessment
    create_effect(move |_| {
        let req_id = selected_req_id.get();
        if req_id.is_none() { return; }
        let req_id = req_id.unwrap();
        let map = assessment_map.get();
        let status = map.get(&req_id.to_string())
            .cloned()
            .unwrap_or_else(|| "not_assessed".to_string());

        // Load full assessment data if it exists
        if let Some(Ok(list)) = assessments.get() {
            if let Some(existing) = list.iter().find(|a| a.requirement_id == req_id) {
                set_form_status.set(status);
                set_form_notes.set(existing.assessor_notes.clone());
                set_form_remediation.set(existing.remediation_plan.clone());
                set_form_target_date.set(
                    existing.target_date.map(|d| d.to_string()).unwrap_or_default()
                );
                set_save_message.set(None);
                return;
            }
        }
        // No existing assessment — reset form
        set_form_status.set(status);
        set_form_notes.set(String::new());
        set_form_remediation.set(String::new());
        set_form_target_date.set(String::new());
        set_save_message.set(None);
    });

    // Handle requirement click
    let on_req_click = Callback::new(move |req_id: Uuid| {
        set_selected_req_id.set(Some(req_id));
    });

    // Save assessment
    let save_assessment = move |_| {
        let system_id = match selected_system_id.get() {
            Some(id) => id,
            None => {
                set_save_message.set(Some("Select an AI system first.".into()));
                return;
            }
        };
        let req_id = match selected_req_id.get() {
            Some(id) => id,
            None => return,
        };

        let status = status_from_str(&form_status.get());
        let notes = form_notes.get();
        let remediation = form_remediation.get();
        let target_str = form_target_date.get();
        let target_date: Option<chrono::NaiveDate> = if target_str.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(&target_str, "%Y-%m-%d").ok()
        };

        set_saving.set(true);
        set_save_message.set(None);

        let sys_uuid = Uuid::parse_str(&system_id).unwrap();
        let dto = UpsertAssessmentDto {
            ai_system_id: sys_uuid,
            requirement_id: req_id,
            status,
            assessor_notes: notes,
            remediation_plan: remediation,
            target_date,
        };

        spawn_local(async move {
            match invoke::call_named::<_, RequirementAssessment>(
                "upsert_assessment", "dto", &dto,
            ).await {
                Ok(_) => {
                    set_save_message.set(Some("Assessment saved.".into()));
                    set_assessments_version.update(|v| *v += 1);
                }
                Err(e) => {
                    set_save_message.set(Some(format!("Error: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };

    view! {
        <div class="page framework-navigator-page">
            <h1>"Framework Navigator"</h1>

            // AI System selector
            <div class="system-selector">
                <Suspense fallback=|| ()>
                    {move || engagements.get().map(|result| match result {
                        Ok(engs) => {
                            view! {
                                <label>
                                    <span class="field-label">"Engagement"</span>
                                    <select on:change=move |e| {
                                        let val = event_target_value(&e);
                                        if val.is_empty() {
                                            set_selected_engagement_id.set(None);
                                            set_selected_system_id.set(None);
                                        } else {
                                            set_selected_engagement_id.set(Some(val));
                                            set_selected_system_id.set(None);
                                        }
                                        set_selected_req_id.set(None);
                                    }>
                                        <option value="">"— Select engagement —"</option>
                                        {engs.into_iter().map(|eng| {
                                            let id = eng.id.to_string();
                                            view! { <option value=id.clone()>{eng.name}</option> }
                                        }).collect_view()}
                                    </select>
                                </label>
                            }.into_view()
                        }
                        Err(_) => view! { <p class="error">"Failed to load engagements"</p> }.into_view(),
                    })}
                </Suspense>
                <Suspense fallback=|| ()>
                    {move || ai_systems.get().map(|result| match result {
                        Ok(systems) if !systems.is_empty() => {
                            view! {
                                <label>
                                    <span class="field-label">"AI System"</span>
                                    <select on:change=move |e| {
                                        let val = event_target_value(&e);
                                        if val.is_empty() {
                                            set_selected_system_id.set(None);
                                        } else {
                                            set_selected_system_id.set(Some(val));
                                        }
                                        set_selected_req_id.set(None);
                                    }>
                                        <option value="">"— Select AI system —"</option>
                                        {systems.into_iter().map(|sys| {
                                            let id = sys.id.to_string();
                                            let risk = serde_json::to_value(&sys.risk_category)
                                                .ok().and_then(|v| v.as_str().map(String::from))
                                                .unwrap_or_default();
                                            let label = format!("{} [{}]", sys.name, risk);
                                            view! { <option value=id.clone()>{label}</option> }
                                        }).collect_view()}
                                    </select>
                                </label>
                            }.into_view()
                        }
                        _ => view! {}.into_view(),
                    })}
                </Suspense>
            </div>

            // Framework tabs
            <div class="framework-tabs">
                <button
                    class=move || if selected_fw.get().is_none() { "tab active" } else { "tab" }
                    on:click=move |_| set_selected_fw.set(None)
                >
                    "All"
                </button>
                {frameworks.into_iter().map(|(key, label)| {
                    let k = key.to_string();
                    let k2 = k.clone();
                    view! {
                        <button
                            class=move || if selected_fw.get().as_deref() == Some(&k) { "tab active" } else { "tab" }
                            on:click=move |_| set_selected_fw.set(Some(k2.clone()))
                        >
                            {label}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Search + filters
            <div class="search-bar" style="display:flex;gap:1rem;align-items:end;">
                <input
                    type="search"
                    placeholder="Search requirements…"
                    prop:value=search_query
                    on:input=move |e| set_search_query.set(event_target_value(&e))
                    style="flex:2;"
                />
                <label style="flex:1;min-width:140px;">"Risk Category"
                    <select prop:value=risk_filter on:change=move |e| set_risk_filter.set(event_target_value(&e))>
                        <option value="">"All"</option>
                        <option value="unacceptable">"Unacceptable"</option>
                        <option value="high">"High"</option>
                        <option value="limited">"Limited"</option>
                        <option value="minimal">"Minimal"</option>
                        <option value="gpai">"GPAI"</option>
                    </select>
                </label>
            </div>

            // Main split: requirements list + detail panel
            <div class="navigator-split">
                // Left: requirements list
                <div class="navigator-list">
                    <Suspense fallback=move || view! { <p>"Loading requirements…"</p> }>
                        {move || requirements.get().map(|result| match result {
                            Ok(reqs) => {
                                // Store for detail panel lookup
                                set_all_reqs.set(reqs.clone());

                                let query = search_query.get().to_lowercase();
                                let map = assessment_map.get();
                                let rf = risk_filter.get();
                                let filtered: Vec<_> = reqs.into_iter().filter(|r| {
                                    if !query.is_empty() {
                                        if !(r.title.to_lowercase().contains(&query)
                                            || r.reference_id.to_lowercase().contains(&query)
                                            || r.description.to_lowercase().contains(&query))
                                        {
                                            return false;
                                        }
                                    }
                                    if !rf.is_empty() {
                                        let has_cat = r.applicable_risk_categories.iter().any(|rc| {
                                            serde_json::to_value(rc).ok()
                                                .and_then(|v| v.as_str().map(String::from))
                                                .map(|s| s == rf)
                                                .unwrap_or(false)
                                        });
                                        if !has_cat { return false; }
                                    }
                                    true
                                }).collect();

                                // Group by category
                                let mut categories: Vec<(String, Vec<FrameworkRequirement>)> = Vec::new();
                                for req in filtered {
                                    if let Some(cat) = categories.iter_mut().find(|(c, _)| *c == req.category) {
                                        cat.1.push(req);
                                    } else {
                                        categories.push((req.category.clone(), vec![req]));
                                    }
                                }

                                view! {
                                    <div class="requirements-list">
                                        {categories.into_iter().map(|(cat, reqs)| {
                                            let map = map.clone();
                                            view! {
                                                <details open>
                                                    <summary><strong>{&cat}</strong>{format!(" ({})", reqs.len())}</summary>
                                                    <table role="grid">
                                                        <thead><tr>
                                                            <th>"Ref"</th><th>"Title"</th><th>"Framework"</th>
                                                            <th>"Article"</th><th>"Status"</th><th>"Type"</th>
                                                        </tr></thead>
                                                        <tbody>
                                                            {reqs.into_iter().map(|r| {
                                                                let status = map.get(&r.id.to_string())
                                                                    .cloned()
                                                                    .unwrap_or_else(|| "not_assessed".to_string());
                                                                view! {
                                                                    <RequirementRow
                                                                        requirement=r
                                                                        assessment_status=status
                                                                        on_click=on_req_click
                                                                    />
                                                                }
                                                            }).collect_view()}
                                                        </tbody>
                                                    </table>
                                                </details>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            }
                            Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                        })}
                    </Suspense>
                </div>

                // Right: detail + assessment panel
                {move || {
                    let req = selected_requirement.get();
                    let sys_selected = selected_system_id.get().is_some();
                    match req {
                        None => view! {
                            <div class="navigator-detail empty-detail">
                                <p class="placeholder-text">"Click a requirement to view details and record an assessment."</p>
                            </div>
                        }.into_view(),
                        Some(r) => {
                            let fw_str = serde_json::to_value(&r.framework)
                                .ok().and_then(|v| v.as_str().map(String::from))
                                .unwrap_or_default();
                            let risk_cats: Vec<String> = r.applicable_risk_categories.iter()
                                .map(|rc| serde_json::to_value(rc).ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default())
                                .collect();
                            let roles: Vec<String> = r.applicable_roles.iter()
                                .map(|rl| serde_json::to_value(rl).ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default())
                                .collect();

                            view! {
                                <div class="navigator-detail">
                                    <div class="detail-panel-header">
                                        <h2>{&r.reference_id}" — "{&r.title}</h2>
                                        <div class="detail-panel-meta">
                                            <FrameworkPill framework=fw_str />
                                            <span class="req-article">{&r.article_clause}</span>
                                            <span class="req-mandatory-tag">
                                                {if r.is_mandatory { "Required" } else { "Recommended" }}
                                            </span>
                                        </div>
                                    </div>

                                    <div class="detail-panel-body">
                                        // Description
                                        <section class="detail-section">
                                            <h3>"Description"</h3>
                                            <p>{&r.description}</p>
                                        </section>

                                        // Applicability
                                        {if !risk_cats.is_empty() || !roles.is_empty() {
                                            view! {
                                                <section class="detail-section">
                                                    <h3>"Applicability"</h3>
                                                    {if !risk_cats.is_empty() {
                                                        view! { <p><strong>"Risk tiers: "</strong>{risk_cats.join(", ")}</p> }.into_view()
                                                    } else { view! {}.into_view() }}
                                                    {if !roles.is_empty() {
                                                        view! { <p><strong>"Roles: "</strong>{roles.join(", ")}</p> }.into_view()
                                                    } else { view! {}.into_view() }}
                                                </section>
                                            }.into_view()
                                        } else { view! {}.into_view() }}

                                        // Guidance
                                        {if !r.guidance_text.is_empty() {
                                            view! {
                                                <section class="detail-section">
                                                    <h3>"Guidance"</h3>
                                                    <p class="guidance-text">{&r.guidance_text}</p>
                                                </section>
                                            }.into_view()
                                        } else { view! {}.into_view() }}

                                        // Implementation Notes
                                        {if !r.implementation_notes.is_empty() {
                                            view! {
                                                <section class="detail-section">
                                                    <h3>"Implementation Notes"</h3>
                                                    <p class="impl-notes">{&r.implementation_notes}</p>
                                                </section>
                                            }.into_view()
                                        } else { view! {}.into_view() }}

                                        // Cross-references
                                        <Suspense fallback=|| ()>
                                            {move || cross_refs.get().map(|refs| {
                                                if refs.is_empty() { return view! {}.into_view(); }
                                                view! {
                                                    <section class="detail-section">
                                                        <h3>"Cross-References"</h3>
                                                        <div class="cross-ref-list">
                                                            {refs.into_iter().map(|cr| {
                                                                let rel = serde_json::to_value(&cr.relationship)
                                                                    .ok().and_then(|v| v.as_str().map(String::from))
                                                                    .unwrap_or_default();
                                                                let target_fw = serde_json::to_value(&cr.target.framework)
                                                                    .ok().and_then(|v| v.as_str().map(String::from))
                                                                    .unwrap_or_default();
                                                                view! {
                                                                    <div class=format!("cross-ref-mini relationship-{}", rel)>
                                                                        <span class="cross-ref-rel">{&rel}</span>
                                                                        <FrameworkPill framework=target_fw />
                                                                        <span class="cross-ref-target">
                                                                            {&cr.target.reference_id}" — "{&cr.target.title}
                                                                        </span>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </section>
                                                }.into_view()
                                            })}
                                        </Suspense>

                                        // Assessment Editor Form
                                        <section class="detail-section assessment-editor">
                                            <h3>"Assessment"</h3>
                                            {if !sys_selected {
                                                view! {
                                                    <p class="placeholder-text">"Select an engagement and AI system above to record an assessment."</p>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    <div class="assessment-form">
                                                        <label>
                                                            <span class="field-label">"Compliance Status"</span>
                                                            <select
                                                                prop:value=form_status
                                                                on:change=move |e| set_form_status.set(event_target_value(&e))
                                                            >
                                                                {STATUS_OPTIONS.iter().map(|(val, label)| {
                                                                    view! { <option value=*val>{*label}</option> }
                                                                }).collect_view()}
                                                            </select>
                                                        </label>
                                                        <label>
                                                            <span class="field-label">"Assessor Notes"</span>
                                                            <textarea
                                                                rows="4"
                                                                placeholder="Document your assessment rationale…"
                                                                prop:value=form_notes
                                                                on:input=move |e| set_form_notes.set(event_target_value(&e))
                                                            ></textarea>
                                                        </label>
                                                        {move || {
                                                            let st = form_status.get();
                                                            if st == "gap" || st == "partial" {
                                                                view! {
                                                                    <label>
                                                                        <span class="field-label">"Remediation Plan"</span>
                                                                        <textarea
                                                                            rows="3"
                                                                            placeholder="Describe required actions to close this gap…"
                                                                            prop:value=form_remediation
                                                                            on:input=move |e| set_form_remediation.set(event_target_value(&e))
                                                                        ></textarea>
                                                                    </label>
                                                                    <label>
                                                                        <span class="field-label">"Target Date"</span>
                                                                        <input
                                                                            type="date"
                                                                            prop:value=form_target_date
                                                                            on:input=move |e| set_form_target_date.set(event_target_value(&e))
                                                                        />
                                                                    </label>
                                                                }.into_view()
                                                            } else {
                                                                view! {}.into_view()
                                                            }
                                                        }}
                                                        <div class="assessment-actions">
                                                            <button
                                                                on:click=save_assessment
                                                                disabled=saving
                                                            >
                                                                {move || if saving.get() { "Saving…" } else { "Save Assessment" }}
                                                            </button>
                                                            {move || save_message.get().map(|msg| {
                                                                let cls = if msg.starts_with("Error") { "error" } else { "save-ok" };
                                                                view! { <span class=cls>{msg}</span> }
                                                            })}
                                                        </div>
                                                    </div>
                                                }.into_view()
                                            }}
                                        </section>
                                    </div>
                                </div>
                            }.into_view()
                        }
                    }
                }}
            </div>
        </div>
    }
}
