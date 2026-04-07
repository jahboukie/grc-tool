use leptos::*;

#[component]
pub fn ChatBubble(
    #[prop(into)] text: String,
    #[prop()] is_user: bool,
) -> impl IntoView {
    let class = if is_user { "chat-bubble user" } else { "chat-bubble assistant" };
    let icon = if is_user { "\u{1F9D1}" } else { "\u{1F916}" };

    view! {
        <div class=class>
            <span class="chat-icon">{icon}</span>
            <div class="chat-text">{text}</div>
        </div>
    }
}
