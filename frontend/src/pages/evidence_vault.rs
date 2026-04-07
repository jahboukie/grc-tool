use leptos::*;
use grc_shared::models::{Engagement, Evidence};
use serde::Serialize;

use crate::api::invoke;
use crate::components::evidence_card::EvidenceCard;

const EVIDENCE_TYPES: &[(&str, &str)] = &[
    ("policy_document", "Policy Document"),
    ("technical_report", "Technical Report"),
    ("assessment_record", "Assessment Record"),
    ("screenshot", "Screenshot"),
    ("attestation", "Attestation"),
    ("meeting_minutes", "Meeting Minutes"),
    ("training_record", "Training Record"),
    ("audit_report", "Audit Report"),
    ("other", "Other"),
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EngagementFilter {
    status_filter: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceFilter {
    engagement_id: String,
    type_filter: Option<String>,
}

#[derive(Serialize)]
struct DeleteEvidenceArg {
    id: String,
}

#[component]
pub fn EvidenceVaultPage() -> impl IntoView {
    let (refresh, set_refresh) = create_signal(0u32);
    let (selected_engagement, set_selected_engagement) = create_signal(String::new());
    let (type_filter, set_type_filter) = create_signal::<Option<String>>(None);

    let engagements = create_resource(
        move || refresh.get(),
        |_| async {
            invoke::call::<_, Vec<Engagement>>("list_engagements", &EngagementFilter { status_filter: None }).await
        },
    );

    let evidence_items = create_resource(
        move || (refresh.get(), selected_engagement.get(), type_filter.get()),
        |(_, engagement_id, type_filter)| async move {
            if engagement_id.is_empty() {
                Ok(Vec::<Evidence>::new())
            } else {
                invoke::call::<_, Vec<Evidence>>(
                    "list_evidence",
                    &EvidenceFilter {
                        engagement_id,
                        type_filter,
                    },
                )
                .await
            }
        },
    );

    create_effect(move |_| {
        if selected_engagement.with(|id| id.is_empty()) {
            if let Some(Ok(list)) = engagements.get() {
                if let Some(first) = list.first() {
                    set_selected_engagement.set(first.id.to_string());
                }
            }
        }
    });

    let delete_evidence = move |id: String| {
        spawn_local(async move {
            let _ = invoke::call::<_, ()>("delete_evidence", &DeleteEvidenceArg { id }).await;
            set_refresh.update(|value| *value += 1);
        });
    };

    view! {
        <div class="page evidence-vault-page">
            <div class="page-header">
                <h1>"Evidence Vault"</h1>
            </div>

            <Suspense fallback=move || view! { <p>"Loading engagements…"</p> }>
                {move || engagements.get().map(|result| match result {
                    Ok(list) if list.is_empty() => view! {
                        <p class="placeholder-text">"No engagements exist yet. Create an engagement first so evidence can be scoped and tracked properly."</p>
                    }.into_view(),
                    Ok(list) => view! {
                        <div class="evidence-toolbar">
                            <label>
                                <span>"Engagement"</span>
                                <select
                                    prop:value=selected_engagement
                                    on:change=move |e| set_selected_engagement.set(event_target_value(&e))
                                >
                                    {list.into_iter().map(|engagement| view! {
                                        <option value=engagement.id.to_string()>
                                            {format!("{} ({})", engagement.name, engagement.client_name)}
                                        </option>
                                    }).collect_view()}
                                </select>
                            </label>

                            <label>
                                <span>"Evidence Type"</span>
                                <select on:change=move |e| {
                                    let val = event_target_value(&e);
                                    set_type_filter.set(if val.is_empty() { None } else { Some(val) });
                                }>
                                    <option value="">"All Types"</option>
                                    {EVIDENCE_TYPES.iter().copied().map(|(val, label)| view! {
                                        <option value=val>{label}</option>
                                    }).collect_view()}
                                </select>
                            </label>
                        </div>

                        <Suspense fallback=move || view! { <p>"Loading evidence…"</p> }>
                            {move || evidence_items.get().map(|items| match items {
                                Ok(items) if items.is_empty() => view! {
                                    <p class="placeholder-text">"No evidence items match the current selection yet."</p>
                                }.into_view(),
                                Ok(items) => {
                                    let count = items.len();
                                    view! {
                                        <p class="audit-note">{format!("Showing {} evidence item(s) for the selected engagement.", count)}</p>
                                        <div class="cards-grid evidence-grid">
                                            {items.into_iter().map(|evidence| {
                                                let delete_id = evidence.id.to_string();
                                                view! {
                                                    <div class="evidence-item">
                                                        <EvidenceCard evidence=evidence />
                                                        <div class="evidence-card-actions">
                                                            <button on:click=move |_| delete_evidence(delete_id.clone())>
                                                                "Delete"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_view()
                                }
                                Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                            })}
                        </Suspense>
                    }.into_view(),
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
