use leptos::*;
use leptos_router::*;
use grc_shared::models::{AiSystem, Evidence, RequirementAssessment, UpdateAiSystemDto};
use grc_shared::enums::RiskCategory;
use serde::Serialize;

use crate::api::invoke;
use crate::components::status_badge::StatusBadge;

#[derive(Serialize)]
struct IdArg { id: String }

#[derive(Serialize)]
struct UpdateArg { id: String, dto: UpdateAiSystemDto }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssessmentListArg { ai_system_id: String, framework: Option<String> }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceListArg { engagement_id: String, type_filter: Option<String> }

fn risk_badge_color(cat: &RiskCategory) -> &'static str {
    match cat {
        RiskCategory::Unacceptable => "background:#d32f2f;color:#fff;",
        RiskCategory::High => "background:#e65100;color:#fff;",
        RiskCategory::Limited => "background:#f9a825;color:#000;",
        RiskCategory::Minimal => "background:#2e7d32;color:#fff;",
        RiskCategory::Gpai => "background:#1565c0;color:#fff;",
    }
}

fn risk_str(cat: &RiskCategory) -> String {
    serde_json::to_value(cat)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default()
}

#[component]
pub fn AiSystemDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let (version, set_version) = create_signal(0u32);
    let (editing, set_editing) = create_signal(false);
    let (msg, set_msg) = create_signal::<Option<String>>(None);

    // Edit form signals
    let (ed_name, set_ed_name) = create_signal(String::new());
    let (ed_desc, set_ed_desc) = create_signal(String::new());
    let (ed_purpose, set_ed_purpose) = create_signal(String::new());
    let (ed_risk, set_ed_risk) = create_signal(String::new());
    let (ed_domain, set_ed_domain) = create_signal(String::new());
    let (ed_gpai, set_ed_gpai) = create_signal(false);
    let (ed_annex, set_ed_annex) = create_signal(false);
    let (ed_safety, set_ed_safety) = create_signal(false);
    let (ed_deploy, set_ed_deploy) = create_signal(String::new());

    let system = create_resource(
        move || (id(), version.get()),
        |(id, _)| async move {
            invoke::call::<_, AiSystem>("get_ai_system", &IdArg { id }).await
        },
    );

    // Recent assessments for this system
    let assessments = create_resource(
        move || (id(), version.get()),
        |(id, _)| async move {
            invoke::call::<_, Vec<RequirementAssessment>>(
                "list_assessments",
                &AssessmentListArg { ai_system_id: id, framework: None },
            ).await
        },
    );

    // Evidence for the engagement (loaded after system resolves)
    let engagement_id_sig = create_memo(move |_| {
        system.get()
            .and_then(|r| r.ok())
            .map(|s| s.engagement_id.to_string())
            .unwrap_or_default()
    });

    let engagement_evidence = create_resource(
        move || engagement_id_sig.get(),
        |eid| async move {
            if eid.is_empty() { return vec![]; }
            invoke::call::<_, Vec<Evidence>>(
                "list_evidence",
                &EvidenceListArg { engagement_id: eid, type_filter: None },
            ).await.unwrap_or_default()
        },
    );

    let populate_edit = move |s: &AiSystem| {
        set_ed_name.set(s.name.clone());
        set_ed_desc.set(s.description.clone());
        set_ed_purpose.set(s.intended_purpose.clone());
        set_ed_risk.set(risk_str(&s.risk_category));
        set_ed_domain.set(s.domain.clone());
        set_ed_gpai.set(s.is_gpai);
        set_ed_annex.set(s.is_high_risk_listed);
        set_ed_safety.set(s.is_safety_component);
        set_ed_deploy.set(s.deployment_context.clone());
    };

    view! {
        <div class="page ai-system-detail-page">
            <Suspense fallback=move || view! { <p>"Loading system…"</p> }>
                {move || {
                    system.get().map(|result| match result {
                    Ok(s) => {
                        let sys_id = s.id.to_string();
                        let sys_id_del = sys_id.clone();
                        let badge_style = format!("padding:0.25rem 0.75rem;border-radius:4px;font-weight:bold;font-size:0.85rem;{}", risk_badge_color(&s.risk_category));
                        let risk_label = s.risk_category.display_name().to_string();

                        view! {
                            <header style="display:flex;align-items:center;gap:1rem;flex-wrap:wrap;">
                                <h1 style="margin:0;">{&s.name}</h1>
                                <span style=badge_style>{&risk_label}</span>
                            </header>

                            <div style="display:flex;gap:0.5rem;margin:1rem 0;">
                                <button on:click={
                                    let s2 = s.clone();
                                    move |_| { populate_edit(&s2); set_editing.set(true); set_msg.set(None); }
                                }>"Edit"</button>
                                <button class="secondary" on:click={
                                    let sys_id_d = sys_id_del.clone();
                                    move |_| {
                                        let confirm = web_sys::window()
                                            .and_then(|w| w.confirm_with_message("Delete this AI system? This cannot be undone.").ok())
                                            .unwrap_or(false);
                                        if confirm {
                                            let sys_id_d2 = sys_id_d.clone();
                                            spawn_local(async move {
                                                match invoke::call::<_, ()>("delete_ai_system", &IdArg { id: sys_id_d2 }).await {
                                                    Ok(_) => {
                                                        let nav = leptos_router::use_navigate();
                                                        nav("/engagements", Default::default());
                                                    }
                                                    Err(e) => { web_sys::window().map(|w| w.alert_with_message(&format!("Delete failed: {e}"))); }
                                                }
                                            });
                                        }
                                    }
                                }>"Delete"</button>
                                <A href=format!("/fria/{}", s.id)>
                                    <button class="outline">"Open FRIA"</button>
                                </A>
                            </div>

                            {move || msg.get().map(|m| view! { <p style="color:var(--pico-primary);">{m}</p> })}

                            <Show when=move || editing.get()>
                                <form on:submit={
                                    let sys_id_e = sys_id.clone();
                                    move |ev: web_sys::SubmitEvent| {
                                        ev.prevent_default();
                                        let sid = sys_id_e.clone();
                                        let dto = UpdateAiSystemDto {
                                            name: Some(ed_name.get()),
                                            description: Some(ed_desc.get()),
                                            intended_purpose: Some(ed_purpose.get()),
                                            risk_category: serde_json::from_value(serde_json::Value::String(ed_risk.get())).ok(),
                                            domain: Some(ed_domain.get()),
                                            is_gpai: Some(ed_gpai.get()),
                                            is_high_risk_listed: Some(ed_annex.get()),
                                            is_safety_component: Some(ed_safety.get()),
                                            deployment_context: Some(ed_deploy.get()),
                                        };
                                        spawn_local(async move {
                                            match invoke::call::<_, AiSystem>("update_ai_system", &UpdateArg { id: sid, dto }).await {
                                                Ok(_) => {
                                                    set_editing.set(false);
                                                    set_msg.set(Some("Updated.".into()));
                                                    set_version.update(|v| *v += 1);
                                                }
                                                Err(e) => set_msg.set(Some(format!("Error: {e}"))),
                                            }
                                        });
                                    }
                                } style="margin-bottom:1.5rem;">
                                    <fieldset>
                                        <legend>"Edit AI System"</legend>
                                        <label>"Name"
                                            <input type="text" prop:value=ed_name on:input=move |e| set_ed_name.set(event_target_value(&e)) />
                                        </label>
                                        <label>"Description"
                                            <textarea prop:value=ed_desc on:input=move |e| set_ed_desc.set(event_target_value(&e)) />
                                        </label>
                                        <label>"Intended Purpose"
                                            <textarea prop:value=ed_purpose on:input=move |e| set_ed_purpose.set(event_target_value(&e)) />
                                        </label>
                                        <label>"Risk Category"
                                            <select prop:value=ed_risk on:change=move |e| set_ed_risk.set(event_target_value(&e))>
                                                <option value="unacceptable">"Unacceptable (Prohibited)"</option>
                                                <option value="high">"High Risk"</option>
                                                <option value="limited">"Limited Risk"</option>
                                                <option value="minimal">"Minimal Risk"</option>
                                                <option value="gpai">"GPAI"</option>
                                            </select>
                                        </label>
                                        <label>"Domain"
                                            <input type="text" prop:value=ed_domain on:input=move |e| set_ed_domain.set(event_target_value(&e)) />
                                        </label>
                                        <label>"Deployment Context"
                                            <textarea prop:value=ed_deploy on:input=move |e| set_ed_deploy.set(event_target_value(&e)) />
                                        </label>
                                        <div style="display:flex;gap:2rem;margin-top:0.5rem;">
                                            <label style="display:flex;align-items:center;gap:0.5rem;">
                                                <input type="checkbox" prop:checked=ed_gpai on:change=move |e| set_ed_gpai.set(event_target_checked(&e)) />
                                                "GPAI Model"
                                            </label>
                                            <label style="display:flex;align-items:center;gap:0.5rem;">
                                                <input type="checkbox" prop:checked=ed_annex on:change=move |e| set_ed_annex.set(event_target_checked(&e)) />
                                                "Annex III Listed"
                                            </label>
                                            <label style="display:flex;align-items:center;gap:0.5rem;">
                                                <input type="checkbox" prop:checked=ed_safety on:change=move |e| set_ed_safety.set(event_target_checked(&e)) />
                                                "Safety Component"
                                            </label>
                                        </div>
                                        <div style="display:flex;gap:0.5rem;margin-top:1rem;">
                                            <button type="submit">"Save"</button>
                                            <button type="button" class="secondary" on:click=move |_| set_editing.set(false)>"Cancel"</button>
                                        </div>
                                    </fieldset>
                                </form>
                            </Show>

                            <Show when=move || !editing.get()>
                                <section class="classification-panel">
                                    <h2>"Classification"</h2>
                                    <dl>
                                        <dt>"Risk Category"</dt><dd>{risk_label.clone()}</dd>
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
                                <section class="description-section">
                                    <h2>"Description"</h2>
                                    <p>{&s.description}</p>
                                </section>
                            </Show>

                            // Recent Assessments
                            <section style="margin-top:1.5rem;">
                                <h2>"Recent Assessments"</h2>
                                <Suspense fallback=move || view! { <p>"Loading assessments…"</p> }>
                                    {move || assessments.get().map(|result| {
                                        let list = result.unwrap_or_default();
                                        if list.is_empty() {
                                            return view! { <p style="color:var(--pico-muted-color);">"No assessments recorded yet."</p> }.into_view();
                                        }
                                        let mut sorted = list;
                                        sorted.sort_by(|a, b| b.assessed_at.cmp(&a.assessed_at));
                                        let recent: Vec<_> = sorted.into_iter().take(5).collect();
                                        view! {
                                            <table role="grid">
                                                <thead><tr>
                                                    <th>"Status"</th><th>"Requirement"</th><th>"Assessed"</th>
                                                </tr></thead>
                                                <tbody>
                                                    {recent.into_iter().map(|a| {
                                                        let st = serde_json::to_value(&a.status).ok()
                                                            .and_then(|v| v.as_str().map(String::from))
                                                            .unwrap_or_default();
                                                        let date = a.assessed_at.format("%Y-%m-%d").to_string();
                                                        view! {
                                                            <tr>
                                                                <td><StatusBadge status=st /></td>
                                                                <td>{a.requirement_id.to_string()}</td>
                                                                <td>{date}</td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        }.into_view()
                                    })}
                                </Suspense>
                            </section>

                            // Evidence from engagement
                            <section style="margin-top:1.5rem;">
                                <h2>"Engagement Evidence"</h2>
                                <Suspense fallback=move || view! { <p>"Loading evidence…"</p> }>
                                    {move || engagement_evidence.get().map(|list| {
                                        if list.is_empty() {
                                            return view! { <p style="color:var(--pico-muted-color);">"No evidence uploaded for this engagement."</p> }.into_view();
                                        }
                                        view! {
                                            <table role="grid">
                                                <thead><tr>
                                                    <th>"File"</th><th>"Type"</th><th>"Description"</th><th>"Uploaded"</th>
                                                </tr></thead>
                                                <tbody>
                                                    {list.into_iter().take(10).map(|ev| {
                                                        let etype = serde_json::to_value(&ev.evidence_type).ok()
                                                            .and_then(|v| v.as_str().map(String::from))
                                                            .unwrap_or_default();
                                                        let date = ev.uploaded_at.format("%Y-%m-%d").to_string();
                                                        view! {
                                                            <tr>
                                                                <td>{ev.file_name}</td>
                                                                <td>{etype}</td>
                                                                <td>{ev.description}</td>
                                                                <td>{date}</td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        }.into_view()
                                    })}
                                </Suspense>
                            </section>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}}
            </Suspense>
        </div>
    }
}
