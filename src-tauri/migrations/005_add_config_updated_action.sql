-- Add 'config_updated' to audit_log action CHECK constraint
ALTER TABLE audit_log DROP CONSTRAINT IF EXISTS audit_log_action_check;

ALTER TABLE audit_log ADD CONSTRAINT audit_log_action_check
    CHECK (action IN (
        'created','updated','status_changed','evidence_attached',
        'evidence_detached','assessment_recorded','report_generated',
        'cross_reference_mapped','risk_scored','deleted',
        'llm_queried','system_exported','config_updated'
    ));
