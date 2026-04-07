use leptos::*;

#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] sub_text: Option<String>,
) -> impl IntoView {
    view! {
        <article class="stat-card">
            <header class="stat-label">{label}</header>
            <div class="stat-value">{value}</div>
            {sub_text.map(|t| view! { <footer class="stat-sub">{t}</footer> })}
        </article>
    }
}
