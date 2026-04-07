use leptos::*;
use grc_shared::models::Task;

use crate::components::status_badge::StatusBadge;

#[component]
pub fn TaskRow(#[prop()] task: Task) -> impl IntoView {
    let status_str = serde_json::to_value(&task.status)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    let priority_str = serde_json::to_value(&task.priority)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    let due = task.due_date.map(|d| d.to_string()).unwrap_or_else(|| "No due date".into());

    view! {
        <tr class=format!("task-row priority-{}", priority_str)>
            <td class="task-title">{&task.title}</td>
            <td><StatusBadge status=status_str /></td>
            <td class="task-priority">{priority_str}</td>
            <td class="task-due">{due}</td>
        </tr>
    }
}
