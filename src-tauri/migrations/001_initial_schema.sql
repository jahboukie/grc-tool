-- =============================================
-- GRC Command Center — 001_initial_schema.sql
-- Full DDL for all tables, constraints, indexes, triggers
-- =============================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================
-- TABLES (ordered by foreign key dependencies)
-- =============================================

CREATE TABLE engagements (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            TEXT NOT NULL,
    client_name     TEXT NOT NULL DEFAULT 'Personal',
    description     TEXT NOT NULL DEFAULT '',
    status          TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active','paused','completed','archived')),
    primary_role    TEXT NOT NULL DEFAULT 'provider'
                    CHECK (primary_role IN (
                        'provider','deployer','importer','distributor',
                        'authorized_representative','product_manufacturer'
                    )),
    frameworks      TEXT[] NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE ai_systems (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    engagement_id       UUID NOT NULL REFERENCES engagements(id) ON DELETE CASCADE,
    name                TEXT NOT NULL,
    description         TEXT NOT NULL DEFAULT '',
    intended_purpose    TEXT NOT NULL DEFAULT '',
    risk_category       TEXT NOT NULL DEFAULT 'minimal'
                        CHECK (risk_category IN ('unacceptable','high','limited','minimal','gpai')),
    domain              TEXT NOT NULL DEFAULT '',
    is_gpai             BOOLEAN NOT NULL DEFAULT FALSE,
    is_high_risk_listed BOOLEAN NOT NULL DEFAULT FALSE,
    is_safety_component BOOLEAN NOT NULL DEFAULT FALSE,
    deployment_context  TEXT NOT NULL DEFAULT '',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE framework_requirements (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    framework                   TEXT NOT NULL
                                CHECK (framework IN (
                                    'eu_ai_act','iso_42001','iso_23894',
                                    'nist_ai_rmf','oecd_ai_principles'
                                )),
    reference_id                TEXT NOT NULL UNIQUE,
    title                       TEXT NOT NULL,
    description                 TEXT NOT NULL DEFAULT '',
    article_clause              TEXT NOT NULL DEFAULT '',
    category                    TEXT NOT NULL DEFAULT '',
    subcategory                 TEXT,
    applicable_risk_categories  TEXT[] NOT NULL DEFAULT '{}',
    applicable_roles            TEXT[] NOT NULL DEFAULT '{}',
    is_mandatory                BOOLEAN NOT NULL DEFAULT TRUE,
    guidance_text               TEXT NOT NULL DEFAULT '',
    implementation_notes        TEXT NOT NULL DEFAULT '',
    sort_order                  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE requirement_assessments (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ai_system_id        UUID NOT NULL REFERENCES ai_systems(id) ON DELETE CASCADE,
    requirement_id      UUID NOT NULL REFERENCES framework_requirements(id) ON DELETE CASCADE,
    status              TEXT NOT NULL DEFAULT 'not_assessed'
                        CHECK (status IN ('not_assessed','met','partial','gap','not_applicable')),
    assessor_notes      TEXT NOT NULL DEFAULT '',
    remediation_plan    TEXT NOT NULL DEFAULT '',
    target_date         DATE,
    assessed_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(ai_system_id, requirement_id)
);

CREATE TABLE cross_references (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_requirement_id   UUID NOT NULL REFERENCES framework_requirements(id) ON DELETE CASCADE,
    target_requirement_id   UUID NOT NULL REFERENCES framework_requirements(id) ON DELETE CASCADE,
    relationship            TEXT NOT NULL
                            CHECK (relationship IN ('equivalent','overlapping','supports','extends')),
    notes                   TEXT NOT NULL DEFAULT '',
    UNIQUE(source_requirement_id, target_requirement_id)
);

CREATE TABLE risk_entries (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ai_system_id            UUID NOT NULL REFERENCES ai_systems(id) ON DELETE CASCADE,
    title                   TEXT NOT NULL,
    description             TEXT NOT NULL DEFAULT '',
    risk_source             TEXT NOT NULL DEFAULT '',
    affected_rights         TEXT[] NOT NULL DEFAULT '{}',
    likelihood              TEXT NOT NULL DEFAULT 'possible'
                            CHECK (likelihood IN ('rare','unlikely','possible','likely','almost_certain')),
    impact                  TEXT NOT NULL DEFAULT 'moderate'
                            CHECK (impact IN ('negligible','minor','moderate','major','catastrophic')),
    inherent_score          INTEGER NOT NULL DEFAULT 9,
    mitigation_measures     TEXT NOT NULL DEFAULT '',
    residual_likelihood     TEXT
                            CHECK (residual_likelihood IN ('rare','unlikely','possible','likely','almost_certain')),
    residual_impact         TEXT
                            CHECK (residual_impact IN ('negligible','minor','moderate','major','catastrophic')),
    residual_score          INTEGER,
    related_requirement_ids UUID[] NOT NULL DEFAULT '{}',
    status                  TEXT NOT NULL DEFAULT 'open'
                            CHECK (status IN ('open','in_progress','blocked','done','deferred')),
    priority                TEXT NOT NULL DEFAULT 'medium'
                            CHECK (priority IN ('critical','high','medium','low')),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tasks (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    engagement_id           UUID NOT NULL REFERENCES engagements(id) ON DELETE CASCADE,
    ai_system_id            UUID REFERENCES ai_systems(id) ON DELETE SET NULL,
    title                   TEXT NOT NULL,
    description             TEXT NOT NULL DEFAULT '',
    framework               TEXT
                            CHECK (framework IN (
                                'eu_ai_act','iso_42001','iso_23894',
                                'nist_ai_rmf','oecd_ai_principles'
                            )),
    related_requirement_id  UUID REFERENCES framework_requirements(id) ON DELETE SET NULL,
    status                  TEXT NOT NULL DEFAULT 'open'
                            CHECK (status IN ('open','in_progress','blocked','done','deferred')),
    priority                TEXT NOT NULL DEFAULT 'medium'
                            CHECK (priority IN ('critical','high','medium','low')),
    due_date                DATE,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE evidence (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    engagement_id       UUID NOT NULL REFERENCES engagements(id) ON DELETE CASCADE,
    file_name           TEXT NOT NULL,
    file_path           TEXT NOT NULL,
    file_size_bytes     BIGINT NOT NULL DEFAULT 0,
    mime_type           TEXT NOT NULL DEFAULT 'application/octet-stream',
    evidence_type       TEXT NOT NULL DEFAULT 'other'
                        CHECK (evidence_type IN (
                            'policy_document','technical_report','assessment_record',
                            'screenshot','attestation','audit_report','training_record',
                            'meeting_minutes','risk_register','conformity_declaration','other'
                        )),
    description         TEXT NOT NULL DEFAULT '',
    tags                TEXT[] NOT NULL DEFAULT '{}',
    uploaded_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE evidence_links (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    evidence_id                 UUID NOT NULL REFERENCES evidence(id) ON DELETE CASCADE,
    requirement_assessment_id   UUID REFERENCES requirement_assessments(id) ON DELETE SET NULL,
    risk_entry_id               UUID REFERENCES risk_entries(id) ON DELETE SET NULL,
    task_id                     UUID REFERENCES tasks(id) ON DELETE SET NULL
);

-- IMMUTABLE audit log — append only. Triggers below prevent UPDATE/DELETE.
CREATE TABLE audit_log (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     TEXT NOT NULL,
    entity_id       UUID NOT NULL,
    action          TEXT NOT NULL
                    CHECK (action IN (
                        'created','updated','status_changed','evidence_attached',
                        'evidence_detached','assessment_recorded','report_generated',
                        'cross_reference_mapped','risk_scored','deleted',
                        'llm_queried','system_exported'
                    )),
    field_changed   TEXT,
    old_value       TEXT,
    new_value       TEXT,
    details         TEXT NOT NULL DEFAULT '',
    timestamp       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE llm_conversations (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    engagement_id   UUID REFERENCES engagements(id) ON DELETE SET NULL,
    ai_system_id    UUID REFERENCES ai_systems(id) ON DELETE SET NULL,
    query           TEXT NOT NULL,
    response        TEXT NOT NULL,
    model_used      TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE app_config (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    llm_provider            TEXT NOT NULL DEFAULT 'openai',
    llm_api_key_encrypted   TEXT NOT NULL DEFAULT '',
    llm_model               TEXT NOT NULL DEFAULT 'gpt-4o',
    evidence_storage_path   TEXT NOT NULL DEFAULT '',
    db_host                 TEXT NOT NULL DEFAULT 'localhost',
    db_port                 INTEGER NOT NULL DEFAULT 5432,
    db_name                 TEXT NOT NULL DEFAULT 'grc_command_center',
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================
-- IMMUTABILITY: Prevent UPDATE/DELETE on audit_log
-- =============================================

CREATE OR REPLACE FUNCTION prevent_audit_mutation()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'audit_log is immutable: % operations are forbidden', TG_OP;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER audit_log_no_update
    BEFORE UPDATE ON audit_log
    FOR EACH ROW EXECUTE FUNCTION prevent_audit_mutation();

CREATE TRIGGER audit_log_no_delete
    BEFORE DELETE ON audit_log
    FOR EACH ROW EXECUTE FUNCTION prevent_audit_mutation();

-- =============================================
-- AUTO-UPDATE updated_at TRIGGERS
-- =============================================

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_engagements_updated
    BEFORE UPDATE ON engagements FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_ai_systems_updated
    BEFORE UPDATE ON ai_systems FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_requirement_assessments_updated
    BEFORE UPDATE ON requirement_assessments FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_risk_entries_updated
    BEFORE UPDATE ON risk_entries FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_tasks_updated
    BEFORE UPDATE ON tasks FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- =============================================
-- INDEXES
-- =============================================

CREATE INDEX idx_ai_systems_engagement ON ai_systems(engagement_id);
CREATE INDEX idx_req_assessments_system ON requirement_assessments(ai_system_id);
CREATE INDEX idx_req_assessments_requirement ON requirement_assessments(requirement_id);
CREATE INDEX idx_fw_requirements_framework ON framework_requirements(framework);
CREATE INDEX idx_fw_requirements_category ON framework_requirements(framework, category);
CREATE INDEX idx_cross_refs_source ON cross_references(source_requirement_id);
CREATE INDEX idx_cross_refs_target ON cross_references(target_requirement_id);
CREATE INDEX idx_risk_entries_system ON risk_entries(ai_system_id);
CREATE INDEX idx_evidence_engagement ON evidence(engagement_id);
CREATE INDEX idx_evidence_links_evidence ON evidence_links(evidence_id);
CREATE INDEX idx_tasks_engagement ON tasks(engagement_id);
CREATE INDEX idx_tasks_system ON tasks(ai_system_id);
CREATE INDEX idx_audit_log_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_timestamp ON audit_log(timestamp DESC);
CREATE INDEX idx_llm_convos_engagement ON llm_conversations(engagement_id);

-- =============================================
-- SEED DEFAULT CONFIG (singleton row)
-- =============================================

INSERT INTO app_config (id, evidence_storage_path, db_name)
VALUES (uuid_generate_v4(), '', 'grc_command_center');
