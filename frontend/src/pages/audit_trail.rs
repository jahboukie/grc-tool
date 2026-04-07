use leptos::*;
use grc_shared::models::AuditLog;
use serde::Serialize;

use crate::api::invoke;
use crate::components::audit_row::AuditRow;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuditFilter {
    entity_type: Option<String>,
    action: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[component]
pub fn AuditTrailPage() -> impl IntoView {
    let (entity_filter, set_entity_filter) = create_signal::<Option<String>>(None);
    let (action_filter, set_action_filter) = create_signal::<Option<String>>(None);
    let (date_from, set_date_from) = create_signal(String::new());
    let (date_to, set_date_to) = create_signal(String::new());
    let (entity_id_search, set_entity_id_search) = create_signal(String::new());
    let (page, set_page) = create_signal(0i64);

    let entries = create_resource(
        move || (entity_filter.get(), action_filter.get(), page.get()),
        |(entity_type, action, p)| async move {
            invoke::call_named::<_, Vec<AuditLog>>(
                "list_audit_log",
                "filter",
                &AuditFilter {
                    entity_type,
                    action,
                    limit: Some(50),
                    offset: Some(p * 50),
                },
            ).await
        },
    );

    let entity_types = vec![
        "engagement", "ai_system", "requirement_assessment",
        "risk_entry", "evidence", "task", "app_config",
    ];

    let action_types = vec![
        "created", "updated", "deleted",
    ];

    view! {
        <div class="page audit-trail-page">
            <h1>"Audit Trail"</h1>
            <p class="audit-note">"Immutable log of all system changes."</p>
            <div class="filter-bar" style="display:flex;gap:1rem;flex-wrap:wrap;align-items:end;margin-bottom:1rem;">
                <label style="min-width:140px;">"Entity Type"
                    <select on:change=move |e| {
                        let val = event_target_value(&e);
                        set_entity_filter.set(if val.is_empty() { None } else { Some(val) });
                        set_page.set(0);
                    }>
                        <option value="">"All Entities"</option>
                        {entity_types.iter().map(|t| view! {
                            <option value=*t>{*t}</option>
                        }).collect_view()}
                    </select>
                </label>
                <label style="min-width:120px;">"Action"
                    <select on:change=move |e| {
                        let val = event_target_value(&e);
                        set_action_filter.set(if val.is_empty() { None } else { Some(val) });
                        set_page.set(0);
                    }>
                        <option value="">"All Actions"</option>
                        {action_types.iter().map(|t| view! {
                            <option value=*t>{*t}</option>
                        }).collect_view()}
                    </select>
                </label>
                <label style="min-width:140px;">"From"
                    <input type="date" prop:value=date_from on:input=move |e| set_date_from.set(event_target_value(&e)) />
                </label>
                <label style="min-width:140px;">"To"
                    <input type="date" prop:value=date_to on:input=move |e| set_date_to.set(event_target_value(&e)) />
                </label>
                <label style="min-width:200px;">"Entity ID"
                    <input type="text" placeholder="Search by UUID…" prop:value=entity_id_search on:input=move |e| set_entity_id_search.set(event_target_value(&e)) />
                </label>
            </div>
            <Suspense fallback=move || view! { <p>"Loading audit log…"</p> }>
                {move || entries.get().map(|result| match result {
                    Ok(logs) => {
                        let df = date_from.get();
                        let dt = date_to.get();
                        let id_q = entity_id_search.get().to_lowercase();
                        let filtered: Vec<_> = logs.into_iter().filter(|entry| {
                            // Client-side date filter
                            if !df.is_empty() {
                                let entry_date = entry.timestamp.format("%Y-%m-%d").to_string();
                                if entry_date < df { return false; }
                            }
                            if !dt.is_empty() {
                                let entry_date = entry.timestamp.format("%Y-%m-%d").to_string();
                                if entry_date > dt { return false; }
                            }
                            // Client-side entity ID filter
                            if !id_q.is_empty() {
                                if !entry.entity_id.to_string().to_lowercase().contains(&id_q) {
                                    return false;
                                }
                            }
                            true
                        }).collect();
                        view! {
                        <table role="grid">
                            <thead><tr>
                                <th>"Time"</th><th>"Entity"</th><th>"Action"</th><th>"Field"</th><th>"Details"</th>
                            </tr></thead>
                            <tbody>
                                {filtered.into_iter().map(|e| view! { <AuditRow entry=e /> }).collect_view()}
                            </tbody>
                        </table>
                        <div class="pagination">
                            <button
                                disabled=move || page.get() == 0
                                on:click=move |_| set_page.update(|p| *p = (*p - 1).max(0))
                            >"← Prev"</button>
                            <span>{move || format!("Page {}", page.get() + 1)}</span>
                            <button on:click=move |_| set_page.update(|p| *p += 1)>"Next →"</button>
                        </div>
                    }.into_view()},
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
