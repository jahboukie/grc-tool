use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;
use serde::Serialize;
use uuid::Uuid;

use crate::api::invoke;
use crate::components::status_badge::StatusBadge;
use crate::components::help_panel::{HelpPanel, HelpSection};
use grc_shared::{
    AiSystem, CreateAiSystemDto, Engagement, Evidence, FriaAssessment, FriaNotificationStatus,
    FriaStatus, RiskCategory, RiskEntry, Task, UpsertFriaAssessmentDto,
};

const RISK_CATEGORY_OPTIONS: &[(&str, &str)] = &[
    ("high", "High Risk"),
    ("limited", "Limited Risk"),
    ("minimal", "Minimal Risk"),
    ("gpai", "GPAI"),
    ("unacceptable", "Unacceptable (Prohibited)"),
];

#[derive(Clone)]
struct FriaFormState {
    status: String,
    scope_summary: String,
    deployer_context: String,
    affected_persons_and_groups: String,
    vulnerable_groups: String,
    fundamental_rights_risks: String,
    human_oversight_measures: String,
    mitigation_measures: String,
    consultation_summary: String,
    conclusion: String,
    authority_notification_status: String,
    review_date: String,
    related_risk_ids: Vec<String>,
    related_task_ids: Vec<String>,
    related_evidence_ids: Vec<String>,
}

