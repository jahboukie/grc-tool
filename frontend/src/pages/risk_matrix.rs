use leptos::*;
use leptos_router::*;
use grc_shared::models::{
    AiSystem, CreateRiskDto, Engagement, RiskEntry, RiskMatrixData,
};
use grc_shared::enums::{Priority, RiskImpact, RiskLikelihood, TaskStatus};
use serde::Serialize;
use uuid::Uuid;

use crate::api::invoke;
use crate::components::risk_heatmap::RiskHeatmap;

// ── IPC arg types ──────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RiskMatrixArg {
    engagement_id: Option<String>,
    ai_system_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementIdArg {
    engagement_id: String,
}

#[derive(Serialize)]
struct IdArg {
    id: String,
}

// ── Helpers ────────────────────────────────────────────────

fn likelihood_from_str(s: &str) -> RiskLikelihood {
    match s {
        "rare" => RiskLikelihood::Rare,
        "unlikely" => RiskLikelihood::Unlikely,
        "possible" => RiskLikelihood::Possible,
        "likely" => RiskLikelihood::Likely,
        "almost_certain" => RiskLikelihood::AlmostCertain,
        _ => RiskLikelihood::Possible,
    }
}

fn impact_from_str(s: &str) -> RiskImpact {
    match s {
        "negligible" => RiskImpact::Negligible,
        "minor" => RiskImpact::Minor,
        "moderate" => RiskImpact::Moderate,
        "major" => RiskImpact::Major,
        "catastrophic" => RiskImpact::Catastrophic,
        _ => RiskImpact::Moderate,
    }
}

fn priority_from_str(s: &str) -> Priority {
    match s {
        "critical" => Priority::Critical,
        "high" => Priority::High,
        "medium" => Priority::Medium,
        "low" => Priority::Low,
        _ => Priority::Medium,
    }
}

fn status_from_str(s: &str) -> TaskStatus {
    match s {
        "open" => TaskStatus::Open,
        "in_progress" => TaskStatus::InProgress,
        "blocked" => TaskStatus::Blocked,
        "done" => TaskStatus::Done,
        "deferred" => TaskStatus::Deferred,
        _ => TaskStatus::Open,
    }
}

fn severity_class(score: i32) -> &'static str {
    match score {
        1..=4 => "risk-low",
        5..=9 => "risk-medium",
        10..=14 => "risk-high",
        15..=19 => "risk-very-high",
        _ => "risk-critical",
    }
}

fn build_residual_matrix(entries: &[RiskEntry]) -> Vec<Vec<Vec<Uuid>>> {
    let mut matrix: Vec<Vec<Vec<Uuid>>> = vec![vec![vec![]; 5]; 5];
    for e in entries {
        if let (Some(rl), Some(ri)) = (&e.residual_likelihood, &e.residual_impact) {
            let li = (rl.value() - 1) as usize;
            let ii = (ri.value() - 1) as usize;
            matrix[li][ii].push(e.id);
        }
    }
    matrix
}

fn likelihood_index(l: &RiskLikelihood) -> usize {
    (l.value() - 1) as usize
}

fn impact_index(i: &RiskImpact) -> usize {
    (i.value() - 1) as usize
}

// ── Page ───────────────────────────────────────────────────

