use tauri::Manager;

mod commands;
mod crypto;
mod db;
mod llm;
mod models;
mod reports;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(db::pool::create_pool())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Engagements
            commands::engagement_cmds::create_engagement,
            commands::engagement_cmds::list_engagements,
            commands::engagement_cmds::get_engagement,
            commands::engagement_cmds::update_engagement,
            commands::engagement_cmds::delete_engagement,
            // AI Systems
            commands::system_cmds::create_ai_system,
            commands::system_cmds::list_ai_systems,
            commands::system_cmds::get_ai_system,
            commands::system_cmds::update_ai_system,
            commands::system_cmds::delete_ai_system,
            // Requirements
            commands::requirement_cmds::list_requirements,
            commands::requirement_cmds::get_requirement,
            commands::requirement_cmds::search_requirements,
            // Assessments
            commands::assessment_cmds::upsert_assessment,
            commands::assessment_cmds::list_assessments,
            commands::assessment_cmds::get_assessment,
            // FRIA
            commands::fria_cmds::upsert_fria_assessment,
            commands::fria_cmds::get_fria_assessment,
            commands::fria_cmds::list_fria_assessments,
            // Cross-References
            commands::requirement_cmds::get_cross_references,
            commands::requirement_cmds::list_cross_references,
            // Risk
            commands::risk_cmds::create_risk_entry,
            commands::risk_cmds::list_risk_entries,
            commands::risk_cmds::update_risk_entry,
            commands::risk_cmds::delete_risk_entry,
            commands::risk_cmds::get_risk_matrix_data,
            // Evidence
            commands::evidence_cmds::pick_evidence_file,
            commands::evidence_cmds::upload_evidence,
            commands::evidence_cmds::list_evidence,
            commands::evidence_cmds::delete_evidence,
            commands::evidence_cmds::link_evidence,
            commands::evidence_cmds::unlink_evidence,
            commands::evidence_cmds::list_evidence_links,
            // Tasks
            commands::task_cmds::create_task,
            commands::task_cmds::list_tasks,
            commands::task_cmds::update_task,
            commands::task_cmds::delete_task,
            // Audit
            commands::audit_cmds::list_audit_log,
            // Reports
            commands::report_cmds::generate_report,
            // LLM
            commands::llm_cmds::query_llm,
            commands::llm_cmds::list_conversations,
            // Dashboard
            commands::dashboard_cmds::get_dashboard_stats,
            commands::dashboard_cmds::get_gap_analysis,
            // Config
            commands::config_cmds::get_config,
            commands::config_cmds::update_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
