use leptos::*;
use leptos_router::*;

use crate::components::sidebar::Sidebar;
use crate::pages::{
    audit_trail::AuditTrailPage,
    ai_system_detail::AiSystemDetailPage,
    cross_reference::CrossReferencePage,
    dashboard::DashboardPage,
    engagement_detail::EngagementDetailPage,
    engagements::EngagementsPage,
    evidence_vault::EvidenceVaultPage,
    fria::FriaPage,
    framework_navigator::FrameworkNavigatorPage,
    gap_analysis::GapAnalysisPage,
    guide::GuidePage,
    llm_assistant::LlmAssistantPage,
    reports::ReportsPage,
    risk_matrix::RiskMatrixPage,
    settings::SettingsPage,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app-layout">
                <Sidebar />
                <main class="main-content">
                    <Routes>
                        <Route path="/" view=DashboardPage />
                        <Route path="/engagements" view=EngagementsPage />
                        <Route path="/engagements/:id" view=EngagementDetailPage />
                        <Route path="/systems/:id" view=AiSystemDetailPage />
                        <Route path="/fria" view=FriaPage />
                        <Route path="/fria/:sys_id" view=FriaPage />
                        <Route path="/frameworks" view=FrameworkNavigatorPage />
                        <Route path="/frameworks/:fw" view=FrameworkNavigatorPage />
                        <Route path="/cross-references" view=CrossReferencePage />
                        <Route path="/risk-matrix" view=RiskMatrixPage />
                        <Route path="/risk-matrix/:sys_id" view=RiskMatrixPage />
                        <Route path="/evidence" view=EvidenceVaultPage />
                        <Route path="/gap-analysis" view=GapAnalysisPage />
                        <Route path="/reports" view=ReportsPage />
                        <Route path="/audit-trail" view=AuditTrailPage />
                        <Route path="/assistant" view=LlmAssistantPage />
                        <Route path="/settings" view=SettingsPage />
                        <Route path="/guide" view=GuidePage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
