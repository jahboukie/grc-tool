-- =============================================
-- GRC Command Center — 004_engagement_intake_scoping.sql
-- Structured engagement intake fields for deterministic scoping
-- =============================================

ALTER TABLE engagements
    ADD COLUMN industry_sector TEXT NOT NULL DEFAULT 'general'
        CHECK (industry_sector IN (
            'general','finance','healthcare','hr_employment',
            'retail_ecommerce','public_sector','critical_infrastructure','education'
        )),
    ADD COLUMN jurisdictions TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN assurance_objective TEXT NOT NULL DEFAULT 'baseline_compliance_review'
        CHECK (assurance_objective IN (
            'baseline_compliance_review','fria','gap_analysis',
            'internal_audit','vendor_due_diligence','post_market_monitoring'
        )),
    ADD COLUMN ai_use_case TEXT NOT NULL DEFAULT 'general_analytics'
        CHECK (ai_use_case IN (
            'general_analytics','credit_scoring','employment_screening',
            'biometric_identification','emotion_recognition','recommender_personalization',
            'conversational_assistant','fraud_detection','safety_component','generative_ai'
        )),
    ADD COLUMN personal_data_profile TEXT NOT NULL DEFAULT 'personal_data'
        CHECK (personal_data_profile IN ('none','personal_data','special_category_data')),
    ADD COLUMN involves_vulnerable_groups BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN is_public_facing BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX idx_engagements_industry_sector ON engagements(industry_sector);