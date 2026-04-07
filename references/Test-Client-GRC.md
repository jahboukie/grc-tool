Test Fixture

Company: Northwind Community Bank

Engagement: Northwind Community Bank — Credit Decisioning Review 2026

Primary role: Deployer

Frameworks: EU AI Act, ISO/IEC 42001, ISO/IEC 23894, NIST AI RMF

AI system: Nimbus Credit Decision Engine v2.4

Domain: Finance

Risk category: High

Intended purpose: support consumer loan underwriting by scoring applications, flagging elevated default risk, and recommending approve, review, or decline outcomes to human credit officers

Deployment context: used by loan operations staff in pre-contract credit decisions; human reviewers can override outputs; logs are retained; the model is supplied by an external vendor but operated locally by the bank

Client description: mid-size regional retail bank operating consumer lending in the EU and UK, using a third-party AI-assisted creditworthiness and affordability scoring workflow

This is a good default because it exercises the parts of the app that matter most: high-risk obligations, risk register, evidence, human oversight, gap analysis, and especially FRIA, since credit scoring is one of the clearest Article 27 test cases.

For repeatable test content, use these standard examples:

Evidence: underwriting policy, vendor technical documentation, model validation report, bias testing report, human oversight SOP, training record, post-market monitoring plan, incident log

Risks: bias against protected groups, poor explain-ability for adverse actions, stale applicant data, automation bias by reviewers, model drift, weak override procedures, incomplete log retention

Tasks: complete FRIA, validate override workflow, refresh bias testing, confirm six-month log retention, review adverse-action explanation template
I’ve also saved this profile into repo memory so I can keep using the same customer consistently in future testing.