impl Default for FriaFormState {
    fn default() -> Self {
        Self {
            status: "draft".to_string(),
            scope_summary: String::new(),
            deployer_context: String::new(),
            affected_persons_and_groups: String::new(),
            vulnerable_groups: String::new(),
            fundamental_rights_risks: String::new(),
            human_oversight_measures: String::new(),
            mitigation_measures: String::new(),
            consultation_summary: String::new(),
            conclusion: String::new(),
            authority_notification_status: "not_started".to_string(),
            review_date: String::new(),
            related_risk_ids: Vec::new(),
            related_task_ids: Vec::new(),
            related_evidence_ids: Vec::new(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementFilter {
    status_filter: Option<String>,
}

#[derive(Serialize)]
struct IdArg {
    id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementIdArg {
    engagement_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemIdArg {
    ai_system_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RiskFilter {
    ai_system_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskFilter {
    engagement_id: Option<String>,
    status: Option<String>,
    priority: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceFilter {
    engagement_id: String,
    type_filter: Option<String>,
}

fn enum_from_string<T>(value: &str) -> T
where
    T: for<'de> serde::Deserialize<'de>,
{
    serde_json::from_value(serde_json::Value::String(value.to_string())).unwrap()
}

fn form_from_assessment(item: &FriaAssessment) -> FriaFormState {
    FriaFormState {
        status: serde_json::to_value(&item.status)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "draft".to_string()),
        scope_summary: item.scope_summary.clone(),
        deployer_context: item.deployer_context.clone(),
        affected_persons_and_groups: item.affected_persons_and_groups.clone(),
        vulnerable_groups: item.vulnerable_groups.clone(),
        fundamental_rights_risks: item.fundamental_rights_risks.clone(),
        human_oversight_measures: item.human_oversight_measures.clone(),
        mitigation_measures: item.mitigation_measures.clone(),
        consultation_summary: item.consultation_summary.clone(),
        conclusion: item.conclusion.clone(),
        authority_notification_status: serde_json::to_value(&item.authority_notification_status)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "not_started".to_string()),
        review_date: item.review_date.map(|d| d.to_string()).unwrap_or_default(),
        related_risk_ids: item.related_risk_ids.iter().map(Uuid::to_string).collect(),
        related_task_ids: item.related_task_ids.iter().map(Uuid::to_string).collect(),
        related_evidence_ids: item.related_evidence_ids.iter().map(Uuid::to_string).collect(),
    }
}

fn toggle_selection(ids: &mut Vec<String>, id: &str, checked: bool) {
    let exists = ids.iter().any(|current| current == id);
    if checked && !exists {
        ids.push(id.to_string());
    }
    if !checked {
        ids.retain(|current| current != id);
    }
}

#[component]
pub fn FriaPage() -> impl IntoView {
    let params = use_params_map();
    let route_system_id = move || params.with(|p| p.get("sys_id").cloned().unwrap_or_default());

    let (refresh, set_refresh) = create_signal(0u32);
    let (selected_engagement, set_selected_engagement) = create_signal(String::new());
    let (selected_system, set_selected_system) = create_signal(String::new());
    let (form, set_form) = create_signal(FriaFormState::default());
    let (save_message, set_save_message) = create_signal(String::new());
    let (system_name, set_system_name) = create_signal(String::new());
    let (system_description, set_system_description) = create_signal(String::new());
    let (intended_purpose, set_intended_purpose) = create_signal(String::new());
    let (system_domain, set_system_domain) = create_signal(String::new());
    let (system_risk_category, set_system_risk_category) = create_signal("high".to_string());
    let (is_gpai, set_is_gpai) = create_signal(false);
    let (is_high_risk_listed, set_is_high_risk_listed) = create_signal(true);
    let (is_safety_component, set_is_safety_component) = create_signal(false);
    let (deployment_context, set_deployment_context) = create_signal(String::new());
    let (system_form_message, set_system_form_message) = create_signal(String::new());
    let (is_creating_system, set_is_creating_system) = create_signal(false);

    let route_system = create_resource(route_system_id, |id| async move {
        if id.is_empty() {
            Ok(None::<AiSystem>)
        } else {
            invoke::call::<_, AiSystem>("get_ai_system", &IdArg { id }).await.map(Some)
        }
    });

    let engagements = create_resource(move || refresh.get(), |_| async {
        invoke::call::<_, Vec<Engagement>>("list_engagements", &EngagementFilter { status_filter: None }).await
    });

    let systems = create_resource(move || (refresh.get(), selected_engagement.get()), |(_, engagement_id)| async move {
        if engagement_id.is_empty() {
            Ok(Vec::<AiSystem>::new())
        } else {
            invoke::call::<_, Vec<AiSystem>>("list_ai_systems", &EngagementIdArg { engagement_id }).await
        }
    });

    let fria = create_resource(move || (refresh.get(), selected_system.get()), |(_, ai_system_id)| async move {
        if ai_system_id.is_empty() {
            Ok(None::<FriaAssessment>)
        } else {
            invoke::call::<_, Option<FriaAssessment>>("get_fria_assessment", &SystemIdArg { ai_system_id }).await
        }
    });

    let risks = create_resource(move || selected_system.get(), |ai_system_id: String| async move {
        if ai_system_id.is_empty() {
            Ok(Vec::<RiskEntry>::new())
        } else {
            invoke::call::<_, Vec<RiskEntry>>(
                "list_risk_entries",
                &RiskFilter {
                    ai_system_id: Some(ai_system_id),
                },
            )
            .await
        }
    });

    let tasks = create_resource(move || selected_engagement.get(), |engagement_id: String| async move {
        if engagement_id.is_empty() {
            Ok(Vec::<Task>::new())
        } else {
            invoke::call::<_, Vec<Task>>(
                "list_tasks",
                &TaskFilter {
                    engagement_id: Some(engagement_id),
                    status: None,
                    priority: None,
                },
            )
            .await
        }
    });

    let evidence = create_resource(move || selected_engagement.get(), |engagement_id: String| async move {
        if engagement_id.is_empty() {
            Ok(Vec::<Evidence>::new())
        } else {
            invoke::call::<_, Vec<Evidence>>(
                "list_evidence",
                &EvidenceFilter {
                    engagement_id,
                    type_filter: None,
                },
            )
            .await
        }
    });

    create_effect(move |_| {
        if let Some(Ok(Some(system))) = route_system.get() {
            set_selected_engagement.set(system.engagement_id.to_string());
            set_selected_system.set(system.id.to_string());
        }
    });

    create_effect(move |_| {
        if route_system_id().is_empty() && selected_engagement.with(|value| value.is_empty()) {
            if let Some(Ok(list)) = engagements.get() {
                if let Some(first) = list.first() {
                    set_selected_engagement.set(first.id.to_string());
                }
            }
        }
    });

    create_effect(move |_| {
        if let Some(Ok(list)) = systems.get() {
            if list.is_empty() {
                set_selected_system.set(String::new());
            } else {
                let current = selected_system.get();
                let exists = list.iter().any(|system| system.id.to_string() == current);
                if current.is_empty() || !exists {
                    set_selected_system.set(list[0].id.to_string());
                }
            }
        }
    });

    create_effect(move |_| {
        if let Some(Ok(result)) = fria.get() {
            match result {
                Some(item) => set_form.set(form_from_assessment(&item)),
                None => set_form.set(FriaFormState::default()),
            }
        }
    });

    let save_fria = move |_| {
        let engagement_id = match Uuid::parse_str(&selected_engagement.get_untracked()) {
            Ok(id) => id,
            Err(_) => {
                set_save_message.set("Select an engagement first.".to_string());
                return;
            }
        };

        let ai_system_id = match Uuid::parse_str(&selected_system.get_untracked()) {
            Ok(id) => id,
            Err(_) => {
                set_save_message.set("Select an AI system first.".to_string());
                return;
            }
        };

        let current_form = form.get_untracked();
        let dto = UpsertFriaAssessmentDto {
            engagement_id,
            ai_system_id,
            status: enum_from_string::<FriaStatus>(&current_form.status),
            scope_summary: current_form.scope_summary,
            deployer_context: current_form.deployer_context,
            affected_persons_and_groups: current_form.affected_persons_and_groups,
            vulnerable_groups: current_form.vulnerable_groups,
            fundamental_rights_risks: current_form.fundamental_rights_risks,
            human_oversight_measures: current_form.human_oversight_measures,
            mitigation_measures: current_form.mitigation_measures,
            consultation_summary: current_form.consultation_summary,
            conclusion: current_form.conclusion,
            authority_notification_status: enum_from_string::<FriaNotificationStatus>(
                &current_form.authority_notification_status,
            ),
            review_date: if current_form.review_date.is_empty() {
                None
            } else {
                NaiveDate::parse_from_str(&current_form.review_date, "%Y-%m-%d").ok()
            },
            related_risk_ids: current_form
                .related_risk_ids
                .iter()
                .filter_map(|id| Uuid::parse_str(id).ok())
                .collect(),
            related_task_ids: current_form
                .related_task_ids
                .iter()
                .filter_map(|id| Uuid::parse_str(id).ok())
                .collect(),
            related_evidence_ids: current_form
                .related_evidence_ids
                .iter()
                .filter_map(|id| Uuid::parse_str(id).ok())
                .collect(),
        };

        spawn_local(async move {
            match invoke::call_named::<_, FriaAssessment>("upsert_fria_assessment", "dto", &dto).await {
                Ok(saved) => {
                    set_save_message.set(format!("FRIA saved. Last updated {}.", saved.updated_at.format("%Y-%m-%d %H:%M UTC")));
                    set_refresh.update(|value| *value += 1);
                }
                Err(error) => set_save_message.set(format!("Save failed: {}", error)),
            }
        });
    };

    let create_system = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let engagement_id = match Uuid::parse_str(&selected_engagement.get_untracked()) {
            Ok(id) => id,
            Err(_) => {
                set_system_form_message.set("Select an engagement first.".to_string());
                return;
            }
        };

        let dto = CreateAiSystemDto {
            engagement_id,
            name: system_name.get_untracked(),
            description: system_description.get_untracked(),
            intended_purpose: intended_purpose.get_untracked(),
            risk_category: enum_from_string::<RiskCategory>(&system_risk_category.get_untracked()),
            domain: system_domain.get_untracked(),
            is_gpai: is_gpai.get_untracked(),
            is_high_risk_listed: is_high_risk_listed.get_untracked(),
            is_safety_component: is_safety_component.get_untracked(),
            deployment_context: deployment_context.get_untracked(),
        };

        set_system_form_message.set(String::new());
        set_is_creating_system.set(true);

        spawn_local(async move {
            match invoke::call_named::<_, AiSystem>("create_ai_system", "dto", &dto).await {
                Ok(system) => {
                    set_system_name.set(String::new());
                    set_system_description.set(String::new());
                    set_intended_purpose.set(String::new());
                    set_system_domain.set(String::new());
                    set_system_risk_category.set("high".to_string());
                    set_is_gpai.set(false);
                    set_is_high_risk_listed.set(true);
                    set_is_safety_component.set(false);
                    set_deployment_context.set(String::new());
                    set_selected_system.set(system.id.to_string());
                    set_system_form_message.set(format!("Created AI system: {}.", system.name));
                    set_refresh.update(|value| *value += 1);
                }
                Err(error) => {
                    set_system_form_message.set(format!("Could not create AI system: {}", error));
                }
            }
            set_is_creating_system.set(false);
        });
    };

    view! {
        <div class="page fria-page">
            <div class="page-header">
                <h1>"Fundamental Rights Impact Assessment"</h1>
            </div>
            <HelpPanel title="FRIA Help">
                <HelpSection heading="When is a FRIA Required?">
                    <p>"Article 27 of the EU AI Act requires deployers of high-risk AI systems to conduct a FRIA before putting the system into use. This applies especially to credit scoring, employment decisions, law enforcement, and public services."</p>
                </HelpSection>
                <HelpSection heading="Completing the FRIA">
                    <p>"Select the engagement and AI system, then work through each section: scope and deployer context, affected groups, rights and risks, human oversight measures, mitigations, and consultation/notification status. Save frequently as you progress."</p>
                </HelpSection>
                <HelpSection heading="Linking Artifacts">
                    <p>"The FRIA integrates with risks, tasks, and evidence. Identified rights risks should be registered in the Risk Matrix. Mitigation measures should be tracked as tasks. Supporting documentation should be uploaded to the Evidence Vault."</p>
                </HelpSection>
            </HelpPanel>
            <p class="audit-note">
                "Use this workspace to record the Article 27 assessment, link supporting risks and evidence, and track notification readiness."
            </p>

            <Suspense fallback=move || view! { <p>"Loading FRIA workspace…"</p> }>
                {move || engagements.get().map(|result| match result {
                    Ok(list) if list.is_empty() => view! {
                        <p class="placeholder-text">"No engagements exist yet. Create an engagement first so FRIA can be scoped to a system."</p>
                    }.into_view(),
                    Ok(list) => {
                        view! {
                            <div class="fria-toolbar">
                                <label>
                                    <span>"Engagement"</span>
                                    <select
                                        prop:value=selected_engagement
                                        on:change=move |e| {
                                            set_selected_engagement.set(event_target_value(&e));
                                            set_save_message.set(String::new());
                                        }
                                    >
                                        {list.into_iter().map(|engagement| view! {
                                            <option value=engagement.id.to_string()>
                                                {format!("{} ({})", engagement.name, engagement.client_name)}
                                            </option>
                                        }).collect_view()}
                                    </select>
                                </label>

                                <label>
                                    <span>"AI System"</span>
                                    <select
                                        prop:value=selected_system
                                        prop:disabled=move || selected_engagement.get().is_empty()
                                        on:change=move |e| {
                                            set_selected_system.set(event_target_value(&e));
                                            set_save_message.set(String::new());
                                        }
                                    >
                                        {move || {
                                            if selected_engagement.get().is_empty() {
                                                view! { <option value="">"Select an engagement first"</option> }.into_view()
                                            } else {
                                                match systems.get() {
                                                    None => view! { <option value="">"Loading systems…"</option> }.into_view(),
                                                    Some(Ok(list)) if list.is_empty() => {
                                                        view! { <option value="">"No AI systems for this engagement"</option> }.into_view()
                                                    }
                                                    Some(Ok(list)) => list.into_iter().map(|system| view! {
                                                        <option value=system.id.to_string()>
                                                            {format!("{} ({})", system.name, system.domain)}
                                                        </option>
                                                    }).collect_view().into_view(),
                                                    Some(Err(_)) => view! { <option value="">"Unable to load systems"</option> }.into_view(),
                                                }
                                            }
                                        }}
                                    </select>
                                </label>
                            </div>

                            <Suspense fallback=move || view! { <p>"Loading system context…"</p> }>
                                {move || systems.get().map(|systems_result| match systems_result {
                                    Ok(systems) if systems.is_empty() => view! {
                                        <section class="fria-empty-state create-form">
                                            <h2>"Create AI System"</h2>
                                            <p class="placeholder-text">"The selected engagement has no AI systems yet. Add one here to continue the FRIA."</p>
                                            <form class="fria-inline-system-form" on:submit=create_system>
                                                <div class="fria-field-grid">
                                                    <label>
                                                        <span class="field-label">"System Name"</span>
                                                        <input
                                                            type="text"
                                                            required
                                                            prop:value=system_name
                                                            on:input=move |e| set_system_name.set(event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Domain"</span>
                                                        <input
                                                            type="text"
                                                            required
                                                            prop:value=system_domain
                                                            on:input=move |e| set_system_domain.set(event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Risk Category"</span>
                                                        <select
                                                            prop:value=system_risk_category
                                                            on:change=move |e| set_system_risk_category.set(event_target_value(&e))
                                                        >
                                                            {RISK_CATEGORY_OPTIONS.iter().copied().map(|(value, label)| view! {
                                                                <option value=value>{label}</option>
                                                            }).collect_view()}
                                                        </select>
                                                    </label>
                                                </div>

                                                <label>
                                                    <span class="field-label">"Description"</span>
                                                    <textarea
                                                        prop:value=system_description
                                                        on:input=move |e| set_system_description.set(event_target_value(&e))
                                                    />
                                                </label>

                                                <label>
                                                    <span class="field-label">"Intended Purpose"</span>
                                                    <textarea
                                                        prop:value=intended_purpose
                                                        on:input=move |e| set_intended_purpose.set(event_target_value(&e))
                                                    />
                                                </label>

                                                <label>
                                                    <span class="field-label">"Deployment Context"</span>
                                                    <textarea
                                                        prop:value=deployment_context
                                                        on:input=move |e| set_deployment_context.set(event_target_value(&e))
                                                    />
                                                </label>

                                                <div class="check-grid">
                                                    <label class="check-card">
                                                        <input
                                                            type="checkbox"
                                                            prop:checked=is_high_risk_listed
                                                            on:change=move |e| set_is_high_risk_listed.set(event_target_checked(&e))
                                                        />
                                                        <span>"Listed high-risk use case under Annex III or equivalent scope"</span>
                                                    </label>
                                                    <label class="check-card">
                                                        <input
                                                            type="checkbox"
                                                            prop:checked=is_gpai
                                                            on:change=move |e| set_is_gpai.set(event_target_checked(&e))
                                                        />
                                                        <span>"General-purpose AI model or capability"</span>
                                                    </label>
                                                    <label class="check-card">
                                                        <input
                                                            type="checkbox"
                                                            prop:checked=is_safety_component
                                                            on:change=move |e| set_is_safety_component.set(event_target_checked(&e))
                                                        />
                                                        <span>"Safety component used within a regulated product or process"</span>
                                                    </label>
                                                </div>

                                                <div class="fria-actions">
                                                    <button type="submit" prop:disabled=is_creating_system>
                                                        {move || if is_creating_system.get() { "Creating AI System…" } else { "Create AI System" }}
                                                    </button>
                                                    <span class="fria-save-message">{move || system_form_message.get()}</span>
                                                </div>
                                            </form>
                                        </section>
                                    }.into_view(),
                                    Ok(systems) => {
                                        let current_system_id = selected_system.get();
                                        let active_system = systems.into_iter().find(|system| system.id.to_string() == current_system_id);
                                        match active_system {
                                            Some(system) => {
                                                let risk_status = serde_json::to_value(&system.risk_category)
                                                    .ok()
                                                    .and_then(|value| value.as_str().map(String::from))
                                                    .unwrap_or_default();
                                                view! {
                                                    <div class="fria-summary">
                                                        <article>
                                                            <h2>"System Context"</h2>
                                                            <dl>
                                                                <dt>"System"</dt><dd>{system.name.clone()}</dd>
                                                                <dt>"Risk"</dt><dd><StatusBadge status=risk_status /></dd>
                                                                <dt>"Domain"</dt><dd>{system.domain.clone()}</dd>
                                                                <dt>"Purpose"</dt><dd>{system.intended_purpose.clone()}</dd>
                                                            </dl>
                                                        </article>
                                                        <article>
                                                            <h2>"Workflow Note"</h2>
                                                            <p>
                                                                "FRIA should capture deployer context, affected groups, vulnerable-group impact, human oversight, mitigation measures, and notification readiness in one place."
                                                            </p>
                                                        </article>
                                                    </div>
                                                }.into_view()
                                            }
                                            None => view! {
                                                <p class="placeholder-text">"Select an AI system to begin the FRIA."</p>
                                            }.into_view(),
                                        }
                                    }
                                    Err(error) => view! { <p class="error">{format!("Error: {}", error)}</p> }.into_view(),
                                })}
                            </Suspense>

                            <Suspense fallback=move || view! { <p>"Loading FRIA record…"</p> }>
                                {move || fria.get().map(|fria_result| match fria_result {
                                    Ok(current) => {
                                        let task_items: Vec<Task> = tasks.get().and_then(Result::ok).unwrap_or_default();
                                        let risk_items: Vec<RiskEntry> = risks.get().and_then(Result::ok).unwrap_or_default();
                                        let evidence_items: Vec<Evidence> = evidence.get().and_then(Result::ok).unwrap_or_default();
                                        let current_system_id = selected_system.get();
                                        let filtered_tasks: Vec<Task> = task_items.into_iter().filter(|task| {
                                            task.ai_system_id
                                                .map(|id| id.to_string())
                                                .map(|id| id == current_system_id)
                                                .unwrap_or(true)
                                        }).collect();

                                        view! {
                                            <form class="fria-form" on:submit=move |ev| {
                                                ev.prevent_default();
                                                save_fria(ev);
                                            }>
                                                <section class="fria-section">
                                                    <h2>"Assessment Status"</h2>
                                                    <div class="fria-field-grid">
                                                        <label>
                                                            <span class="field-label">"FRIA Status"</span>
                                                            <select
                                                                prop:value=move || form.with(|state| state.status.clone())
                                                                on:change=move |e| set_form.update(|state| state.status = event_target_value(&e))
                                                            >
                                                                <option value="draft">"Draft"</option>
                                                                <option value="in_progress">"In Progress"</option>
                                                                <option value="completed">"Completed"</option>
                                                                <option value="not_required">"Not Required"</option>
                                                            </select>
                                                        </label>
                                                        <label>
                                                            <span class="field-label">"Authority Notification"</span>
                                                            <select
                                                                prop:value=move || form.with(|state| state.authority_notification_status.clone())
                                                                on:change=move |e| set_form.update(|state| state.authority_notification_status = event_target_value(&e))
                                                            >
                                                                <option value="not_started">"Not Started"</option>
                                                                <option value="pending">"Pending"</option>
                                                                <option value="notified">"Notified"</option>
                                                                <option value="not_required">"Not Required"</option>
                                                            </select>
                                                        </label>
                                                        <label>
                                                            <span class="field-label">"Review Date"</span>
                                                            <input
                                                                type="date"
                                                                prop:value=move || form.with(|state| state.review_date.clone())
                                                                on:input=move |e| set_form.update(|state| state.review_date = event_target_value(&e))
                                                            />
                                                        </label>
                                                    </div>
                                                    {current.as_ref().map(|item| view! {
                                                        <p class="audit-note">{format!("Existing record last updated {}.", item.updated_at.format("%Y-%m-%d %H:%M UTC"))}</p>
                                                    })}
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Scope and Context"</h2>
                                                    <label>
                                                        <span class="field-label">"Scope Summary"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.scope_summary.clone())
                                                            on:input=move |e| set_form.update(|state| state.scope_summary = event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Deployer Context"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.deployer_context.clone())
                                                            on:input=move |e| set_form.update(|state| state.deployer_context = event_target_value(&e))
                                                        />
                                                    </label>
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Affected Persons and Risks"</h2>
                                                    <label>
                                                        <span class="field-label">"Affected Persons and Groups"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.affected_persons_and_groups.clone())
                                                            on:input=move |e| set_form.update(|state| state.affected_persons_and_groups = event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Vulnerable Groups"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.vulnerable_groups.clone())
                                                            on:input=move |e| set_form.update(|state| state.vulnerable_groups = event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Specific Risks to Fundamental Rights"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.fundamental_rights_risks.clone())
                                                            on:input=move |e| set_form.update(|state| state.fundamental_rights_risks = event_target_value(&e))
                                                        />
                                                    </label>
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Oversight and Mitigation"</h2>
                                                    <label>
                                                        <span class="field-label">"Human Oversight Measures"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.human_oversight_measures.clone())
                                                            on:input=move |e| set_form.update(|state| state.human_oversight_measures = event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Mitigation Measures"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.mitigation_measures.clone())
                                                            on:input=move |e| set_form.update(|state| state.mitigation_measures = event_target_value(&e))
                                                        />
                                                    </label>
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Consultation and Conclusion"</h2>
                                                    <label>
                                                        <span class="field-label">"Consultation Summary"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.consultation_summary.clone())
                                                            on:input=move |e| set_form.update(|state| state.consultation_summary = event_target_value(&e))
                                                        />
                                                    </label>
                                                    <label>
                                                        <span class="field-label">"Conclusion"</span>
                                                        <textarea
                                                            prop:value=move || form.with(|state| state.conclusion.clone())
                                                            on:input=move |e| set_form.update(|state| state.conclusion = event_target_value(&e))
                                                        />
                                                    </label>
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Linked Risks"</h2>
                                                    {if risk_items.is_empty() {
                                                        view! { <p class="placeholder-text">"No risks are recorded for this AI system yet."</p> }.into_view()
                                                    } else {
                                                        view! {
                                                            <div class="fria-checkbox-grid">
                                                                {risk_items.into_iter().map(|risk| {
                                                                    let risk_id = risk.id.to_string();
                                                                    let title = risk.title.clone();
                                                                    let severity = risk.inherent_score;
                                                                    let source = risk.risk_source.clone();
                                                                    let checked_id = risk_id.clone();
                                                                    let toggle_id = risk_id.clone();
                                                                    view! {
                                                                        <label class="check-card">
                                                                            <input
                                                                                type="checkbox"
                                                                                prop:checked=move || form.with(|state| state.related_risk_ids.contains(&checked_id))
                                                                                on:change=move |e| {
                                                                                    let checked = event_target_checked(&e);
                                                                                    set_form.update(|state| toggle_selection(&mut state.related_risk_ids, &toggle_id, checked));
                                                                                }
                                                                            />
                                                                            <div>
                                                                                <strong>{title}</strong>
                                                                                <span>{format!("Inherent score: {}", severity)}</span>
                                                                                <span>{source}</span>
                                                                            </div>
                                                                        </label>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        }.into_view()
                                                    }}
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Linked Tasks"</h2>
                                                    {if filtered_tasks.is_empty() {
                                                        view! { <p class="placeholder-text">"No tasks are available for this engagement or system yet."</p> }.into_view()
                                                    } else {
                                                        view! {
                                                            <div class="fria-checkbox-grid">
                                                                {filtered_tasks.into_iter().map(|task| {
                                                                    let task_id = task.id.to_string();
                                                                    let title = task.title.clone();
                                                                    let description = task.description.clone();
                                                                    let checked_id = task_id.clone();
                                                                    let toggle_id = task_id.clone();
                                                                    view! {
                                                                        <label class="check-card">
                                                                            <input
                                                                                type="checkbox"
                                                                                prop:checked=move || form.with(|state| state.related_task_ids.contains(&checked_id))
                                                                                on:change=move |e| {
                                                                                    let checked = event_target_checked(&e);
                                                                                    set_form.update(|state| toggle_selection(&mut state.related_task_ids, &toggle_id, checked));
                                                                                }
                                                                            />
                                                                            <div>
                                                                                <strong>{title}</strong>
                                                                                <span>{description}</span>
                                                                            </div>
                                                                        </label>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        }.into_view()
                                                    }}
                                                </section>

                                                <section class="fria-section">
                                                    <h2>"Linked Evidence"</h2>
                                                    {if evidence_items.is_empty() {
                                                        view! { <p class="placeholder-text">"No evidence is available for this engagement yet."</p> }.into_view()
                                                    } else {
                                                        view! {
                                                            <div class="fria-checkbox-grid">
                                                                {evidence_items.into_iter().map(|item| {
                                                                    let evidence_id = item.id.to_string();
                                                                    let name = item.file_name.clone();
                                                                    let description = item.description.clone();
                                                                    let checked_id = evidence_id.clone();
                                                                    let toggle_id = evidence_id.clone();
                                                                    view! {
                                                                        <label class="check-card">
                                                                            <input
                                                                                type="checkbox"
                                                                                prop:checked=move || form.with(|state| state.related_evidence_ids.contains(&checked_id))
                                                                                on:change=move |e| {
                                                                                    let checked = event_target_checked(&e);
                                                                                    set_form.update(|state| toggle_selection(&mut state.related_evidence_ids, &toggle_id, checked));
                                                                                }
                                                                            />
                                                                            <div>
                                                                                <strong>{name}</strong>
                                                                                <span>{description}</span>
                                                                            </div>
                                                                        </label>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        }.into_view()
                                                    }}
                                                </section>

                                                <div class="fria-actions">
                                                    <button type="submit">"Save FRIA"</button>
                                                    <span class="fria-save-message">{move || save_message.get()}</span>
                                                </div>
                                            </form>
                                        }.into_view()
                                    }
                                    Err(error) => view! { <p class="error">{format!("Error: {}", error)}</p> }.into_view(),
                                })}
                            </Suspense>
                        }.into_view()
                    }
                    Err(error) => view! { <p class="error">{format!("Error: {}", error)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}