use leptos::*;
use leptos_router::*;
use grc_shared::{
    suggest_frameworks_for_scope, AiSystem, AiUseCase, AssuranceObjective, CreateAiSystemDto,
    CreateTaskDto, Engagement, Framework, IndustrySector, Jurisdiction, ObligationRole,
    PersonalDataProfile, Priority, RiskCategory, Task, TaskStatus, UpdateEngagementDto,
    UpdateTaskDto,
};
use serde::Serialize;

use crate::api::invoke;
use crate::components::engagement_intake::{
    enum_from_string, enum_to_string, toggle_string, INDUSTRY_OPTIONS, JURISDICTION_OPTIONS,
    OBJECTIVE_OPTIONS, PERSONAL_DATA_OPTIONS, ROLE_OPTIONS, USE_CASE_OPTIONS,
};
use crate::components::status_badge::StatusBadge;
use crate::components::framework_pill::FrameworkPill;

#[derive(Serialize)]
struct IdArg { id: String }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementIdArg { engagement_id: String }

#[derive(Serialize)]
struct UpdateEngagementArg {
    id: String,
    dto: UpdateEngagementDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskListArg {
    engagement_id: Option<String>,
    status: Option<String>,
    priority: Option<String>,
}

#[derive(Serialize)]
struct TaskUpdateArg {
    id: String,
    dto: UpdateTaskDto,
}

#[derive(Serialize)]
struct TaskDeleteArg {
    id: String,
}

const TASK_STATUS_OPTIONS: &[(&str, &str)] = &[
    ("open", "Open"),
    ("in_progress", "In Progress"),
    ("blocked", "Blocked"),
    ("done", "Done"),
    ("deferred", "Deferred"),
];

const PRIORITY_OPTIONS: &[(&str, &str)] = &[
    ("critical", "Critical"),
    ("high", "High"),
    ("medium", "Medium"),
    ("low", "Low"),
];

struct AiSystemSeed {
    name: String,
    description: String,
    intended_purpose: String,
    domain: String,
    risk_category: String,
    is_gpai: bool,
    is_high_risk_listed: bool,
    is_safety_component: bool,
    deployment_context: String,
}

const RISK_CATEGORY_OPTIONS: &[(&str, &str)] = &[
    ("high", "High Risk"),
    ("limited", "Limited Risk"),
    ("minimal", "Minimal Risk"),
    ("gpai", "GPAI"),
    ("unacceptable", "Unacceptable (Prohibited)"),
];

fn ai_system_name_for_use_case(use_case: &AiUseCase) -> &'static str {
    match use_case {
        AiUseCase::GeneralAnalytics => "Decision Support System",
        AiUseCase::CreditScoring => "Credit Decisioning Engine",
        AiUseCase::EmploymentScreening => "Employment Screening Engine",
        AiUseCase::BiometricIdentification => "Biometric Identification System",
        AiUseCase::EmotionRecognition => "Emotion Recognition System",
        AiUseCase::RecommenderPersonalization => "Personalization Engine",
        AiUseCase::ConversationalAssistant => "Conversational Assistant",
        AiUseCase::FraudDetection => "Fraud Detection System",
        AiUseCase::SafetyComponent => "Safety Component AI System",
        AiUseCase::GenerativeAi => "Generative AI Assistant",
    }
}

fn ai_system_purpose_for_use_case(use_case: &AiUseCase) -> &'static str {
    match use_case {
        AiUseCase::GeneralAnalytics => "Support internal decision-making and analytics workflows.",
        AiUseCase::CreditScoring => "Score applicants, support underwriting decisions, and recommend approve/review/decline outcomes to human reviewers.",
        AiUseCase::EmploymentScreening => "Screen or rank candidates to support hiring decisions made by human reviewers.",
        AiUseCase::BiometricIdentification => "Identify or verify individuals using biometric data within the scoped operational workflow.",
        AiUseCase::EmotionRecognition => "Infer emotional state signals to support downstream operational decisions.",
        AiUseCase::RecommenderPersonalization => "Personalize content, offers, or experiences for end users.",
        AiUseCase::ConversationalAssistant => "Provide conversational assistance to users or staff in the scoped business process.",
        AiUseCase::FraudDetection => "Detect potentially fraudulent activity for human review and intervention.",
        AiUseCase::SafetyComponent => "Operate as a safety-relevant component within a regulated product or process.",
        AiUseCase::GenerativeAi => "Generate text or other content to support the scoped business workflow.",
    }
}