#[component]
pub fn RiskMatrixPage() -> impl IntoView {
    let params = use_params_map();
    let route_sys_id = move || params.with(|p| p.get("sys_id").cloned());

    // ── Scope selectors ────────────────────────────────────
    let (selected_engagement_id, set_selected_engagement_id) = create_signal(Option::<String>::None);
    let (selected_system_id, set_selected_system_id) = create_signal(route_sys_id());
    let (show_residual, set_show_residual) = create_signal(false);
    let (data_version, set_data_version) = create_signal(0u32);

    // ── Form state ─────────────────────────────────────────
    let (show_form, set_show_form) = create_signal(false);
    let (editing_id, set_editing_id) = create_signal(Option::<Uuid>::None);
    let (form_title, set_form_title) = create_signal(String::new());
    let (form_desc, set_form_desc) = create_signal(String::new());
    let (form_source, set_form_source) = create_signal(String::new());
    let (form_rights, set_form_rights) = create_signal(String::new());
    let (form_likelihood, set_form_likelihood) = create_signal("possible".to_string());
    let (form_impact, set_form_impact) = create_signal("moderate".to_string());
    let (form_mitigation, set_form_mitigation) = create_signal(String::new());
    let (form_res_likelihood, set_form_res_likelihood) = create_signal(String::new());
    let (form_res_impact, set_form_res_impact) = create_signal(String::new());
    let (form_priority, set_form_priority) = create_signal("medium".to_string());
    let (form_status, set_form_status) = create_signal("open".to_string());
    let (form_msg, set_form_msg) = create_signal(Option::<String>::None);

    // ── Cell click filter ──────────────────────────────────
    let (cell_filter, set_cell_filter) = create_signal(Option::<(usize, usize)>::None);

    // ── Data resources ─────────────────────────────────────

    let engagements = create_resource(|| (), |_| async {
        invoke::call::<_, Vec<Engagement>>(
            "list_engagements",
            &serde_json::json!({ "statusFilter": null }),
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

    let matrix_data = create_resource(
        move || (selected_engagement_id.get(), selected_system_id.get(), data_version.get()),
        |(eid, sid, _)| async move {
            invoke::call::<_, RiskMatrixData>(
                "get_risk_matrix_data",
                &RiskMatrixArg { engagement_id: eid, ai_system_id: sid },
            ).await
        },
    );

    // ── Form helpers ───────────────────────────────────────

    let reset_form = move || {
        set_editing_id.set(None);
        set_form_title.set(String::new());
        set_form_desc.set(String::new());
        set_form_source.set(String::new());
        set_form_rights.set(String::new());
        set_form_likelihood.set("possible".into());
        set_form_impact.set("moderate".into());
        set_form_mitigation.set(String::new());
        set_form_res_likelihood.set(String::new());
        set_form_res_impact.set(String::new());
        set_form_priority.set("medium".into());
        set_form_status.set("open".into());
        set_form_msg.set(None);
    };

    let open_create = move |_| {
        reset_form();
        set_show_form.set(true);
    };

    let populate_edit = move |r: &RiskEntry| {
        set_editing_id.set(Some(r.id));
        set_form_title.set(r.title.clone());
        set_form_desc.set(r.description.clone());
        set_form_source.set(r.risk_source.clone());
        set_form_rights.set(r.affected_rights.join(", "));
        set_form_likelihood.set(serde_json::to_value(&r.likelihood).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or_default());
        set_form_impact.set(serde_json::to_value(&r.impact).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or_default());
        set_form_mitigation.set(r.mitigation_measures.clone());
        set_form_res_likelihood.set(r.residual_likelihood.as_ref().and_then(|v| serde_json::to_value(v).ok()).and_then(|v| v.as_str().map(String::from)).unwrap_or_default());
        set_form_res_impact.set(r.residual_impact.as_ref().and_then(|v| serde_json::to_value(v).ok()).and_then(|v| v.as_str().map(String::from)).unwrap_or_default());
        set_form_priority.set(serde_json::to_value(&r.priority).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or("medium".into()));
        set_form_status.set(serde_json::to_value(&r.status).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or("open".into()));
        set_form_msg.set(None);
        set_show_form.set(true);
    };

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let sid = selected_system_id.get();
        let eid = editing_id.get();
        let title = form_title.get();
        let desc = form_desc.get();
        let source = form_source.get();
        let rights: Vec<String> = form_rights.get().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let likelihood = likelihood_from_str(&form_likelihood.get());
        let impact = impact_from_str(&form_impact.get());
        let mitigation = form_mitigation.get();
        let res_l_str = form_res_likelihood.get();
        let res_i_str = form_res_impact.get();
        let res_l = if res_l_str.is_empty() { None } else { Some(likelihood_from_str(&res_l_str)) };
        let res_i = if res_i_str.is_empty() { None } else { Some(impact_from_str(&res_i_str)) };
        let priority = priority_from_str(&form_priority.get());
        let status = status_from_str(&form_status.get());

        spawn_local(async move {
            let result = if let Some(risk_id) = eid {
                // Update existing
                invoke::call::<_, RiskEntry>(
                    "update_risk_entry",
                    &serde_json::json!({
                        "id": risk_id.to_string(),
                        "dto": {
                            "title": title,
                            "description": desc,
                            "risk_source": source,
                            "affected_rights": rights,
                            "likelihood": likelihood,
                            "impact": impact,
                            "mitigation_measures": mitigation,
                            "residual_likelihood": res_l,
                            "residual_impact": res_i,
                            "status": status,
                            "priority": priority,
                        }
                    }),
                ).await
            } else {
                // Create new — need an AI system selected
                let Some(system_id) = sid else {
                    set_form_msg.set(Some("Select an AI System first.".into()));
                    return;
                };
                let Ok(parsed_id) = Uuid::parse_str(&system_id) else {
                    set_form_msg.set(Some("Invalid AI system ID.".into()));
                    return;
                };
                let dto = CreateRiskDto {
                    ai_system_id: parsed_id,
                    title,
                    description: desc,
                    risk_source: source,
                    affected_rights: rights,
                    likelihood,
                    impact,
                    mitigation_measures: mitigation,
                    residual_likelihood: res_l,
                    residual_impact: res_i,
                    related_requirement_ids: vec![],
                    priority,
                };
                invoke::call_named::<_, RiskEntry>("create_risk_entry", "dto", &dto).await
            };

            match result {
                Ok(_) => {
                    set_show_form.set(false);
                    reset_form();
                    set_data_version.update(|v| *v += 1);
                }
                Err(e) => set_form_msg.set(Some(format!("Error: {}", e))),
            }
        });
    };

    let on_delete = move |risk_id: Uuid| {
        spawn_local(async move {
            match invoke::call::<_, ()>("delete_risk_entry", &IdArg { id: risk_id.to_string() }).await {
                Ok(_) => set_data_version.update(|v| *v += 1),
                Err(e) => set_form_msg.set(Some(format!("Delete error: {}", e))),
            }
        });
    };

    // ── View ───────────────────────────────────────────────

    view! {
        <div class="page risk-matrix-page">
            <h1>"Risk Assessment Matrix"</h1>

            // ── Scope selectors ────────────────────────────
            <div class="scope-selectors" style="display:flex;gap:1rem;align-items:end;margin-bottom:1rem;">
                <label style="flex:1">"Engagement"
                    <Suspense fallback=move || view! { <select disabled><option>"Loading…"</option></select> }>
                        {move || engagements.get().map(|result| match result {
                            Ok(engs) => view! {
                                <select on:change=move |e| {
                                    let v = event_target_value(&e);
                                    if v.is_empty() { set_selected_engagement_id.set(None); set_selected_system_id.set(None); }
                                    else { set_selected_engagement_id.set(Some(v)); set_selected_system_id.set(None); }
                                }>
                                    <option value="">"All engagements"</option>
                                    {engs.into_iter().map(|eng| {
                                        let id = eng.id.to_string();
                                        view! { <option value=id.clone()>{eng.name}</option> }
                                    }).collect_view()}
                                </select>
                            }.into_view(),
                            Err(_) => view! { <select disabled><option>"Error"</option></select> }.into_view(),
                        })}
                    </Suspense>
                </label>

                <label style="flex:1">"AI System"
                    <Suspense fallback=move || view! { <select disabled><option>"—"</option></select> }>
                        {move || ai_systems.get().map(|result| match result {
                            Ok(systems) => view! {
                                <select
                                    prop:value=move || selected_system_id.get().unwrap_or_default()
                                    on:change=move |e| {
                                        let v = event_target_value(&e);
                                        if v.is_empty() { set_selected_system_id.set(None); }
                                        else { set_selected_system_id.set(Some(v)); }
                                    }
                                >
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

                <fieldset role="group" style="margin-bottom:0">
                    <label style="display:inline-flex;gap:0.3rem;align-items:center;cursor:pointer">
                        <input type="radio" name="risk_view"
                            checked=move || !show_residual.get()
                            on:change=move |_| set_show_residual.set(false) />
                        "Inherent"
                    </label>
                    <label style="display:inline-flex;gap:0.3rem;align-items:center;cursor:pointer">
                        <input type="radio" name="risk_view"
                            checked=move || show_residual.get()
                            on:change=move |_| set_show_residual.set(true) />
                        "Residual"
                    </label>
                </fieldset>

                <button on:click=open_create>"+ Add Risk"</button>
            </div>

            // ── Risk form (create / edit) ──────────────────
            <Show when=move || show_form.get()>
                <article class="risk-form">
                    <header>
                        <strong>{move || if editing_id.get().is_some() { "Edit Risk Entry" } else { "New Risk Entry" }}</strong>
                        <button class="outline" style="margin-left:auto" on:click=move |_| { set_show_form.set(false); reset_form(); }>"✕"</button>
                    </header>
                    <form on:submit=on_submit>
                        <div class="grid">
                            <label>"Title"
                                <input type="text" required prop:value=form_title on:input=move |e| set_form_title.set(event_target_value(&e)) />
                            </label>
                            <label>"Priority"
                                <select prop:value=form_priority on:change=move |e| set_form_priority.set(event_target_value(&e))>
                                    <option value="critical">"Critical"</option>
                                    <option value="high">"High"</option>
                                    <option value="medium">"Medium"</option>
                                    <option value="low">"Low"</option>
                                </select>
                            </label>
                        </div>
                        <label>"Description"
                            <textarea rows=2 prop:value=form_desc on:input=move |e| set_form_desc.set(event_target_value(&e))></textarea>
                        </label>
                        <div class="grid">
                            <label>"Risk Source"
                                <input type="text" prop:value=form_source on:input=move |e| set_form_source.set(event_target_value(&e)) />
                            </label>
                            <label>"Affected Rights (comma-separated)"
                                <input type="text" placeholder="e.g. Non-discrimination, Privacy" prop:value=form_rights on:input=move |e| set_form_rights.set(event_target_value(&e)) />
                            </label>
                        </div>
                        <fieldset>
                            <legend>"Inherent Risk"</legend>
                            <div class="grid">
                                <label>"Likelihood"
                                    <select prop:value=form_likelihood on:change=move |e| set_form_likelihood.set(event_target_value(&e))>
                                        <option value="rare">"Rare (1)"</option>
                                        <option value="unlikely">"Unlikely (2)"</option>
                                        <option value="possible">"Possible (3)"</option>
                                        <option value="likely">"Likely (4)"</option>
                                        <option value="almost_certain">"Almost Certain (5)"</option>
                                    </select>
                                </label>
                                <label>"Impact"
                                    <select prop:value=form_impact on:change=move |e| set_form_impact.set(event_target_value(&e))>
                                        <option value="negligible">"Negligible (1)"</option>
                                        <option value="minor">"Minor (2)"</option>
                                        <option value="moderate">"Moderate (3)"</option>
                                        <option value="major">"Major (4)"</option>
                                        <option value="catastrophic">"Catastrophic (5)"</option>
                                    </select>
                                </label>
                            </div>
                        </fieldset>
                        <label>"Mitigation Measures"
                            <textarea rows=2 prop:value=form_mitigation on:input=move |e| set_form_mitigation.set(event_target_value(&e))></textarea>
                        </label>
                        <fieldset>
                            <legend>"Residual Risk (after mitigation, optional)"</legend>
                            <div class="grid">
                                <label>"Residual Likelihood"
                                    <select prop:value=form_res_likelihood on:change=move |e| set_form_res_likelihood.set(event_target_value(&e))>
                                        <option value="">"—"</option>
                                        <option value="rare">"Rare (1)"</option>
                                        <option value="unlikely">"Unlikely (2)"</option>
                                        <option value="possible">"Possible (3)"</option>
                                        <option value="likely">"Likely (4)"</option>
                                        <option value="almost_certain">"Almost Certain (5)"</option>
                                    </select>
                                </label>
                                <label>"Residual Impact"
                                    <select prop:value=form_res_impact on:change=move |e| set_form_res_impact.set(event_target_value(&e))>
                                        <option value="">"—"</option>
                                        <option value="negligible">"Negligible (1)"</option>
                                        <option value="minor">"Minor (2)"</option>
                                        <option value="moderate">"Moderate (3)"</option>
                                        <option value="major">"Major (4)"</option>
                                        <option value="catastrophic">"Catastrophic (5)"</option>
                                    </select>
                                </label>
                            </div>
                        </fieldset>
                        <Show when=move || editing_id.get().is_some()>
                            <label>"Status"
                                <select prop:value=form_status on:change=move |e| set_form_status.set(event_target_value(&e))>
                                    <option value="open">"Open"</option>
                                    <option value="in_progress">"In Progress"</option>
                                    <option value="blocked">"Blocked"</option>
                                    <option value="done">"Done"</option>
                                    <option value="deferred">"Deferred"</option>
                                </select>
                            </label>
                        </Show>
                        {move || form_msg.get().map(|m| view! { <p class="error">{m}</p> })}
                        <button type="submit">{move || if editing_id.get().is_some() { "Save Changes" } else { "Create Risk" }}</button>
                    </form>
                </article>
            </Show>

            // ── Matrix + register ──────────────────────────
            <Suspense fallback=move || view! { <p>"Loading risk data…"</p> }>
                {move || matrix_data.get().map(|result| match result {
                    Ok(data) => {
                        let severity_counts = |min: usize, max: usize| {
                            data.entries.iter().filter(|e| {
                                let s = e.inherent_score as usize;
                                s >= min && s <= max
                            }).count()
                        };
                        let display_matrix = if show_residual.get() {
                            build_residual_matrix(&data.entries)
                        } else {
                            data.matrix.clone()
                        };

                        view! {
                            <div class="risk-summary">
                                <span class="risk-low">{format!("Low: {}", severity_counts(1, 4))}</span>
                                <span class="risk-medium">{format!("Medium: {}", severity_counts(5, 9))}</span>
                                <span class="risk-high">{format!("High: {}", severity_counts(10, 14))}</span>
                                <span class="risk-very-high">{format!("Very High: {}", severity_counts(15, 19))}</span>
                                <span class="risk-critical">{format!("Critical: {}", severity_counts(20, 25))}</span>
                            </div>
                            <RiskHeatmap matrix=display_matrix on_cell_click=Callback::new(move |(li, ii)| {
                                let current = cell_filter.get();
                                if current == Some((li, ii)) {
                                    set_cell_filter.set(None);
                                } else {
                                    set_cell_filter.set(Some((li, ii)));
                                }
                            }) />
                            {move || cell_filter.get().map(|_| view! {
                                <button class="outline secondary" style="margin-bottom:0.5rem;font-size:0.8rem;" on:click=move |_| set_cell_filter.set(None)>"Clear cell filter"</button>
                            })}
                            <section class="risk-entries">
                                <h2>"Risk Register"</h2>
                                <table role="grid">
                                    <thead><tr>
                                        <th>"Title"</th>
                                        <th>"Inherent"</th>
                                        <th>"Residual"</th>
                                        <th>"Status"</th>
                                        <th>"Priority"</th>
                                        <th>"Actions"</th>
                                    </tr></thead>
                                    <tbody>
                                        {data.entries.iter().filter(|r| {
                                            match cell_filter.get() {
                                                None => true,
                                                Some((li, ii)) => {
                                                    let rl = likelihood_index(&r.likelihood);
                                                    let ri = impact_index(&r.impact);
                                                    rl == li && ri == ii
                                                }
                                            }
                                        }).map(|r| {
                                            let r_edit = r.clone();
                                            let r_id = r.id;
                                            let sev = severity_class(r.inherent_score);
                                            let res_score = r.residual_score.map(|s| s.to_string()).unwrap_or_else(|| "—".into());
                                            let res_sev = r.residual_score.map(|s| severity_class(s)).unwrap_or("");
                                            let status_str = serde_json::to_value(&r.status).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or_default();
                                            let priority_str = serde_json::to_value(&r.priority).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or_default();
                                            view! {
                                                <tr>
                                                    <td><strong>{&r.title}</strong><br/><small>{&r.risk_source}</small></td>
                                                    <td class=sev>{r.inherent_score.to_string()}</td>
                                                    <td class=res_sev>{res_score}</td>
                                                    <td>{status_str}</td>
                                                    <td>{priority_str}</td>
                                                    <td>
                                                        <div style="display:flex;gap:0.3rem">
                                                            <button class="outline" style="padding:0.2rem 0.5rem;font-size:0.8rem" on:click=move |_| populate_edit(&r_edit)>"Edit"</button>
                                                            <button class="outline secondary" style="padding:0.2rem 0.5rem;font-size:0.8rem" on:click=move |_| on_delete(r_id)>"Del"</button>
                                                        </div>
                                                    </td>
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
