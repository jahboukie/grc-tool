use leptos::*;
use grc_shared::models::{AiSystem, AppConfig, Engagement, LlmQueryDto, LlmConversation};
use serde::Serialize;

use crate::api::invoke;
use crate::components::chat_bubble::ChatBubble;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementFilter {
    status_filter: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementIdArg {
    engagement_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ListConvosArg {
    engagement_id: Option<String>,
}

#[component]
pub fn LlmAssistantPage() -> impl IntoView {
    let (messages, set_messages) = create_signal::<Vec<(String, bool)>>(vec![]);
    let (input, set_input) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);
    let (selected_engagement_id, set_selected_engagement_id) = create_signal(Option::<String>::None);
    let (selected_system_id, set_selected_system_id) = create_signal(Option::<String>::None);

    let engagements = create_resource(|| (), |_| async {
        invoke::call::<_, Vec<Engagement>>(
            "list_engagements",
            &EngagementFilter { status_filter: None },
        ).await
    });

    let config = create_resource(|| (), |_| async {
        invoke::call_no_args::<AppConfig>("get_config").await.ok()
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

    // Load conversation history for the selected engagement
    let history = create_resource(
        move || selected_engagement_id.get(),
        |eid| async move {
            invoke::call::<_, Vec<LlmConversation>>(
                "list_conversations",
                &ListConvosArg { engagement_id: eid },
            ).await.unwrap_or_default()
        },
    );

    // When history loads, populate messages
    create_effect(move |_| {
        if let Some(convos) = history.get() {
            let mut msgs = Vec::new();
            // History comes newest-first, reverse for chronological display
            for c in convos.into_iter().rev() {
                msgs.push((c.query, true));
                msgs.push((c.response, false));
            }
            set_messages.set(msgs);
        }
    });

    let send_message = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let query = input.get();
        if query.trim().is_empty() { return; }
        set_input.set(String::new());

        set_messages.update(|msgs| msgs.push((query.clone(), true)));
        set_loading.set(true);

        let eid = selected_engagement_id.get().and_then(|s| uuid::Uuid::parse_str(&s).ok());
        let sid = selected_system_id.get().and_then(|s| uuid::Uuid::parse_str(&s).ok());

        spawn_local(async move {
            let req = LlmQueryDto {
                query,
                engagement_id: eid,
                ai_system_id: sid,
            };
            match invoke::call_named::<_, LlmConversation>("query_llm", "dto", &req).await {
                Ok(convo) => {
                    set_messages.update(|msgs| msgs.push((convo.response, false)));
                }
                Err(e) => {
                    set_messages.update(|msgs| msgs.push((format!("Error: {}", e), false)));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="page llm-assistant-page">
            <h1>"AI Governance Assistant"</h1>
            <Suspense fallback=|| ()>
                {move || config.get().flatten().map(|cfg| {
                    if cfg.llm_provider.is_empty() { return view! {}.into_view(); }
                    let label = format!("{} / {}", cfg.llm_provider, if cfg.llm_model.is_empty() { "default" } else { &cfg.llm_model });
                    view! {
                        <p style="font-size:0.85rem;opacity:0.7;margin-top:-0.5rem;">"Active model: "<strong>{label}</strong></p>
                    }.into_view()
                })}
            </Suspense>

            <div class="scope-selectors" style="display:flex;gap:1rem;align-items:end;margin-bottom:1rem;">
                <label style="flex:1">"Engagement Context"
                    <Suspense fallback=move || view! { <select disabled><option>"Loading…"</option></select> }>
                        {move || engagements.get().map(|result| match result {
                            Ok(engs) => view! {
                                <select on:change=move |e| {
                                    let v = event_target_value(&e);
                                    if v.is_empty() { set_selected_engagement_id.set(None); set_selected_system_id.set(None); }
                                    else { set_selected_engagement_id.set(Some(v)); set_selected_system_id.set(None); }
                                }>
                                    <option value="">"No context"</option>
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
                                <select on:change=move |e| {
                                    let v = event_target_value(&e);
                                    if v.is_empty() { set_selected_system_id.set(None); }
                                    else { set_selected_system_id.set(Some(v)); }
                                }>
                                    <option value="">"None"</option>
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
            </div>

            <div class="chat-container">
                <div class="chat-messages">
                    {move || {
                        messages.get().into_iter().map(|(text, is_user)| {
                            view! { <ChatBubble text=text is_user=is_user /> }
                        }).collect_view()
                    }}
                    <Show when=move || loading.get()>
                        <div class="chat-loading">"Thinking…"</div>
                    </Show>
                </div>
                <form class="chat-input" on:submit=send_message>
                    <input
                        type="text"
                        placeholder="Ask about any regulation or requirement…"
                        prop:value=input
                        on:input=move |e| set_input.set(event_target_value(&e))
                        disabled=loading
                    />
                    <button type="submit" disabled=loading>"Send"</button>
                </form>
            </div>
        </div>
    }
}
