use leptos::*;
use grc_shared::models::{AppConfig, UpdateConfigDto};

use crate::api::invoke;
use crate::components::help_panel::{HelpPanel, HelpSection};

const LOCAL_PROVIDERS: &[(&str, &str)] = &[
    ("ollama", "Ollama"),
    ("lm_studio", "LM Studio"),
];

const CLOUD_PROVIDERS: &[(&str, &str)] = &[
    ("openai", "OpenAI"),
    ("anthropic", "Anthropic"),
    ("gemini", "Google Gemini"),
];

fn default_model(provider: &str) -> &'static str {
    match provider {
        "openai" => "gpt-4o",
        "anthropic" => "claude-sonnet-4-20250514",
        "ollama" => "llama3",
        "lm_studio" => "gemma-4-e2b-it",
        "gemini" => "gemini-2.5-flash",
        _ => "",
    }
}

fn needs_api_key(provider: &str) -> bool {
    matches!(provider, "openai" | "anthropic" | "gemini")
}

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

    let on_provider_change = move |e: web_sys::Event| {
        let val = event_target_value(&e);
        set_provider.set(val.clone());
        set_model.set(default_model(&val).to_string());
    };

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
            <HelpPanel title="Settings Help">
                <HelpSection heading="LLM Provider">
                    <p>"Choose between local providers (Ollama, LM Studio) for privacy-sensitive work, or cloud providers (OpenAI, Anthropic, Gemini) for more powerful models. Cloud providers require an API key."</p>
                </HelpSection>
                <HelpSection heading="Recommended Setup">
                    <p>"For sensitive compliance work, use Ollama (local, free, private). Install from ollama.ai, run 'ollama pull llama3.1', then select Ollama provider and llama3.1 model here."</p>
                </HelpSection>
                <HelpSection heading="Evidence Path">
                    <p>"The evidence storage path sets where uploaded evidence files are stored on your local filesystem. Choose a location you back up regularly."</p>
                </HelpSection>
            </HelpPanel>
            <Suspense fallback=move || view! { <p>"Loading settings…"</p> }>
                {move || config.get().map(|result| match result {
                    Ok(cfg) => {
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
                                            on:change=on_provider_change
                                        >
                                            <option value="">"None"</option>
                                            <optgroup label="Local (no API key)">
                                                {LOCAL_PROVIDERS.iter().map(|(val, label)| view! {
                                                    <option value=*val>{*label}</option>
                                                }).collect_view()}
                                            </optgroup>
                                            <optgroup label="Cloud (requires API key)">
                                                {CLOUD_PROVIDERS.iter().map(|(val, label)| view! {
                                                    <option value=*val>{*label}</option>
                                                }).collect_view()}
                                            </optgroup>
                                        </select>
                                    </label>
                                    <label>"Model"
                                        <input type="text"
                                            placeholder=move || format!("e.g. {}", default_model(&provider.get()))
                                            prop:value=model
                                            on:input=move |e| set_model.set(event_target_value(&e))
                                        />
                                    </label>
                                    <Show when=move || needs_api_key(&provider.get())>
                                        <label>"API Key"
                                            <input type="password" placeholder="Leave blank to keep existing"
                                                prop:value=api_key
                                                on:input=move |e| set_api_key.set(event_target_value(&e))
                                            />
                                        </label>
                                    </Show>
                                    <Show when=move || !needs_api_key(&provider.get()) && !provider.get().is_empty()>
                                        <p style="font-size:0.85rem;opacity:0.7;">"Local provider — no API key needed."</p>
                                    </Show>
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
                                        <input type="text" readonly value=format!(
                                            "{} @ {}:{}",
                                            cfg.db_name, cfg.db_host, cfg.db_port
                                        ) />
                                    </label>
                                </fieldset>
                                <button type="submit">"Save Settings"</button>
                            </form>
                            <fieldset style="margin-top:2rem;">
                                <legend>"About"</legend>
                                <dl>
                                    <dt>"Product"</dt><dd>"GRC Command Center"</dd>
                                    <dt>"Version"</dt><dd>"0.1.0"</dd>
                                    <dt>"Build Date"</dt><dd>"2026-04-07"</dd>
                                    <dt>"Stack"</dt><dd>"Tauri 2 · Leptos · PostgreSQL"</dd>
                                </dl>
                            </fieldset>
                        }.into_view()
                    }
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
            {move || save_msg.get().map(|m| view! { <p class="save-message">{m}</p> })}
        </div>
    }
}
