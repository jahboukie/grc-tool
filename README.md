# GRC Command Center

**Personal AI Governance, Risk & Compliance platform built by a Certified AI Governance Professional.**

A local-first desktop application designed to systematically manage AI governance engagements, track compliance across five international regulatory frameworks, assess risks, maintain auditable evidence, and generate compliance reports — replacing ad-hoc spreadsheets with a purpose-built engineering solution.

---

## Why This Exists

An AI governance professional managing multiple client engagements cannot rely on memory or scattered documents to track hundreds of specific legal requirements across overlapping regulatory frameworks. The EU AI Act alone contains 83+ distinct obligations that vary by risk tier and supply-chain role. Multiply that across ISO 42001, ISO 23894, NIST AI RMF, and the OECD AI Principles — then across multiple clients, each with different AI systems and risk profiles — and the cognitive load becomes unmanageable without tooling.

This application externalizes that cognitive load into a structured, auditable system. Every assessment decision, every status change, every piece of evidence is logged with an immutable audit trail. When a regulator or client asks "show me your diligence," the answer is the export from this tool.

---

## Frameworks Covered

| Framework | Identifier | Seeded Requirements |
|-----------|-----------|:-------------------:|
| **EU AI Act** | Regulation (EU) 2024/1689 | ~83 |
| **ISO/IEC 42001** | AI Management System (2023) | ~50 |
| **ISO/IEC 23894** | AI Risk Management (2023) | ~40 |
| **NIST AI RMF** | AI Risk Management Framework 1.0 | ~45 |
| **OECD AI Principles** | Recommendation of the Council on AI (2019/2024) | ~23 |

**241 requirements** seeded from primary regulatory sources, with **cross-reference mappings** showing where frameworks overlap (e.g., EU AI Act Article 9 ↔ ISO 23894 Clause 6 ↔ NIST GOVERN-1).

---

## Core Capabilities

### Engagement & AI System Management
Track multiple client engagements, each scoped by industry sector, jurisdiction, obligation role, AI use case, and data profile. Register AI systems per engagement with EU AI Act risk classification (Unacceptable / High / Limited / Minimal / GPAI). Intake forms auto-suggest applicable frameworks based on scoping inputs.

### Framework Requirement Navigator
Browse all 241 requirements grouped by category, filterable by framework, risk tier, category, and free-text search. Select an engagement's AI system, then assess each requirement inline — recording compliance status (Met / Partial / Gap / N/A), assessor notes, remediation plans, and target dates.

### Cross-Framework Reference Map
Visualize how requirements map across frameworks. Filter by relationship type (equivalent, overlapping, supporting, partial). When an AI system is selected, each cross-referenced requirement shows its current assessment status — revealing which compliance work transfers across frameworks.

### Fundamental Rights Impact Assessment (FRIA)
Structured FRIA workflow aligned with EU AI Act Article 27, covering affected groups, impact severity ratings, mitigation measures, and proportionality assessments. Full lifecycle from draft through review to final approval.

### Risk Matrix
5×5 risk heat map (Likelihood × Impact) with inherent and residual risk scoring. Register risks linked to specific AI systems, categories, and requirements. Visual placement on the matrix for at-a-glance portfolio risk posture.

### Evidence Vault
Central repository of compliance artifacts. Upload evidence and link it to specific requirements, risks, or tasks. Track evidence type (policy, technical report, test result, training record, etc.) and acceptance status. Evidence connections propagate through the assessment layer.

### Task Management
Create, prioritize, and track compliance tasks within each engagement. Inline status and priority controls. Tasks link to specific AI systems, frameworks, and requirements for full traceability.

### Compliance Reports
Generate HTML reports from structured data:
- **Compliance Report** — per-system assessment summary across all applicable requirements
- **Gap Analysis Report** — focused view of unmet requirements with remediation timelines
- **Risk Report** — risk register with matrix visualization and mitigation status

Reports render server-side via Askama templates, written to local filesystem, and opened in the default browser.

### LLM Regulatory Assistant
Built-in conversational assistant for regulatory research. Supports three provider backends:
- **OpenAI** (GPT-4o, GPT-4 Turbo)
- **Anthropic** (Claude 3.5 Sonnet, Claude 3 Opus)
- **Ollama** (local models, fully offline)

API keys encrypted at rest using AES-256-GCM. Conversation history persisted. The assistant does not make compliance decisions — it assists with regulatory interpretation and drafting.

### Immutable Audit Trail
Every entity mutation writes to an append-only `audit_log` table. PostgreSQL triggers prevent UPDATE and DELETE on audit records. The trail captures entity type, action, field changed, old value, new value, and timestamp. Fully searchable and filterable by entity, action type, and date range.

### Dashboard
At-a-glance metrics: total engagements, active AI systems, compliance posture breakdown, priority tasks, and recent audit activity.

---

## Technical Architecture

