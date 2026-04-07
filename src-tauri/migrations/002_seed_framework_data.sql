-- =============================================
-- GRC Command Center — 002_seed_framework_data.sql
-- Seeds ~241 framework requirements + cross-references
-- =============================================

-- =============================================
-- EU AI ACT (~83 requirements)
-- =============================================

-- Prohibited Practices (Art. 5) — 8 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART5-1', 'Subliminal Manipulation Prohibition', 'AI system must not deploy subliminal techniques beyond a person''s consciousness to distort behavior causing significant harm.', 'Article 5(1)(a)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer,importer,distributor}', true, 'Screen the system for any technique that operates below the threshold of awareness to manipulate decisions.', 'Document screening results confirming the system does not use subliminal techniques.', 10),
('eu_ai_act', 'EU-AIA-ART5-2', 'Vulnerability Exploitation Prohibition', 'AI must not exploit vulnerabilities of persons due to age, disability, or social/economic situation.', 'Article 5(1)(b)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer,importer,distributor}', true, 'Identify if any user group could be exploited due to known vulnerabilities.', 'Perform and document a vulnerability exploitation assessment.', 20),
('eu_ai_act', 'EU-AIA-ART5-3', 'Social Scoring Prohibition', 'Prohibits AI-based social scoring by public authorities leading to detrimental treatment.', 'Article 5(1)(c)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Confirm the system does not evaluate or classify persons based on social behavior for punitive purposes.', 'Document that no social scoring functionality exists.', 30),
('eu_ai_act', 'EU-AIA-ART5-4', 'Real-Time Remote Biometric ID Prohibition', 'Prohibits real-time remote biometric identification in publicly accessible spaces for law enforcement (with exceptions).', 'Article 5(1)(d)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Determine if the system performs real-time biometric identification in public spaces.', 'Document that no prohibited biometric identification is performed, or that a valid exception applies.', 40),
('eu_ai_act', 'EU-AIA-ART5-5', 'Emotion Inference Prohibition (Workplace/Education)', 'Prohibits AI inferring emotions in workplace and educational settings except for medical/safety reasons.', 'Article 5(1)(e)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Check if the system infers emotions in workplace or educational contexts.', 'Document that emotion inference is not used or that a valid medical/safety exception applies.', 50),
('eu_ai_act', 'EU-AIA-ART5-6', 'Untargeted Facial Image Scraping Prohibition', 'Prohibits scraping facial images from the internet or CCTV to build facial recognition databases.', 'Article 5(1)(f)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Verify the system does not scrape facial images from untargeted sources.', 'Document data sourcing practices confirming no untargeted scraping.', 60),
('eu_ai_act', 'EU-AIA-ART5-7', 'Biometric Categorization Prohibition', 'Prohibits biometric categorization to infer race, political opinions, trade union membership, religion, sex life or orientation.', 'Article 5(1)(g)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Screen for any biometric-based classification of sensitive attributes.', 'Document that no prohibited biometric categorization is performed.', 70),
('eu_ai_act', 'EU-AIA-ART5-8', 'Individual Criminal Risk Assessment Prohibition', 'Prohibits AI systems making risk assessments of natural persons to predict criminal offences solely based on profiling.', 'Article 5(1)(h)', 'Prohibited Practices', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Verify the system does not predict criminal behavior based solely on profiling or personality traits.', 'Document that no predictive criminal profiling functionality exists.', 80);

-- Risk Classification (Art. 6) — 5 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART6-1', 'High-Risk Classification Assessment', 'Determine if the AI system falls under high-risk classification per Annex I or Annex III.', 'Article 6(1)-(2)', 'Risk Classification', '{unacceptable,high,limited,minimal,gpai}', '{provider,deployer}', true, 'Evaluate the system against Annex I (safety component) and Annex III (listed use cases) criteria.', 'Document the classification decision with supporting rationale.', 90),
('eu_ai_act', 'EU-AIA-ART6-2', 'Annex III Use Case Check', 'Verify whether the AI system is listed in Annex III high-risk use case areas.', 'Article 6(2), Annex III', 'Risk Classification', '{high}', '{provider}', true, 'Check all 8 areas of Annex III including biometrics, critical infrastructure, education, employment, essential services, law enforcement, migration, and justice.', 'Record which Annex III area applies and why.', 100),
('eu_ai_act', 'EU-AIA-ART6-3', 'Safety Component Assessment', 'Determine if the AI system is a safety component of a product covered by EU harmonization legislation in Annex I.', 'Article 6(1), Annex I', 'Risk Classification', '{high}', '{provider,product_manufacturer}', true, 'Review Annex I legislation list to determine if the system is a safety component.', 'Document the safety component determination.', 110),
('eu_ai_act', 'EU-AIA-ART6-4', 'Reclassification Exception', 'Assess if the Art. 6(3) exception for reclassification applies, allowing a high-risk listed system to be classified lower.', 'Article 6(3)', 'Risk Classification', '{high}', '{provider}', false, 'Evaluate if the system poses no significant risk to health, safety, or fundamental rights.', 'If claiming reclassification, document the justification and notify the relevant authority.', 120),
('eu_ai_act', 'EU-AIA-ART6-5', 'GPAI Model Classification', 'Determine if the AI system qualifies as a General Purpose AI model under Articles 51-56.', 'Article 3(63), Art. 51', 'Risk Classification', '{gpai}', '{provider}', true, 'Assess if the model was trained on broad data using self-supervision at scale and can serve a wide range of tasks.', 'Document the GPAI classification decision.', 130);

-- Risk Management System (Art. 9) — 6 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART9-1', 'Risk Management System Establishment', 'Establish, implement, document, and maintain a continuous risk management system throughout the AI system lifecycle.', 'Article 9(1)', 'Risk Management System', '{high}', '{provider}', true, 'The RMS must be a continuous iterative process running throughout the entire lifecycle.', 'Create a documented RMS with lifecycle coverage, reviewed at least annually.', 140),
('eu_ai_act', 'EU-AIA-ART9-2', 'Risk Identification and Analysis', 'Identify and analyze known and reasonably foreseeable risks to health, safety, and fundamental rights.', 'Article 9(2)(a)', 'Risk Management System', '{high}', '{provider}', true, 'Identify risks considering the intended purpose and reasonably foreseeable misuse.', 'Maintain a risk register with identified risks, analysis methodology, and severity ratings.', 150),
('eu_ai_act', 'EU-AIA-ART9-3', 'Risk Estimation and Evaluation', 'Estimate and evaluate risks arising from use and reasonably foreseeable misuse.', 'Article 9(2)(b)', 'Risk Management System', '{high}', '{provider}', true, 'Use structured risk estimation methodologies to evaluate identified risks.', 'Document risk estimation criteria, results, and scoring methodology.', 160),
('eu_ai_act', 'EU-AIA-ART9-4', 'Post-Market Risk Evaluation', 'Evaluate risks using post-market monitoring data after the system is placed on the market.', 'Article 9(2)(c)', 'Risk Management System', '{high}', '{provider}', true, 'Incorporate field data and user feedback into ongoing risk evaluation.', 'Establish processes to collect and analyze post-market data for risk updates.', 170),
('eu_ai_act', 'EU-AIA-ART9-5', 'Risk Management Measures', 'Adopt suitable risk management measures to address identified risks.', 'Article 9(2)(d)', 'Risk Management System', '{high}', '{provider}', true, 'Risk measures must eliminate or reduce risks through design, technical safeguards, or information to users.', 'Document risk treatment plans with responsible parties and timelines.', 180),
('eu_ai_act', 'EU-AIA-ART9-6', 'Pre-Market Testing', 'Ensure testing procedures are suitable for the AI system prior to placing on market.', 'Article 9(7)-(8)', 'Risk Management System', '{high}', '{provider}', true, 'Testing must use appropriate metrics and probabilistic thresholds.', 'Document testing methodology, results, and acceptance criteria.', 190);

-- Data & Data Governance (Art. 10) — 5 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART10-1', 'Training Data Governance', 'Training, validation, and testing datasets must be subject to appropriate data governance and management practices.', 'Article 10(1)-(2)', 'Data & Data Governance', '{high}', '{provider}', true, 'Establish data governance covering design choices, data collection, preparation, and relevant assumptions.', 'Document data governance policies and procedures for all datasets.', 200),
('eu_ai_act', 'EU-AIA-ART10-2', 'Data Quality Requirements', 'Datasets must be relevant, sufficiently representative, and to the best extent free of errors.', 'Article 10(3)', 'Data & Data Governance', '{high}', '{provider}', true, 'Ensure datasets meet quality criteria including relevance, representativeness, and accuracy.', 'Perform and document data quality assessments.', 210),
('eu_ai_act', 'EU-AIA-ART10-3', 'Dataset Characteristics Documentation', 'Document the statistical properties, limitations, and potential gaps of datasets.', 'Article 10(2)(f)', 'Data & Data Governance', '{high}', '{provider}', true, 'Describe statistical properties, shortcomings, and how gaps were addressed.', 'Create dataset documentation cards covering characteristics and limitations.', 220),
('eu_ai_act', 'EU-AIA-ART10-4', 'Bias Examination', 'Examine datasets for possible biases likely to affect health, safety, or fundamental rights.', 'Article 10(2)(f)-(g)', 'Data & Data Governance', '{high}', '{provider}', true, 'Actively check for biases related to protected characteristics and geographic/contextual factors.', 'Document bias examination methodology and findings with mitigation measures.', 230),
('eu_ai_act', 'EU-AIA-ART10-5', 'Special Category Data Processing', 'Processing of special categories of personal data must be strictly necessary for bias monitoring and detection.', 'Article 10(5)', 'Data & Data Governance', '{high}', '{provider}', true, 'Only process sensitive personal data when strictly necessary for bias detection with appropriate safeguards.', 'Document the necessity assessment and safeguards for any special category data processing.', 240);

-- Technical Documentation (Art. 11) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART11-1', 'Technical Documentation Preparation', 'Draw up technical documentation before the system is placed on the market, kept up to date.', 'Article 11(1)', 'Technical Documentation', '{high}', '{provider}', true, 'Technical documentation must demonstrate compliance and provide authorities with information to assess compliance.', 'Prepare documentation per Annex IV covering all required elements.', 250),
('eu_ai_act', 'EU-AIA-ART11-2', 'System Description Documentation', 'Document the general description of the AI system including intended purpose, provider details, and system architecture.', 'Annex IV(1)', 'Technical Documentation', '{high}', '{provider}', true, 'Include provider identity, system version, hardware/software requirements, and architecture description.', 'Create a system description document covering all Annex IV Section 1 elements.', 260),
('eu_ai_act', 'EU-AIA-ART11-3', 'Development Process Documentation', 'Document the development process including design specifications, data requirements, and training methodologies.', 'Annex IV(2)-(3)', 'Technical Documentation', '{high}', '{provider}', true, 'Cover design choices, system architecture, computational resources, and training/testing procedures.', 'Document design specs, training methods, and validation approaches.', 270),
('eu_ai_act', 'EU-AIA-ART11-4', 'Performance and Limitations Documentation', 'Document performance metrics, known limitations, and foreseeable risks.', 'Annex IV(4)', 'Technical Documentation', '{high}', '{provider}', true, 'Include metrics used, levels of accuracy, foreseeable unintended outcomes, and risk specifics.', 'Create performance documentation with metrics, limitations, and residual risks.', 280);

-- Record-Keeping & Logging (Art. 12) — 3 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART12-1', 'Automatic Logging Capability', 'High-risk AI systems must have automatic logging capabilities to record events during operation.', 'Article 12(1)', 'Record-Keeping & Logging', '{high}', '{provider}', true, 'Logging must enable traceability of the system functioning throughout its lifecycle.', 'Implement logging infrastructure capturing operational events with timestamps.', 290),
('eu_ai_act', 'EU-AIA-ART12-2', 'Log Content Requirements', 'Logs must record the period of use, reference database, input data, and identification of involved persons.', 'Article 12(2)-(3)', 'Record-Keeping & Logging', '{high}', '{provider}', true, 'Logs should capture sufficient detail for post-incident analysis and auditing.', 'Define log schema covering all required data points per Article 12.', 300),
('eu_ai_act', 'EU-AIA-ART12-3', 'Log Retention and Access', 'Logs must be retained for an appropriate period and be accessible to relevant authorities.', 'Article 12(4)', 'Record-Keeping & Logging', '{high}', '{provider,deployer}', true, 'Establish retention period proportionate to the intended purpose and applicable legal obligations.', 'Implement log retention policy with secure storage and access controls.', 310);

-- Transparency to Deployers (Art. 13) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART13-1', 'Transparency of Operation', 'High-risk AI systems must be designed to enable deployers to interpret output and use it appropriately.', 'Article 13(1)', 'Transparency to Deployers', '{high}', '{provider}', true, 'Design the system so deployers can understand its functioning and interpret outputs.', 'Implement explainability features and clear output presentation.', 320),
('eu_ai_act', 'EU-AIA-ART13-2', 'Instructions for Use', 'Provide deployers with instructions for use containing provider identity, system characteristics, capabilities, and limitations.', 'Article 13(3)(a)-(b)', 'Transparency to Deployers', '{high}', '{provider}', true, 'Instructions must include performance metrics, intended purpose, and known limitations.', 'Create comprehensive instructions for use document per Art. 13(3) checklist.', 330),
('eu_ai_act', 'EU-AIA-ART13-3', 'Performance Metrics Disclosure', 'Disclose the level of accuracy and relevant metrics for the specific persons or groups the system is intended for.', 'Article 13(3)(b)(ii)', 'Transparency to Deployers', '{high}', '{provider}', true, 'Accuracy metrics must be specific to target population and use context.', 'Document and disclose performance metrics with test dataset characteristics.', 340),
('eu_ai_act', 'EU-AIA-ART13-4', 'Known Limitations Communication', 'Communicate foreseeable misuse scenarios and known circumstances affecting accuracy.', 'Article 13(3)(b)(iv)', 'Transparency to Deployers', '{high}', '{provider}', true, 'Clearly describe conditions under which the system may underperform.', 'Document and communicate known limitations and failure modes.', 350);

-- Human Oversight (Art. 14) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART14-1', 'Human Oversight Design', 'Design the system to be effectively overseen by natural persons during its period of use.', 'Article 14(1)-(2)', 'Human Oversight', '{high}', '{provider}', true, 'Oversight measures must be identified and built into the system by the provider.', 'Design oversight interfaces enabling human intervention and decision override.', 360),
('eu_ai_act', 'EU-AIA-ART14-2', 'Oversight Interface Capability', 'Oversight persons must be able to fully understand capabilities and limitations and monitor operation.', 'Article 14(4)(a)-(b)', 'Human Oversight', '{high}', '{provider,deployer}', true, 'Ensure oversight personnel can properly monitor and understand system operation.', 'Provide monitoring dashboards and alerts for oversight personnel.', 370),
('eu_ai_act', 'EU-AIA-ART14-3', 'Intervention and Override Capability', 'Enable the human overseer to decide not to use the system, override, or reverse its output.', 'Article 14(4)(c)-(d)', 'Human Oversight', '{high}', '{provider,deployer}', true, 'Humans must be able to override, reverse, or stop the system at any time.', 'Implement stop, override, and revert mechanisms accessible to oversight personnel.', 380),
('eu_ai_act', 'EU-AIA-ART14-4', 'Automation Bias Mitigation', 'Address the risk of automation bias, especially for high-frequency or routine decisions.', 'Article 14(4)(e)', 'Human Oversight', '{high}', '{provider,deployer}', true, 'Design measures to help users avoid over-reliance on AI output.', 'Implement automation bias countermeasures such as confidence indicators and periodic prompts.', 390);

-- Accuracy, Robustness, Security (Art. 15) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART15-1', 'Accuracy Requirements', 'Achieve and maintain an appropriate level of accuracy for the intended purpose.', 'Article 15(1)', 'Accuracy, Robustness, Security', '{high}', '{provider}', true, 'Accuracy levels must be declared in instructions for use.', 'Define, measure, and document accuracy targets with validation results.', 400),
('eu_ai_act', 'EU-AIA-ART15-2', 'Robustness Against Errors', 'AI system must be resilient to errors, faults, and inconsistencies within the system or environment.', 'Article 15(2)-(3)', 'Accuracy, Robustness, Security', '{high}', '{provider}', true, 'Implement redundancy and fail-safe mechanisms for robust operation.', 'Test and document system robustness under adverse conditions.', 410),
('eu_ai_act', 'EU-AIA-ART15-3', 'Cybersecurity Resilience', 'Protect against attempts by unauthorized third parties to exploit system vulnerabilities.', 'Article 15(4)', 'Accuracy, Robustness, Security', '{high}', '{provider}', true, 'Address adversarial attacks including data poisoning, model manipulation, and adversarial inputs.', 'Conduct security testing including adversarial robustness assessment.', 420),
('eu_ai_act', 'EU-AIA-ART15-4', 'Feedback Loop Control', 'Address risks from feedback loops in continuously learning systems, including appropriate mitigation.', 'Article 15(5)', 'Accuracy, Robustness, Security', '{high}', '{provider}', true, 'Monitor and control feedback loops that could degrade performance over time.', 'Implement feedback loop monitoring and drift detection mechanisms.', 430);

-- Provider Obligations (Art. 16-21) — 6 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART16-1', 'Compliance System Establishment', 'Providers must ensure their high-risk AI systems comply with all Chapter III, Section 2 requirements.', 'Article 16(a)', 'Provider Obligations', '{high}', '{provider}', true, 'Establish a comprehensive compliance system covering all high-risk requirements.', 'Create a compliance management framework with responsibilities and controls.', 440),
('eu_ai_act', 'EU-AIA-ART16-2', 'Contact Information and Identification', 'Indicate provider name, registered trade name, and contact address on the system or packaging.', 'Article 16(b)-(c)', 'Provider Obligations', '{high}', '{provider}', true, 'Ensure clear identification of the provider on the system and documentation.', 'Include provider contact details on the system interface and in documentation.', 450),
('eu_ai_act', 'EU-AIA-ART16-3', 'CE Marking Affixation', 'Affix the CE marking to the AI system or its accompanying document.', 'Article 16(h), Art. 48', 'Provider Obligations', '{high}', '{provider}', true, 'CE marking must be affixed after successful conformity assessment.', 'Affix CE marking per Article 48 requirements after conformity assessment.', 460),
('eu_ai_act', 'EU-AIA-ART16-4', 'EU Declaration of Conformity', 'Draw up and maintain an EU declaration of conformity per Article 47.', 'Article 16(f), Art. 47', 'Provider Obligations', '{high}', '{provider}', true, 'Declaration must cover all required elements per Article 47.', 'Prepare EU Declaration of Conformity and keep it available for 10 years.', 470),
('eu_ai_act', 'EU-AIA-ART16-5', 'Registration in EU Database', 'Register the AI system in the EU database before placing on market.', 'Article 16(i), Art. 49', 'Provider Obligations', '{high}', '{provider}', true, 'Register the system in the EU database with required information per Article 49.', 'Complete EU database registration with all required fields.', 480),
('eu_ai_act', 'EU-AIA-ART16-6', 'Corrective Action Obligations', 'Take necessary corrective actions when the system is not in conformity, and inform authorities.', 'Article 16(j), Art. 20', 'Provider Obligations', '{high}', '{provider}', true, 'When non-conformity is identified, take immediate corrective action and notify authorities.', 'Establish corrective action procedures with notification protocols.', 490);

-- Deployer Obligations (Art. 26) — 5 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART26-1', 'Use in Accordance with Instructions', 'Use the high-risk AI system in accordance with the instructions for use accompanying the system.', 'Article 26(1)', 'Deployer Obligations', '{high}', '{deployer}', true, 'Follow provider instructions for use and monitor the system accordingly.', 'Document adherence to instructions for use and note any deviations.', 500),
('eu_ai_act', 'EU-AIA-ART26-2', 'Human Oversight Implementation', 'Assign human oversight to natural persons with necessary competence, training, and authority.', 'Article 26(2)', 'Deployer Obligations', '{high}', '{deployer}', true, 'Oversight personnel must have adequate competence and authority to intervene.', 'Designate and train human overseers with documented authority and competence records.', 510),
('eu_ai_act', 'EU-AIA-ART26-3', 'Input Data Relevance', 'Ensure that input data is relevant and sufficiently representative for the intended purpose.', 'Article 26(4)', 'Deployer Obligations', '{high}', '{deployer}', true, 'Verify input data quality is appropriate for the operational context.', 'Implement input data quality checks and document data relevance assessments.', 520),
('eu_ai_act', 'EU-AIA-ART26-4', 'Monitoring of Operation', 'Monitor the operation of the high-risk AI system based on instructions for use.', 'Article 26(5)', 'Deployer Obligations', '{high}', '{deployer}', true, 'Continuously monitor the system and report issues to the provider.', 'Implement operational monitoring and incident escalation procedures.', 530),
('eu_ai_act', 'EU-AIA-ART26-5', 'Log Retention by Deployer', 'Keep logs automatically generated by the system to the extent under their control for at least six months.', 'Article 26(6)', 'Deployer Obligations', '{high}', '{deployer}', true, 'Retain all system-generated logs for minimum 6 months unless otherwise specified.', 'Implement log retention with secure storage meeting the 6-month minimum.', 540);

-- Fundamental Rights Impact (Art. 27) — 3 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART27-1', 'FRIA Obligation', 'Conduct a fundamental rights impact assessment before deploying a high-risk AI system.', 'Article 27(1)', 'Fundamental Rights Impact', '{high}', '{deployer}', true, 'Required for public bodies and entities providing public services, credit scoring, health/life insurance AI.', 'Conduct FRIA using a structured template before first deployment.', 550),
('eu_ai_act', 'EU-AIA-ART27-2', 'FRIA Content Requirements', 'FRIA must include deployer processes, affected persons, specific risks, human oversight measures, and impact on vulnerable groups.', 'Article 27(2)-(3)', 'Fundamental Rights Impact', '{high}', '{deployer}', true, 'Cover all required elements including risk identification and vulnerable group analysis.', 'Complete all FRIA sections with supporting evidence and analysis.', 560),
('eu_ai_act', 'EU-AIA-ART27-3', 'FRIA Notification', 'Notify the national supervisory authority of the FRIA results.', 'Article 27(4)', 'Fundamental Rights Impact', '{high}', '{deployer}', true, 'Submit FRIA results to the relevant market surveillance authority.', 'File FRIA notification with the national authority per prescribed format.', 570);

-- Conformity Assessment (Art. 43) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART43-1', 'Conformity Assessment Procedure', 'Undergo a conformity assessment procedure before placing high-risk AI on the market.', 'Article 43(1)', 'Conformity Assessment', '{high}', '{provider}', true, 'Select appropriate procedure: internal control (Annex VI) or with notified body (Annex VII).', 'Complete conformity assessment per Article 43 and maintain all evidence.', 580),
('eu_ai_act', 'EU-AIA-ART43-2', 'Internal Control Assessment', 'Apply internal control procedure per Annex VI for most high-risk AI systems.', 'Article 43(2), Annex VI', 'Conformity Assessment', '{high}', '{provider}', true, 'Internal control must verify compliance with all Chapter III Section 2 requirements.', 'Document internal control assessment results per Annex VI checklist.', 590),
('eu_ai_act', 'EU-AIA-ART43-3', 'Notified Body Assessment', 'Use notified body assessment per Annex VII for biometric identification systems.', 'Article 43(1), Annex VII', 'Conformity Assessment', '{high}', '{provider}', true, 'Required for remote biometric identification systems — involves third-party audit.', 'Engage a notified body for assessment if the system uses biometric identification.', 600),
('eu_ai_act', 'EU-AIA-ART43-4', 'Re-Assessment on Modification', 'Repeat conformity assessment when the system is substantially modified.', 'Article 43(4)', 'Conformity Assessment', '{high}', '{provider}', true, 'Substantial modifications require a new conformity assessment.', 'Define substantial modification criteria and trigger re-assessment processes.', 610);

-- Quality Management System (Art. 17) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART17-1', 'QMS Establishment', 'Establish a quality management system ensuring compliance with the regulation.', 'Article 17(1)', 'Quality Management System', '{high}', '{provider}', true, 'QMS must be documented and systematic, proportionate to the size of the organization.', 'Create a documented QMS covering all Article 17(1) elements.', 620),
('eu_ai_act', 'EU-AIA-ART17-2', 'QMS Design and Development Procedures', 'QMS must include procedures for design, development, quality control, and quality assurance.', 'Article 17(1)(b)-(d)', 'Quality Management System', '{high}', '{provider}', true, 'Include examination, testing, and validation procedures before, during, and after development.', 'Document design, testing, and validation procedures within the QMS.', 630),
('eu_ai_act', 'EU-AIA-ART17-3', 'QMS Data Management Standards', 'QMS must include standards and procedures for data management per Article 10.', 'Article 17(1)(f)', 'Quality Management System', '{high}', '{provider}', true, 'Data management within QMS must cover collection, analysis, labelling, storage, and deletion.', 'Integrate data management standards into QMS documentation.', 640),
('eu_ai_act', 'EU-AIA-ART17-4', 'QMS Accountability Framework', 'QMS must include accountability framework with responsibilities across all relevant processes.', 'Article 17(1)(j)', 'Quality Management System', '{high}', '{provider}', true, 'Define clear accountability for compliance at each stage of the AI lifecycle.', 'Document roles, responsibilities, and accountability in the QMS.', 650);

-- Transparency — Limited Risk (Art. 50) — 4 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART50-1', 'AI Interaction Disclosure', 'Inform persons that they are interacting with an AI system unless obvious from context.', 'Article 50(1)', 'Transparency (Limited Risk)', '{limited,high}', '{provider,deployer}', true, 'Users must be clearly informed of AI interaction at the start of engagement.', 'Implement clear AI disclosure notice at the point of interaction.', 660),
('eu_ai_act', 'EU-AIA-ART50-2', 'Deep Fake Labelling', 'Mark AI-generated or manipulated image, audio, or video content (deep fakes) as artificially generated.', 'Article 50(2)', 'Transparency (Limited Risk)', '{limited,minimal}', '{provider,deployer}', true, 'Content must be marked in a machine-readable format indicating AI generation.', 'Implement content labelling with machine-readable metadata for AI-generated content.', 670),
('eu_ai_act', 'EU-AIA-ART50-3', 'Text Generation Disclosure', 'Label AI-generated text published to inform the public on matters of public interest as AI-generated.', 'Article 50(3)', 'Transparency (Limited Risk)', '{limited,minimal}', '{provider,deployer}', true, 'Published AI-generated text must be labelled unless editorially reviewed by a human.', 'Implement disclosure labels for AI-generated public interest text.', 680),
('eu_ai_act', 'EU-AIA-ART50-4', 'Emotion Recognition Disclosure', 'Inform exposed persons that an emotion recognition or biometric categorization system is in operation.', 'Article 50(4)', 'Transparency (Limited Risk)', '{limited,high}', '{provider,deployer}', true, 'Persons must be informed before being subjected to emotion recognition.', 'Provide clear notice of emotion recognition system operation.', 690);

-- GPAI Model Obligations (Art. 51-56) — 6 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART51-1', 'GPAI Technical Documentation', 'Maintain up-to-date technical documentation for the GPAI model.', 'Article 53(1)(a)', 'GPAI Model Obligations', '{gpai}', '{provider}', true, 'Documentation must include training process, testing, and evaluation results.', 'Create and maintain GPAI technical documentation per Annex XI.', 700),
('eu_ai_act', 'EU-AIA-ART51-2', 'GPAI Downstream Provider Information', 'Provide information and documentation to downstream providers integrating the GPAI model.', 'Article 53(1)(b)', 'GPAI Model Obligations', '{gpai}', '{provider}', true, 'Downstream providers need sufficient information to comply with their own obligations.', 'Prepare integration documentation package for downstream providers.', 710),
('eu_ai_act', 'EU-AIA-ART51-3', 'GPAI Copyright Compliance', 'Put in place a policy to comply with EU copyright law, particularly Article 4(3) of Directive (EU) 2019/790.', 'Article 53(1)(c)', 'GPAI Model Obligations', '{gpai}', '{provider}', true, 'Respect text and data mining opt-outs and maintain compliance documentation.', 'Implement copyright compliance policy with opt-out respect mechanisms.', 720),
('eu_ai_act', 'EU-AIA-ART51-4', 'GPAI Training Data Summary', 'Make publicly available a sufficiently detailed summary of training data content.', 'Article 53(1)(d)', 'GPAI Model Obligations', '{gpai}', '{provider}', true, 'Summary must follow the template provided by the AI Office.', 'Publish training data summary per the official template.', 730),
('eu_ai_act', 'EU-AIA-ART51-5', 'Systemic Risk Identification', 'Identify and assess systemic risks of GPAI models with systemic risk classification.', 'Article 55(1)', 'GPAI Model Obligations', '{gpai}', '{provider}', true, 'Models with systemic risk must perform model evaluation and adversarial testing.', 'Conduct systemic risk assessment for GPAI models exceeding threshold criteria.', 740),
('eu_ai_act', 'EU-AIA-ART51-6', 'Serious Incident Reporting (GPAI)', 'Report serious incidents related to GPAI models with systemic risk to the AI Office.', 'Article 55(1)(c)', 'GPAI Model Obligations', '{gpai}', '{provider}', true, 'Incidents must be reported without undue delay to the AI Office and relevant authorities.', 'Establish incident reporting procedures with AI Office notification protocols.', 750);

-- Post-Market Monitoring (Art. 72) — 3 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART72-1', 'Post-Market Monitoring System', 'Establish a post-market monitoring system proportionate to the nature and risks of the AI system.', 'Article 72(1)', 'Post-Market Monitoring', '{high}', '{provider}', true, 'Actively and systematically collect and analyze data on performance throughout the lifecycle.', 'Implement post-market monitoring system with data collection and analysis processes.', 760),
('eu_ai_act', 'EU-AIA-ART72-2', 'Post-Market Monitoring Plan', 'Establish and document a post-market monitoring plan as part of technical documentation.', 'Article 72(2)', 'Post-Market Monitoring', '{high}', '{provider}', true, 'Plan must define data collection methods, frequency, and escalation triggers.', 'Create a post-market monitoring plan included in technical documentation.', 770),
('eu_ai_act', 'EU-AIA-ART72-3', 'Post-Market Monitoring Data Analysis', 'Analyze monitoring data to determine if any corrective or preventive action is needed.', 'Article 72(3)', 'Post-Market Monitoring', '{high}', '{provider}', true, 'Regular analysis of monitoring data to identify emerging risks or degradation.', 'Establish periodic review of monitoring data with documented analysis.', 780);

-- Serious Incident Reporting (Art. 73) — 3 requirements
INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
('eu_ai_act', 'EU-AIA-ART73-1', 'Serious Incident Notification', 'Report any serious incident to the market surveillance authorities of the Member State where it occurred.', 'Article 73(1)', 'Serious Incident Reporting', '{high}', '{provider}', true, 'Report without undue delay after establishing causal link, within 15 days maximum.', 'Establish incident classification and notification procedures.', 790),
('eu_ai_act', 'EU-AIA-ART73-2', 'Incident Report Content', 'Provide all relevant information including system identification, circumstances, and corrective actions taken.', 'Article 73(2)-(3)', 'Serious Incident Reporting', '{high}', '{provider}', true, 'Report must contain sufficient detail for authority investigation.', 'Create incident report template covering all required elements.', 800),
('eu_ai_act', 'EU-AIA-ART73-3', 'Incident Follow-Up Actions', 'Investigate the incident and take corrective actions to prevent recurrence.', 'Article 73(4)', 'Serious Incident Reporting', '{high}', '{provider}', true, 'Conduct root cause analysis and implement preventive measures.', 'Document investigation findings, root cause, and corrective/preventive actions.', 810);

-- =============================================
-- ISO/IEC 42001 (~41 requirements)
-- =============================================

INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
-- Context of the Organization (4.1-4.4)
('iso_42001', 'ISO42001-4.1', 'Understanding the Organization', 'Determine external and internal issues relevant to the organization''s purpose and AI management system.', 'Clause 4.1', 'Context of the Organization', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Identify strategic context factors that affect AI system development and deployment.', 'Document internal/external context analysis including stakeholder landscape.', 10),
('iso_42001', 'ISO42001-4.2', 'Interested Parties', 'Determine interested parties relevant to the AIMS and their requirements.', 'Clause 4.2', 'Context of the Organization', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Identify all stakeholders affected by or affecting the AI management system.', 'Create a stakeholder register with their requirements and expectations.', 20),
('iso_42001', 'ISO42001-4.3', 'Scope of the AIMS', 'Determine the boundaries and applicability of the AI management system.', 'Clause 4.3', 'Context of the Organization', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Define which AI systems, processes, and functions are within scope.', 'Document the AIMS scope statement with clear boundaries.', 30),
('iso_42001', 'ISO42001-4.4', 'AI Management System', 'Establish, implement, maintain, and continually improve the AIMS.', 'Clause 4.4', 'Context of the Organization', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'The AIMS must include processes, their interactions, and resources needed.', 'Document the AIMS with process maps showing interactions and information flows.', 40),

-- Leadership & Commitment (5.1-5.3)
('iso_42001', 'ISO42001-5.1', 'Leadership and Commitment', 'Top management must demonstrate leadership and commitment to the AIMS.', 'Clause 5.1', 'Leadership & Commitment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Leadership must ensure policy, objectives, resources, and integration of the AIMS.', 'Document management commitment through policy endorsement and resource allocation.', 50),
('iso_42001', 'ISO42001-5.2', 'AI Policy', 'Establish an AI policy appropriate to the purpose of the organization.', 'Clause 5.2', 'Leadership & Commitment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'AI policy must include commitment to ethical AI, compliance, and continual improvement.', 'Create and publish an AI policy approved by top management.', 60),
('iso_42001', 'ISO42001-5.3', 'Roles and Responsibilities', 'Assign and communicate roles, responsibilities, and authorities for the AIMS.', 'Clause 5.3', 'Leadership & Commitment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Ensure clear accountability for AIMS conformity and performance reporting.', 'Define and document AIMS roles with responsibility assignments.', 70),

-- Planning — Risks & Opportunities (6.1)
('iso_42001', 'ISO42001-6.1-1', 'Risk and Opportunity Assessment', 'Determine risks and opportunities that need to be addressed by the AIMS.', 'Clause 6.1.1', 'Planning — Risks & Opportunities', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Consider context issues and stakeholder requirements when identifying risks.', 'Conduct risk assessment for the AIMS and document results.', 80),
('iso_42001', 'ISO42001-6.1-2', 'AI System Impact Assessment', 'Conduct an impact assessment for AI systems considering potential consequences on individuals and groups.', 'Clause 6.1.2', 'Planning — Risks & Opportunities', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Assess impacts on individuals, groups, and societies from AI system deployment.', 'Perform and document AI impact assessments for systems in scope.', 90),
('iso_42001', 'ISO42001-6.1-3', 'AI Risk Treatment', 'Plan actions to address risks and opportunities, integrated into AIMS processes.', 'Clause 6.1.3', 'Planning — Risks & Opportunities', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Risk treatment must be proportionate and effective.', 'Create risk treatment plans with controls mapped to identified risks.', 100),

-- AI Management System Objectives (6.2)
('iso_42001', 'ISO42001-6.2-1', 'AIMS Objectives', 'Establish AI management system objectives at relevant functions and levels.', 'Clause 6.2.1', 'AI Management System Objectives', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Objectives must be consistent with AI policy and be measurable.', 'Define SMART objectives for the AIMS and communicate them.', 110),
('iso_42001', 'ISO42001-6.2-2', 'Objectives Planning', 'Plan how to achieve AIMS objectives including resources, responsibilities, and timelines.', 'Clause 6.2.2', 'AI Management System Objectives', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Planning must cover what, who, when, and how evaluation.', 'Create action plans for each objective with milestones and metrics.', 120),

-- Support — Resources & Competence (7.1-7.3)
('iso_42001', 'ISO42001-7.1', 'Resources', 'Determine and provide resources needed for the AIMS.', 'Clause 7.1', 'Support — Resources & Competence', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Ensure adequate human, technical, and financial resources.', 'Document resource allocation for AIMS activities.', 130),
('iso_42001', 'ISO42001-7.2', 'Competence', 'Determine necessary competence for persons doing work affecting AI performance.', 'Clause 7.2', 'Support — Resources & Competence', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Ensure staff have education, training, or experience for their AI-related roles.', 'Maintain competence records and provide AI-specific training programmes.', 140),
('iso_42001', 'ISO42001-7.3', 'Awareness', 'Ensure persons doing work are aware of AI policy, their contribution, and implications of non-conformity.', 'Clause 7.3', 'Support — Resources & Competence', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'AI awareness must cover ethical considerations and organizational commitments.', 'Conduct awareness sessions and document attendance and content.', 150),

-- Support — Communication (7.4)
('iso_42001', 'ISO42001-7.4', 'Communication', 'Determine internal and external communications relevant to the AIMS.', 'Clause 7.4', 'Support — Communication', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Define what, when, with whom, and how to communicate about AI matters.', 'Establish a communication plan for AIMS stakeholders.', 160),

-- Support — Documented Information (7.5)
('iso_42001', 'ISO42001-7.5-1', 'Documented Information Requirements', 'Include documented information required by the standard and determined necessary by the organization.', 'Clause 7.5.1', 'Support — Documented Information', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Documentation extent depends on organization size, activities, and processes.', 'Create and maintain required AIMS documents and records.', 170),
('iso_42001', 'ISO42001-7.5-2', 'Document Control', 'Control documented information to ensure availability, suitability, and adequate protection.', 'Clause 7.5.2-3', 'Support — Documented Information', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Implement document control processes for distribution, access, retrieval, and retention.', 'Establish document management system with version control and access policies.', 180),

-- Operational Planning & Control (8.1-8.4)
('iso_42001', 'ISO42001-8.1', 'Operational Planning and Control', 'Plan, implement, and control processes needed to meet AIMS requirements.', 'Clause 8.1', 'Operational Planning & Control', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Establish criteria for processes and implement control in accordance with criteria.', 'Document operational procedures for AI system lifecycle processes.', 190),
('iso_42001', 'ISO42001-8.2', 'AI Risk Assessment Execution', 'Perform AI risk assessments at planned intervals and when significant changes occur.', 'Clause 8.2', 'Operational Planning & Control', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Risk assessments must use consistent methodology and retain documented results.', 'Schedule and execute periodic risk assessments with documented results.', 200),
('iso_42001', 'ISO42001-8.3', 'AI Risk Treatment Execution', 'Implement the AI risk treatment plan and retain documented information on results.', 'Clause 8.3', 'Operational Planning & Control', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Treatment implementation must be monitored for effectiveness.', 'Execute risk treatment plans and document implementation evidence.', 210),
('iso_42001', 'ISO42001-8.4', 'AI System Impact Assessment Execution', 'Perform AI system impact assessments for systems within scope of the AIMS.', 'Clause 8.4', 'Operational Planning & Control', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Impact assessments must consider the system lifecycle and affected stakeholders.', 'Conduct and document impact assessments for all in-scope AI systems.', 220),

-- Performance Evaluation (9.1-9.3)
('iso_42001', 'ISO42001-9.1', 'Monitoring, Measurement, Analysis', 'Determine what needs to be monitored and measured for AIMS effectiveness.', 'Clause 9.1', 'Performance Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Define metrics, methods, and frequency for monitoring AIMS performance.', 'Establish KPIs and monitoring mechanisms for AIMS performance.', 230),
('iso_42001', 'ISO42001-9.2-1', 'Internal Audit Programme', 'Conduct internal audits at planned intervals to verify AIMS conformity.', 'Clause 9.2.1', 'Performance Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Audit programme must consider importance of processes and previous audit results.', 'Plan and conduct internal AIMS audits with qualified auditors.', 240),
('iso_42001', 'ISO42001-9.2-2', 'Internal Audit Execution', 'Define audit criteria, scope, select auditors, and ensure objectivity.', 'Clause 9.2.2', 'Performance Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Audit results must be reported to relevant management.', 'Execute audits per plan, document findings, and report to management.', 250),
('iso_42001', 'ISO42001-9.3', 'Management Review', 'Top management must review the AIMS at planned intervals.', 'Clause 9.3', 'Performance Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Review must cover audit results, stakeholder feedback, risks, and improvement opportunities.', 'Conduct management reviews with documented minutes and decisions.', 260),

-- Improvement (10.1-10.2)
('iso_42001', 'ISO42001-10.1', 'Nonconformity and Corrective Action', 'React to nonconformities, take action to control and correct, and address consequences.', 'Clause 10.1', 'Improvement', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Evaluate the need for action to eliminate root cause and prevent recurrence.', 'Implement corrective action procedures with root cause analysis.', 270),
('iso_42001', 'ISO42001-10.2', 'Continual Improvement', 'Continually improve the suitability, adequacy, and effectiveness of the AIMS.', 'Clause 10.2', 'Improvement', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Improvement should be driven by audit results, data analysis, and management review outputs.', 'Document and implement improvement initiatives from AIMS performance data.', 280),

-- Annex A — AI Policies (A.2-A.4)
('iso_42001', 'ISO42001-A.2', 'AI Policy Objectives', 'Define and document the organization''s objectives relating to the responsible development and use of AI.', 'Annex A.2', 'Annex A — AI Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'AI policy objectives must align with organizational values and ethical principles.', 'Create AI policy objectives document aligned with organizational strategy.', 290),
('iso_42001', 'ISO42001-A.3', 'Internal AI Governance', 'Establish internal AI governance structures, roles, and responsibilities.', 'Annex A.3', 'Annex A — AI Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Governance structure must have authority to make decisions about AI system lifecycle.', 'Document AI governance structure with clear decision-making authority.', 300),
('iso_42001', 'ISO42001-A.4', 'AI Resources and Competence Policy', 'Ensure adequate resources and competence for responsible AI activities.', 'Annex A.4', 'Annex A — AI Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Resource allocation must cover development, deployment, monitoring, and training.', 'Document resource requirements and competence frameworks for AI roles.', 310),

-- Annex A — AI System Lifecycle (A.5-A.6)
('iso_42001', 'ISO42001-A.5', 'AI System Lifecycle Management', 'Manage AI systems throughout their lifecycle from conception to retirement.', 'Annex A.5', 'Annex A — AI System Lifecycle', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Lifecycle management must cover all phases with appropriate controls.', 'Define lifecycle phases with stage-gate criteria and controls.', 320),
('iso_42001', 'ISO42001-A.6-1', 'AI System Development Process', 'Establish processes for the design, development, and testing of AI systems.', 'Annex A.6.1', 'Annex A — AI System Lifecycle', '{high,limited,minimal,gpai}', '{provider}', true, 'Development processes must include requirements specification, design, and validation.', 'Document AI development processes with quality checkpoints.', 330),
('iso_42001', 'ISO42001-A.6-2', 'AI System Deployment and Operation', 'Establish processes for deployment, operation, and monitoring of AI systems.', 'Annex A.6.2', 'Annex A — AI System Lifecycle', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Deployment processes must include pre-deployment checks and operational monitoring.', 'Document deployment procedures with go-live checklists and monitoring plans.', 340),

-- Annex A — Data Management (A.7)
('iso_42001', 'ISO42001-A.7-1', 'Data Quality for AI', 'Ensure data used in AI systems meets quality requirements for the intended purpose.', 'Annex A.7.1', 'Annex A — Data Management', '{high,limited,minimal,gpai}', '{provider}', true, 'Data quality must be assessed for completeness, accuracy, and representativeness.', 'Implement data quality assessment processes and document results.', 350),
('iso_42001', 'ISO42001-A.7-2', 'Data Provenance and Governance', 'Establish data governance including provenance tracking and data lifecycle management.', 'Annex A.7.2', 'Annex A — Data Management', '{high,limited,minimal,gpai}', '{provider}', true, 'Track data origin, transformations, and lineage throughout the AI lifecycle.', 'Implement data governance with provenance tracking and retention policies.', 360),

-- Annex A — Transparency & Oversight (A.8-A.9)
('iso_42001', 'ISO42001-A.8-1', 'AI System Transparency', 'Ensure transparency of AI system operation and decision-making to relevant stakeholders.', 'Annex A.8.1', 'Annex A — Transparency & Oversight', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Transparency includes explainability of AI decisions and clear communication to users.', 'Implement transparency measures appropriate to stakeholder needs.', 370),
('iso_42001', 'ISO42001-A.8-2', 'AI System Information Provision', 'Provide relevant information about AI system behavior, capabilities, and limitations.', 'Annex A.8.2', 'Annex A — Transparency & Oversight', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Information provision must be timely, accessible, and comprehensible.', 'Create user-facing documentation about AI system behavior and limitations.', 380),
('iso_42001', 'ISO42001-A.9', 'Human Oversight Controls', 'Establish human oversight controls for AI systems proportionate to risk.', 'Annex A.9', 'Annex A — Transparency & Oversight', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Oversight must enable human intervention, override, and shutdown when needed.', 'Design and document human oversight mechanisms for AI systems.', 390),

-- Annex A — System Lifecycle (A.10)
('iso_42001', 'ISO42001-A.10-1', 'AI System Monitoring and Review', 'Monitor and review AI system performance throughout operation.', 'Annex A.10.1', 'Annex A — System Lifecycle', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Continuous monitoring must detect performance degradation and unexpected behavior.', 'Implement monitoring dashboards and alerting for AI system performance.', 400),
('iso_42001', 'ISO42001-A.10-2', 'AI System Retirement', 'Establish procedures for the responsible retirement and decommissioning of AI systems.', 'Annex A.10.2', 'Annex A — System Lifecycle', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Retirement must address data disposal, stakeholder notification, and knowledge preservation.', 'Document retirement procedures covering data, models, and stakeholder communication.', 410);

-- =============================================
-- ISO/IEC 23894 (~30 requirements)
-- =============================================

INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
-- Principles (5.1-5.8)
('iso_23894', 'ISO23894-5.1', 'Risk Management Integration', 'AI risk management must be an integral part of all organizational activities.', 'Clause 5.1', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Risk management should not be a standalone activity but embedded in governance.', 'Integrate AI risk management into existing organizational processes.', 10),
('iso_23894', 'ISO23894-5.2', 'Structured and Comprehensive Approach', 'A structured and comprehensive approach contributes to consistent and comparable results.', 'Clause 5.2', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Use a systematic methodology for AI risk identification and assessment.', 'Adopt a documented risk assessment methodology applied consistently.', 20),
('iso_23894', 'ISO23894-5.3', 'Customized Risk Framework', 'The risk management framework must be customized and proportionate.', 'Clause 5.3', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Proportionate to the level of risk and the context of the AI system.', 'Tailor risk framework to organizational context and AI system risks.', 30),
('iso_23894', 'ISO23894-5.4', 'Inclusive Stakeholder Involvement', 'Appropriate and timely involvement of stakeholders enables their knowledge to be considered.', 'Clause 5.4', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Include affected parties in risk identification and evaluation processes.', 'Establish stakeholder engagement processes for AI risk management.', 40),
('iso_23894', 'ISO23894-5.5', 'Dynamic and Responsive', 'Risk management must anticipate, detect, acknowledge, and respond to changes.', 'Clause 5.5', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'AI risks can emerge and change rapidly; management must be adaptive.', 'Implement continuous risk monitoring with change detection mechanisms.', 50),
('iso_23894', 'ISO23894-5.6', 'Best Available Information', 'Risk management inputs must be based on the best available information.', 'Clause 5.6', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Use current, verifiable information while acknowledging uncertainty.', 'Document information sources and acknowledge limitations in risk assessments.', 60),
('iso_23894', 'ISO23894-5.7', 'Human and Cultural Factors', 'Human behavior and culture significantly influence all aspects of risk management.', 'Clause 5.7', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Consider human factors including organizational culture and user behavior.', 'Assess human factors in AI risk including automation bias and skill degradation.', 70),
('iso_23894', 'ISO23894-5.8', 'Continual Improvement', 'Risk management is continually improved through learning and experience.', 'Clause 5.8', 'Principles', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Learn from risk events, near-misses, and emerging best practices.', 'Establish lessons-learned processes for AI risk management.', 80),

-- Framework — Leadership & Commitment (6.2)
('iso_23894', 'ISO23894-6.2-1', 'Leadership Commitment to AI Risk', 'Top management must demonstrate commitment to AI risk management.', 'Clause 6.2', 'Framework — Leadership & Commitment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Leadership must allocate resources and integrate AI risk into strategic planning.', 'Document leadership commitment through policy and resource allocation.', 90),
('iso_23894', 'ISO23894-6.2-2', 'AI Risk Management Policy', 'Establish a clear AI risk management policy aligned with organizational objectives.', 'Clause 6.2', 'Framework — Leadership & Commitment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Policy must state commitment to managing AI-specific risks.', 'Create and publish an AI risk management policy.', 100),

-- Framework — Integration (6.3)
('iso_23894', 'ISO23894-6.3', 'Integration into Organizational Processes', 'Integrate AI risk management into all organizational processes and decision-making.', 'Clause 6.3', 'Framework — Integration', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'AI risk management should inform procurement, development, and deployment decisions.', 'Embed AI risk considerations into existing governance and decision processes.', 110),

-- Framework — Design (6.4)
('iso_23894', 'ISO23894-6.4-1', 'Understanding Context for AI Risk', 'Understand and articulate the internal and external context for AI risk management.', 'Clause 6.4.1', 'Framework — Design', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Context includes legal, regulatory, technological, and societal factors.', 'Document contextual factors affecting AI risk management approach.', 120),
('iso_23894', 'ISO23894-6.4-2', 'AI Risk Criteria Definition', 'Define criteria to evaluate the significance of AI-related risks.', 'Clause 6.4.2', 'Framework — Design', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Criteria must reflect organizational values, objectives, and stakeholder expectations.', 'Establish and document risk evaluation criteria and thresholds.', 130),

-- Framework — Implementation (6.5)
('iso_23894', 'ISO23894-6.5-1', 'AI Risk Framework Implementation', 'Implement the AI risk management framework through an appropriate plan.', 'Clause 6.5.1', 'Framework — Implementation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Implementation plan must include timelines, resources, responsibilities.', 'Execute framework implementation plan with documented milestones.', 140),
('iso_23894', 'ISO23894-6.5-2', 'AI Risk Decision-Making Integration', 'Ensure AI risk management informs decision-making at all relevant levels.', 'Clause 6.5.2', 'Framework — Implementation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Risk information should be available to decision-makers in a timely manner.', 'Integrate risk dashboards and reporting into decision-making processes.', 150),

-- Framework — Evaluation (6.6)
('iso_23894', 'ISO23894-6.6', 'Framework Effectiveness Evaluation', 'Periodically evaluate the effectiveness of the AI risk management framework.', 'Clause 6.6', 'Framework — Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Assess whether the framework achieves its intended purpose.', 'Conduct periodic reviews of framework effectiveness with documented findings.', 160),

-- Framework — Improvement (6.7)
('iso_23894', 'ISO23894-6.7', 'Continuous Framework Improvement', 'Continually adapt and improve the AI risk management framework.', 'Clause 6.7', 'Framework — Improvement', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Adapt the framework based on evaluation findings and changing context.', 'Document and implement framework improvements based on evaluation results.', 170),

-- Process — Communication & Consultation (7.1)
('iso_23894', 'ISO23894-7.1', 'Risk Communication and Consultation', 'Establish processes for communication and consultation with stakeholders on AI risks.', 'Clause 7.1', 'Process — Communication & Consultation', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Communication should be two-way and include affected communities.', 'Create communication protocols for AI risk information sharing.', 180),

-- Process — Scope, Context, Criteria (7.2)
('iso_23894', 'ISO23894-7.2-1', 'AI Risk Context Establishment', 'Define the scope, external/internal context, and risk criteria for AI risk assessment.', 'Clause 7.2.1', 'Process — Scope, Context, Criteria', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Context must cover the AI system purpose, stakeholders, and operational environment.', 'Document scope and context for each AI risk assessment activity.', 190),
('iso_23894', 'ISO23894-7.2-2', 'AI-Specific Risk Criteria', 'Define risk criteria considering AI-specific characteristics like autonomy, opacity, and data dependency.', 'Clause 7.2.2', 'Process — Scope, Context, Criteria', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Criteria should address fairness, transparency, accountability, and safety.', 'Establish AI-specific risk criteria covering ethical and technical dimensions.', 200),

-- Process — Risk Assessment (7.3)
('iso_23894', 'ISO23894-7.3-1', 'AI Risk Identification', 'Identify AI-specific risks including data quality, model limitations, and deployment context risks.', 'Clause 7.3.1', 'Process — Risk Assessment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Consider risks from data, algorithms, deployment, and human interaction.', 'Conduct structured AI risk identification workshops and document findings.', 210),
('iso_23894', 'ISO23894-7.3-2', 'AI Risk Analysis', 'Analyze identified AI risks considering their nature, likelihood, and consequences.', 'Clause 7.3.2', 'Process — Risk Assessment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Analysis should consider both quantitative and qualitative factors.', 'Perform risk analysis documenting severity, likelihood, and contributing factors.', 220),
('iso_23894', 'ISO23894-7.3-3', 'AI Risk Evaluation', 'Evaluate AI risks by comparing analysis results against established criteria.', 'Clause 7.3.3', 'Process — Risk Assessment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Evaluation determines which risks need treatment and their priority.', 'Document risk evaluation results with treatment prioritization.', 230),
('iso_23894', 'ISO23894-7.3-4', 'Bias and Fairness Risk Assessment', 'Assess risks related to bias, discrimination, and fairness in AI systems.', 'Clause 7.3 (AI-specific)', 'Process — Risk Assessment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Bias risks must consider protected characteristics and vulnerable groups.', 'Conduct fairness-focused risk assessment as part of the overall process.', 240),

-- Process — Risk Treatment (7.4)
('iso_23894', 'ISO23894-7.4-1', 'AI Risk Treatment Selection', 'Select appropriate AI risk treatment options including avoid, mitigate, transfer, or accept.', 'Clause 7.4.1', 'Process — Risk Treatment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Treatment selection must consider cost-effectiveness and stakeholder impact.', 'Document treatment decisions with rationale for selected options.', 250),
('iso_23894', 'ISO23894-7.4-2', 'AI Risk Treatment Plan', 'Prepare and implement risk treatment plans for accepted treatments.', 'Clause 7.4.2', 'Process — Risk Treatment', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Plans must include responsibilities, resources, timelines, and success criteria.', 'Create treatment plans with implementation schedules and effectiveness measures.', 260),

-- Process — Monitoring & Review (7.5)
('iso_23894', 'ISO23894-7.5-1', 'AI Risk Monitoring', 'Continuously monitor AI risks and the effectiveness of risk treatments.', 'Clause 7.5.1', 'Process — Monitoring & Review', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Monitoring must detect changes in risk level and treatment effectiveness.', 'Implement risk monitoring with KRIs and periodic review cycles.', 270),
('iso_23894', 'ISO23894-7.5-2', 'AI Risk Review', 'Periodically review the AI risk management process and its outcomes.', 'Clause 7.5.2', 'Process — Monitoring & Review', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Reviews should assess process adequacy and identify improvement opportunities.', 'Schedule periodic risk management reviews with documented findings.', 280),

-- Process — Recording & Reporting (7.6)
('iso_23894', 'ISO23894-7.6-1', 'AI Risk Recording', 'Record AI risk management activities and their outcomes.', 'Clause 7.6.1', 'Process — Recording & Reporting', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Records provide evidence of risk management and support accountability.', 'Maintain risk registers and assessment documentation.', 290),
('iso_23894', 'ISO23894-7.6-2', 'AI Risk Reporting', 'Report risk management results to relevant decision-makers and stakeholders.', 'Clause 7.6.2', 'Process — Recording & Reporting', '{high,limited,minimal,gpai}', '{provider,deployer}', true, 'Reports must be timely, accurate, and tailored to the audience.', 'Produce periodic risk reports for management and governance bodies.', 300);

-- =============================================
-- NIST AI RMF (~70 requirements)
-- =============================================

INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
-- GOVERN 1 — Policies (1.1-1.7)
('nist_ai_rmf', 'NIST-GOV1-1', 'AI Risk Management Policies', 'Policies are in place for governing AI risk management.', 'GOVERN 1.1', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Establish organizational policies that address AI risk across the lifecycle.', 'Create and maintain AI risk management policy documents.', 10),
('nist_ai_rmf', 'NIST-GOV1-2', 'Legal and Regulatory Compliance', 'Policies reflect compliance with applicable AI-related laws and regulations.', 'GOVERN 1.2', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Identify and map all applicable AI regulations and standards.', 'Maintain a regulatory requirements register and map to internal controls.', 20),
('nist_ai_rmf', 'NIST-GOV1-3', 'Organizational Values Alignment', 'AI policies align with organizational values, principles, and strategic priorities.', 'GOVERN 1.3', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Ensure AI policies reflect stated ethical commitments and values.', 'Map AI policies to organizational values and ethical frameworks.', 30),
('nist_ai_rmf', 'NIST-GOV1-4', 'AI System Lifecycle Governance', 'Processes are in place to manage AI risks across the system lifecycle.', 'GOVERN 1.4', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Governance must cover design, development, deployment, monitoring, and decommissioning.', 'Define governance processes for each lifecycle stage.', 40),
('nist_ai_rmf', 'NIST-GOV1-5', 'Risk Tolerance Processes', 'Ongoing processes define and document organizational AI risk tolerances.', 'GOVERN 1.5', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Risk tolerance must be defined at organizational and system levels.', 'Document risk appetite and tolerance levels for AI systems.', 50),
('nist_ai_rmf', 'NIST-GOV1-6', 'AI Risk Management Integration', 'AI risk management is integrated into broader enterprise risk management.', 'GOVERN 1.6', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'AI risks should be managed alongside other operational and strategic risks.', 'Integrate AI risk reporting into enterprise risk management framework.', 60),
('nist_ai_rmf', 'NIST-GOV1-7', 'Third-Party AI Risk', 'Processes address risks from third-party AI entities across the value chain.', 'GOVERN 1.7', 'GOVERN 1 — Policies', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Manage risks from third-party AI models, data, and services.', 'Establish third-party AI risk assessment and due diligence processes.', 70),

-- GOVERN 2 — Accountability (2.1-2.3)
('nist_ai_rmf', 'NIST-GOV2-1', 'Roles and Responsibilities', 'Roles and responsibilities for AI risk management are defined and understood.', 'GOVERN 2.1', 'GOVERN 2 — Accountability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Clear accountability structures for AI risk management activities.', 'Document RACI matrix for AI risk management activities.', 80),
('nist_ai_rmf', 'NIST-GOV2-2', 'Diverse and Multidisciplinary Teams', 'AI actors reflect diverse demographics and multidisciplinary expertise.', 'GOVERN 2.2', 'GOVERN 2 — Accountability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Teams should include diverse perspectives for better risk identification.', 'Assess and promote diversity in AI development and oversight teams.', 90),
('nist_ai_rmf', 'NIST-GOV2-3', 'Executive Accountability', 'Senior leadership is accountable for AI risk management decisions.', 'GOVERN 2.3', 'GOVERN 2 — Accountability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Executive ownership ensures organizational commitment to responsible AI.', 'Assign executive sponsors for AI risk management with clear accountability.', 100),

-- GOVERN 3 — Workforce (3.1-3.2)
('nist_ai_rmf', 'NIST-GOV3-1', 'Workforce AI Literacy', 'Personnel are AI-literate and have appropriate skills for their roles.', 'GOVERN 3.1', 'GOVERN 3 — Workforce', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Provide training to ensure staff can effectively manage AI risks.', 'Implement AI literacy training programmes for relevant staff.', 110),
('nist_ai_rmf', 'NIST-GOV3-2', 'Inclusive AI Culture', 'Organizational culture promotes inclusive and equitable AI practices.', 'GOVERN 3.2', 'GOVERN 3 — Workforce', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Foster a culture where AI concerns can be raised without fear.', 'Establish mechanisms for reporting AI concerns and ethical issues.', 120),

-- GOVERN 4 — Organizational Practices (4.1-4.3)
('nist_ai_rmf', 'NIST-GOV4-1', 'Risk Management Process Documentation', 'AI risk management processes are documented and regularly reviewed.', 'GOVERN 4.1', 'GOVERN 4 — Organizational Practices', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Document standard operating procedures for risk management activities.', 'Create and maintain AI risk management process documentation.', 130),
('nist_ai_rmf', 'NIST-GOV4-2', 'AI System Inventory', 'Maintain an inventory of AI systems and their risk profiles.', 'GOVERN 4.2', 'GOVERN 4 — Organizational Practices', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Track all AI systems with classification and risk assessment status.', 'Build and maintain an AI system inventory with risk categorization.', 140),
('nist_ai_rmf', 'NIST-GOV4-3', 'AI System Documentation Standards', 'Standards for AI system documentation are established and followed.', 'GOVERN 4.3', 'GOVERN 4 — Organizational Practices', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Documentation standards support transparency and reproducibility.', 'Define documentation templates and standards for AI systems.', 150),

-- GOVERN 5 — Engagement (5.1-5.2)
('nist_ai_rmf', 'NIST-GOV5-1', 'Stakeholder Engagement', 'Processes for AI stakeholder engagement are defined and implemented.', 'GOVERN 5.1', 'GOVERN 5 — Engagement', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Engage affected communities, domain experts, and civil society.', 'Establish stakeholder engagement plan for AI risk management.', 160),
('nist_ai_rmf', 'NIST-GOV5-2', 'Feedback Mechanisms', 'Mechanisms are in place for stakeholder feedback on AI system impacts.', 'GOVERN 5.2', 'GOVERN 5 — Engagement', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Enable meaningful feedback from users and affected parties.', 'Implement feedback channels and grievance mechanisms for AI systems.', 170),

-- GOVERN 6 — Oversight (6.1-6.2)
('nist_ai_rmf', 'NIST-GOV6-1', 'AI Oversight Mechanisms', 'Effective oversight mechanisms are in place for AI systems.', 'GOVERN 6.1', 'GOVERN 6 — Oversight', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Oversight should be proportionate to the risk level of the AI system.', 'Design oversight mechanisms with escalation protocols.', 180),
('nist_ai_rmf', 'NIST-GOV6-2', 'Emergency Response Protocols', 'Procedures address emergency situations involving AI systems.', 'GOVERN 6.2', 'GOVERN 6 — Oversight', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Include procedures for system shutdown, incident response, and recovery.', 'Create AI incident response and emergency protocols.', 190),

-- MAP 1 — Context (1.1-1.6)
('nist_ai_rmf', 'NIST-MAP1-1', 'Intended Purpose Documentation', 'The intended purpose and context of use of the AI system are documented.', 'MAP 1.1', 'MAP 1 — Context', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Document who, what, when, where, and why the system is used.', 'Create a system purpose document with use context and constraints.', 200),
('nist_ai_rmf', 'NIST-MAP1-2', 'Interdisciplinary Input', 'Diverse expertise is engaged in mapping AI system context and potential impacts.', 'MAP 1.2', 'MAP 1 — Context', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Include domain experts, ethicists, legal counsel, and affected communities.', 'Document the breadth of expertise consulted during context mapping.', 210),
('nist_ai_rmf', 'NIST-MAP1-3', 'Scientific Integrity', 'AI system design and development follow scientific best practices.', 'MAP 1.3', 'MAP 1 — Context', '{high,limited,minimal,gpai}', '{provider}', false, 'Ensure reproducibility, peer review, and methodological rigor.', 'Document adherence to scientific integrity standards.', 220),
('nist_ai_rmf', 'NIST-MAP1-4', 'Legal and Regulatory Mapping', 'Applicable legal and regulatory requirements are mapped to the AI system.', 'MAP 1.4', 'MAP 1 — Context', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Identify all jurisdictional requirements applicable to the system.', 'Create a regulatory requirements mapping for the AI system.', 230),
('nist_ai_rmf', 'NIST-MAP1-5', 'Societal Impact Consideration', 'Potential broader societal impacts of the AI system are considered.', 'MAP 1.5', 'MAP 1 — Context', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Consider impacts on democracy, social equity, and public trust.', 'Document societal impact analysis as part of context mapping.', 240),
('nist_ai_rmf', 'NIST-MAP1-6', 'Deployment Environment Assessment', 'The operational environment and deployment conditions are assessed.', 'MAP 1.6', 'MAP 1 — Context', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Understand the real-world conditions where the system operates.', 'Document deployment environment characteristics and constraints.', 250),

-- MAP 2 — Categorization (2.1-2.3)
('nist_ai_rmf', 'NIST-MAP2-1', 'AI System Categorization', 'AI systems are categorized based on functionality, risk level, and deployment context.', 'MAP 2.1', 'MAP 2 — Categorization', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Categorization informs the depth and rigor of risk management.', 'Apply the organizational AI categorization framework to the system.', 260),
('nist_ai_rmf', 'NIST-MAP2-2', 'Technology Suitability Assessment', 'The suitability of AI technology for the planned task is assessed.', 'MAP 2.2', 'MAP 2 — Categorization', '{high,limited,minimal,gpai}', '{provider}', false, 'Consider whether AI is the appropriate solution for the problem.', 'Document the rationale for using AI versus alternative approaches.', 270),
('nist_ai_rmf', 'NIST-MAP2-3', 'Trustworthiness Requirements', 'Trustworthiness requirements are identified and documented.', 'MAP 2.3', 'MAP 2 — Categorization', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Cover validity, reliability, safety, security, fairness, and explainability.', 'Define trustworthiness requirements for the AI system.', 280),

-- MAP 3 — Benefits, Costs, Risks (3.1-3.5)
('nist_ai_rmf', 'NIST-MAP3-1', 'Benefit Assessment', 'Expected benefits of the AI system are documented and validated.', 'MAP 3.1', 'MAP 3 — Benefits & Risks', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Validate that expected benefits are achievable and measurable.', 'Document expected benefits with measurement criteria.', 290),
('nist_ai_rmf', 'NIST-MAP3-2', 'Cost Assessment', 'Costs of the AI system including negative impacts are identified.', 'MAP 3.2', 'MAP 3 — Benefits & Risks', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Include direct costs, opportunity costs, and potential harm.', 'Conduct cost-benefit analysis including negative externalities.', 300),
('nist_ai_rmf', 'NIST-MAP3-3', 'Risk Identification for AI', 'Risks specific to the AI system and its deployment are identified.', 'MAP 3.3', 'MAP 3 — Benefits & Risks', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Consider risks across all trustworthiness characteristics.', 'Perform structured risk identification for the AI system.', 310),
('nist_ai_rmf', 'NIST-MAP3-4', 'Data Risk Assessment', 'Risks associated with data quality, provenance, and usage are assessed.', 'MAP 3.4', 'MAP 3 — Benefits & Risks', '{high,limited,minimal,gpai}', '{provider}', false, 'Assess data-related risks including bias, privacy, and representativeness.', 'Document data risk assessment findings.', 320),
('nist_ai_rmf', 'NIST-MAP3-5', 'Impact on Affected Populations', 'Potential impacts on individuals and communities directly affected are assessed.', 'MAP 3.5', 'MAP 3 — Benefits & Risks', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Prioritize impacts on vulnerable or historically marginalized groups.', 'Assess and document impacts on affected populations.', 330),

-- MAP 4 — Risk Prioritization (4.1-4.2)
('nist_ai_rmf', 'NIST-MAP4-1', 'Risk Prioritization', 'Identified risks are prioritized based on likelihood, impact, and organizational context.', 'MAP 4.1', 'MAP 4 — Risk Prioritization', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Use consistent criteria for prioritizing risks across systems.', 'Prioritize risks using the organizational risk assessment methodology.', 340),
('nist_ai_rmf', 'NIST-MAP4-2', 'Risk-Benefit Tradeoff Analysis', 'Tradeoffs between risks and benefits are examined and documented.', 'MAP 4.2', 'MAP 4 — Risk Prioritization', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Document decision rationale when accepting risks for benefits.', 'Conduct and document risk-benefit tradeoff analysis.', 350),

-- MAP 5 — Stakeholder Engagement (5.1-5.2)
('nist_ai_rmf', 'NIST-MAP5-1', 'Affected Community Input', 'Input from potentially affected communities is gathered and incorporated.', 'MAP 5.1', 'MAP 5 — Stakeholder Engagement', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Engagement should be early and meaningful, not perfunctory.', 'Document community engagement activities and how input was incorporated.', 360),
('nist_ai_rmf', 'NIST-MAP5-2', 'Domain Expert Consultation', 'Relevant domain expertise is consulted in risk mapping.', 'MAP 5.2', 'MAP 5 — Stakeholder Engagement', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Domain experts help identify context-specific risks and impacts.', 'Record domain expert consultations and their contributions.', 370),

-- MEASURE 1 — Metrics (1.1-1.3)
('nist_ai_rmf', 'NIST-MEA1-1', 'Metrics Selection', 'Appropriate metrics are identified to assess trustworthiness characteristics.', 'MEASURE 1.1', 'MEASURE 1 — Metrics', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Metrics should be quantifiable, relevant, and aligned to use context.', 'Select and document metrics for each trustworthiness characteristic.', 380),
('nist_ai_rmf', 'NIST-MEA1-2', 'Measurement Methodology', 'Approaches for measuring AI system trustworthiness are established.', 'MEASURE 1.2', 'MEASURE 1 — Metrics', '{high,limited,minimal,gpai}', '{provider}', false, 'Include both quantitative testing and qualitative assessment.', 'Document measurement methodologies for each metric.', 390),
('nist_ai_rmf', 'NIST-MEA1-3', 'Internal and External Expert Input', 'Internal and external experts provide input on measurement approaches.', 'MEASURE 1.3', 'MEASURE 1 — Metrics', '{high,limited,minimal,gpai}', '{provider}', false, 'Diverse expertise improves measurement validity and coverage.', 'Engage relevant experts in measurement design and validation.', 400),

-- MEASURE 2 — Evaluation (2.1-2.13)
('nist_ai_rmf', 'NIST-MEA2-1', 'Accuracy and Performance Evaluation', 'AI system accuracy and performance are evaluated and documented.', 'MEASURE 2.1', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Evaluate performance on relevant benchmarks and real-world scenarios.', 'Document accuracy metrics with test methodology and results.', 410),
('nist_ai_rmf', 'NIST-MEA2-2', 'Reliability and Consistency', 'System reliability and consistency of outputs are tested.', 'MEASURE 2.2', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Test for consistent behavior across similar inputs and conditions.', 'Document reliability testing methodology and results.', 420),
('nist_ai_rmf', 'NIST-MEA2-3', 'Safety Evaluation', 'Safety risks are evaluated through appropriate testing.', 'MEASURE 2.3', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Safety testing must cover both normal and edge-case scenarios.', 'Conduct safety evaluation with documented test cases and results.', 430),
('nist_ai_rmf', 'NIST-MEA2-4', 'Security and Resilience Testing', 'AI system security and adversarial resilience are tested.', 'MEASURE 2.4', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Test for adversarial robustness, data poisoning, and model extraction.', 'Perform security testing including adversarial attack simulations.', 440),
('nist_ai_rmf', 'NIST-MEA2-5', 'Fairness Assessment', 'Bias and fairness are assessed across relevant population groups.', 'MEASURE 2.5', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Evaluate fairness using appropriate metrics for the use context.', 'Conduct bias audits with documented findings and mitigations.', 450),
('nist_ai_rmf', 'NIST-MEA2-6', 'Explainability Assessment', 'AI system explainability is evaluated for the intended audience.', 'MEASURE 2.6', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Explainability requirements vary by audience (users, operators, regulators).', 'Evaluate and document explainability features and their effectiveness.', 460),
('nist_ai_rmf', 'NIST-MEA2-7', 'Privacy Risk Assessment', 'Privacy risks including re-identification and data leakage are assessed.', 'MEASURE 2.7', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Consider training data privacy, inference privacy, and model memorization.', 'Perform privacy risk assessment and document mitigations.', 470),
('nist_ai_rmf', 'NIST-MEA2-8', 'Environmental Impact Assessment', 'Environmental impacts of AI system development and operation are assessed.', 'MEASURE 2.8', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Consider energy consumption, carbon footprint, and resource usage.', 'Document environmental impact assessment for the AI system.', 480),
('nist_ai_rmf', 'NIST-MEA2-9', 'Sociotechnical System Testing', 'AI system is tested in the broader sociotechnical context of deployment.', 'MEASURE 2.9', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Test human-AI interaction and system behavior in realistic settings.', 'Conduct user testing in representative deployment conditions.', 490),
('nist_ai_rmf', 'NIST-MEA2-10', 'Validity and Generalizability', 'Test validity and ability to generalize across deployment contexts.', 'MEASURE 2.10', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Assess whether test results transfer to real-world deployment.', 'Document validity assessment including generalizability analysis.', 500),
('nist_ai_rmf', 'NIST-MEA2-11', 'Human Factors Evaluation', 'Human factors in AI system use are evaluated including cognitive load.', 'MEASURE 2.11', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Assess automation bias, decision fatigue, and skill degradation.', 'Conduct human factors evaluation with documented findings.', 510),
('nist_ai_rmf', 'NIST-MEA2-12', 'Pre-Deployment Testing', 'Comprehensive testing is performed before deployment.', 'MEASURE 2.12', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider}', false, 'Pre-deployment testing must cover all identified risk categories.', 'Complete pre-deployment test plan with sign-off criteria.', 520),
('nist_ai_rmf', 'NIST-MEA2-13', 'Post-Deployment Monitoring', 'Ongoing monitoring continues after deployment to detect emergent issues.', 'MEASURE 2.13', 'MEASURE 2 — Evaluation', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Monitor for performance drift, new risks, and changing context.', 'Implement post-deployment monitoring with alerting and review cycles.', 530),

-- MEASURE 3 — Tracking (3.1-3.3)
('nist_ai_rmf', 'NIST-MEA3-1', 'Risk Tracking Mechanisms', 'Mechanisms are in place to track identified risks over time.', 'MEASURE 3.1', 'MEASURE 3 — Tracking', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Track risk status changes and treatment effectiveness.', 'Maintain risk tracking system with status updates.', 540),
('nist_ai_rmf', 'NIST-MEA3-2', 'Performance Trend Analysis', 'AI system performance trends are analyzed over time.', 'MEASURE 3.2', 'MEASURE 3 — Tracking', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Detect degradation, drift, and emergent patterns.', 'Implement performance trend analysis with baseline comparisons.', 550),
('nist_ai_rmf', 'NIST-MEA3-3', 'Measurement Results Communication', 'Measurement results are communicated to relevant stakeholders.', 'MEASURE 3.3', 'MEASURE 3 — Tracking', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Results must reach decision-makers in actionable format.', 'Establish reporting channels for measurement results.', 560),

-- MEASURE 4 — Feedback (4.1-4.2)
('nist_ai_rmf', 'NIST-MEA4-1', 'User Feedback Integration', 'Feedback from AI system users is collected and acted upon.', 'MEASURE 4.1', 'MEASURE 4 — Feedback', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'User feedback is a critical signal for risk identification.', 'Implement user feedback mechanisms and incorporate into risk assessments.', 570),
('nist_ai_rmf', 'NIST-MEA4-2', 'External Feedback Channels', 'External stakeholders can provide feedback on AI system impacts.', 'MEASURE 4.2', 'MEASURE 4 — Feedback', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Enable affected parties to report concerns and negative impacts.', 'Establish accessible external feedback and complaint channels.', 580),

-- MANAGE 1 — Response (1.1-1.4)
('nist_ai_rmf', 'NIST-MAN1-1', 'Risk Response Plans', 'Risk response plans are developed and implemented.', 'MANAGE 1.1', 'MANAGE 1 — Response', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Plans must cover risk mitigation, transfer, avoidance, or acceptance.', 'Create risk response plans for prioritized AI risks.', 590),
('nist_ai_rmf', 'NIST-MAN1-2', 'Risk Mitigation Implementation', 'Risk mitigation strategies are implemented and monitored.', 'MANAGE 1.2', 'MANAGE 1 — Response', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Monitor mitigation effectiveness and adjust as needed.', 'Implement and track risk mitigations with effectiveness measures.', 600),
('nist_ai_rmf', 'NIST-MAN1-3', 'Incident Response for AI', 'Incident response procedures address AI-specific failure modes.', 'MANAGE 1.3', 'MANAGE 1 — Response', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'AI incidents may include model failure, adversarial attacks, or bias incidents.', 'Develop AI-specific incident response procedures.', 610),
('nist_ai_rmf', 'NIST-MAN1-4', 'System Decommissioning Procedures', 'Procedures exist for responsible decommissioning of AI systems.', 'MANAGE 1.4', 'MANAGE 1 — Response', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Address data disposal, model archiving, and user notification.', 'Document decommissioning procedures with data and model handling.', 620),

-- MANAGE 2 — Maximize Benefit (2.1-2.4)
('nist_ai_rmf', 'NIST-MAN2-1', 'Benefit Maximization Strategy', 'Strategies are in place to maximize AI system benefits while minimizing harms.', 'MANAGE 2.1', 'MANAGE 2 — Maximize Benefit', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Balance benefit optimization with harm prevention.', 'Document benefit maximization strategies alongside risk management.', 630),
('nist_ai_rmf', 'NIST-MAN2-2', 'Equitable Benefit Distribution', 'Actions ensure AI benefits are equitably distributed.', 'MANAGE 2.2', 'MANAGE 2 — Maximize Benefit', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Consider who benefits and who bears the costs of the AI system.', 'Assess and document benefit distribution across affected groups.', 640),
('nist_ai_rmf', 'NIST-MAN2-3', 'Positive Impact Amplification', 'Opportunities for positive social impact are identified and pursued.', 'MANAGE 2.3', 'MANAGE 2 — Maximize Benefit', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Look beyond harm prevention to active benefit creation.', 'Document positive impact opportunities and actions taken.', 650),
('nist_ai_rmf', 'NIST-MAN2-4', 'Value Alignment Verification', 'AI system outcomes align with stated organizational and societal values.', 'MANAGE 2.4', 'MANAGE 2 — Maximize Benefit', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Regularly verify that system outcomes reflect intended values.', 'Conduct periodic value alignment assessments.', 660),

-- MANAGE 3 — Documentation (3.1-3.2)
('nist_ai_rmf', 'NIST-MAN3-1', 'Risk Decision Documentation', 'AI risk management decisions are documented with rationale.', 'MANAGE 3.1', 'MANAGE 3 — Documentation', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Create an audit trail of risk decisions and their justification.', 'Maintain decision log for all risk management actions.', 670),
('nist_ai_rmf', 'NIST-MAN3-2', 'Transparency Reporting', 'Regular reporting on AI risk management activities and outcomes.', 'MANAGE 3.2', 'MANAGE 3 — Documentation', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Reports should be accessible to relevant stakeholders.', 'Produce periodic AI risk management transparency reports.', 680),

-- MANAGE 4 — Residual Risk (4.1-4.2)
('nist_ai_rmf', 'NIST-MAN4-1', 'Residual Risk Documentation', 'Residual risks after mitigation are documented and communicated.', 'MANAGE 4.1', 'MANAGE 4 — Residual Risk', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Stakeholders must understand remaining risks after mitigation.', 'Document residual risks with acceptance rationale.', 690),
('nist_ai_rmf', 'NIST-MAN4-2', 'Residual Risk Monitoring', 'Residual risks are monitored for changes in risk level.', 'MANAGE 4.2', 'MANAGE 4 — Residual Risk', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Residual risks may increase as context changes or new information emerges.', 'Implement monitoring for residual risk level changes.', 700);

-- =============================================
-- OECD AI PRINCIPLES (~16 requirements)
-- =============================================

INSERT INTO framework_requirements (framework, reference_id, title, description, article_clause, category, applicable_risk_categories, applicable_roles, is_mandatory, guidance_text, implementation_notes, sort_order) VALUES
-- Principle 1 — Inclusive Growth (3)
('oecd_ai_principles', 'OECD-P1-1', 'Inclusive Growth Consideration', 'AI systems should benefit people and the planet by driving inclusive growth and sustainable development.', 'Principle 1.1', 'Inclusive Growth & Well-being', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Consider how AI supports broad societal benefit and reduces inequality.', 'Assess and document how the AI system contributes to inclusive growth.', 10),
('oecd_ai_principles', 'OECD-P1-2', 'Sustainable Development Alignment', 'AI development should align with sustainable development goals and environmental protection.', 'Principle 1.2', 'Inclusive Growth & Well-being', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Consider environmental impact and alignment with SDGs.', 'Document sustainability assessment for the AI system.', 20),
('oecd_ai_principles', 'OECD-P1-3', 'Well-being Enhancement', 'AI should augment human capabilities and enhance well-being.', 'Principle 1.3', 'Inclusive Growth & Well-being', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Ensure the system enhances rather than diminishes quality of life.', 'Evaluate and document well-being impacts of the AI system.', 30),

-- Principle 2 — Human-Centred Values (4)
('oecd_ai_principles', 'OECD-P2-1', 'Human Rights Respect', 'AI systems should respect human rights, democratic values, and diversity.', 'Principle 2.1', 'Human-Centred Values & Fairness', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Ensure the system does not infringe on fundamental rights.', 'Conduct human rights impact assessment for the AI system.', 40),
('oecd_ai_principles', 'OECD-P2-2', 'Fairness and Non-Discrimination', 'AI systems should be designed to be fair and avoid creating or reinforcing unfair bias.', 'Principle 2.2', 'Human-Centred Values & Fairness', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Implement fairness measures throughout the AI lifecycle.', 'Document fairness assessment methodology and results.', 50),
('oecd_ai_principles', 'OECD-P2-3', 'Privacy and Data Protection', 'AI stakeholders should respect privacy and data protection.', 'Principle 2.3', 'Human-Centred Values & Fairness', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Privacy by design should be applied throughout the AI system lifecycle.', 'Implement and document privacy and data protection measures.', 60),
('oecd_ai_principles', 'OECD-P2-4', 'User Autonomy Protection', 'AI should respect user autonomy and not undermine the ability for informed decision-making.', 'Principle 2.4', 'Human-Centred Values & Fairness', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Users should be able to understand AI influence on their decisions.', 'Assess and document safeguards for user autonomy.', 70),

-- Principle 3 — Transparency (3)
('oecd_ai_principles', 'OECD-P3-1', 'Meaningful Transparency', 'Provide meaningful information about AI systems for stakeholder understanding.', 'Principle 3.1', 'Transparency & Explainability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Transparency must be meaningful to the audience, not just technical disclosure.', 'Implement transparency measures tailored to different stakeholder needs.', 80),
('oecd_ai_principles', 'OECD-P3-2', 'AI System Disclosure', 'Stakeholders should be aware when they are interacting with or subject to AI systems.', 'Principle 3.2', 'Transparency & Explainability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Disclosure must be timely and understandable to the affected person.', 'Implement AI interaction disclosure at point of contact.', 90),
('oecd_ai_principles', 'OECD-P3-3', 'Explainability of Outcomes', 'Enable people affected by AI to understand and challenge AI-based outcomes.', 'Principle 3.3', 'Transparency & Explainability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Persons affected should be able to obtain explanations and challenge decisions.', 'Provide explanation mechanisms and challenge/appeal processes.', 100),

-- Principle 4 — Robustness & Safety (3)
('oecd_ai_principles', 'OECD-P4-1', 'Safety Throughout Lifecycle', 'AI systems should not pose unreasonable safety risks throughout their lifecycle.', 'Principle 4.1', 'Robustness, Security & Safety', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Continuous safety assessment and risk management is required.', 'Implement safety assessment across the system lifecycle.', 110),
('oecd_ai_principles', 'OECD-P4-2', 'Security and Resilience', 'AI systems should be secure and resilient against attacks and adversarial conditions.', 'Principle 4.2', 'Robustness, Security & Safety', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Address cybersecurity risks specific to AI including adversarial ML.', 'Conduct security testing and implement resilience measures.', 120),
('oecd_ai_principles', 'OECD-P4-3', 'Traceability and Auditability', 'Enable traceability of AI system decisions and actions for audit and accountability.', 'Principle 4.3', 'Robustness, Security & Safety', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Maintain logs and records enabling reverse-engineering of decision paths.', 'Implement comprehensive logging for traceability and audit.', 130),

-- Principle 5 — Accountability (3)
('oecd_ai_principles', 'OECD-P5-1', 'Accountability Framework', 'Organizations and individuals developing or deploying AI should be accountable for proper functioning.', 'Principle 5.1', 'Accountability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Accountability must be clear and enforceable across the value chain.', 'Establish accountability framework with defined responsibilities.', 140),
('oecd_ai_principles', 'OECD-P5-2', 'Risk Assessment and Mitigation', 'AI actors should manage risks through impact assessments and risk mitigation.', 'Principle 5.2', 'Accountability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Risk management is a core accountability obligation for AI actors.', 'Document risk management activities as evidence of accountability.', 150),
('oecd_ai_principles', 'OECD-P5-3', 'Redress Mechanisms', 'Appropriate mechanisms for redress should be available when AI systems cause harm.', 'Principle 5.3', 'Accountability', '{high,limited,minimal,gpai}', '{provider,deployer}', false, 'Affected persons must have accessible pathways to challenge AI decisions and seek redress.', 'Establish and document redress and complaint mechanisms.', 160);


-- =============================================
-- CROSS-REFERENCES (from Spec Section 9)
-- =============================================

-- 1. Risk Management: EU Art. 9 ↔ ISO 42001 6.1 ↔ ISO 23894 7.3 ↔ NIST MAP ↔ OECD P4
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both require systematic risk identification and management for AI systems.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART9-1' AND t.reference_id = 'ISO42001-6.1-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both mandate structured AI risk assessment processes.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART9-1' AND t.reference_id = 'ISO23894-7.3-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'NIST MAP context establishment aligns with EU AI Act risk management scope.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART9-1' AND t.reference_id = 'NIST-MAP1-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'OECD robustness principle supports EU AI Act risk management objectives.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART9-1' AND t.reference_id = 'OECD-P4-1';

-- 2. Data Governance: EU Art. 10 ↔ ISO 42001 A.7 ↔ NIST MAP 3 ↔ OECD P2
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both address data quality and governance for AI systems.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART10-1' AND t.reference_id = 'ISO42001-A.7-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'NIST data risk assessment complements EU data governance requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART10-1' AND t.reference_id = 'NIST-MAP3-4';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'OECD fairness principle supports EU data governance bias requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART10-4' AND t.reference_id = 'OECD-P2-2';

-- 3. Technical Documentation: EU Art. 11 ↔ ISO 42001 7.5 ↔ ISO 23894 7.6 ↔ NIST GOVERN 4 ↔ OECD P3
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'ISO documented information requirements support EU technical documentation obligations.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART11-1' AND t.reference_id = 'ISO42001-7.5-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'ISO 23894 recording requirements support EU documentation obligations.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART11-1' AND t.reference_id = 'ISO23894-7.6-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'NIST documentation standards align with EU technical documentation requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART11-1' AND t.reference_id = 'NIST-GOV4-3';

-- 4. Transparency: EU Art. 13 ↔ ISO 42001 A.8 ↔ NIST GOVERN 2 ↔ OECD P3
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'equivalent', 'Both require transparency of AI system operation and decision-making to stakeholders.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART13-1' AND t.reference_id = 'ISO42001-A.8-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'equivalent', 'OECD transparency principle directly aligns with EU Art. 13 transparency requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART13-1' AND t.reference_id = 'OECD-P3-1';

-- 5. Human Oversight: EU Art. 14 ↔ ISO 42001 A.9 ↔ NIST MANAGE 3 ↔ OECD P2
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'equivalent', 'Both require human oversight controls proportionate to AI system risk.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART14-1' AND t.reference_id = 'ISO42001-A.9';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'NIST risk decision documentation supports EU human oversight requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART14-1' AND t.reference_id = 'NIST-MAN3-1';

-- 6. Accuracy/Robustness: EU Art. 15 ↔ ISO 42001 A.10 ↔ ISO 23894 7.4 ↔ NIST MEASURE 2 ↔ OECD P4
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both address ongoing monitoring and performance management of AI systems.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART15-1' AND t.reference_id = 'ISO42001-A.10-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both address risk treatment and robustness measures.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART15-1' AND t.reference_id = 'ISO23894-7.4-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'NIST evaluation framework directly supports EU accuracy and robustness requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART15-1' AND t.reference_id = 'NIST-MEA2-1';

-- 7. QMS: EU Art. 17 ↔ ISO 42001 8.1 ↔ ISO 23894 6.5 ↔ NIST GOVERN 1 ↔ OECD P5
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both require operational planning and control for AI management.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART17-1' AND t.reference_id = 'ISO42001-8.1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Quality management aligns with NIST AI risk governance policies.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART17-1' AND t.reference_id = 'NIST-GOV1-1';

-- 8. FRIA: EU Art. 27 ↔ ISO 42001 4.1 ↔ ISO 23894 7.2 ↔ NIST MAP 1 ↔ OECD P1+P2
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'ISO context analysis supports EU fundamental rights impact assessment.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART27-1' AND t.reference_id = 'ISO42001-4.1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'OECD human-centred values principle supports FRIA objectives.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART27-1' AND t.reference_id = 'OECD-P2-1';

-- 9. Conformity Assessment: EU Art. 43 ↔ ISO 42001 9.2 ↔ ISO 23894 6.6 ↔ NIST MEASURE 1 ↔ OECD P5
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both require systematic assessment and audit of AI system compliance.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART43-1' AND t.reference_id = 'ISO42001-9.2-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'OECD accountability supports EU conformity assessment objectives.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART43-1' AND t.reference_id = 'OECD-P5-1';

-- 10. Post-Market Monitoring: EU Art. 72 ↔ ISO 42001 9.1 ↔ ISO 23894 7.5 ↔ NIST MANAGE 4 ↔ OECD P4
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both require ongoing monitoring and measurement of AI system performance.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART72-1' AND t.reference_id = 'ISO42001-9.1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both require monitoring AI risks post-deployment.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART72-1' AND t.reference_id = 'ISO23894-7.5-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'NIST residual risk monitoring supports EU post-market monitoring.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART72-1' AND t.reference_id = 'NIST-MAN4-2';

-- 11. Incident Reporting: EU Art. 73 ↔ ISO 42001 10.1 ↔ ISO 23894 7.5 ↔ NIST MANAGE 1 ↔ OECD P5
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'Both address nonconformity management and corrective action for AI systems.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART73-1' AND t.reference_id = 'ISO42001-10.1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'overlapping', 'NIST incident response directly supports EU incident reporting requirements.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART73-1' AND t.reference_id = 'NIST-MAN1-3';

-- 12. Prohibited Practices: EU Art. 5 ↔ ISO 23894 7.3.2 ↔ NIST MAP 2 ↔ OECD P2
INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'ISO risk identification process supports screening for prohibited AI practices.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART5-1' AND t.reference_id = 'ISO23894-7.3-1';

INSERT INTO cross_references (source_requirement_id, target_requirement_id, relationship, notes)
SELECT s.id, t.id, 'supports', 'OECD human rights principle supports the prohibition of harmful AI practices.'
FROM framework_requirements s, framework_requirements t
WHERE s.reference_id = 'EU-AIA-ART5-1' AND t.reference_id = 'OECD-P2-1';
