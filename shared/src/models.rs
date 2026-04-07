use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::*;

// ─────────────────────────────────────────────
// Engagement
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engagement {
    pub id: Uuid,
    pub name: String,
    pub client_name: String,
    pub description: String,
    pub status: EngagementStatus,
    pub primary_role: ObligationRole,
    pub industry_sector: IndustrySector,
    pub jurisdictions: Vec<Jurisdiction>,
    pub assurance_objective: AssuranceObjective,
    pub ai_use_case: AiUseCase,
    pub personal_data_profile: PersonalDataProfile,
    pub involves_vulnerable_groups: bool,
    pub is_public_facing: bool,
    pub frameworks: Vec<Framework>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEngagementDto {
    pub name: String,
    pub client_name: String,
    pub description: String,
    pub primary_role: ObligationRole,
    pub industry_sector: IndustrySector,
    pub jurisdictions: Vec<Jurisdiction>,
    pub assurance_objective: AssuranceObjective,
    pub ai_use_case: AiUseCase,
    pub personal_data_profile: PersonalDataProfile,
    pub involves_vulnerable_groups: bool,
    pub is_public_facing: bool,
    pub frameworks: Vec<Framework>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEngagementDto {
    pub name: Option<String>,
    pub client_name: Option<String>,
    pub description: Option<String>,
    pub status: Option<EngagementStatus>,
    pub primary_role: Option<ObligationRole>,
    pub industry_sector: Option<IndustrySector>,
    pub jurisdictions: Option<Vec<Jurisdiction>>,
    pub assurance_objective: Option<AssuranceObjective>,
    pub ai_use_case: Option<AiUseCase>,
    pub personal_data_profile: Option<PersonalDataProfile>,
    pub involves_vulnerable_groups: Option<bool>,
    pub is_public_facing: Option<bool>,
    pub frameworks: Option<Vec<Framework>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrameworkSuggestion {
    pub framework: Framework,
    pub reason: String,
}

fn push_framework_suggestion(
    suggestions: &mut Vec<FrameworkSuggestion>,
    framework: Framework,
    reason: impl Into<String>,
) {
    if !suggestions.iter().any(|item| item.framework == framework) {
        suggestions.push(FrameworkSuggestion {
            framework,
            reason: reason.into(),
        });
    }
}

pub fn suggest_frameworks_for_scope(
    primary_role: &ObligationRole,
    industry_sector: &IndustrySector,
    jurisdictions: &[Jurisdiction],
    assurance_objective: &AssuranceObjective,
    ai_use_case: &AiUseCase,
    personal_data_profile: &PersonalDataProfile,
    involves_vulnerable_groups: bool,
    is_public_facing: bool,
) -> Vec<FrameworkSuggestion> {
    let mut suggestions = Vec::new();

    if jurisdictions.contains(&Jurisdiction::Eu) {
        push_framework_suggestion(
            &mut suggestions,
            Framework::EuAiAct,
            "EU jurisdiction selected, so EU AI Act obligations need to be checked.",
        );
    }

    if matches!(primary_role, ObligationRole::Provider | ObligationRole::Deployer)
        || matches!(
            assurance_objective,
            AssuranceObjective::BaselineComplianceReview
                | AssuranceObjective::InternalAudit
                | AssuranceObjective::VendorDueDiligence
                | AssuranceObjective::PostMarketMonitoring
        )
    {
        push_framework_suggestion(
            &mut suggestions,
            Framework::Iso42001,
            "The intake indicates a governance or management-system review, which maps well to ISO/IEC 42001.",
        );
    }

    if matches!(
        assurance_objective,
        AssuranceObjective::BaselineComplianceReview
            | AssuranceObjective::Fria
            | AssuranceObjective::GapAnalysis
            | AssuranceObjective::PostMarketMonitoring
    ) || matches!(
        ai_use_case,
        AiUseCase::CreditScoring
            | AiUseCase::EmploymentScreening
            | AiUseCase::BiometricIdentification
            | AiUseCase::EmotionRecognition
            | AiUseCase::SafetyComponent
    ) || !matches!(personal_data_profile, PersonalDataProfile::None)
        || involves_vulnerable_groups
    {
        push_framework_suggestion(
            &mut suggestions,
            Framework::Iso23894,
            "The scope points to a structured AI risk assessment and treatment workflow, which ISO/IEC 23894 supports directly.",
        );
    }

    if matches!(
        industry_sector,
        IndustrySector::Finance
            | IndustrySector::Healthcare
            | IndustrySector::HrEmployment
            | IndustrySector::PublicSector
            | IndustrySector::CriticalInfrastructure
    ) || matches!(
        assurance_objective,
        AssuranceObjective::GapAnalysis
            | AssuranceObjective::VendorDueDiligence
            | AssuranceObjective::PostMarketMonitoring
            | AssuranceObjective::InternalAudit
    ) || is_public_facing
    {
        push_framework_suggestion(
            &mut suggestions,
            Framework::NistAiRmf,
            "The intake suggests a broader governance and risk-management lens, so NIST AI RMF is a useful companion framework.",
        );
    }

    if is_public_facing
        || involves_vulnerable_groups
        || matches!(
            ai_use_case,
            AiUseCase::GenerativeAi
                | AiUseCase::ConversationalAssistant
                | AiUseCase::RecommenderPersonalization
                | AiUseCase::EmploymentScreening
                | AiUseCase::CreditScoring
                | AiUseCase::EmotionRecognition
        )
    {
        push_framework_suggestion(
            &mut suggestions,
            Framework::OecdAiPrinciples,
            "The intake indicates a need for a public-interest or trustworthy-AI overlay, which the OECD AI Principles provide.",
        );
    }

    if suggestions.is_empty() {
        push_framework_suggestion(
            &mut suggestions,
            Framework::Iso42001,
            "Defaulting to ISO/IEC 42001 as the baseline AI governance management-system framework.",
        );
        push_framework_suggestion(
            &mut suggestions,
            Framework::Iso23894,
            "Defaulting to ISO/IEC 23894 as the baseline AI risk-management framework.",
        );
    }

    suggestions
}

// ─────────────────────────────────────────────
// AI System
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSystem {
    pub id: Uuid,
    pub engagement_id: Uuid,
    pub name: String,
    pub description: String,
    pub intended_purpose: String,
    pub risk_category: RiskCategory,
    pub domain: String,
    pub is_gpai: bool,
    pub is_high_risk_listed: bool,
    pub is_safety_component: bool,
    pub deployment_context: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAiSystemDto {
    pub engagement_id: Uuid,
    pub name: String,
    pub description: String,
    pub intended_purpose: String,
    pub risk_category: RiskCategory,
    pub domain: String,
    pub is_gpai: bool,
    pub is_high_risk_listed: bool,
    pub is_safety_component: bool,
    pub deployment_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAiSystemDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub intended_purpose: Option<String>,
    pub risk_category: Option<RiskCategory>,
    pub domain: Option<String>,
    pub is_gpai: Option<bool>,
    pub is_high_risk_listed: Option<bool>,
    pub is_safety_component: Option<bool>,
    pub deployment_context: Option<String>,
}

// ─────────────────────────────────────────────
// Framework Requirement (seed data — read-only)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkRequirement {
    pub id: Uuid,
    pub framework: Framework,
    pub reference_id: String,
    pub title: String,
    pub description: String,
    pub article_clause: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub applicable_risk_categories: Vec<RiskCategory>,
    pub applicable_roles: Vec<ObligationRole>,
    pub is_mandatory: bool,
    pub guidance_text: String,
    pub implementation_notes: String,
    pub sort_order: i32,
}

// ─────────────────────────────────────────────
// Requirement Assessment
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementAssessment {
    pub id: Uuid,
    pub ai_system_id: Uuid,
    pub requirement_id: Uuid,
    pub status: ComplianceStatus,
    pub assessor_notes: String,
    pub remediation_plan: String,
    pub target_date: Option<NaiveDate>,
    pub assessed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frameworks_for(suggestions: Vec<FrameworkSuggestion>) -> Vec<Framework> {
        suggestions.into_iter().map(|item| item.framework).collect()
    }

    #[test]
    fn suggests_expected_frameworks_for_eu_credit_fria_scope() {
        let frameworks = frameworks_for(suggest_frameworks_for_scope(
            &ObligationRole::Deployer,
            &IndustrySector::Finance,
            &[Jurisdiction::Eu, Jurisdiction::Uk],
            &AssuranceObjective::Fria,
            &AiUseCase::CreditScoring,
            &PersonalDataProfile::SpecialCategoryData,
            true,
            false,
        ));

        assert!(frameworks.contains(&Framework::EuAiAct));
        assert!(frameworks.contains(&Framework::Iso42001));
        assert!(frameworks.contains(&Framework::Iso23894));
        assert!(frameworks.contains(&Framework::NistAiRmf));
        assert!(frameworks.contains(&Framework::OecdAiPrinciples));
    }

    #[test]
    fn falls_back_to_iso_baselines_for_general_scope() {
        let frameworks = frameworks_for(suggest_frameworks_for_scope(
            &ObligationRole::AuthorizedRepresentative,
            &IndustrySector::General,
            &[],
            &AssuranceObjective::BaselineComplianceReview,
            &AiUseCase::GeneralAnalytics,
            &PersonalDataProfile::None,
            false,
            false,
        ));

        assert_eq!(frameworks.len(), 2);
        assert!(frameworks.contains(&Framework::Iso42001));
        assert!(frameworks.contains(&Framework::Iso23894));
    }

    #[test]
    fn public_facing_scope_adds_public_interest_overlays() {
        let frameworks = frameworks_for(suggest_frameworks_for_scope(
            &ObligationRole::Provider,
            &IndustrySector::RetailEcommerce,
            &[Jurisdiction::Global],
            &AssuranceObjective::VendorDueDiligence,
            &AiUseCase::ConversationalAssistant,
            &PersonalDataProfile::PersonalData,
            false,
            true,
        ));

        assert!(frameworks.contains(&Framework::Iso42001));
        assert!(frameworks.contains(&Framework::Iso23894));
        assert!(frameworks.contains(&Framework::NistAiRmf));
        assert!(frameworks.contains(&Framework::OecdAiPrinciples));
        assert!(!frameworks.contains(&Framework::EuAiAct));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertAssessmentDto {
    pub ai_system_id: Uuid,
    pub requirement_id: Uuid,
    pub status: ComplianceStatus,
    pub assessor_notes: String,
    pub remediation_plan: String,
    pub target_date: Option<NaiveDate>,
}

// ─────────────────────────────────────────────
// Fundamental Rights Impact Assessment (FRIA)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriaAssessment {
    pub id: Uuid,
    pub engagement_id: Uuid,
    pub ai_system_id: Uuid,
    pub status: FriaStatus,
    pub scope_summary: String,
    pub deployer_context: String,
    pub affected_persons_and_groups: String,
    pub vulnerable_groups: String,
    pub fundamental_rights_risks: String,
    pub human_oversight_measures: String,
    pub mitigation_measures: String,
    pub consultation_summary: String,
    pub conclusion: String,
    pub authority_notification_status: FriaNotificationStatus,
    pub review_date: Option<NaiveDate>,
    pub related_risk_ids: Vec<Uuid>,
    pub related_task_ids: Vec<Uuid>,
    pub related_evidence_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertFriaAssessmentDto {
    pub engagement_id: Uuid,
    pub ai_system_id: Uuid,
    pub status: FriaStatus,
    pub scope_summary: String,
    pub deployer_context: String,
    pub affected_persons_and_groups: String,
    pub vulnerable_groups: String,
    pub fundamental_rights_risks: String,
    pub human_oversight_measures: String,
    pub mitigation_measures: String,
    pub consultation_summary: String,
    pub conclusion: String,
    pub authority_notification_status: FriaNotificationStatus,
    pub review_date: Option<NaiveDate>,
    pub related_risk_ids: Vec<Uuid>,
    pub related_task_ids: Vec<Uuid>,
    pub related_evidence_ids: Vec<Uuid>,
}

// ─────────────────────────────────────────────
// Cross-Reference
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub id: Uuid,
    pub source_requirement_id: Uuid,
    pub target_requirement_id: Uuid,
    pub relationship: CrossRefRelationship,
    pub notes: String,
}

/// A cross-reference expanded with full requirement details on both sides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReferenceExpanded {
    pub id: Uuid,
    pub source: FrameworkRequirement,
    pub target: FrameworkRequirement,
    pub relationship: CrossRefRelationship,
    pub notes: String,
}

// ─────────────────────────────────────────────
// Risk Entry
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEntry {
    pub id: Uuid,
    pub ai_system_id: Uuid,
    pub title: String,
    pub description: String,
    pub risk_source: String,
    pub affected_rights: Vec<String>,
    pub likelihood: RiskLikelihood,
    pub impact: RiskImpact,
    pub inherent_score: i32,
    pub mitigation_measures: String,
    pub residual_likelihood: Option<RiskLikelihood>,
    pub residual_impact: Option<RiskImpact>,
    pub residual_score: Option<i32>,
    pub related_requirement_ids: Vec<Uuid>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRiskDto {
    pub ai_system_id: Uuid,
    pub title: String,
    pub description: String,
    pub risk_source: String,
    pub affected_rights: Vec<String>,
    pub likelihood: RiskLikelihood,
    pub impact: RiskImpact,
    pub mitigation_measures: String,
    pub residual_likelihood: Option<RiskLikelihood>,
    pub residual_impact: Option<RiskImpact>,
    pub related_requirement_ids: Vec<Uuid>,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRiskDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub risk_source: Option<String>,
    pub affected_rights: Option<Vec<String>>,
    pub likelihood: Option<RiskLikelihood>,
    pub impact: Option<RiskImpact>,
    pub mitigation_measures: Option<String>,
    pub residual_likelihood: Option<RiskLikelihood>,
    pub residual_impact: Option<RiskImpact>,
    pub related_requirement_ids: Option<Vec<Uuid>>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
}

// ─────────────────────────────────────────────
// Evidence
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub engagement_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub tags: Vec<String>,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadEvidenceDto {
    pub engagement_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceLink {
    pub id: Uuid,
    pub evidence_id: Uuid,
    pub requirement_assessment_id: Option<Uuid>,
    pub risk_entry_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkEvidenceDto {
    pub evidence_id: Uuid,
    pub requirement_assessment_id: Option<Uuid>,
    pub risk_entry_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
}

// ─────────────────────────────────────────────
// Task
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub engagement_id: Uuid,
    pub ai_system_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub framework: Option<Framework>,
    pub related_requirement_id: Option<Uuid>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskDto {
    pub engagement_id: Uuid,
    pub ai_system_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub framework: Option<Framework>,
    pub related_requirement_id: Option<Uuid>,
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub framework: Option<Framework>,
    pub related_requirement_id: Option<Uuid>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub due_date: Option<NaiveDate>,
}

// ─────────────────────────────────────────────
// Audit Log (immutable)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: AuditAction,
    pub field_changed: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub details: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFilterDto {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub action: Option<AuditAction>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ─────────────────────────────────────────────
// LLM Conversation
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConversation {
    pub id: Uuid,
    pub engagement_id: Option<Uuid>,
    pub ai_system_id: Option<Uuid>,
    pub query: String,
    pub response: String,
    pub model_used: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmQueryDto {
    pub engagement_id: Option<Uuid>,
    pub ai_system_id: Option<Uuid>,
    pub query: String,
}

// ─────────────────────────────────────────────
// App Config (singleton)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub id: Uuid,
    pub llm_provider: String,
    pub llm_api_key_encrypted: String,
    pub llm_model: String,
    pub evidence_storage_path: String,
    pub db_host: String,
    pub db_port: i32,
    pub db_name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigDto {
    pub llm_provider: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
    pub evidence_storage_path: Option<String>,
}

// ─────────────────────────────────────────────
// Dashboard / Aggregate Types
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub active_engagements: i64,
    pub total_ai_systems: i64,
    pub compliance_by_framework: Vec<FrameworkCompliance>,
    pub open_tasks: i64,
    pub open_risks: i64,
    pub total_gaps: i64,
    pub total_evidence: i64,
    pub fria_in_scope: i64,
    pub fria_completed: i64,
    pub priority_tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkCompliance {
    pub framework: Framework,
    pub total_applicable: i64,
    pub met: i64,
    pub partial: i64,
    pub gap: i64,
    pub not_assessed: i64,
    pub pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMatrixData {
    pub entries: Vec<RiskEntry>,
    pub cells: Vec<RiskMatrixCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMatrixCell {
    pub likelihood: i32,
    pub impact: i32,
    pub score: i32,
    pub risk_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysisData {
    pub frameworks: Vec<FrameworkGaps>,
    pub total_gaps: i64,
    pub total_partial: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkGaps {
    pub framework: Framework,
    pub framework_name: String,
    pub gaps: Vec<GapEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapEntry {
    pub requirement: FrameworkRequirement,
    pub assessment: RequirementAssessment,
    pub cross_references: Vec<CrossReference>,
}

// ─────────────────────────────────────────────
// Report Types
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    FullCompliance,
    GapAnalysis,
    RiskAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub report_type: ReportType,
    pub engagement_id: Uuid,
    pub ai_system_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResult {
    pub file_path: String,
    pub report_type: ReportType,
    pub generated_at: DateTime<Utc>,
}
