use leptos::*;
use uuid::Uuid;

#[component]
pub fn RiskHeatmap(
    /// 5x5 matrix: [likelihood_idx][impact_idx] -> list of risk IDs
    #[prop()] matrix: [[Vec<Uuid>; 5]; 5],
) -> impl IntoView {
    let impact_labels = ["Negligible", "Minor", "Moderate", "Major", "Catastrophic"];
    let likelihood_labels = ["Rare", "Unlikely", "Possible", "Likely", "Almost Certain"];

    view! {
        <div class="risk-heatmap">
            <div class="heatmap-grid">
                // Header row
                <div class="heatmap-corner"></div>
                {impact_labels.iter().enumerate().map(|(_, label)| {
                    view! { <div class="heatmap-header">{*label}</div> }
                }).collect_view()}

                // Data rows (highest likelihood at top)
                {(0..5).rev().map(|li| {
                    view! {
                        <div class="heatmap-row-label">{likelihood_labels[li]}</div>
                        {(0..5).map(|ii| {
                            let count = matrix[li][ii].len();
                            let score = (li + 1) * (ii + 1);
                            let severity_class = match score {
                                1..=4 => "risk-low",
                                5..=9 => "risk-medium",
                                10..=14 => "risk-high",
                                15..=19 => "risk-very-high",
                                _ => "risk-critical",
                            };
                            view! {
                                <div class=format!("heatmap-cell {}", severity_class)>
                                    {if count > 0 { format!("{}", count) } else { String::new() }}
                                </div>
                            }
                        }).collect_view()}
                    }
                }).collect_view()}
            </div>
            <div class="heatmap-axis-labels">
                <span class="heatmap-x-label">"Impact →"</span>
                <span class="heatmap-y-label">"Likelihood →"</span>
            </div>
        </div>
    }
}
