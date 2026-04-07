use leptos::*;
use grc_shared::models::{AppConfig, UpdateConfigDto};

use crate::api::invoke;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let config = create_resource(|| (), |_| async {
        invoke::call_no_args::<AppConfig>("get_config").await
    });

    let (provider, set_provider) = create_signal(String::new());
    let (model, set_model) = create_signal(String::new());
    let (api_key, set_api_key) = create_signal(String::new());
    let (evidence_path, set_evidence_path) = create_signal(String::new());
    let (save_msg, set_save_msg) = create_signal::<Option<String>>(None);

    let on_save = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let p = provider.get();
        let m = model.get();
        let k = api_key.get();
        let ep = evidence_path.get();
        spawn_local(async move {
            let dto = UpdateConfigDto {
                llm_provider: if p.is_empty() { None } else { Some(p) },
                llm_model: if m.is_empty() { None } else { Some(m) },
                llm_api_key: if k.is_empty() { None } else { Some(k) },
                evidence_storage_path: if ep.is_empty() { None } else { Some(ep) },
            };
            match invoke::call_named::<_, AppConfig>("update_config", "dto", &dto).await {
                Ok(_) => set_save_msg.set(Some("Settings saved.".into())),
                Err(e) => set_save_msg.set(Some(format!("Error: {}", e))),
            }
        });
    };

    view! {
        <div class="page settings-page">
            <h1>"Settings"</h1>
            <Suspense fallback=move || view! { <p>"Loading settings…"</p> }>
                {move || config.get().map(|result| match result {
                    Ok(cfg) => {
                        // Initialize signals with current config
                        set_provider.set(cfg.llm_provider.clone());
                        set_model.set(cfg.llm_model.clone());
                        set_evidence_path.set(cfg.evidence_storage_path.clone());
                        view! {
                            <form on:submit=on_save>
                                <fieldset>
                                    <legend>"LLM Configuration"</legend>
                                    <label>"Provider"
                                        <select
                                            prop:value=provider
                                            on:change=move |e| set_provider.set(event_target_value(&e))
                                        >
                                            <option value="">"None"</option>
                                            <option value="openai">"OpenAI"</option>
                                            <option value="anthropic">"Anthropic"</option>
                                            <option value="ollama">"Ollama (local)"</option>
                                        </select>
                                    </label>
                                    <label>"Model"
                                        <input type="text" placeholder="e.g. gpt-4o, claude-sonnet-4-20250514, llama3"
                                            prop:value=model
                                            on:input=move |e| set_model.set(event_target_value(&e))
                                        />
                                    </label>
                                    <label>"API Key"
                                        <input type="password" placeholder="Leave blank to keep existing"
                                            prop:value=api_key
                                            on:input=move |e| set_api_key.set(event_target_value(&e))
                                        />
                                    </label>
                                </fieldset>
                                <fieldset>
                                    <legend>"Storage"</legend>
                                    <label>"Evidence Storage Path"
                                        <input type="text"
                                            prop:value=evidence_path
                                            on:input=move |e| set_evidence_path.set(event_target_value(&e))
                                        />
                                    </label>
                                    <label>"Database"
                                        <input type="text" readonly value=cfg.db_name />
                                    </label>
                                </fieldset>
                                <button type="submit">"Save Settings"</button>
                            </form>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
            {move || save_msg.get().map(|m| view! { <p class="save-message">{m}</p> })}
        </div>
    }
}
