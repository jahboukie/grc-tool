use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let nav_items = vec![
        ("/", "Dashboard", "command-center"),
        ("/engagements", "Engagements", "engagements"),
        ("/fria", "FRIA", "fria"),
        ("/frameworks", "Frameworks", "frameworks"),
        ("/cross-references", "Cross-References", "cross-refs"),
        ("/risk-matrix", "Risk Matrix", "risk-matrix"),
        ("/evidence", "Evidence Vault", "evidence"),
        ("/gap-analysis", "Gap Analysis", "gap-analysis"),
        ("/reports", "Reports", "reports"),
        ("/audit-trail", "Audit Trail", "audit-trail"),
        ("/assistant", "AI Assistant", "assistant"),
        ("/settings", "Settings", "settings"),
        ("/guide", "User Guide", "guide"),
    ];

    view! {
        <nav class="sidebar" aria-label="Main navigation">
            <div class="sidebar-header">
                <h2>"GRC Command Center"</h2>
            </div>
            <ul class="sidebar-nav">
                {nav_items.into_iter().map(|(href, label, class)| {
                    view! {
                        <li class=format!("nav-item nav-{}", class)>
                            <A href=href>{label}</A>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </nav>
    }
}
