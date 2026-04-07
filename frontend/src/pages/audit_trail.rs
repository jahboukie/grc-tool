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
    let (page, set_page) = create_signal(0i64);

    let entries = create_resource(
        move || (entity_filter.get(), page.get()),
        |(entity_type, p)| async move {
            invoke::call_named::<_, Vec<AuditLog>>(
                "list_audit_log",
                "filter",
                &AuditFilter {
                    entity_type,
                    action: None,
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

    view! {
        <div class="page audit-trail-page">
            <h1>"Audit Trail"</h1>
            <p class="audit-note">"Immutable log of all system changes."</p>
            <div class="filter-bar">
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
            </div>
            <Suspense fallback=move || view! { <p>"Loading audit log…"</p> }>
                {move || entries.get().map(|result| match result {
                    Ok(logs) => view! {
                        <table role="grid">
                            <thead><tr>
                                <th>"Time"</th><th>"Entity"</th><th>"Action"</th><th>"Field"</th><th>"Details"</th>
                            </tr></thead>
                            <tbody>
                                {logs.into_iter().map(|e| view! { <AuditRow entry=e /> }).collect_view()}
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
                    }.into_view(),
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
