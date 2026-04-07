use leptos::*;
use grc_shared::models::{
    Engagement, Evidence, EvidenceLink, LinkEvidenceDto, RiskEntry, UploadEvidenceDto,
};
use grc_shared::enums::EvidenceType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    ("risk_register", "Risk Register"),
    ("conformity_declaration", "Conformity Declaration"),
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

#[derive(Debug, Clone, Deserialize)]
struct PickedFileInfo {
    file_name: String,
    file_path: String,
    file_size_bytes: i64,
    mime_type: String,
}

#[derive(Serialize)]
struct EvidenceIdArg {
    evidence_id: String,
}

#[derive(Serialize)]
struct LinkIdArg {
    link_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RiskListArg {
    ai_system_id: Option<String>,
}

fn evidence_type_from_str(s: &str) -> EvidenceType {
    match s {
        "policy_document" => EvidenceType::PolicyDocument,
        "technical_report" => EvidenceType::TechnicalReport,
        "assessment_record" => EvidenceType::AssessmentRecord,
        "screenshot" => EvidenceType::Screenshot,
        "attestation" => EvidenceType::Attestation,
        "meeting_minutes" => EvidenceType::MeetingMinutes,
        "training_record" => EvidenceType::TrainingRecord,
        "audit_report" => EvidenceType::AuditReport,
        "risk_register" => EvidenceType::RiskRegister,
        "conformity_declaration" => EvidenceType::ConformityDeclaration,
        _ => EvidenceType::Other,
    }
}

#[component]
pub fn EvidenceVaultPage() -> impl IntoView {
    let (refresh, set_refresh) = create_signal(0u32);
    let (selected_engagement, set_selected_engagement) = create_signal(String::new());
    let (type_filter, set_type_filter) = create_signal::<Option<String>>(None);

    // Upload form state
    let (show_upload, set_show_upload) = create_signal(false);
    let (picked_file, set_picked_file) = create_signal(Option::<PickedFileInfo>::None);
    let (upload_desc, set_upload_desc) = create_signal(String::new());
    let (upload_type, set_upload_type) = create_signal("other".to_string());
    let (upload_tags, set_upload_tags) = create_signal(String::new());
    let (upload_msg, set_upload_msg) = create_signal(Option::<String>::None);
    let (search_query, set_search_query) = create_signal(String::new());

    // Linking state
    let (linking_evidence_id, set_linking_evidence_id) = create_signal(Option::<String>::None);
    let (link_type, set_link_type) = create_signal("risk".to_string());
    let (link_target_id, set_link_target_id) = create_signal(String::new());
    let (link_msg, set_link_msg) = create_signal(Option::<String>::None);

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

    // Fetch risks for linking
    let risk_entries = create_resource(
        move || refresh.get(),
        |_| async {
            invoke::call::<_, Vec<RiskEntry>>(
                "list_risk_entries",
                &RiskListArg { ai_system_id: None },
            ).await.unwrap_or_default()
        },
    );

    let on_link_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let Some(eid) = linking_evidence_id.get() else { return; };
        let target = link_target_id.get();
        if target.is_empty() {
            set_link_msg.set(Some("Select a target entity.".into()));
            return;
        }
        let Ok(evidence_uuid) = Uuid::parse_str(&eid) else { return; };
        let Ok(target_uuid) = Uuid::parse_str(&target) else { return; };
        let lt = link_type.get();

        let dto = LinkEvidenceDto {
            evidence_id: evidence_uuid,
            requirement_assessment_id: if lt == "assessment" { Some(target_uuid) } else { None },
            risk_entry_id: if lt == "risk" { Some(target_uuid) } else { None },
            task_id: if lt == "task" { Some(target_uuid) } else { None },
        };

        spawn_local(async move {
            match invoke::call_named::<_, EvidenceLink>("link_evidence", "dto", &dto).await {
                Ok(_) => {
                    set_linking_evidence_id.set(None);
                    set_link_msg.set(None);
                    set_refresh.update(|v| *v += 1);
                }
                Err(e) => set_link_msg.set(Some(format!("Link error: {}", e))),
            }
        });
    };

    let on_unlink = move |lid: String| {
        spawn_local(async move {
            let _ = invoke::call::<_, ()>("unlink_evidence", &LinkIdArg { link_id: lid }).await;
            set_refresh.update(|v| *v += 1);
        });
    };

    let reset_upload = move || {
        set_picked_file.set(None);
        set_upload_desc.set(String::new());
        set_upload_type.set("other".into());
        set_upload_tags.set(String::new());
        set_upload_msg.set(None);
    };