fn ai_system_risk_for_use_case(use_case: &AiUseCase) -> RiskCategory {
    match use_case {
        AiUseCase::CreditScoring
        | AiUseCase::EmploymentScreening
        | AiUseCase::BiometricIdentification
        | AiUseCase::EmotionRecognition
        | AiUseCase::SafetyComponent => RiskCategory::High,
        AiUseCase::GenerativeAi => RiskCategory::Gpai,
        AiUseCase::GeneralAnalytics
        | AiUseCase::RecommenderPersonalization
        | AiUseCase::ConversationalAssistant
        | AiUseCase::FraudDetection => RiskCategory::Limited,
    }
}

fn ai_system_seed_from_engagement(engagement: &Engagement) -> AiSystemSeed {
    let risk_category = ai_system_risk_for_use_case(&engagement.ai_use_case);
    AiSystemSeed {
        name: format!(
            "{} — {}",
            engagement.client_name,
            ai_system_name_for_use_case(&engagement.ai_use_case)
        ),
        description: if engagement.description.trim().is_empty() {
            format!(
                "AI system scoped from engagement intake for {}.",
                engagement.ai_use_case.display_name()
            )
        } else {
            engagement.description.clone()
        },
        intended_purpose: ai_system_purpose_for_use_case(&engagement.ai_use_case).to_string(),
        domain: engagement.industry_sector.display_name().to_string(),
        risk_category: enum_to_string(&risk_category),
        is_gpai: matches!(engagement.ai_use_case, AiUseCase::GenerativeAi),
        is_high_risk_listed: matches!(
            engagement.ai_use_case,
            AiUseCase::CreditScoring
                | AiUseCase::EmploymentScreening
                | AiUseCase::BiometricIdentification
                | AiUseCase::EmotionRecognition
                | AiUseCase::SafetyComponent
        ),
        is_safety_component: matches!(engagement.ai_use_case, AiUseCase::SafetyComponent),
        deployment_context: format!(
            "{} deployment for engagement '{}'.",
            engagement.primary_role.display_name(),
            engagement.name
        ),
    }
}