```
Leptos (WASM) → tauri::invoke() → Tauri Commands (Rust) → SQLx → PostgreSQL
```

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Desktop Shell | **Tauri 2** | Native window, ~10 MB binary (vs ~150 MB Electron), Rust-native IPC |
| Frontend | **Leptos** (CSR → WASM) | Reactive UI in Rust, compiles via `trunk`, zero npm |
| Backend | **Rust** (44 Tauri commands) | Type-safe, async, shared types across IPC boundary |
| Database | **PostgreSQL 16+** | Complex queries, JSON arrays, immutable audit triggers |
| DB Driver | **SQLx** | Async, compile-time checked queries |
| Reports | **Askama** | Compile-time HTML template verification |
| Styling | **Pico CSS** (vendored) | Classless semantic defaults, no build pipeline |
| Encryption | **AES-256-GCM** | API key encryption at rest |
| HTTP | **reqwest** | LLM provider API calls |

**Zero npm. Zero Node.js. The entire application is Rust, compiled through `cargo` and `trunk`.**

### Project Structure

```
grc-tool/
├── shared/            # Shared types crate — 20 enums, 36 structs
│   └── src/           #   used by both frontend and backend
├── src-tauri/         # Tauri backend
│   ├── migrations/    #   5 SQL migrations (schema + 241 seeded requirements)
│   ├── src/
│   │   ├── commands/  #   13 command modules (44 IPC commands)
│   │   ├── models/    #   Data access layer (SQLx queries)
│   │   ├── reports/   #   Askama report generation
│   │   └── llm/       #   Multi-provider LLM client
│   └── templates/     #   HTML report templates
├── frontend/          # Leptos WASM frontend
│   └── src/
│       ├── pages/     #   15 page components
│       ├── components/#   12 reusable UI components
│       └── api/       #   Tauri invoke wrappers
└── references/        # Primary regulatory source material
```

**~9,600 lines of Rust** | **~800 lines of SQL** | **44 backend commands** | **15 pages** | **12 components**

---

## Data Model Highlights

The system is designed **data-structures-first** (Rob Pike Rule 5 — if you get the data structures right, the rest falls into place). Key design decisions:

- **All primary keys are UUID v4** — no sequential IDs, no collision risk across environments
- **All timestamps UTC** via `chrono` — unambiguous temporal ordering
- **Enums are PascalCase in Rust, snake_case in PostgreSQL** — `serde(rename_all)` bridges both
- **Shared crate** ensures frontend and backend always agree on types across the IPC boundary
- **Framework requirements are seed data in migrations, not hardcoded in Rust** — adding a regulation means adding a migration, not modifying application code
- **Cross-references stored relationally** with typed relationships (equivalent, overlapping, supporting, partial)
- **Audit log is append-only** — PostgreSQL triggers reject any UPDATE or DELETE attempt

---

## Build & Run

**Prerequisites:** Rust stable toolchain, PostgreSQL 16+, `trunk` (`cargo install trunk`)

```bash
# Create database
sqlx database create

# Run migrations (schema + seed data)
sqlx migrate run

# Development mode (hot-reload frontend + backend)
cargo tauri dev

# Production build
cargo tauri build
```

---

## Design Decisions

**Why build a custom tool instead of using existing GRC platforms?**

Commercial GRC platforms (ServiceNow, OneTrust, Archer) are designed for enterprise teams with dedicated IT staff. They require cloud subscriptions, lengthy onboarding, and impose workflow assumptions that don't match a solo practitioner's needs. This tool is purpose-built for one user who needs to move fast across multiple frameworks without platform overhead.

**Why Rust end-to-end?**

A single language across the entire stack eliminates serialization mismatches, type disagreements, and npm supply-chain risk. The shared types crate guarantees that if the backend compiles, the frontend's expectations are satisfied. This is particularly important for a compliance tool where data integrity is non-negotiable.

**Why local-first with PostgreSQL?**

Client engagement data is sensitive. A local-first architecture means no cloud dependency, no data residency questions, and no third-party access. PostgreSQL was chosen over SQLite because the user already manages a local pgAdmin4 instance, and PostgreSQL's trigger system enables the immutable audit trail.

**Why an immutable audit trail?**

Professional diligence is demonstrable or it isn't. When a client or regulator asks "when did you assess this requirement, and what was the result?", the answer needs to be tamper-evident. The append-only audit log with database-enforced immutability provides that guarantee.

---

## Status

All planned features are implemented and compile. The application covers the full engagement lifecycle: intake → scoping → framework selection → requirement assessment → risk scoring → evidence management → task tracking → report generation — with an immutable audit trail across every step.

---

## Author

Built as a personal professional tool by a **Certified AI Governance Professional (CAIGP)** to support hands-on regulatory compliance work across the EU AI Act, ISO/IEC 42001, ISO/IEC 23894, NIST AI RMF, and OECD AI Principles.

---

## License

Private repository. Not open-source.
