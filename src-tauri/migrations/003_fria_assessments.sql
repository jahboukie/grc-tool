-- =============================================
-- GRC Command Center — 003_fria_assessments.sql
-- Fundamental Rights Impact Assessment workflow
-- =============================================

CREATE TABLE fria_assessments (
    id                              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    engagement_id                   UUID NOT NULL REFERENCES engagements(id) ON DELETE CASCADE,
    ai_system_id                    UUID NOT NULL UNIQUE REFERENCES ai_systems(id) ON DELETE CASCADE,
    status                          TEXT NOT NULL DEFAULT 'draft'
                                    CHECK (status IN ('draft','in_progress','completed','not_required')),
    scope_summary                   TEXT NOT NULL DEFAULT '',
    deployer_context                TEXT NOT NULL DEFAULT '',
    affected_persons_and_groups     TEXT NOT NULL DEFAULT '',
    vulnerable_groups               TEXT NOT NULL DEFAULT '',
    fundamental_rights_risks        TEXT NOT NULL DEFAULT '',
    human_oversight_measures        TEXT NOT NULL DEFAULT '',
    mitigation_measures             TEXT NOT NULL DEFAULT '',
    consultation_summary            TEXT NOT NULL DEFAULT '',
    conclusion                      TEXT NOT NULL DEFAULT '',
    authority_notification_status   TEXT NOT NULL DEFAULT 'not_started'
                                    CHECK (authority_notification_status IN ('not_started','pending','notified','not_required')),
    review_date                     DATE,
    related_risk_ids                UUID[] NOT NULL DEFAULT '{}',
    related_task_ids                UUID[] NOT NULL DEFAULT '{}',
    related_evidence_ids            UUID[] NOT NULL DEFAULT '{}',
    created_at                      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trg_fria_assessments_updated
    BEFORE UPDATE ON fria_assessments FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE INDEX idx_fria_assessments_engagement ON fria_assessments(engagement_id);
CREATE INDEX idx_fria_assessments_system ON fria_assessments(ai_system_id);