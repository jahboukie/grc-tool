use leptos::*;

#[component]
pub fn GuidePage() -> impl IntoView {
    let sections = vec![
        "overview",
        "dashboard",
        "engagements",
        "ai-systems",
        "fria",
        "frameworks",
        "cross-references",
        "risk-matrix",
        "evidence",
        "gap-analysis",
        "reports",
        "audit-trail",
        "assistant",
        "settings",
        "workflow",
    ];

    let (active, set_active) = create_signal("overview".to_string());

    view! {
        <div class="page guide-page">
            <h1>"User Guide"</h1>
            <p class="guide-subtitle">
                "Comprehensive tutorials and reference for every section of the GRC Command Center."
            </p>
            <div class="guide-layout">
                <nav class="guide-toc">
                    <h3>"Contents"</h3>
                    <ul>
                        {sections.into_iter().map(|s| {
                            let label = toc_label(s);
                            let id = s.to_string();
                            let id2 = s.to_string();
                            view! {
                                <li class:toc-active=move || active.get() == id>
                                    <a
                                        href=format!("#{}", id2)
                                        on:click=move |_| set_active.set(id2.clone())
                                    >
                                        {label}
                                    </a>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </nav>
                <div class="guide-body">
                    <GuideOverview />
                    <GuideDashboard />
                    <GuideEngagements />
                    <GuideAiSystems />
                    <GuideFria />
                    <GuideFrameworks />
                    <GuideCrossReferences />
                    <GuideRiskMatrix />
                    <GuideEvidence />
                    <GuideGapAnalysis />
                    <GuideReports />
                    <GuideAuditTrail />
                    <GuideAssistant />
                    <GuideSettings />
                    <GuideWorkflow />
                </div>
            </div>
        </div>
    }
}

fn toc_label(id: &str) -> &str {
    match id {
        "overview" => "Overview",
        "dashboard" => "Dashboard",
        "engagements" => "Engagements",
        "ai-systems" => "AI Systems",
        "fria" => "FRIA",
        "frameworks" => "Framework Navigator",
        "cross-references" => "Cross-References",
        "risk-matrix" => "Risk Matrix",
        "evidence" => "Evidence Vault",
        "gap-analysis" => "Gap Analysis",
        "reports" => "Reports",
        "audit-trail" => "Audit Trail",
        "assistant" => "AI Assistant",
        "settings" => "Settings",
        "workflow" => "End-to-End Workflow",
        _ => "",
    }
}

// ───────────────────────────────────────────────
// Individual guide sections
// ───────────────────────────────────────────────

#[component]
fn GuideOverview() -> impl IntoView {
    view! {
        <section id="overview" class="guide-section">
            <h2>"Overview"</h2>
            <p>
                "The GRC Command Center is a personal AI Governance, Risk & Compliance desktop application "
                "designed for Certified AI Governance Professionals. It provides a structured, "
                "evidence-backed workflow for assessing AI systems against five major regulatory and "
                "standards frameworks:"
            </p>
            <ul>
                <li><strong>"EU AI Act"</strong>" (Regulation 2024/1689) — the legally binding European regulation on AI systems"</li>
                <li><strong>"ISO/IEC 42001:2023"</strong>" — the international standard for AI Management Systems"</li>
                <li><strong>"ISO/IEC 23894:2023"</strong>" — AI-specific risk management guidance"</li>
                <li><strong>"NIST AI RMF 1.0"</strong>" — the US National Institute of Standards and Technology AI Risk Management Framework"</li>
                <li><strong>"OECD AI Principles"</strong>" — the policy-level principles for responsible AI (2019, updated 2024)"</li>
            </ul>
            <h3>"Core Concepts"</h3>
            <p>
                "The tool is organized around a hierarchy of key entities:"
            </p>
            <ol>
                <li><strong>"Engagement"</strong>" — a scoped audit or review project for a specific client and context (e.g., \"Northwind Bank — Credit Decisioning Review 2026\")"</li>
                <li><strong>"AI System"</strong>" — a specific AI product or service being assessed within an engagement (e.g., \"Nimbus Credit Decision Engine v2.4\")"</li>
                <li><strong>"Framework Requirements"</strong>" — the individual obligations, controls, and principles from the five frameworks that the AI system must be assessed against"</li>
                <li><strong>"Assessments"</strong>" — your compliance judgments (Met / Partial / Gap / Not Assessed / Not Applicable) for each requirement against each AI system"</li>
                <li><strong>"Evidence"</strong>" — documents, reports, and artifacts that support your assessments"</li>
                <li><strong>"Risks"</strong>" — identified risks with likelihood/impact scoring, mitigations, and residual scores"</li>
                <li><strong>"Tasks"</strong>" — remediation work items linked to engagements"</li>
            </ol>
            <p>
                "Every mutation in the system (create, update, delete) is logged in an immutable audit trail. "
                "The database enforces this — audit records cannot be modified or removed."
            </p>
        </section>
    }
}

#[component]
fn GuideDashboard() -> impl IntoView {
    view! {
        <section id="dashboard" class="guide-section">
            <h2>"Dashboard"</h2>
            <p>
                "The Dashboard is your command center overview — a single screen showing the health "
                "of all active work at a glance."
            </p>
            <h3>"KPI Cards"</h3>
            <p>
                "The top section displays eight key performance indicator cards:"
            </p>
            <ul>
                <li><strong>"Active Engagements"</strong>" — number of engagements in 'active' status"</li>
                <li><strong>"AI Systems"</strong>" — total AI systems across all engagements"</li>
                <li><strong>"Open Tasks"</strong>" — tasks not yet marked 'done' or 'deferred'"</li>
                <li><strong>"Open Risks"</strong>" — risk entries with status 'open'"</li>
                <li><strong>"Total Gaps"</strong>" — assessments currently rated as 'gap' (non-compliant)"</li>
                <li><strong>"Evidence Items"</strong>" — total uploaded evidence files"</li>
                <li><strong>"FRIAs In Scope"</strong>" — AI systems where a Fundamental Rights Impact Assessment is required"</li>
                <li><strong>"FRIAs Completed"</strong>" — AI systems with completed FRIAs"</li>
            </ul>
            <h3>"Framework Compliance Bars"</h3>
            <p>
                "Below the KPI cards, horizontal compliance bars show the breakdown across all five "
                "frameworks. Each bar is color-coded:"
            </p>
            <ul>
                <li><span style="color: #16a34a">"● Green"</span>" — Met (compliant)"</li>
                <li><span style="color: #d97706">"● Amber"</span>" — Partial (partially compliant)"</li>
                <li><span style="color: #dc2626">"● Red"</span>" — Gap (non-compliant)"</li>
                <li><span style="color: #9ca3af">"● Grey"</span>" — Not Assessed"</li>
            </ul>
            <p>
                "Click any compliance bar to jump directly to that framework in the Framework Navigator."
            </p>
            <h3>"Priority Tasks"</h3>
            <p>
                "The bottom section lists the most recent open tasks — remediation work items that "
                "need attention. Each row shows the task title, status badge, target date, and linked engagement."
            </p>
            <h3>"Step-by-Step: Reading the Dashboard"</h3>
            <ol>
                <li>"Open the app — Dashboard loads automatically"</li>
                <li>"Check KPI cards for any concerning numbers (high gap count, overdue tasks)"</li>
                <li>"Review compliance bars to identify which frameworks have the most gaps"</li>
                <li>"Click a compliance bar to drill into that framework's requirements"</li>
                <li>"Review priority tasks and click through to the linked engagement for action"</li>
            </ol>
        </section>
    }
}

#[component]
fn GuideEngagements() -> impl IntoView {
    view! {
        <section id="engagements" class="guide-section">
            <h2>"Engagements"</h2>
            <p>
                "An Engagement represents a scoped GRC review or audit project for a specific client. "
                "It captures the organizational context, regulatory scope, and risk profile needed to "
                "select the right frameworks and drive the assessment."
            </p>
            <h3>"Creating an Engagement"</h3>
            <p>
                "Click \"New Engagement\" to open the intake form. The form uses a structured scoping "
                "approach that automatically suggests applicable frameworks based on your inputs."
            </p>
            <h4>"Required Fields"</h4>
            <ul>
                <li><strong>"Name"</strong>" — a descriptive engagement title (e.g., \"Northwind Community Bank — Credit Decisioning Review 2026\")"</li>
                <li><strong>"Client"</strong>" — the organization being assessed"</li>
            </ul>
            <h4>"Scoping Fields"</h4>
            <ul>
                <li><strong>"Primary Role"</strong>" — your client's role: Provider (builds AI), Deployer (operates AI), Importer, Distributor, or Dual. This affects which EU AI Act obligations apply."</li>
                <li><strong>"Industry Sector"</strong>" — Finance, Healthcare, Public Admin, Telecom, etc. Certain industries trigger stricter requirements."</li>
                <li><strong>"Assurance Objective"</strong>" — the goal: Baseline Compliance, Pre-Market, Post-Market, Risk-Focused, Certification, or Innovation Sandbox."</li>
                <li><strong>"AI Use Case"</strong>" — the specific application: Credit Scoring, Medical Diagnosis, Recruitment, Facial Recognition, Content Moderation, etc."</li>
                <li><strong>"Personal Data Profile"</strong>" — whether the system processes personal data, special-category data, or anonymized data."</li>
                <li><strong>"Jurisdictions"</strong>" — where the system operates: EU, UK, US, Global, etc. Multi-select."</li>
                <li><strong>"Involves Vulnerable Groups"</strong>" — whether the AI impacts children, elderly, disabled, or other vulnerable populations."</li>
                <li><strong>"Public Facing"</strong>" — whether end-users interact directly with the AI."</li>
            </ul>
            <h4>"Framework Auto-Suggestion"</h4>
            <p>
                "As you fill in scoping fields, the system calculates which frameworks are applicable. "
                "For example, selecting an EU jurisdiction and a high-risk use case like credit scoring "
                "will suggest EU AI Act + ISO 42001 + ISO 23894. You can accept, modify, or override the suggestions."
            </p>
            <h3>"Step-by-Step: Creating Your First Engagement"</h3>
            <ol>
                <li>"Navigate to Engagements from the sidebar"</li>
                <li>"Click \"New Engagement\""</li>
                <li>"Enter the engagement name and client name"</li>
                <li>"Select the primary role (e.g., Deployer for a bank using a vendor's AI)"</li>
                <li>"Choose the industry sector (e.g., Finance)"</li>
                <li>"Select the AI use case (e.g., Credit Scoring)"</li>
                <li>"Set jurisdictions (e.g., EU)"</li>
                <li>"Toggle vulnerable groups and public-facing flags as needed"</li>
                <li>"Review the auto-suggested frameworks and adjust if needed"</li>
                <li>"Click \"Create\" to save the engagement"</li>
                <li>"Click the engagement name in the list to open its detail page"</li>
            </ol>
            <h3>"Engagement Detail Page"</h3>
            <p>
                "Clicking an engagement opens its detail view where you can:"
            </p>
            <ul>
                <li>"Edit the engagement metadata and scoping fields"</li>
                <li>"View and manage linked AI systems"</li>
                <li>"View and manage tasks (remediation items)"</li>
                <li>"See framework selections and update them"</li>
                <li>"Change the engagement status (Active, Paused, Completed, Archived)"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideAiSystems() -> impl IntoView {
    view! {
        <section id="ai-systems" class="guide-section">
            <h2>"AI Systems"</h2>
            <p>
                "Each engagement contains one or more AI Systems — the specific products or services "
                "being assessed. The AI System is the primary entity that gets assessed against "
                "framework requirements, has risks registered against it, and accumulates evidence."
            </p>
            <h3>"AI System Fields"</h3>
            <ul>
                <li><strong>"Name"</strong>" — the system name and version (e.g., \"Nimbus Credit Decision Engine v2.4\")"</li>
                <li><strong>"Domain"</strong>" — the business domain (Finance, Healthcare, Education, etc.)"</li>
                <li><strong>"Intended Purpose"</strong>" — a clear description of what the AI system does and how it's used"</li>
                <li><strong>"Risk Category"</strong>" — the EU AI Act classification: Unacceptable, High, Limited, Minimal. This drives which requirements are applicable."</li>
                <li><strong>"GPAI Flag"</strong>" — whether the system is a General-Purpose AI model (triggers GPAI-specific requirements)"</li>
                <li><strong>"Annex III Flag"</strong>" — whether the system falls under EU AI Act Annex III (biometrics, critical infrastructure, education, employment, law enforcement, etc.)"</li>
                <li><strong>"Safety Component Flag"</strong>" — whether the AI is a safety component of another product"</li>
            </ul>
            <h3>"AI System Detail Page"</h3>
            <p>
                "Clicking an AI system from an engagement opens its detail page showing:"
            </p>
            <ul>
                <li>"System metadata and classification flags (editable)"</li>
                <li>"Recent requirement assessments for this system"</li>
                <li>"Linked evidence items"</li>
                <li>"Navigation links to the Risk Matrix and FRIA for this system"</li>
            </ul>
            <h3>"Step-by-Step: Adding an AI System"</h3>
            <ol>
                <li>"Open an engagement's detail page"</li>
                <li>"In the AI Systems section, click \"Add AI System\""</li>
                <li>"Enter the system name, domain, and intended purpose"</li>
                <li>"Select the risk category based on your EU AI Act classification analysis"</li>
                <li>"Set GPAI, Annex III, and Safety Component flags as appropriate"</li>
                <li>"Click \"Create\" to save"</li>
                <li>"Click the system name to open its detail page"</li>
            </ol>
            <h3>"Example: Northwind Credit Decision Engine"</h3>
            <p>
                "For a credit scoring AI used by a regional bank:"
            </p>
            <ul>
                <li>"Name: Nimbus Credit Decision Engine v2.4"</li>
                <li>"Domain: Finance"</li>
                <li>"Purpose: Support consumer loan underwriting by scoring applications and recommending approve/review/decline outcomes"</li>
                <li>"Risk Category: High (credit scoring falls under Annex III, point 5(b))"</li>
                <li>"GPAI: No (purpose-built model, not general-purpose)"</li>
                <li>"Annex III: Yes (creditworthiness assessment)"</li>
                <li>"Safety Component: No"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideFria() -> impl IntoView {
    view! {
        <section id="fria" class="guide-section">
            <h2>"Fundamental Rights Impact Assessment (FRIA)"</h2>
            <p>
                "The FRIA is required under Article 27 of the EU AI Act for deployers of high-risk AI systems. "
                "It assesses the impact of the AI system on fundamental rights of affected persons "
                "before the system is put into use."
            </p>
            <h3>"When is a FRIA Required?"</h3>
            <ul>
                <li>"The AI system is classified as high-risk"</li>
                <li>"Your role is Deployer (you operate the system, not build it)"</li>
                <li>"The system is used in areas affecting fundamental rights (credit, employment, justice, public services, etc.)"</li>
            </ul>
            <h3>"FRIA Sections"</h3>
            <p>"The FRIA form is divided into structured sections that mirror Article 27 requirements:"</p>
            <h4>"1. Scope & Deployer Context"</h4>
            <p>
                "Document who you are (deployer identity), what the system does, the deployment context, "
                "the categories of persons affected, and the geographic/temporal scope."
            </p>
            <h4>"2. Affected Groups"</h4>
            <p>
                "Identify all groups potentially affected by the AI system: direct users, data subjects, "
                "and third parties. Pay special attention to vulnerable groups (children, elderly, "
                "disabled persons, economically disadvantaged)."
            </p>
            <h4>"3. Rights & Risks Assessment"</h4>
            <p>
                "For each fundamental right that may be impacted, assess the risk. Common rights affected "
                "by credit scoring AI include: non-discrimination, privacy, access to essential services, "
                "effective remedy, and explanation of automated decisions."
            </p>
            <h4>"4. Human Oversight Measures"</h4>
            <p>
                "Document how human oversight is implemented: who reviews AI outputs, what override "
                "mechanisms exist, what training has been provided, and what escalation procedures are in place."
            </p>
            <h4>"5. Mitigation & Safeguards"</h4>
            <p>
                "List the specific measures taken to reduce identified risks: technical safeguards, "
                "procedural controls, monitoring arrangements, and complaint mechanisms."
            </p>
            <h4>"6. Consultation & Notification"</h4>
            <p>
                "Record whether affected persons or their representatives were consulted, "
                "and whether the relevant national supervisory authority has been notified of the FRIA results."
            </p>
            <h3>"Step-by-Step: Completing a FRIA"</h3>
            <ol>
                <li>"Navigate to FRIA from the sidebar"</li>
                <li>"Select the engagement and AI system from the dropdowns"</li>
                <li>"Work through each section, filling in the structured fields"</li>
                <li>"For the rights assessment section, add each affected right with its risk rating"</li>
                <li>"Document human oversight measures with specific procedure references"</li>
                <li>"List all mitigation measures and link them to the corresponding risks"</li>
                <li>"Record consultation activities and notification status"</li>
                <li>"Save the FRIA — it will appear in the Dashboard's FRIA counters"</li>
            </ol>
            <h3>"Example: Credit Scoring FRIA"</h3>
            <p>"For Northwind Bank's credit decision engine, the FRIA would typically cover:"</p>
            <ul>
                <li>"Affected groups: loan applicants, guarantors, potentially vulnerable consumers (low-income, first-time borrowers)"</li>
                <li>"Key rights at risk: non-discrimination (protected characteristics in credit decisions), privacy (financial data processing), access to credit (essential service), right to explanation (adverse action notices)"</li>
                <li>"Human oversight: credit officers review all AI recommendations; mandatory human decision for adverse actions above a threshold; override logging and periodic review"</li>
                <li>"Mitigations: bias testing on protected attributes, explainability reports per decision, appeal procedure for applicants, model monitoring for drift"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideFrameworks() -> impl IntoView {
    view! {
        <section id="frameworks" class="guide-section">
            <h2>"Framework Navigator"</h2>
            <p>
                "The Framework Navigator is the core assessment workspace. It provides a split-panel "
                "interface for browsing requirements from all five frameworks and recording your "
                "compliance assessments against specific AI systems."
            </p>
            <h3>"Left Panel: Requirement Browser"</h3>
            <ul>
                <li>"Switch between frameworks using the tab bar at the top"</li>
                <li>"Requirements are organized by category (articles, clauses, functions, principles)"</li>
                <li>"Each requirement shows its code, title, and current assessment status badge"</li>
                <li>"Use the search bar to filter requirements by text"</li>
                <li>"Click a requirement to load its detail in the right panel"</li>
            </ul>
            <h3>"Right Panel: Requirement Detail"</h3>
            <p>"When you select a requirement, the right panel shows:"</p>
            <ul>
                <li><strong>"Description"</strong>" — the full text of the requirement or obligation"</li>
                <li><strong>"Guidance"</strong>" — implementation guidance and interpretation notes"</li>
                <li><strong>"Cross-References"</strong>" — related requirements from other frameworks (with relationship types: equivalent, overlapping, supports, extends)"</li>
                <li><strong>"Assessment Editor"</strong>" — where you record your judgment for a specific engagement + AI system"</li>
            </ul>
            <h3>"Recording Assessments"</h3>
            <p>"The assessment editor lets you record:"</p>
            <ul>
                <li><strong>"Status"</strong>" — Met, Partial, Gap, Not Assessed, or Not Applicable"</li>
                <li><strong>"Findings"</strong>" — your observations and evidence summary"</li>
                <li><strong>"Remediation"</strong>" — the recommended corrective action (especially for Partial/Gap)"</li>
            </ul>
            <h3>"Assessment Status Meanings"</h3>
            <ul>
                <li><span class="status-badge status-met">"Met"</span>" — the requirement is fully satisfied with supporting evidence"</li>
                <li><span class="status-badge status-partial">"Partial"</span>" — some elements are addressed but gaps remain"</li>
                <li><span class="status-badge status-gap">"Gap"</span>" — the requirement is not met; remediation needed"</li>
                <li><span class="status-badge status-not_assessed">"Not Assessed"</span>" — not yet evaluated"</li>
                <li><span class="status-badge status-not_applicable">"N/A"</span>" — the requirement does not apply to this system"</li>
            </ul>
            <h3>"Step-by-Step: Assessing Requirements"</h3>
            <ol>
                <li>"Navigate to Framework Navigator from the sidebar"</li>
                <li>"Select the framework tab (e.g., EU AI Act)"</li>
                <li>"Browse or search for a requirement"</li>
                <li>"Click the requirement to open its detail panel"</li>
                <li>"In the assessment editor, select the engagement and AI system"</li>
                <li>"Set the status (Met, Partial, Gap, etc.)"</li>
                <li>"Enter your findings — describe what you observed"</li>
                <li>"If Partial or Gap, enter a remediation plan"</li>
                <li>"Save the assessment"</li>
                <li>"Move to the next requirement and repeat"</li>
            </ol>
            <h3>"Tips"</h3>
            <ul>
                <li>"Start by working through all high-priority requirements (EU AI Act Articles for high-risk systems)"</li>
                <li>"Use cross-references to avoid duplicate work — if EU AI Act Art 9 maps to ISO 23894 Clause 6, assess them together"</li>
                <li>"Use the search bar to quickly find requirements related to specific topics (e.g., \"transparency\", \"human oversight\")"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideCrossReferences() -> impl IntoView {
    view! {
        <section id="cross-references" class="guide-section">
            <h2>"Cross-References"</h2>
            <p>
                "The Cross-Reference page visualizes the linkage matrix between all five frameworks. "
                "Many requirements across frameworks address similar or overlapping concerns — "
                "cross-references help you avoid duplicate work and ensure consistent assessments."
            </p>
            <h3>"Relationship Types"</h3>
            <ul>
                <li><strong>"Equivalent"</strong>" — the two requirements address essentially the same obligation"</li>
                <li><strong>"Overlapping"</strong>" — the requirements share common ground but each has unique elements"</li>
                <li><strong>"Supports"</strong>" — one requirement provides a foundation for or contributes to another"</li>
                <li><strong>"Extends"</strong>" — one requirement goes beyond the other with additional obligations"</li>
            </ul>
            <h3>"Using Cross-References"</h3>
            <p>
                "When you assess a requirement in one framework, check its cross-references to see "
                "which requirements in other frameworks are related. If two requirements are equivalent, "
                "your findings for one can largely be reused for the other."
            </p>
            <h3>"Example"</h3>
            <p>
                "EU AI Act Article 9 (Risk Management System) cross-references to:"
            </p>
            <ul>
                <li>"ISO 42001 Clause 6.1 (Actions to address risks and opportunities) — Overlapping"</li>
                <li>"ISO 23894 Clause 6 (Risk assessment) — Equivalent"</li>
                <li>"NIST AI RMF Map function — Supports"</li>
                <li>"OECD Principle 1.4 (Robustness, security and safety) — Overlapping"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideRiskMatrix() -> impl IntoView {
    view! {
        <section id="risk-matrix" class="guide-section">
            <h2>"Risk Matrix"</h2>
            <p>
                "The Risk Matrix provides a visual 5×5 heatmap for risk assessment and a full risk "
                "register for documenting, scoring, and tracking risks associated with AI systems."
            </p>
            <h3>"Risk Heatmap"</h3>
            <p>
                "The heatmap plots risks on a Likelihood (1-5) × Impact (1-5) grid. Each cell is "
                "color-coded from green (low) to red (critical). Click any cell to filter the risk "
                "register to risks at that intersection."
            </p>
            <h3>"Risk Entry Fields"</h3>
            <ul>
                <li><strong>"Title"</strong>" — a descriptive name for the risk"</li>
                <li><strong>"Description"</strong>" — detailed explanation of the risk scenario"</li>
                <li><strong>"Category"</strong>" — the risk domain (Bias, Transparency, Security, Safety, Privacy, etc.)"</li>
                <li><strong>"Inherent Likelihood"</strong>" (1-5) — the probability before any mitigations"</li>
                <li><strong>"Inherent Impact"</strong>" (1-5) — the severity before any mitigations"</li>
                <li><strong>"Mitigation"</strong>" — the controls and measures to reduce the risk"</li>
                <li><strong>"Residual Likelihood"</strong>" (1-5) — the probability after mitigations"</li>
                <li><strong>"Residual Impact"</strong>" (1-5) — the severity after mitigations"</li>
                <li><strong>"Status"</strong>" — Open, Mitigated, Accepted, Transferred, Closed"</li>
            </ul>
            <h3>"Scoring Guide"</h3>
            <table>
                <thead><tr><th>"Score"</th><th>"Likelihood"</th><th>"Impact"</th></tr></thead>
                <tbody>
                    <tr><td>"1"</td><td>"Rare"</td><td>"Negligible"</td></tr>
                    <tr><td>"2"</td><td>"Unlikely"</td><td>"Minor"</td></tr>
                    <tr><td>"3"</td><td>"Possible"</td><td>"Moderate"</td></tr>
                    <tr><td>"4"</td><td>"Likely"</td><td>"Major"</td></tr>
                    <tr><td>"5"</td><td>"Almost Certain"</td><td>"Severe"</td></tr>
                </tbody>
            </table>
            <h3>"Step-by-Step: Adding a Risk"</h3>
            <ol>
                <li>"Navigate to Risk Matrix from the sidebar"</li>
                <li>"Select the engagement and AI system"</li>
                <li>"Click \"Add Risk\""</li>
                <li>"Enter the title and detailed description"</li>
                <li>"Select the risk category"</li>
                <li>"Set inherent likelihood and impact scores"</li>
                <li>"Describe the mitigation measures"</li>
                <li>"Set residual likelihood and impact scores (should be <= inherent scores)"</li>
                <li>"Save — the risk will appear in both the heatmap and the register table"</li>
            </ol>
            <h3>"Example Risks for Credit Scoring AI"</h3>
            <ul>
                <li><strong>"Bias against protected groups"</strong>" — Likelihood: 3, Impact: 5 → Inherent severity: High. Mitigation: regular bias testing on protected attributes, demographic parity monitoring. Residual: 2/4."</li>
                <li><strong>"Poor explainability for adverse actions"</strong>" — Likelihood: 4, Impact: 4. Mitigation: per-decision feature importance reports, plain-language denial letters. Residual: 2/3."</li>
                <li><strong>"Model drift"</strong>" — Likelihood: 3, Impact: 3. Mitigation: quarterly performance monitoring, automated drift detection alerts. Residual: 2/2."</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideEvidence() -> impl IntoView {
    view! {
        <section id="evidence" class="guide-section">
            <h2>"Evidence Vault"</h2>
            <p>
                "The Evidence Vault stores and manages all supporting documents and artifacts for your "
                "compliance assessments. Evidence can be linked to specific assessments, risks, and tasks "
                "to create a complete audit trail."
            </p>
            <h3>"Evidence Types"</h3>
            <ul>
                <li><strong>"Policy"</strong>" — organizational policies and procedures"</li>
                <li><strong>"Technical"</strong>" — technical documentation, model cards, architecture diagrams"</li>
                <li><strong>"Audit"</strong>" — internal or external audit reports"</li>
                <li><strong>"Training"</strong>" — staff training records and materials"</li>
                <li><strong>"Testing"</strong>" — test results, validation reports, bias testing"</li>
                <li><strong>"Contract"</strong>" — vendor agreements, DPAs, SLAs"</li>
                <li><strong>"Communication"</strong>" — stakeholder communications, consultation records"</li>
                <li><strong>"Incident"</strong>" — incident reports and post-mortems"</li>
                <li><strong>"Monitoring"</strong>" — monitoring dashboards, performance logs"</li>
                <li><strong>"Regulatory"</strong>" — regulatory submissions, notifications, correspondence"</li>
                <li><strong>"Other"</strong>" — any evidence that doesn't fit the above categories"</li>
            </ul>
            <h3>"Uploading Evidence"</h3>
            <ol>
                <li>"Navigate to Evidence Vault from the sidebar"</li>
                <li>"Click \"Upload Evidence\""</li>
                <li>"Use the file picker to select the file from your filesystem"</li>
                <li>"Enter a description explaining what the document evidences"</li>
                <li>"Select the evidence type"</li>
                <li>"Add tags for searchability (e.g., \"bias\", \"model-card\", \"quarterly-review\")"</li>
                <li>"Click \"Upload\" to store the evidence"</li>
            </ol>
            <h3>"Linking Evidence"</h3>
            <p>
                "After uploading, you can link evidence to specific entities:"
            </p>
            <ul>
                <li><strong>"Assessments"</strong>" — link a policy document to a requirement assessment to prove compliance"</li>
                <li><strong>"Risks"</strong>" — link a bias testing report to a discrimination risk to show mitigation"</li>
                <li><strong>"Tasks"</strong>" — link deliverables to remediation tasks to show completion"</li>
            </ul>
            <h3>"Example Evidence for Credit Scoring"</h3>
            <ul>
                <li>"Model card for Nimbus Credit Decision Engine v2.4 (Technical)"</li>
                <li>"Vendor technical documentation and API specifications (Technical)"</li>
                <li>"Credit underwriting policy v3.1 (Policy)"</li>
                <li>"Bias testing report — Q1 2026 (Testing)"</li>
                <li>"Model validation report — annual (Testing)"</li>
                <li>"Human oversight SOP for credit decisions (Policy)"</li>
                <li>"Credit officer AI training completion records (Training)"</li>
                <li>"Post-market monitoring plan (Monitoring)"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideGapAnalysis() -> impl IntoView {
    view! {
        <section id="gap-analysis" class="guide-section">
            <h2>"Gap Analysis"</h2>
            <p>
                "The Gap Analysis page provides a consolidated view of all compliance gaps across "
                "frameworks. It aggregates your assessments and highlights where remediation is needed."
            </p>
            <h3>"What It Shows"</h3>
            <ul>
                <li>"A summary bar per framework showing the count of gaps vs. total requirements"</li>
                <li>"Expandable sections listing every assessment rated as 'Gap' or 'Partial'"</li>
                <li>"For each gap, the requirement details, your findings, and the remediation plan"</li>
                <li>"Cross-reference notes showing if the same gap affects multiple frameworks"</li>
            </ul>
            <h3>"Using Gap Analysis"</h3>
            <ol>
                <li>"Navigate to Gap Analysis from the sidebar"</li>
                <li>"Review the framework summary bars to identify which frameworks have the most gaps"</li>
                <li>"Expand a framework section to see individual gaps"</li>
                <li>"For each gap, review the remediation plan"</li>
                <li>"Use the \"Generate Report\" button to create an HTML gap analysis report"</li>
            </ol>
            <h3>"Tips"</h3>
            <ul>
                <li>"Address gaps that affect multiple frameworks first (check cross-references) — one remediation may close gaps in several frameworks simultaneously"</li>
                <li>"Focus on high-risk requirements first: EU AI Act transparency and human oversight obligations for high-risk systems are enforcement priorities"</li>
                <li>"Create tasks from gaps to track remediation progress"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideReports() -> impl IntoView {
    view! {
        <section id="reports" class="guide-section">
            <h2>"Reports"</h2>
            <p>
                "The Reports page generates professional HTML reports that open in your default browser. "
                "Reports are rendered server-side and saved to your local filesystem."
            </p>
            <h3>"Report Types"</h3>
            <h4>"1. Compliance Report"</h4>
            <p>
                "A comprehensive framework status report showing every requirement, its assessment status, "
                "findings, and remediation plans. Grouped by framework with summary statistics."
            </p>
            <h4>"2. Gap Analysis Report"</h4>
            <p>
                "A focused report listing only gaps and partial assessments with remediation plans. "
                "Ideal for sharing with stakeholders who need to understand what needs fixing."
            </p>
            <h4>"3. Risk Report"</h4>
            <p>
                "The full risk register with heatmap visualization, risk details, mitigation measures, "
                "and residual scores. Requires selecting a specific AI system."
            </p>
            <h3>"Generating a Report"</h3>
            <ol>
                <li>"Navigate to Reports from the sidebar"</li>
                <li>"Select the engagement from the dropdown"</li>
                <li>"Choose the report type"</li>
                <li>"For Risk Reports, also select the AI system"</li>
                <li>"Click \"Generate\" — the report will open in your browser"</li>
                <li>"From the browser, use Ctrl+P to print or save as PDF if needed"</li>
            </ol>
        </section>
    }
}

#[component]
fn GuideAuditTrail() -> impl IntoView {
    view! {
        <section id="audit-trail" class="guide-section">
            <h2>"Audit Trail"</h2>
            <p>
                "Every action in the GRC Command Center is logged in an immutable audit trail. "
                "The database enforces immutability through triggers — audit records cannot be "
                "modified or deleted, even by direct database access."
            </p>
            <h3>"What Gets Logged"</h3>
            <ul>
                <li>"Every entity creation (engagements, AI systems, assessments, risks, evidence, tasks)"</li>
                <li>"Every update (with before/after values for changed fields)"</li>
                <li>"Every deletion"</li>
                <li>"Configuration changes"</li>
            </ul>
            <h3>"Audit Entry Fields"</h3>
            <ul>
                <li><strong>"Timestamp"</strong>" — when the action occurred (UTC)"</li>
                <li><strong>"Entity Type"</strong>" — what was changed (engagement, ai_system, assessment, etc.)"</li>
                <li><strong>"Entity ID"</strong>" — the UUID of the affected record"</li>
                <li><strong>"Action"</strong>" — Created, Updated, or Deleted"</li>
                <li><strong>"Changed Field"</strong>" — which field was modified (for updates)"</li>
                <li><strong>"Before Value"</strong>" — the previous value"</li>
                <li><strong>"After Value"</strong>" — the new value"</li>
            </ul>
            <h3>"Filtering the Audit Trail"</h3>
            <ul>
                <li>"Filter by entity type to see only changes to specific objects"</li>
                <li>"Filter by action (Created, Updated, Deleted) to focus on specific events"</li>
                <li>"Filter by date range for time-bounded reviews"</li>
                <li>"Search by entity ID to trace the full history of a specific record"</li>
            </ul>
            <h3>"Why Immutability Matters"</h3>
            <p>
                "Regulatory frameworks like the EU AI Act (Article 12) and ISO 42001 (Clause 7.5) "
                "require that documented information be retained and protected from unauthorized modification. "
                "The immutable audit trail ensures that all compliance activities are defensibly recorded "
                "and tamper-resistant."
            </p>
        </section>
    }
}

#[component]
fn GuideAssistant() -> impl IntoView {
    view! {
        <section id="assistant" class="guide-section">
            <h2>"AI Assistant"</h2>
            <p>
                "The AI Assistant is a built-in chat interface that lets you ask governance, risk, "
                "and compliance questions. It can use your engagement and AI system context to provide "
                "more relevant answers."
            </p>
            <h3>"Supported LLM Providers"</h3>
            <ul>
                <li><strong>"Ollama"</strong>" — local, free, private (recommended for sensitive work)"</li>
                <li><strong>"LM Studio"</strong>" — local desktop LLM runner"</li>
                <li><strong>"OpenAI"</strong>" — cloud API (requires API key)"</li>
                <li><strong>"Anthropic"</strong>" — cloud API (requires API key)"</li>
                <li><strong>"Gemini"</strong>" — cloud API (requires API key)"</li>
            </ul>
            <h3>"Using the Assistant"</h3>
            <ol>
                <li>"First, configure your LLM provider in Settings (see Settings section)"</li>
                <li>"Navigate to AI Assistant from the sidebar"</li>
                <li>"Optionally select an engagement and AI system for context"</li>
                <li>"Type your question and press Enter or click Send"</li>
                <li>"The assistant will respond with relevant guidance"</li>
            </ol>
            <h3>"Good Questions to Ask"</h3>
            <ul>
                <li>"\"What are the key transparency requirements under the EU AI Act for high-risk credit scoring systems?\""</li>
                <li>"\"How should I document human oversight measures for Article 14 compliance?\""</li>
                <li>"\"What evidence do I need to demonstrate ISO 42001 Clause 8.4 (AI impact assessment) compliance?\""</li>
                <li>"\"Summarize the relationship between ISO 23894 risk assessment and EU AI Act Article 9\""</li>
                <li>"\"Draft a remediation plan for insufficient model explainability\""</li>
            </ul>
            <h3>"Important Note"</h3>
            <p>
                "The AI Assistant provides guidance and suggestions only — its outputs are not "
                "authoritative compliance determinations. Always verify LLM-generated content against "
                "the actual regulatory text and apply professional judgment."
            </p>
        </section>
    }
}

#[component]
fn GuideSettings() -> impl IntoView {
    view! {
        <section id="settings" class="guide-section">
            <h2>"Settings"</h2>
            <p>
                "The Settings page lets you configure the AI Assistant's LLM connection and view "
                "system information."
            </p>
            <h3>"LLM Configuration"</h3>
            <ul>
                <li><strong>"Provider"</strong>" — select from Ollama, LM Studio, OpenAI, Anthropic, or Gemini"</li>
                <li><strong>"Model Name"</strong>" — the specific model to use (e.g., \"llama3.1\" for Ollama, \"gpt-4o\" for OpenAI)"</li>
                <li><strong>"API Key"</strong>" — required for cloud providers (OpenAI, Anthropic, Gemini); not needed for local providers"</li>
                <li><strong>"Evidence Storage Path"</strong>" — the local directory where uploaded evidence files are stored"</li>
            </ul>
            <h3>"Setting Up a Local LLM (Recommended)"</h3>
            <ol>
                <li>"Install Ollama from https://ollama.ai"</li>
                <li>"Open a terminal and run: ollama pull llama3.1"</li>
                <li>"In Settings, select Provider: Ollama and Model: llama3.1"</li>
                <li>"Save — the AI Assistant is now ready to use with full privacy"</li>
            </ol>
            <h3>"System Information"</h3>
            <p>
                "The bottom of the Settings page displays:"
            </p>
            <ul>
                <li>"Application version"</li>
                <li>"Database connection details"</li>
                <li>"Tech stack information"</li>
            </ul>
        </section>
    }
}

#[component]
fn GuideWorkflow() -> impl IntoView {
    view! {
        <section id="workflow" class="guide-section">
            <h2>"End-to-End Workflow"</h2>
            <p>
                "This section walks through a complete assessment from start to finish, using the "
                "example of assessing a credit scoring AI system for a regional bank."
            </p>
            <h3>"Phase 1: Setup"</h3>
            <ol>
                <li><strong>"Create the Engagement"</strong>" — \"Northwind Community Bank — Credit Decisioning Review 2026\""</li>
                <li>"Set the scoping fields: Deployer, Finance, EU jurisdiction, Credit Scoring use case"</li>
                <li>"Accept the auto-suggested frameworks: EU AI Act, ISO 42001, ISO 23894, NIST AI RMF"</li>
                <li><strong>"Add the AI System"</strong>" — \"Nimbus Credit Decision Engine v2.4\""</li>
                <li>"Classify as High Risk, Annex III, non-GPAI"</li>
            </ol>
            <h3>"Phase 2: Evidence Collection"</h3>
            <ol>
                <li>"Upload all available documentation to the Evidence Vault"</li>
                <li>"Tag and categorize each item (model card → Technical, policy → Policy, etc.)"</li>
                <li>"Identify evidence gaps early — missing documentation is itself a finding"</li>
            </ol>
            <h3>"Phase 3: Framework Assessment"</h3>
            <ol>
                <li>"Open the Framework Navigator"</li>
                <li>"Start with the EU AI Act (the binding regulation)"</li>
                <li>"Work through each requirement systematically:"</li>
                <li class="guide-indent">"Read the requirement and guidance"</li>
                <li class="guide-indent">"Check the cross-references to understand the broader context"</li>
                <li class="guide-indent">"Review available evidence"</li>
                <li class="guide-indent">"Record your assessment (Met/Partial/Gap) with findings"</li>
                <li class="guide-indent">"For Partial/Gap, write a remediation plan"</li>
                <li>"Move to ISO 42001 — leverage cross-references to reuse EU AI Act findings where applicable"</li>
                <li>"Complete ISO 23894 and NIST AI RMF"</li>
            </ol>
            <h3>"Phase 4: FRIA & Risk Assessment"</h3>
            <ol>
                <li>"Complete the FRIA for the credit scoring system (required for high-risk deployers)"</li>
                <li>"Open the Risk Matrix and register all identified risks"</li>
                <li>"Score each risk (inherent likelihood × impact)"</li>
                <li>"Document mitigations and residual scores"</li>
                <li>"Link evidence to risks (bias testing report → discrimination risk)"</li>
            </ol>
            <h3>"Phase 5: Remediation & Tasks"</h3>
            <ol>
                <li>"Review the Gap Analysis page for a consolidated view of all issues"</li>
                <li>"Create tasks for each required remediation"</li>
                <li>"Assign target dates and track progress"</li>
                <li>"As remediations are completed, update assessments and attach new evidence"</li>
            </ol>
            <h3>"Phase 6: Reporting"</h3>
            <ol>
                <li>"Generate the Compliance Report for a full status overview"</li>
                <li>"Generate the Gap Analysis Report for remediation tracking"</li>
                <li>"Generate the Risk Report for the risk register and heatmap"</li>
                <li>"Review the Audit Trail to verify all activities are logged"</li>
            </ol>
        </section>
    }
}
