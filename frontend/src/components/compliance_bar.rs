use leptos::*;

#[component]
pub fn ComplianceBar(
    #[prop(into)] label: String,
    #[prop()] met: i64,
    #[prop()] partial: i64,
    #[prop()] gap: i64,
    #[prop()] not_assessed: i64,
) -> impl IntoView {
    let total = met + partial + gap + not_assessed;
    let pct = |n: i64| {
        if total == 0 { 0.0 } else { (n as f64 / total as f64) * 100.0 }
    };
    let met_pct = pct(met);
    let partial_pct = pct(partial);
    let gap_pct = pct(gap);
    let overall_pct = if total == 0 { 0.0 } else { (met as f64 / total as f64) * 100.0 };

    view! {
        <div class="compliance-bar-container">
            <div class="compliance-bar-header">
                <span class="compliance-bar-label">{label}</span>
                <span class="compliance-bar-pct">{format!("{:.0}%", overall_pct)}</span>
            </div>
            <div class="compliance-bar" role="progressbar" aria-valuenow=overall_pct as u32 aria-valuemin=0 aria-valuemax=100>
                <div class="compliance-segment met" style=format!("width: {:.1}%", met_pct)></div>
                <div class="compliance-segment partial" style=format!("width: {:.1}%", partial_pct)></div>
                <div class="compliance-segment gap" style=format!("width: {:.1}%", gap_pct)></div>
            </div>
            <div class="compliance-bar-legend">
                <span class="legend-met">{format!("Met: {}", met)}</span>
                <span class="legend-partial">{format!("Partial: {}", partial)}</span>
                <span class="legend-gap">{format!("Gap: {}", gap)}</span>
                <span class="legend-na">{format!("N/A: {}", not_assessed)}</span>
            </div>
        </div>
    }
}
