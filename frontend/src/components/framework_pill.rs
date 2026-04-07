use leptos::*;

#[component]
pub fn FrameworkPill(#[prop(into)] framework: String) -> impl IntoView {
    let label = match framework.as_str() {
        "eu_ai_act" => "EU AI Act",
        "iso_42001" => "ISO 42001",
        "iso_23894" => "ISO 23894",
        "nist_ai_rmf" => "NIST AI RMF",
        "oecd_ai_principles" => "OECD AI",
        other => other,
    }.to_string();
    let css = format!("framework-pill fw-{}", framework);

    view! {
        <span class=css>{label}</span>
    }
}
