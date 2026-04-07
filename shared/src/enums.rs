use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────
// Regulatory Framework
// ─────────────────────────────────────────────

/// Which regulatory/standards framework a requirement belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Framework {
    EuAiAct,
    #[serde(rename = "iso_42001")]
    Iso42001,
    #[serde(rename = "iso_23894")]
    Iso23894,
    NistAiRmf,
    OecdAiPrinciples,
}

impl Framework {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::EuAiAct => "EU AI Act",
            Self::Iso42001 => "ISO/IEC 42001",
            Self::Iso23894 => "ISO/IEC 23894",
            Self::NistAiRmf => "NIST AI RMF",
            Self::OecdAiPrinciples => "OECD AI Principles",
        }
    }

    pub fn all() -> &'static [Framework] {
        &[
            Self::EuAiAct,
            Self::Iso42001,
            Self::Iso23894,
            Self::NistAiRmf,
            Self::OecdAiPrinciples,
        ]
    }
}

// ─────────────────────────────────────────────
// Risk Classification (EU AI Act)
// ─────────────────────────────────────────────

/// EU AI Act risk tier. Determines which obligations apply.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RiskCategory {
    Unacceptable,
    High,
    Limited,
    Minimal,
    Gpai,
}

impl RiskCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Unacceptable => "Unacceptable (Prohibited)",
            Self::High => "High Risk",
            Self::Limited => "Limited Risk",
            Self::Minimal => "Minimal Risk",
            Self::Gpai => "GPAI",
        }
    }
}

// ─────────────────────────────────────────────
// Compliance Status
// ─────────────────────────────────────────────

/// Assessment status for a requirement against a specific AI system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    NotAssessed,
    Met,
    Partial,
    Gap,
    NotApplicable,
}

impl ComplianceStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::NotAssessed => "Not Assessed",
            Self::Met => "Met",
            Self::Partial => "Partial",
            Self::Gap => "Gap",
            Self::NotApplicable => "N/A",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::NotAssessed => "status-not-assessed",
            Self::Met => "status-met",
            Self::Partial => "status-partial",
            Self::Gap => "status-gap",
            Self::NotApplicable => "status-na",
        }
    }
}

// ─────────────────────────────────────────────
// FRIA Status
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FriaStatus {
    Draft,
    InProgress,
    Completed,
    NotRequired,
}

impl FriaStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
            Self::NotRequired => "Not Required",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FriaNotificationStatus {
    NotStarted,
    Pending,
    Notified,
    NotRequired,
}

impl FriaNotificationStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::NotStarted => "Not Started",
            Self::Pending => "Pending",
            Self::Notified => "Notified",
            Self::NotRequired => "Not Required",
        }
    }
}

// ─────────────────────────────────────────────
// Task Status
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Open,
    InProgress,
    Blocked,
    Done,
    Deferred,
}

// ─────────────────────────────────────────────
// Priority
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

// ─────────────────────────────────────────────
// Engagement Status
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngagementStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

// ─────────────────────────────────────────────
// Evidence Type
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    PolicyDocument,
    TechnicalReport,
    AssessmentRecord,
    Screenshot,
    Attestation,
    AuditReport,
    TrainingRecord,
    MeetingMinutes,
    RiskRegister,
    ConformityDeclaration,
    Other,
}

// ─────────────────────────────────────────────
// Risk Likelihood (1–5)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RiskLikelihood {
    Rare,
    Unlikely,
    Possible,
    Likely,
    AlmostCertain,
}

impl RiskLikelihood {
    pub fn value(&self) -> i32 {
        match self {
            Self::Rare => 1,
            Self::Unlikely => 2,
            Self::Possible => 3,
            Self::Likely => 4,
            Self::AlmostCertain => 5,
        }
    }
}

// ─────────────────────────────────────────────
// Risk Impact (1–5)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RiskImpact {
    Negligible,
    Minor,
    Moderate,
    Major,
    Catastrophic,
}

impl RiskImpact {
    pub fn value(&self) -> i32 {
        match self {
            Self::Negligible => 1,
            Self::Minor => 2,
            Self::Moderate => 3,
            Self::Major => 4,
            Self::Catastrophic => 5,
        }
    }
}

/// Compute risk score from likelihood × impact.
pub fn risk_score(likelihood: &RiskLikelihood, impact: &RiskImpact) -> i32 {
    likelihood.value() * impact.value()
}

