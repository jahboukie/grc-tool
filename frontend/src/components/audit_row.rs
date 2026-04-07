use leptos::*;
use grc_shared::models::AuditLog;

#[component]
pub fn AuditRow(#[prop()] entry: AuditLog) -> impl IntoView {
    let action_str = serde_json::to_value(&entry.action)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    let ts = entry.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
    let entity_type = entry.entity_type.clone();
    let field = entry.field_changed.clone().unwrap_or_else(|| "-".to_string());
    let details_title = entry.details.clone();
    let details_text = if entry.details.len() > 80 {
        format!("{}…", &entry.details[..80])
    } else {
        entry.details.clone()
    };

    view! {
        <tr class="audit-row">
            <td class="audit-time">{ts}</td>
            <td class="audit-entity">{entity_type}</td>
            <td class="audit-action">{action_str}</td>
            <td class="audit-field">{field}</td>
            <td class="audit-details" title=details_title>{details_text}</td>
        </tr>
    }
}
