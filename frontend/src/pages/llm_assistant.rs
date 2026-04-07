use leptos::*;
use grc_shared::models::{LlmQueryDto, LlmConversation};

use crate::api::invoke;
use crate::components::chat_bubble::ChatBubble;

#[component]
pub fn LlmAssistantPage() -> impl IntoView {
    let (messages, set_messages) = create_signal::<Vec<(String, bool)>>(vec![]);
    let (input, set_input) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);

    let send_message = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let query = input.get();
        if query.trim().is_empty() { return; }
        set_input.set(String::new());

        set_messages.update(|msgs| msgs.push((query.clone(), true)));
        set_loading.set(true);

        spawn_local(async move {
            let req = LlmQueryDto {
                query,
                engagement_id: None,
                ai_system_id: None,
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