#[component]
fn EditEngagementForm(
    engagement: Engagement,
    on_saved: Callback<String>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (name, set_name) = create_signal(engagement.name.clone());
    let (client, set_client) = create_signal(engagement.client_name.clone());
    let (desc, set_desc) = create_signal(engagement.description.clone());
    let (primary_role, set_primary_role) = create_signal(enum_to_string(&engagement.primary_role));
    let (industry_sector, set_industry_sector) = create_signal(enum_to_string(&engagement.industry_sector));
    let (assurance_objective, set_assurance_objective) =
        create_signal(enum_to_string(&engagement.assurance_objective));
    let (ai_use_case, set_ai_use_case) = create_signal(enum_to_string(&engagement.ai_use_case));
    let (personal_data_profile, set_personal_data_profile) =
        create_signal(enum_to_string(&engagement.personal_data_profile));
    let (jurisdictions, set_jurisdictions) = create_signal(
        engagement
            .jurisdictions
            .iter()
            .map(enum_to_string)
            .collect::<Vec<_>>(),
    );
    let (involves_vulnerable_groups, set_involves_vulnerable_groups) =
        create_signal(engagement.involves_vulnerable_groups);
    let (is_public_facing, set_is_public_facing) = create_signal(engagement.is_public_facing);
    let (selected_frameworks, set_selected_frameworks) = create_signal(
        engagement
            .frameworks
            .iter()
            .map(enum_to_string)
            .collect::<Vec<_>>(),
    );
    let (form_message, set_form_message) = create_signal::<Option<String>>(None);
    let (is_saving, set_is_saving) = create_signal(false);

    let framework_suggestions = create_memo(move |_| {
        suggest_frameworks_for_scope(
            &enum_from_string::<ObligationRole>(&primary_role.get()),
            &enum_from_string::<IndustrySector>(&industry_sector.get()),
            &jurisdictions
                .get()
                .into_iter()
                .map(|value| enum_from_string::<Jurisdiction>(&value))
                .collect::<Vec<_>>(),
            &enum_from_string::<AssuranceObjective>(&assurance_objective.get()),
            &enum_from_string::<AiUseCase>(&ai_use_case.get()),
            &enum_from_string::<PersonalDataProfile>(&personal_data_profile.get()),
            involves_vulnerable_groups.get(),
            is_public_facing.get(),
        )
    });

    create_effect(move |_| {
        let selected = framework_suggestions
            .get()
            .into_iter()
            .map(|item| enum_to_string(&item.framework))
            .collect::<Vec<_>>();
        set_selected_frameworks.set(selected);
    });

    let engagement_id = engagement.id.to_string();
    let engagement_name = engagement.name.clone();

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let name_value = name.get();
        let client_value = client.get();
        let desc_value = desc.get();
        let role_value = primary_role.get();
        let industry_value = industry_sector.get();
        let objective_value = assurance_objective.get();
        let use_case_value = ai_use_case.get();
        let personal_data_value = personal_data_profile.get();
        let jurisdiction_values = jurisdictions.get();
        let framework_values = selected_frameworks.get();
        let vulnerable_groups = involves_vulnerable_groups.get();
        let public_facing = is_public_facing.get();
        let id_value = engagement_id.clone();
        let saved_name = engagement_name.clone();
        set_form_message.set(None);
        set_is_saving.set(true);

        spawn_local(async move {
            let dto = UpdateEngagementDto {
                name: Some(name_value),
                client_name: Some(client_value),
                description: Some(desc_value),
                status: None,
                primary_role: Some(enum_from_string::<ObligationRole>(&role_value)),
                industry_sector: Some(enum_from_string::<IndustrySector>(&industry_value)),
                jurisdictions: Some(
                    jurisdiction_values
                        .iter()
                        .map(|value| enum_from_string::<Jurisdiction>(value))
                        .collect(),
                ),
                assurance_objective: Some(
                    enum_from_string::<AssuranceObjective>(&objective_value),
                ),
                ai_use_case: Some(enum_from_string::<AiUseCase>(&use_case_value)),
                personal_data_profile: Some(
                    enum_from_string::<PersonalDataProfile>(&personal_data_value),
                ),
                involves_vulnerable_groups: Some(vulnerable_groups),
                is_public_facing: Some(public_facing),
                frameworks: Some(
                    framework_values
                        .iter()
                        .map(|value| enum_from_string::<Framework>(value))
                        .collect(),
                ),
            };

            let result = invoke::call::<_, Engagement>(
                "update_engagement",
                &UpdateEngagementArg {
                    id: id_value,
                    dto,
                },
            )
            .await;

            set_is_saving.set(false);
            match result {
                Ok(_) => on_saved.call(format!("Updated intake for {}.", saved_name)),
                Err(error) => {
                    set_form_message.set(Some(format!("Could not update engagement: {}", error)));
                }
            }
        });
    };

    view! {
        <form on:submit=on_submit class="create-form">
            <div class="page-header detail-edit-header">
                <h2>"Edit Intake"</h2>
                <div class="detail-edit-header-actions">
                    <button type="button" class="secondary" on:click=move |_| on_cancel.call(())>
                        "Cancel"
                    </button>
                    <button type="submit" prop:disabled=is_saving>
                        {move || if is_saving.get() { "Saving…" } else { "Save Intake" }}
                    </button>
                </div>
            </div>

            <div class="intake-grid">
                <label>
                    "Name"
                    <input type="text" required prop:value=name on:input=move |e| set_name.set(event_target_value(&e)) />
                </label>
                <label>
                    "Client"
                    <input type="text" required prop:value=client on:input=move |e| set_client.set(event_target_value(&e)) />
                </label>
            </div>
            <label>
                "Description"
                <textarea prop:value=desc on:input=move |e| set_desc.set(event_target_value(&e))></textarea>
            </label>

            <section class="intake-section">
                <h2>"Scoping Inputs"</h2>
                <div class="intake-grid">
                    <label>
                        "Primary Role"
                        <select prop:value=primary_role on:change=move |e| set_primary_role.set(event_target_value(&e))>
                            {ROLE_OPTIONS.iter().copied().map(|(value, label)| view! { <option value=value>{label}</option> }).collect_view()}
                        </select>
                    </label>
                    <label>
                        "Industry Sector"
                        <select prop:value=industry_sector on:change=move |e| set_industry_sector.set(event_target_value(&e))>
                            {INDUSTRY_OPTIONS.iter().copied().map(|(value, label)| view! { <option value=value>{label}</option> }).collect_view()}
                        </select>
                    </label>
                    <label>
                        "Assurance Objective"
                        <select prop:value=assurance_objective on:change=move |e| set_assurance_objective.set(event_target_value(&e))>
                            {OBJECTIVE_OPTIONS.iter().copied().map(|(value, label)| view! { <option value=value>{label}</option> }).collect_view()}
                        </select>
                    </label>
                    <label>
                        "AI Use Case"
                        <select prop:value=ai_use_case on:change=move |e| set_ai_use_case.set(event_target_value(&e))>
                            {USE_CASE_OPTIONS.iter().copied().map(|(value, label)| view! { <option value=value>{label}</option> }).collect_view()}
                        </select>
                    </label>
                    <label>
                        "Personal Data Profile"
                        <select prop:value=personal_data_profile on:change=move |e| set_personal_data_profile.set(event_target_value(&e))>
                            {PERSONAL_DATA_OPTIONS.iter().copied().map(|(value, label)| view! { <option value=value>{label}</option> }).collect_view()}
                        </select>
                    </label>
                </div>
            </section>

            <section class="intake-section">
                <h2>"Jurisdictions"</h2>
                <div class="check-grid">
                    {JURISDICTION_OPTIONS.iter().copied().map(|(value, label)| {
                        let checked_value = value.to_string();
                        let toggle_value = value.to_string();
                        view! {
                            <label class="check-card">
                                <input
                                    type="checkbox"
                                    prop:checked=move || jurisdictions.with(|items| items.contains(&checked_value))
                                    on:change=move |e| {
                                        let checked = event_target_checked(&e);
                                        set_jurisdictions.update(|items| toggle_string(items, &toggle_value, checked));
                                    }
                                />
                                <span>{label}</span>
                            </label>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class="intake-section">
                <h2>"Exposure Flags"</h2>
                <div class="check-grid">
                    <label class="check-card">
                        <input
                            type="checkbox"
                            prop:checked=involves_vulnerable_groups
                            on:change=move |e| set_involves_vulnerable_groups.set(event_target_checked(&e))
                        />
                        <span>"Impacts children, elderly persons, or other vulnerable groups"</span>
                    </label>
                    <label class="check-card">
                        <input
                            type="checkbox"
                            prop:checked=is_public_facing
                            on:change=move |e| set_is_public_facing.set(event_target_checked(&e))
                        />
                        <span>"Public-facing or customer-facing interaction"</span>
                    </label>
                </div>
            </section>

            <section class="intake-section">
                <h2>"Suggested Frameworks"</h2>
                <p class="audit-note">"Selections stay editable. Use the suggestions as guidance when you correct the classification."</p>
                <div class="check-grid framework-check-grid">
                    {Framework::all().iter().cloned().map(|framework| {
                        let framework_value = enum_to_string(&framework);
                        let checked_value = framework_value.clone();
                        let toggle_value = framework_value.clone();
                        let display_name = framework.display_name().to_string();
                        let suggestion_reason = framework_suggestions
                            .get()
                            .into_iter()
                            .find(|item| item.framework == framework)
                            .map(|item| item.reason)
                            .unwrap_or_else(|| "Not suggested by the current intake answers, but still available for manual inclusion.".to_string());
                        view! {
                            <label class="check-card">
                                <input
                                    type="checkbox"
                                    prop:checked=move || selected_frameworks.with(|items| items.contains(&checked_value))
                                    on:change=move |e| {
                                        let checked = event_target_checked(&e);
                                        set_selected_frameworks.update(|items| toggle_string(items, &toggle_value, checked));
                                    }
                                />
                                <div>
                                    <strong>{display_name}</strong>
                                    <span>{suggestion_reason}</span>
                                </div>
                            </label>
                        }
                    }).collect_view()}
                </div>
            </section>

            {move || form_message.get().map(|message| view! {
                <p class="error">{message}</p>
            })}

            <div class="detail-edit-actions">
                <button type="submit" prop:disabled=is_saving>
                    {move || if is_saving.get() { "Saving…" } else { "Save Intake" }}
                </button>
            </div>
        </form>
    }
}

#[component]
fn TasksSection(
    engagement_id: String,
    refresh: ReadSignal<u32>,
    set_refresh: WriteSignal<u32>,
) -> impl IntoView {
    let eid = engagement_id.clone();
    let (show_form, set_show_form) = create_signal(false);
    let (task_title, set_task_title) = create_signal(String::new());
    let (task_desc, set_task_desc) = create_signal(String::new());
    let (task_priority, set_task_priority) = create_signal("medium".to_string());
    let (task_due, set_task_due) = create_signal(String::new());
    let (task_msg, set_task_msg) = create_signal::<Option<String>>(None);
    let (is_creating, set_is_creating) = create_signal(false);
    let (task_refresh, set_task_refresh) = create_signal(0u32);

    let tasks = create_resource(
        move || (eid.clone(), refresh.get(), task_refresh.get()),
        |(eid, _, _)| async move {
            invoke::call::<_, Vec<Task>>(
                "list_tasks",
                &TaskListArg {
                    engagement_id: Some(eid),
                    status: None,
                    priority: None,
                },
            )
            .await
        },
    );

    let form_eid = engagement_id.clone();
    let on_create = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let eid = form_eid.clone();
        let parsed_eid = match uuid::Uuid::parse_str(&eid) {
            Ok(id) => id,
            Err(_) => {
                set_task_msg.set(Some("Invalid engagement ID.".to_string()));
                return;
            }
        };

        let title = task_title.get_untracked();
        let description = task_desc.get_untracked();
        let priority_str = task_priority.get_untracked();
        let due_str = task_due.get_untracked();

        let due_date = if due_str.is_empty() {
            None
        } else {
            match chrono::NaiveDate::parse_from_str(&due_str, "%Y-%m-%d") {
                Ok(d) => Some(d),
                Err(_) => {
                    set_task_msg.set(Some("Invalid date format.".to_string()));
                    return;
                }
            }
        };

        let dto = CreateTaskDto {
            engagement_id: parsed_eid,
            ai_system_id: None,
            title,
            description,
            framework: None,
            related_requirement_id: None,
            priority: enum_from_string::<Priority>(&priority_str),
            due_date,
        };

        set_is_creating.set(true);
        set_task_msg.set(None);

        spawn_local(async move {
            match invoke::call_named::<_, Task>("create_task", "dto", &dto).await {
                Ok(t) => {
                    set_task_title.set(String::new());
                    set_task_desc.set(String::new());
                    set_task_priority.set("medium".to_string());
                    set_task_due.set(String::new());
                    set_show_form.set(false);
                    set_task_msg.set(Some(format!("Created task: {}", t.title)));
                    set_task_refresh.update(|v| *v += 1);
                }
                Err(e) => {
                    set_task_msg.set(Some(format!("Error: {}", e)));
                }
            }
            set_is_creating.set(false);
        });
    };

    let on_create = Callback::new(on_create);

    view! {
        <section class="tasks-section" style="margin-top:2rem;">
            <div class="page-header" style="display:flex;justify-content:space-between;align-items:center;">
                <h2>"Tasks"</h2>
                <button on:click=move |_| {
                    set_task_msg.set(None);
                    set_show_form.update(|v| *v = !*v);
                }>
                    {move || if show_form.get() { "Cancel" } else { "+ Add Task" }}
                </button>
            </div>

            {move || task_msg.get().map(|m| view! { <p class="audit-note">{m}</p> })}

            <Show when=move || show_form.get()>
                <form class="create-form" on:submit=move |ev| on_create.call(ev)>
                    <label>
                        "Title"
                        <input type="text" required prop:value=task_title on:input=move |e| set_task_title.set(event_target_value(&e)) />
                    </label>
                    <label>
                        "Description"
                        <textarea prop:value=task_desc on:input=move |e| set_task_desc.set(event_target_value(&e))></textarea>
                    </label>
                    <div style="display:grid;grid-template-columns:1fr 1fr;gap:1rem;">
                        <label>
                            "Priority"
                            <select prop:value=task_priority on:change=move |e| set_task_priority.set(event_target_value(&e))>
                                {PRIORITY_OPTIONS.iter().copied().map(|(v, l)| view! { <option value=v>{l}</option> }).collect_view()}
                            </select>
                        </label>
                        <label>
                            "Due Date"
                            <input type="date" prop:value=task_due on:input=move |e| set_task_due.set(event_target_value(&e)) />
                        </label>
                    </div>
                    <button type="submit" prop:disabled=is_creating>
                        {move || if is_creating.get() { "Creating…" } else { "Create Task" }}
                    </button>
                </form>
            </Show>

            <Suspense fallback=move || view! { <p>"Loading tasks…"</p> }>
                {move || tasks.get().map(|result| match result {
                    Ok(list) if list.is_empty() => view! {
                        <p class="placeholder-text">"No tasks for this engagement yet."</p>
                    }.into_view(),
                    Ok(list) => view! {
                        <table role="grid">
                            <thead>
                                <tr>
                                    <th>"Title"</th>
                                    <th>"Status"</th>
                                    <th>"Priority"</th>
                                    <th>"Due"</th>
                                    <th></th>
                                </tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|t| {
                                    let tid = t.id.to_string();
                                    let status_val = enum_to_string(&t.status);
                                    let priority_val = enum_to_string(&t.priority);
                                    let due = t.due_date.map(|d| d.to_string()).unwrap_or_default();
                                    let status_tid = tid.clone();
                                    let priority_tid = tid.clone();
                                    let delete_tid = tid.clone();
                                    let title = t.title.clone();
                                    view! {
                                        <tr>
                                            <td>{title}</td>
                                            <td>
                                                <select
                                                    prop:value=status_val
                                                    on:change=move |e| {
                                                        let new_status = event_target_value(&e);
                                                        let id = status_tid.clone();
                                                        spawn_local(async move {
                                                            let _ = invoke::call::<_, Task>(
                                                                "update_task",
                                                                &TaskUpdateArg {
                                                                    id,
                                                                    dto: UpdateTaskDto {
                                                                        title: None,
                                                                        description: None,
                                                                        framework: None,
                                                                        related_requirement_id: None,
                                                                        status: Some(enum_from_string::<TaskStatus>(&new_status)),
                                                                        priority: None,
                                                                        due_date: None,
                                                                    },
                                                                },
                                                            ).await;
                                                            set_task_refresh.update(|v| *v += 1);
                                                        });
                                                    }
                                                >
                                                    {TASK_STATUS_OPTIONS.iter().copied().map(|(v, l)| view! { <option value=v>{l}</option> }).collect_view()}
                                                </select>
                                            </td>
                                            <td>
                                                <select
                                                    prop:value=priority_val
                                                    on:change=move |e| {
                                                        let new_priority = event_target_value(&e);
                                                        let id = priority_tid.clone();
                                                        spawn_local(async move {
                                                            let _ = invoke::call::<_, Task>(
                                                                "update_task",
                                                                &TaskUpdateArg {
                                                                    id,
                                                                    dto: UpdateTaskDto {
                                                                        title: None,
                                                                        description: None,
                                                                        framework: None,
                                                                        related_requirement_id: None,
                                                                        status: None,
                                                                        priority: Some(enum_from_string::<Priority>(&new_priority)),
                                                                        due_date: None,
                                                                    },
                                                                },
                                                            ).await;
                                                            set_task_refresh.update(|v| *v += 1);
                                                        });
                                                    }
                                                >
                                                    {PRIORITY_OPTIONS.iter().copied().map(|(v, l)| view! { <option value=v>{l}</option> }).collect_view()}
                                                </select>
                                            </td>
                                            <td>{due}</td>
                                            <td>
                                                <button
                                                    class="outline secondary"
                                                    style="padding:0.25rem 0.5rem;font-size:0.85rem;"
                                                    on:click=move |_| {
                                                        let id = delete_tid.clone();
                                                        spawn_local(async move {
                                                            let _ = invoke::call::<_, ()>(
                                                                "delete_task",
                                                                &TaskDeleteArg { id },
                                                            ).await;
                                                            set_task_refresh.update(|v| *v += 1);
                                                        });
                                                    }
                                                >
                                                    "Delete"
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    }.into_view(),
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </section>
    }
}

#[component]
pub fn EngagementDetailPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let navigate = use_navigate();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let (refresh, set_refresh) = create_signal(0u32);
    let (is_editing, set_is_editing) = create_signal(false);
    let (page_message, set_page_message) = create_signal::<Option<String>>(None);
    let (show_system_form, set_show_system_form) = create_signal(false);
    let (system_name, set_system_name) = create_signal(String::new());
    let (system_description, set_system_description) = create_signal(String::new());
    let (system_purpose, set_system_purpose) = create_signal(String::new());
    let (system_domain, set_system_domain) = create_signal(String::new());
    let (system_risk_category, set_system_risk_category) = create_signal("high".to_string());
    let (is_gpai, set_is_gpai) = create_signal(false);
    let (is_high_risk_listed, set_is_high_risk_listed) = create_signal(true);
    let (is_safety_component, set_is_safety_component) = create_signal(false);
    let (deployment_context, set_deployment_context) = create_signal(String::new());
    let (system_form_message, set_system_form_message) = create_signal::<Option<String>>(None);
    let (is_creating_system, set_is_creating_system) = create_signal(false);
    let (system_form_seeded, set_system_form_seeded) = create_signal(false);

    let engagement = create_resource(
        move || (id(), refresh.get()),
        |(id, _)| async move {
            invoke::call::<_, Engagement>("get_engagement", &IdArg { id }).await
        },
    );

    let systems = create_resource(
        move || (id(), refresh.get()),
        |(id, _)| async move {
            invoke::call::<_, Vec<AiSystem>>("list_ai_systems", &EngagementIdArg { engagement_id: id }).await
        },
    );

    create_effect(move |_| {
        let should_prompt = query.with(|params| {
            params
                .get("prompt")
                .map(|value| value == "ai-system")
                .unwrap_or(false)
        });

        if !should_prompt {
            set_system_form_seeded.set(false);
            return;
        }

        if system_form_seeded.get() {
            return;
        }

        if let Some(Ok(engagement)) = engagement.get() {
            let seed = ai_system_seed_from_engagement(&engagement);
            set_system_name.set(seed.name);
            set_system_description.set(seed.description);
            set_system_purpose.set(seed.intended_purpose);
            set_system_domain.set(seed.domain);
            set_system_risk_category.set(seed.risk_category);
            set_is_gpai.set(seed.is_gpai);
            set_is_high_risk_listed.set(seed.is_high_risk_listed);
            set_is_safety_component.set(seed.is_safety_component);
            set_deployment_context.set(seed.deployment_context);
            set_show_system_form.set(true);
            set_system_form_seeded.set(true);
            set_page_message.set(Some("Next step: add the scoped AI system so FRIA and linked evidence can attach to it.".to_string()));
        }
    });

    let create_system = Callback::new(move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let nav = navigate.clone();
        let route_id = id();
        let engagement_id = match uuid::Uuid::parse_str(&route_id) {
            Ok(id) => id,
            Err(_) => {
                set_system_form_message.set(Some("Could not resolve the current engagement.".to_string()));
                return;
            }
        };

        let dto = CreateAiSystemDto {
            engagement_id,
            name: system_name.get_untracked(),
            description: system_description.get_untracked(),
            intended_purpose: system_purpose.get_untracked(),
            risk_category: enum_from_string::<RiskCategory>(&system_risk_category.get_untracked()),
            domain: system_domain.get_untracked(),
            is_gpai: is_gpai.get_untracked(),
            is_high_risk_listed: is_high_risk_listed.get_untracked(),
            is_safety_component: is_safety_component.get_untracked(),
            deployment_context: deployment_context.get_untracked(),
        };

        set_system_form_message.set(None);
        set_is_creating_system.set(true);

        spawn_local(async move {
            match invoke::call_named::<_, AiSystem>("create_ai_system", "dto", &dto).await {
                Ok(system) => {
                    set_system_name.set(String::new());
                    set_system_description.set(String::new());
                    set_system_purpose.set(String::new());
                    set_system_domain.set(String::new());
                    set_system_risk_category.set("high".to_string());
                    set_is_gpai.set(false);
                    set_is_high_risk_listed.set(true);
                    set_is_safety_component.set(false);
                    set_deployment_context.set(String::new());
                    set_show_system_form.set(false);
                    set_system_form_seeded.set(false);
                    set_system_form_message.set(Some(format!("Created AI system: {}.", system.name)));
                    set_refresh.update(|value| *value += 1);
                    nav(&format!("/engagements/{}", route_id), Default::default());
                }
                Err(error) => {
                    set_system_form_message.set(Some(format!("Could not create AI system: {}", error)));
                }
            }
            set_is_creating_system.set(false);
        });
    });

    view! {
        <div class="page engagement-detail-page">
            <Suspense fallback=move || view! { <p>"Loading engagement…"</p> }>
                {move || engagement.get().map(|result| match result {
                    Ok(e) => {
                        let status_str = serde_json::to_value(&e.status)
                            .ok()
                            .and_then(|v| v.as_str().map(String::from))
                            .unwrap_or_default();
                        let editable_engagement = e.clone();
                        let handle_saved = Callback::new(move |message: String| {
                            set_is_editing.set(false);
                            set_page_message.set(Some(message));
                            set_refresh.update(|value| *value += 1);
                        });
                        let cancel_edit = Callback::new(move |_| {
                            set_is_editing.set(false);
                        });
                        view! {
                            <header class="detail-header">
                                <h1>{&e.name}</h1>
                                <StatusBadge status=status_str />
                            </header>
                            <div class="detail-actions">
                                <button on:click=move |_| {
                                    set_page_message.set(None);
                                    set_is_editing.set(true);
                                }>
                                    "Edit Intake"
                                </button>
                            </div>
                            {move || page_message.get().map(|message| view! {
                                <p class="audit-note">{message}</p>
                            })}
                            <Show when=move || is_editing.get()>
                                <EditEngagementForm
                                    engagement=editable_engagement.clone()
                                    on_saved=handle_saved
                                    on_cancel=cancel_edit
                                />
                            </Show>
                            <dl class="detail-meta">
                                <dt>"Client"</dt><dd>{&e.client_name}</dd>
                                <dt>"Description"</dt><dd>{&e.description}</dd>
                                <dt>"Primary Role"</dt><dd>{e.primary_role.display_name()}</dd>
                                <dt>"Industry"</dt><dd>{e.industry_sector.display_name()}</dd>
                                <dt>"Objective"</dt><dd>{e.assurance_objective.display_name()}</dd>
                                <dt>"AI Use Case"</dt><dd>{e.ai_use_case.display_name()}</dd>
                                <dt>"Data Profile"</dt><dd>{e.personal_data_profile.display_name()}</dd>
                                <dt>"Vulnerable Groups"</dt><dd>{if e.involves_vulnerable_groups { "Yes" } else { "No" }}</dd>
                                <dt>"Public Facing"</dt><dd>{if e.is_public_facing { "Yes" } else { "No" }}</dd>
                                <dt>"Jurisdictions"</dt>
                                <dd>
                                    {e.jurisdictions.iter().map(|jurisdiction| {
                                        view! { <span class="tag">{jurisdiction.display_name()}</span> }
                                    }).collect_view()}
                                </dd>
                                <dt>"Frameworks"</dt>
                                <dd>
                                    {e.frameworks.iter().map(|f| {
                                        let fw = serde_json::to_value(f)
                                            .ok()
                                            .and_then(|v| v.as_str().map(String::from))
                                            .unwrap_or_default();
                                        view! { <FrameworkPill framework=fw /> }
                                    }).collect_view()}
                                </dd>
                            </dl>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>

            <section class="ai-systems-section">
                <div class="page-header ai-systems-header">
                    <h2>"AI Systems"</h2>
                    <button on:click=move |_| {
                        set_system_form_message.set(None);
                        set_show_system_form.update(|value| *value = !*value);
                    }>
                        {move || if show_system_form.get() { "Cancel" } else { "+ Add AI System" }}
                    </button>
                </div>

                {move || system_form_message.get().map(|message| view! {
                    <p class="audit-note">{message}</p>
                })}

                <Show when=move || show_system_form.get()>
                    <form class="create-form" on:submit=move |ev| create_system.call(ev)>
                        <div class="intake-grid">
                            <label>
                                "System Name"
                                <input type="text" required prop:value=system_name on:input=move |e| set_system_name.set(event_target_value(&e)) />
                            </label>
                            <label>
                                "Domain"
                                <input type="text" required prop:value=system_domain on:input=move |e| set_system_domain.set(event_target_value(&e)) />
                            </label>
                            <label>
                                "Risk Category"
                                <select prop:value=system_risk_category on:change=move |e| set_system_risk_category.set(event_target_value(&e))>
                                    {RISK_CATEGORY_OPTIONS.iter().copied().map(|(value, label)| view! { <option value=value>{label}</option> }).collect_view()}
                                </select>
                            </label>
                        </div>

                        <label>
                            "Description"
                            <textarea prop:value=system_description on:input=move |e| set_system_description.set(event_target_value(&e))></textarea>
                        </label>
                        <label>
                            "Intended Purpose"
                            <textarea prop:value=system_purpose on:input=move |e| set_system_purpose.set(event_target_value(&e))></textarea>
                        </label>
                        <label>
                            "Deployment Context"
                            <textarea prop:value=deployment_context on:input=move |e| set_deployment_context.set(event_target_value(&e))></textarea>
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

                        <div class="detail-edit-actions">
                            <button type="submit" prop:disabled=is_creating_system>
                                {move || if is_creating_system.get() { "Creating AI System…" } else { "Create AI System" }}
                            </button>
                        </div>
                    </form>
                </Show>

                <Suspense fallback=move || view! { <p>"Loading systems…"</p> }>
                    {move || systems.get().map(|result| match result {
                        Ok(list) if list.is_empty() => view! {
                            <p class="placeholder-text">"No AI systems have been added for this engagement yet. Add one here so FRIA, risks, tasks, and evidence can be linked to a specific system."</p>
                        }.into_view(),
                        Ok(list) => view! {
                            <div class="cards-grid">
                                {list.into_iter().map(|s| {
                                    let sid = s.id.to_string();
                                    let risk_str = serde_json::to_value(&s.risk_category)
                                        .ok()
                                        .and_then(|v| v.as_str().map(String::from))
                                        .unwrap_or_default();
                                    view! {
                                        <article class="system-card">
                                            <header>
                                                <A href=format!("/systems/{}", sid)>{&s.name}</A>
                                                <StatusBadge status=risk_str />
                                            </header>
                                            <p>{&s.domain}</p>
                                            <p>{&s.intended_purpose}</p>
                                        </article>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_view(),
                        Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                    })}
                </Suspense>
            </section>

            <TasksSection engagement_id=id() refresh=refresh set_refresh=set_refresh />

            <section style="margin-top:2rem;">
                <h2>"Evidence & Reports"</h2>
                <div style="display:flex;gap:1rem;">
                    <A href=format!("/evidence")>
                        <button class="outline">"Evidence Vault"</button>
                    </A>
                    <A href=format!("/gap-analysis")>
                        <button class="outline">"Gap Analysis"</button>
                    </A>
                    <A href=format!("/reports")>
                        <button class="outline">"Reports"</button>
                    </A>
                </div>
            </section>
        </div>
    }
}
