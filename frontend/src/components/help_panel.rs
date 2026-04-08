use leptos::*;

/// A collapsible contextual help panel that can be placed on any page.
/// Shows a "?" button that expands to reveal help content.
#[component]
pub fn HelpPanel(
    /// The title shown at the top of the expanded help panel.
    title: &'static str,
    /// The help content as a list of (heading, body) pairs.
    children: Children,
) -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <div class="help-panel" class:help-panel-open=open>
            <button
                class="help-toggle"
                on:click=move |_| set_open.update(|v| *v = !*v)
                title="Toggle help"
            >
                {move || if open.get() { "✕" } else { "?" }}
            </button>
            <div class="help-content" style:display=move || if open.get() { "block" } else { "none" }>
                <h3 class="help-title">{title}</h3>
                {children()}
            </div>
        </div>
    }
}

/// A single section within a help panel.
#[component]
pub fn HelpSection(
    heading: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="help-section">
            <h4 class="help-section-heading">{heading}</h4>
            <div class="help-section-body">{children()}</div>
        </div>
    }
}