/// Risk severity label from score.
pub fn risk_severity(score: i32) -> &'static str {
    match score {
        1..=4 => "Low",
        5..=9 => "Medium",
        10..=14 => "High",
        15..=19 => "Very High",
        20..=25 => "Critical",
        _ => "Unknown",
    }
}

// ─────────────────────────────────────────────
// Obligation Role (EU AI Act value chain)
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObligationRole {
    Provider,
    Deployer,
    Importer,
    Distributor,
    AuthorizedRepresentative,
    ProductManufacturer,
}

impl ObligationRole {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Provider => "Provider",
            Self::Deployer => "Deployer",
            Self::Importer => "Importer",
            Self::Distributor => "Distributor",
            Self::AuthorizedRepresentative => "Authorized Representative",
            Self::ProductManufacturer => "Product Manufacturer",
        }
    }
}

// ─────────────────────────────────────────────
// Engagement Scoping Enums
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IndustrySector {
    General,
    Finance,
    Healthcare,
    HrEmployment,
    RetailEcommerce,
    PublicSector,
    CriticalInfrastructure,
    Education,
}

impl IndustrySector {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Finance => "Finance",
            Self::Healthcare => "Healthcare",
            Self::HrEmployment => "HR / Employment",
            Self::RetailEcommerce => "Retail / E-Commerce",
            Self::PublicSector => "Public Sector",
            Self::CriticalInfrastructure => "Critical Infrastructure",
            Self::Education => "Education",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Jurisdiction {
    Eu,
    Uk,
    UnitedStates,
    Canada,
    Global,
}

impl Jurisdiction {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Eu => "European Union",
            Self::Uk => "United Kingdom",
            Self::UnitedStates => "United States",
            Self::Canada => "Canada",
            Self::Global => "Global / Multi-Region",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceObjective {
    BaselineComplianceReview,
    Fria,
    GapAnalysis,
    InternalAudit,
    VendorDueDiligence,
    PostMarketMonitoring,
}

impl AssuranceObjective {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::BaselineComplianceReview => "Baseline Compliance Review",
            Self::Fria => "Fundamental Rights Impact Assessment",
            Self::GapAnalysis => "Gap Analysis",
            Self::InternalAudit => "Internal Audit",
            Self::VendorDueDiligence => "Vendor Due Diligence",
            Self::PostMarketMonitoring => "Post-Market Monitoring",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiUseCase {
    GeneralAnalytics,
    CreditScoring,
    EmploymentScreening,
    BiometricIdentification,
    EmotionRecognition,
    RecommenderPersonalization,
    ConversationalAssistant,
    FraudDetection,
    SafetyComponent,
    GenerativeAi,
}

impl AiUseCase {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::GeneralAnalytics => "General Analytics / Decision Support",
            Self::CreditScoring => "Credit Scoring / Underwriting",
            Self::EmploymentScreening => "Employment Screening / Hiring",
            Self::BiometricIdentification => "Biometric Identification",
            Self::EmotionRecognition => "Emotion Recognition",
            Self::RecommenderPersonalization => "Recommender / Personalization",
            Self::ConversationalAssistant => "Conversational Assistant",
            Self::FraudDetection => "Fraud Detection",
            Self::SafetyComponent => "Safety Component",
            Self::GenerativeAi => "Generative AI",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PersonalDataProfile {
    None,
    PersonalData,
    SpecialCategoryData,
}

impl PersonalDataProfile {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "No Personal Data",
            Self::PersonalData => "Personal Data",
            Self::SpecialCategoryData => "Special Category / Sensitive Data",
        }
    }
}

// ─────────────────────────────────────────────
// Audit Action
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    Created,
    Updated,
    StatusChanged,
    EvidenceAttached,
    EvidenceDetached,
    AssessmentRecorded,
    ReportGenerated,
    CrossReferenceMapped,
    RiskScored,
    Deleted,
    LlmQueried,
    SystemExported,
}

// ─────────────────────────────────────────────
// Cross-Reference Relationship
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CrossRefRelationship {
    Equivalent,
    Overlapping,
    Supports,
    Extends,
}

impl CrossRefRelationship {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Equivalent => "Equivalent",
            Self::Overlapping => "Overlapping",
            Self::Supports => "Supports",
            Self::Extends => "Extends",
        }
    }
}
