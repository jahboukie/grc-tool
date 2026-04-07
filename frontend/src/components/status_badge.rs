use leptos::*;

#[component]
pub fn StatusBadge(#[prop(into)] status: String) -> impl IntoView {
    let label = match status.as_str() {
        "met" => "Met",
        "partial" => "Partial",
        "gap" => "Gap",
        "not_assessed" => "Not Assessed",
        "not_applicable" => "N/A",
        "open" => "Open",
        "in_progress" => "In Progress",
        "blocked" => "Blocked",
        "done" => "Done",
        "deferred" => "Deferred",
        "active" => "Active",
        "paused" => "Paused",
        "completed" => "Completed",
        "archived" => "Archived",
        other => other,
    }.to_string();
    let css = format!("status-badge status-{}", status);

    view! {
        <span class=css>{label}</span>
    }
}
