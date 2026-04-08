use leptos::*;
use leptos_router::*;
use grc_shared::{
    suggest_frameworks_for_scope, AiUseCase, AssuranceObjective, CreateEngagementDto, Engagement,
    Framework, IndustrySector, Jurisdiction, ObligationRole, PersonalDataProfile,
};

use crate::api::invoke;
use crate::components::engagement_intake::{
    enum_from_string, INDUSTRY_OPTIONS, JURISDICTION_OPTIONS, OBJECTIVE_OPTIONS,
    PERSONAL_DATA_OPTIONS, ROLE_OPTIONS, USE_CASE_OPTIONS, toggle_string,
};
use crate::components::status_badge::StatusBadge;
use crate::components::framework_pill::FrameworkPill;
use crate::components::help_panel::{HelpPanel, HelpSection};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListFilter {
    status_filter: Option<String>,
}

#[component]
pub fn EngagementsPage() -> impl IntoView {
    let (refresh, set_refresh) = create_signal(0u32);
    let navigate = use_navigate();

    let engagements = create_resource(
        move || refresh.get(),
        |_| async {
            invoke::call::<_, Vec<Engagement>>("list_engagements", &ListFilter { status_filter: None }).await
        },
    );

    let (show_form, set_show_form) = create_signal(false);
    let (name, set_name) = create_signal(String::new());
    let (client, set_client) = create_signal(String::new());
    let (desc, set_desc) = create_signal(String::new());
    let (primary_role, set_primary_role) = create_signal("deployer".to_string());
    let (industry_sector, set_industry_sector) = create_signal("finance".to_string());
    let (assurance_objective, set_assurance_objective) = create_signal("baseline_compliance_review".to_string());
    let (ai_use_case, set_ai_use_case) = create_signal("credit_scoring".to_string());
    let (personal_data_profile, set_personal_data_profile) = create_signal("personal_data".to_string());
    let (jurisdictions, set_jurisdictions) = create_signal(vec!["eu".to_string()]);
    let (involves_vulnerable_groups, set_involves_vulnerable_groups) = create_signal(false);
    let (is_public_facing, set_is_public_facing) = create_signal(false);
    let (selected_frameworks, set_selected_frameworks) = create_signal::<Vec<String>>(Vec::new());
    let (form_message, set_form_message) = create_signal::<Option<String>>(None);

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
            .filter_map(|item| serde_json::to_value(item.framework).ok())
            .filter_map(|value| value.as_str().map(String::from))
            .collect::<Vec<_>>();
        set_selected_frameworks.set(selected);
    });

    let on_create = Callback::new(move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let nav = navigate.clone();
        let n = name.get();
        let c = client.get();
        let d = desc.get();
        let role = primary_role.get();
        let sector = industry_sector.get();
        let objective = assurance_objective.get();
        let use_case = ai_use_case.get();
        let personal = personal_data_profile.get();
        let selected_jurisdictions = jurisdictions.get();
        let selected_framework_values = selected_frameworks.get();
        let vulnerable_groups = involves_vulnerable_groups.get();
        let public_facing = is_public_facing.get();
        set_form_message.set(None);
        spawn_local(async move {
            let dto = CreateEngagementDto {
                name: n,
                client_name: c,
                description: d,
                primary_role: enum_from_string::<ObligationRole>(&role),
                industry_sector: enum_from_string::<IndustrySector>(&sector),
                jurisdictions: selected_jurisdictions
                    .iter()
                    .map(|value| enum_from_string::<Jurisdiction>(value))
                    .collect(),
                assurance_objective: enum_from_string::<AssuranceObjective>(&objective),
                ai_use_case: enum_from_string::<AiUseCase>(&use_case),
                personal_data_profile: enum_from_string::<PersonalDataProfile>(&personal),
                involves_vulnerable_groups: vulnerable_groups,
                is_public_facing: public_facing,
                frameworks: selected_framework_values
                    .iter()
                    .map(|value| enum_from_string::<Framework>(value))
                    .collect(),
            };
            match invoke::call_named::<_, Engagement>("create_engagement", "dto", &dto).await {
                Ok(created) => {
                    set_name.set(String::new());
                    set_client.set(String::new());
                    set_desc.set(String::new());
                    set_primary_role.set("deployer".to_string());
                    set_industry_sector.set("finance".to_string());
                    set_assurance_objective.set("baseline_compliance_review".to_string());
                    set_ai_use_case.set("credit_scoring".to_string());
                    set_personal_data_profile.set("personal_data".to_string());
                    set_jurisdictions.set(vec!["eu".to_string()]);
                    set_involves_vulnerable_groups.set(false);
                    set_is_public_facing.set(false);
                    set_show_form.set(false);
                    set_refresh.update(|r| *r += 1);
                    nav(
                        &format!("/engagements/{}?prompt=ai-system", created.id),
                        Default::default(),
                    );
                }
                Err(error) => {
                    set_form_message.set(Some(format!("Could not create engagement: {}", error)));
                }
            }
        });
    });

    view! {
        <div class="page engagements-page">
            <div class="page-header">
                <h1>"Engagements"</h1>
                <button on:click=move |_| set_show_form.update(|v| *v = !*v)>
                    {move || if show_form.get() { "Cancel" } else { "+ New Engagement" }}
                </button>
            </div>

            <HelpPanel title="Engagements Help">
                <HelpSection heading="What is an Engagement?">
                    <p>"An Engagement is a scoped GRC review or audit project for a specific client. It captures the organizational context, regulatory scope, and risk profile that drive framework selection and assessment."</p>
                </HelpSection>
                <HelpSection heading="Creating an Engagement">
                    <p>"Click '+ New Engagement' to open the intake form. Fill in the client name and scoping fields (role, industry, jurisdiction, use case). The system will auto-suggest applicable frameworks based on your selections. Review and adjust the suggestions, then click Create."</p>
                </HelpSection>
                <HelpSection heading="Scoping Fields">
                    <p>"Primary Role determines your client's obligations (Provider builds AI, Deployer operates it). Industry and Jurisdiction affect which frameworks and requirements apply. AI Use Case and Personal Data Profile trigger specific regulatory provisions."</p>
                </HelpSection>
            </HelpPanel>
            <Show when=move || show_form.get()>
                <form on:submit=move |ev| on_create.call(ev) class="create-form">
                    <div class="intake-grid">
                        <label>"Name" <input type="text" required prop:value=name on:input=move |e| set_name.set(event_target_value(&e)) /></label>
                        <label>"Client" <input type="text" required prop:value=client on:input=move |e| set_client.set(event_target_value(&e)) /></label>
                    </div>
                    <label>"Description" <textarea prop:value=desc on:input=move |e| set_desc.set(event_target_value(&e))></textarea></label>

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
                        <p class="audit-note">"These frameworks are preselected from the intake answers. You can adjust them before creating the engagement."</p>
                        <div class="check-grid framework-check-grid">
                            {Framework::all().iter().cloned().map(|framework| {
                                let framework_value = serde_json::to_value(&framework)
                                    .ok()
                                    .and_then(|value| value.as_str().map(String::from))
                                    .unwrap_or_default();
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
                    <button type="submit">"Create"</button>
                </form>
            </Show>

            {move || form_message.get().map(|message| view! {
                <p class="audit-note">{message}</p>
            })}

            <Suspense fallback=move || view! { <p>"Loading…"</p> }>
                {move || engagements.get().map(|result| match result {
                    Ok(list) => view! {
                        <table role="grid">
                            <thead><tr>
                                <th>"Name"</th><th>"Client"</th><th>"Status"</th><th>"Frameworks"</th><th>"Actions"</th>
                            </tr></thead>
                            <tbody>
                                {list.into_iter().map(|e| {
                                    let id = e.id.to_string();
                                    let del_id = id.clone();
                                    let status_str = serde_json::to_value(&e.status)
                                        .ok()
                                        .and_then(|v| v.as_str().map(String::from))
                                        .unwrap_or_default();
                                    view! {
                                        <tr class="clickable-row">
                                            <td><A href=format!("/engagements/{}", id)>{&e.name}</A></td>
                                            <td>{&e.client_name}</td>
                                            <td><StatusBadge status=status_str /></td>
                                            <td>
                                                {e.frameworks.iter().map(|f| {
                                                    let fw = serde_json::to_value(f)
                                                        .ok()
                                                        .and_then(|v| v.as_str().map(String::from))
                                                        .unwrap_or_default();
                                                    view! { <FrameworkPill framework=fw /> }
                                                }).collect_view()}
                                            </td>
                                            <td>
                                                <button class="secondary outline" style="padding:0.25rem 0.5rem;font-size:0.8rem;" on:click=move |_| {
                                                    let confirm = web_sys::window()
                                                        .and_then(|w| w.confirm_with_message("Delete this engagement and all its data? This cannot be undone.").ok())
                                                        .unwrap_or(false);
                                                    if confirm {
                                                        let did = del_id.clone();
                                                        spawn_local(async move {
                                                            #[derive(Serialize)]
                                                            struct DelArg { id: String }
                                                            match invoke::call::<_, ()>("delete_engagement", &DelArg { id: did }).await {
                                                                Ok(_) => set_refresh.update(|r| *r += 1),
                                                                Err(e) => { web_sys::window().map(|w| w.alert_with_message(&format!("Delete failed: {e}"))); }
                                                            }
                                                        });
                                                    }
                                                }>"Delete"</button>
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
        </div>
    }
}
