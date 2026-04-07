use leptos::*;
use grc_shared::models::Evidence;

#[component]
pub fn EvidenceCard(#[prop()] evidence: Evidence) -> impl IntoView {
    let type_str = serde_json::to_value(&evidence.evidence_type)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    let size_kb = evidence.file_size_bytes / 1024;
    let file_path = evidence.file_path.clone();
    let mime_type = evidence.mime_type.clone();

    view! {
        <article class="evidence-card">
            <header>
                <span class="evidence-name">{&evidence.file_name}</span>
                <span class="evidence-type">{&type_str}</span>
            </header>
            <p class="evidence-desc">{&evidence.description}</p>
            <p class="evidence-meta-line">{mime_type}</p>
            <p class="evidence-path">{file_path}</p>
            <footer>
                <span class="evidence-size">{format!("{} KB", size_kb)}</span>
                <span class="evidence-date">{evidence.uploaded_at.format("%Y-%m-%d").to_string()}</span>
                {(!evidence.tags.is_empty()).then(|| view! {
                    <span class="evidence-tags">
                        {evidence.tags.iter().map(|t| view! {
                            <span class="tag">{t.clone()}</span>
                        }).collect_view()}
                    </span>
                })}
            </footer>
        </article>
    }
}
