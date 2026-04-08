use leptos::*;
use leptos_router::*;
use grc_shared::models::DashboardStats;

use crate::api::invoke;
use crate::components::stat_card::StatCard;
use crate::components::compliance_bar::ComplianceBar;
use crate::components::task_row::TaskRow;
use crate::components::help_panel::{HelpPanel, HelpSection};

#[component]
pub fn DashboardPage() -> impl IntoView {
    let stats = create_resource(|| (), |_| async {
        invoke::call_no_args::<DashboardStats>("get_dashboard_stats").await
    });

    view! {
        <div class="page dashboard-page">
            <h1>"Command Center"</h1>            <HelpPanel title="Dashboard Help">
                <HelpSection heading="KPI Cards">
                    <p>"The eight cards at the top show key metrics: active engagements, AI systems under review, open tasks and risks, compliance gaps, evidence items, and FRIA status. A rising gap count or overdue tasks signal areas needing attention."</p>
                </HelpSection>
                <HelpSection heading="Compliance Bars">
                    <p>"Each horizontal bar represents one framework. Green = Met, Amber = Partial, Red = Gap, Grey = Not Assessed. Click any bar to jump directly into that framework's requirements in the Framework Navigator."</p>
                </HelpSection>
                <HelpSection heading="Priority Tasks">
                    <p>"The bottom table lists recent open remediation tasks. Click through to the linked engagement for action."</p>
                </HelpSection>
            </HelpPanel>            <Suspense fallback=move || view! { <p>"Loading dashboard…"</p> }>
                {move || stats.get().map(|result| match result {
                    Ok(s) => view! {
                        <div class="stats-grid">
                            <StatCard label="Active Engagements" value=s.active_engagements.to_string() />
                            <StatCard label="AI Systems" value=s.total_ai_systems.to_string() />
                            <StatCard label="Open Tasks" value=s.open_tasks.to_string() />
                            <StatCard label="Open Risks" value=s.open_risks.to_string() />
                            <StatCard label="Total Gaps" value=s.total_gaps.to_string() />
                            <StatCard label="Evidence Items" value=s.total_evidence.to_string() />
                            <StatCard label="FRIAs In Scope" value=s.fria_in_scope.to_string() />
                            <StatCard label="FRIAs Completed" value=s.fria_completed.to_string() />
                        </div>
                        <section class="compliance-overview">
                            <h2>"Framework Compliance"</h2>
                            {s.compliance_by_framework.clone().into_iter().map(|fc| {
                                let fw_str = serde_json::to_value(&fc.framework)
                                    .ok()
                                    .and_then(|v| v.as_str().map(String::from))
                                    .unwrap_or_default();
                                let href = format!("/frameworks/{}", fw_str);
                                view! {
                                    <A href=href>
                                        <ComplianceBar
                                            label=fw_str
                                            met=fc.met
                                            partial=fc.partial
                                            gap=fc.gap
                                            not_assessed=fc.not_assessed
                                        />
                                    </A>
                                }
                            }).collect_view()}
                        </section>
                        <section class="priority-tasks">
                            <h2>"Priority Tasks"</h2>
                            <table role="grid">
                                <thead>
                                    <tr>
                                        <th>"Task"</th><th>"Status"</th><th>"Priority"</th><th>"Due"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {s.priority_tasks.into_iter().map(|t| {
                                        view! { <TaskRow task=t /> }
                                    }).collect_view()}
                                </tbody>
                            </table>
                        </section>
                    }.into_view(),
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