    let on_pick_file = move |_| {
        spawn_local(async move {
            match invoke::call_no_args::<Option<PickedFileInfo>>("pick_evidence_file").await {
                Ok(Some(info)) => set_picked_file.set(Some(info)),
                Ok(None) => {} // user cancelled
                Err(e) => set_upload_msg.set(Some(format!("File pick error: {}", e))),
            }
        });
    };

    let on_upload_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let engagement_id_str = selected_engagement.get();
        let file = picked_file.get();
        let desc = upload_desc.get();
        let ev_type = upload_type.get();
        let tags_str = upload_tags.get();

        let Some(file_info) = file else {
            set_upload_msg.set(Some("Please choose a file first.".into()));
            return;
        };

        let Ok(engagement_id) = Uuid::parse_str(&engagement_id_str) else {
            set_upload_msg.set(Some("Select an engagement first.".into()));
            return;
        };

        let tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

        let dto = UploadEvidenceDto {
            engagement_id,
            file_name: file_info.file_name,
            file_path: file_info.file_path,
            file_size_bytes: file_info.file_size_bytes,
            mime_type: file_info.mime_type,
            evidence_type: evidence_type_from_str(&ev_type),
            description: desc,
            tags,
        };

        spawn_local(async move {
            match invoke::call_named::<_, Evidence>("upload_evidence", "dto", &dto).await {
                Ok(_) => {
                    set_show_upload.set(false);
                    reset_upload();
                    set_refresh.update(|v| *v += 1);
                }
                Err(e) => set_upload_msg.set(Some(format!("Upload error: {}", e))),
            }
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
                        <div class="evidence-toolbar" style="display:flex;gap:1rem;align-items:end;margin-bottom:1rem;">
                            <label style="flex:1">
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

                            <label style="flex:1">
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

                            <button on:click=move |_| { reset_upload(); set_show_upload.set(true); }>"+ Upload Evidence"</button>
                        </div>
                        <div style="margin-bottom:1rem;">
                            <input type="search" placeholder="Search by file name, description, or tag…" prop:value=search_query on:input=move |e| set_search_query.set(event_target_value(&e)) />
                        </div>

                        // Upload form
                        <Show when=move || show_upload.get()>
                            <article class="upload-form">
                                <header>
                                    <strong>"Upload Evidence"</strong>
                                    <button class="outline" style="margin-left:auto" on:click=move |_| { set_show_upload.set(false); reset_upload(); }>"✕"</button>
                                </header>
                                <form on:submit=on_upload_submit>
                                    <div style="display:flex;gap:1rem;align-items:end;margin-bottom:1rem;">
                                        <button type="button" class="outline" on:click=on_pick_file>"Choose File…"</button>
                                        {move || picked_file.get().map(|f| view! {
                                            <span><strong>{&f.file_name}</strong>{format!(" ({} KB, {})", f.file_size_bytes / 1024, f.mime_type)}</span>
                                        })}
                                    </div>
                                    <div class="grid">
                                        <label>"Evidence Type"
                                            <select prop:value=upload_type on:change=move |e| set_upload_type.set(event_target_value(&e))>
                                                {EVIDENCE_TYPES.iter().copied().map(|(val, label)| view! {
                                                    <option value=val>{label}</option>
                                                }).collect_view()}
                                            </select>
                                        </label>
                                        <label>"Tags (comma-separated)"
                                            <input type="text" placeholder="e.g. audit, 2024, EU-AI-Act" prop:value=upload_tags on:input=move |e| set_upload_tags.set(event_target_value(&e)) />
                                        </label>
                                    </div>
                                    <label>"Description"
                                        <textarea rows=2 prop:value=upload_desc on:input=move |e| set_upload_desc.set(event_target_value(&e))></textarea>
                                    </label>
                                    {move || upload_msg.get().map(|m| view! { <p class="error">{m}</p> })}
                                    <button type="submit">"Upload"</button>
                                </form>
                            </article>
                        </Show>

                        <Suspense fallback=move || view! { <p>"Loading evidence…"</p> }>
                            {move || evidence_items.get().map(|items| match items {
                                Ok(items) if items.is_empty() => view! {
                                    <p class="placeholder-text">"No evidence items match the current selection yet."</p>
                                }.into_view(),
                                Ok(items) => {
                                    let q = search_query.get().to_lowercase();
                                    let filtered: Vec<_> = items.into_iter().filter(|ev| {
                                        if q.is_empty() { return true; }
                                        ev.file_name.to_lowercase().contains(&q)
                                            || ev.description.to_lowercase().contains(&q)
                                            || ev.tags.iter().any(|t| t.to_lowercase().contains(&q))
                                    }).collect();
                                    if filtered.is_empty() {
                                        return view! { <p class="placeholder-text">"No evidence matches the search."</p> }.into_view();
                                    }
                                    let count = filtered.len();
                                    view! {
                                        <p class="audit-note">{format!("Showing {} evidence item(s) for the selected engagement.", count)}</p>
                                        <div class="cards-grid evidence-grid">
                                            {filtered.into_iter().map(|evidence| {
                                                let delete_id = evidence.id.to_string();
                                                let link_id = evidence.id.to_string();
                                                let ev_id_for_links = evidence.id.to_string();
                                                view! {
                                                    <div class="evidence-item">
                                                        <EvidenceCard evidence=evidence />
                                                        <div class="evidence-card-actions" style="display:flex;gap:0.3rem;margin-top:0.3rem;">
                                                            <button class="outline" style="padding:0.2rem 0.5rem;font-size:0.8rem" on:click={
                                                                let lid = link_id.clone();
                                                                move |_| {
                                                                    set_linking_evidence_id.set(Some(lid.clone()));
                                                                    set_link_target_id.set(String::new());
                                                                    set_link_msg.set(None);
                                                                }
                                                            }>"Link"</button>
                                                            <button class="outline secondary" style="padding:0.2rem 0.5rem;font-size:0.8rem" on:click={
                                                                let did = delete_id.clone();
                                                                move |_| delete_evidence(did.clone())
                                                            }>"Delete"</button>
                                                        </div>
                                                        <EvidenceLinkList evidence_id=ev_id_for_links refresh=refresh on_unlink=on_unlink />
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

            // Link evidence dialog
            <Show when=move || linking_evidence_id.get().is_some()>
                <article style="margin-top:1rem;">
                    <header>
                        <strong>"Link Evidence"</strong>
                        <button class="outline" style="margin-left:auto" on:click=move |_| set_linking_evidence_id.set(None)>"✕"</button>
                    </header>
                    <form on:submit=on_link_submit>
                        <div class="grid">
                            <label>"Link To"
                                <select prop:value=link_type on:change=move |e| set_link_type.set(event_target_value(&e))>
                                    <option value="risk">"Risk Entry"</option>
                                </select>
                            </label>
                            <label>"Target"
                                <Suspense fallback=move || view! { <select disabled><option>"Loading…"</option></select> }>
                                    {move || risk_entries.get().map(|risks| view! {
                                        <select prop:value=link_target_id on:change=move |e| set_link_target_id.set(event_target_value(&e))>
                                            <option value="">"Select…"</option>
                                            {risks.into_iter().map(|r| {
                                                let id = r.id.to_string();
                                                view! { <option value=id>{r.title}</option> }
                                            }).collect_view()}
                                        </select>
                                    })}
                                </Suspense>
                            </label>
                        </div>
                        {move || link_msg.get().map(|m| view! { <p class="error">{m}</p> })}
                        <button type="submit">"Create Link"</button>
                    </form>
                </article>
            </Show>
        </div>
    }
}

// ── Inline subcomponent: evidence link list ────────────────

#[component]
fn EvidenceLinkList(
    evidence_id: String,
    refresh: ReadSignal<u32>,
    on_unlink: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    let eid = evidence_id.clone();
    let links = create_resource(
        move || (refresh.get(), eid.clone()),
        |(_, eid)| async move {
            invoke::call::<_, Vec<EvidenceLink>>(
                "list_evidence_links",
                &EvidenceIdArg { evidence_id: eid },
            ).await.unwrap_or_default()
        },
    );

    view! {
        <Suspense fallback=|| ()>
            {move || links.get().map(|link_list| {
                if link_list.is_empty() {
                    view! {}.into_view()
                } else {
                    view! {
                        <div class="evidence-links" style="font-size:0.8rem;margin-top:0.3rem;">
                            <strong>"Links: "</strong>
                            {link_list.into_iter().map(|l| {
                                let lid = l.id.to_string();
                                let label = if l.risk_entry_id.is_some() {
                                    format!("Risk {}", l.risk_entry_id.unwrap().to_string().split('-').next().unwrap_or(""))
                                } else if l.requirement_assessment_id.is_some() {
                                    "Assessment".to_string()
                                } else if l.task_id.is_some() {
                                    "Task".to_string()
                                } else {
                                    "Unknown".to_string()
                                };
                                view! {
                                    <span class="tag" style="margin-right:0.3rem;">
                                        {label}
                                        <button class="outline" style="padding:0 0.2rem;font-size:0.7rem;margin-left:0.2rem" on:click={
                                            let lid = lid.clone();
                                            move |_| on_unlink(lid.clone())
                                        }>"×"</button>
                                    </span>
                                }
                            }).collect_view()}
                        </div>
                    }.into_view()
                }
            })}
        </Suspense>
    }
}
