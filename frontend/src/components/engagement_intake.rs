use serde::Serialize;

pub const ROLE_OPTIONS: &[(&str, &str)] = &[
    ("provider", "Provider"),
    ("deployer", "Deployer"),
    ("importer", "Importer"),
    ("distributor", "Distributor"),
    ("authorized_representative", "Authorized Representative"),
    ("product_manufacturer", "Product Manufacturer"),
];

pub const INDUSTRY_OPTIONS: &[(&str, &str)] = &[
    ("general", "General"),
    ("finance", "Finance"),
    ("healthcare", "Healthcare"),
    ("hr_employment", "HR / Employment"),
    ("retail_ecommerce", "Retail / E-Commerce"),
    ("public_sector", "Public Sector"),
    ("critical_infrastructure", "Critical Infrastructure"),
    ("education", "Education"),
];

pub const OBJECTIVE_OPTIONS: &[(&str, &str)] = &[
    ("baseline_compliance_review", "Baseline Compliance Review"),
    ("fria", "Fundamental Rights Impact Assessment"),
    ("gap_analysis", "Gap Analysis"),
    ("internal_audit", "Internal Audit"),
    ("vendor_due_diligence", "Vendor Due Diligence"),
    ("post_market_monitoring", "Post-Market Monitoring"),
];

pub const USE_CASE_OPTIONS: &[(&str, &str)] = &[
    ("general_analytics", "General Analytics / Decision Support"),
    ("credit_scoring", "Credit Scoring / Underwriting"),
    ("employment_screening", "Employment Screening / Hiring"),
    ("biometric_identification", "Biometric Identification"),
    ("emotion_recognition", "Emotion Recognition"),
    ("recommender_personalization", "Recommender / Personalization"),
    ("conversational_assistant", "Conversational Assistant"),
    ("fraud_detection", "Fraud Detection"),
    ("safety_component", "Safety Component"),
    ("generative_ai", "Generative AI"),
];

pub const PERSONAL_DATA_OPTIONS: &[(&str, &str)] = &[
    ("none", "No Personal Data"),
    ("personal_data", "Personal Data"),
    ("special_category_data", "Special Category / Sensitive Data"),
];

pub const JURISDICTION_OPTIONS: &[(&str, &str)] = &[
    ("eu", "European Union"),
    ("uk", "United Kingdom"),
    ("united_states", "United States"),
    ("canada", "Canada"),
    ("global", "Global / Multi-Region"),
];

pub fn enum_from_string<T>(value: &str) -> T
where
    T: for<'de> serde::Deserialize<'de>,
{
    serde_json::from_value(serde_json::Value::String(value.to_string())).unwrap()
}

pub fn enum_to_string<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(String::from))
        .unwrap_or_default()
}

pub fn toggle_string(values: &mut Vec<String>, value: &str, checked: bool) {
    if checked {
        if !values.iter().any(|item| item == value) {
            values.push(value.to_string());
        }
    } else {
        values.retain(|item| item != value);
    }
}