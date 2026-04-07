use leptos::*;
use grc_shared::models::FrameworkRequirement;

use crate::components::status_badge::StatusBadge;
use crate::components::framework_pill::FrameworkPill;

#[component]
pub fn RequirementRow(
    #[prop()] requirement: FrameworkRequirement,
    #[prop(optional, into)] assessment_status: Option<String>,
    #[prop(optional)] on_click: Option<Callback<uuid::Uuid>>,
) -> impl IntoView {
    let fw_str = serde_json::to_value(&requirement.framework)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    let status = assessment_status.unwrap_or_else(|| "not_assessed".into());
    let req_id = requirement.id;

    let handle_click = move |_| {
        if let Some(cb) = &on_click {
            cb.call(req_id);
        }
    };

    view! {
        <tr class="requirement-row" on:click=handle_click>
            <td class="req-ref">{&requirement.reference_id}</td>
            <td class="req-title">{&requirement.title}</td>
            <td><FrameworkPill framework=fw_str /></td>
            <td class="req-article">{&requirement.article_clause}</td>
            <td><StatusBadge status=status /></td>
            <td class="req-mandatory">
                {if requirement.is_mandatory { "Required" } else { "Recommended" }}
            </td>
        </tr>
    }
}
