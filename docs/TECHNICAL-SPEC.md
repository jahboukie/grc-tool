# GRC Command Center — Technical Specification

**Version:** 1.0
**Date:** April 1, 2026
**Status:** Approved for implementation

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Design Philosophy](#2-design-philosophy)
3. [Architecture Overview](#3-architecture-overview)
4. [Technology Stack](#4-technology-stack)
5. [Project Structure](#5-project-structure)
6. [Data Model — Data Structures First](#6-data-model--data-structures-first)
7. [Database Schema (PostgreSQL DDL)](#7-database-schema-postgresql-ddl)
8. [Framework Requirement Coverage](#8-framework-requirement-coverage)
9. [Cross-Reference Map](#9-cross-reference-map)
10. [Feature Specifications](#10-feature-specifications)
11. [Screen Wireframes](#11-screen-wireframes)
12. [Tauri IPC Command Reference](#12-tauri-ipc-command-reference)
13. [LLM Integration](#13-llm-integration)
14. [Report Generation](#14-report-generation)
15. [Security Considerations](#15-security-considerations)
16. [Extensibility](#16-extensibility)
17. [Build & Development](#17-build--development)

---

## 1. Executive Summary

The GRC Command Center is a **local-first desktop application** for a Certified AI Governance Professional. It serves as the **single source of truth** for managing AI governance engagements across multiple clients, tracking compliance against five regulatory frameworks, assessing risks, managing evidence, and generating compliance reports.

**Core Problem:** A governance professional cannot rely on memory alone to track hundreds of specific legal requirements across multiple frameworks and client engagements. This tool externalizes that cognitive load into a systematic, auditable platform.

**User:** Single user — the certified AI governance professional. No multi-user, no auth.

**Deployment:** Native desktop application (Windows .exe). Fully offline-capable. All data in local PostgreSQL.

### Frameworks Covered
| Framework | Identifier | Version |
|-----------|-----------|---------|
| EU AI Act | Regulation (EU) 2024/1689 | Published Aug 1, 2024 |
| ISO/IEC 42001 | AI Management System | 2023 edition |
| ISO/IEC 23894 | AI Risk Management | 2023 edition |
| NIST AI RMF | AI Risk Management Framework | 1.0 (Jan 2023) |
| OECD AI Principles | Recommendation of the Council on AI | 2019, updated 2024 |

### Key Capabilities
- Multi-engagement tracking with per-client AI system management
- Comprehensive framework requirement browser with compliance status tracking
- Cross-framework requirement mapping engine (shows overlaps)
- Visual risk assessment matrix (5×5 heat map, inherent + residual)
- Evidence vault with artifact linking to requirements, risks, and tasks
- Immutable audit trail for professional diligence
- HTML compliance report generation
- LLM-powered regulatory assistant
- Extensible architecture for future regulations

---

## 2. Design Philosophy

### Data Dominates (Rob Pike Rule 5)
> "If you've gotten your data structures right, the algorithms will almost always be self-evident."

The **entire system is designed data-structures-first**. Every feature flows naturally from the data model. Enums are defined before structs. Structs before algorithms. The PostgreSQL schema is a direct translation of the Rust types. The UI is a direct projection of the data.

### Simplicity (Rob Pike Rules 3 & 4)
No fancy algorithms. The app is CRUD operations, simple queries, straightforward templates, and a risk score that is literal multiplication (likelihood × impact). The complexity lives in the **comprehensiveness of the framework coverage**, not the code.

### Rust Only
Zero npm. Zero Node.js. The entire build pipeline runs through `cargo` and `trunk`. Styling is a vendored CSS file. The only external runtime dependency is PostgreSQL.

### Local-First
All data lives on the user's machine in PostgreSQL. No cloud dependency. No telemetry. Evidence files stored on the local filesystem. The binary is fully self-contained minus the database.

### Professional Audit Trail
Every mutation is logged. The audit trail is immutable (PostgreSQL triggers reject UPDATE/DELETE). This provides proof of professional diligence — when, what, and how every assessment decision was made.

---

## 3. Architecture Overview

```
┌──────────────────────────────────────────────────────────────┐
│                       Tauri 2 Shell                          │
│  ┌──────────────────────┐  ┌──────────────────────────────┐  │
│  │   Leptos Frontend    │  │       Rust Backend            │  │
│  │   (WASM in WebView)  │  │    (Tauri Commands)           │  │
│  │                      │  │                              │  │
│  │  • Pages/Routes      │◄─┤  • DB Layer (SQLx + PG)     │  │
│  │  • Components        │─►│  • Models (shared crate)     │  │
│  │  • Tauri IPC calls   │  │  • Report Gen (Askama)       │  │
│  │  • Reactive signals  │  │  • LLM Client (reqwest)      │  │
│  │  • Pico CSS styling  │  │  • Audit Logger              │  │
│  └──────────────────────┘  └────────────┬─────────────────┘  │
│                                         │                    │
└─────────────────────────────────────────┼────────────────────┘
                                          │
                                ┌─────────▼─────────┐
                                │   PostgreSQL 16+   │
                                │   localhost:5432   │
                                │   trust auth       │
                                │   DB: grc_command  │
                                │       _center      │
                                └────────────────────┘
```

**Data Flow:**
1. User interacts with Leptos UI (WASM running in Tauri WebView)
2. UI calls `tauri::invoke("command_name", args)` via JS interop
3. Tauri routes to `#[tauri::command]` handler in Rust backend
4. Handler uses SQLx to query/mutate PostgreSQL
5. Handler writes to `audit_log` for every mutation
6. Response serialized (serde_json) back to frontend
7. Leptos reactive signals update the UI

**Shared Types:**
The `shared` crate contains all enums and struct types. Both the Tauri backend and Leptos frontend depend on it. This ensures type safety across the IPC boundary.

---

## 4. Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Desktop Shell | Tauri 2.x | Native window, IPC, filesystem access, bundling |
| Frontend Framework | Leptos (CSR mode) | Reactive UI compiled to WASM |
| Frontend Build | trunk | WASM compilation, asset bundling |
| Language | Rust (edition 2021) | All code |
| Database | PostgreSQL 16+ | All persistent data |
| DB Driver | SQLx (async, compile-time checked) | Type-safe queries |
| Templating | Askama | HTML report generation |
| Styling | Pico CSS (vendored) + custom CSS | Classless base + overrides |
| Serialization | serde + serde_json | IPC, DB, config serialization |
| UUIDs | uuid (v4) | Primary keys |
| DateTime | chrono | All timestamps (UTC) |
| HTTP Client | reqwest | LLM API calls |
| Encryption | ring or aes-gcm | API key encryption at rest |
| Package Manager | cargo | Only package manager |

### Why These Choices
- **Tauri 2 over Electron:** Rust-native, tiny binary (~10MB vs ~150MB), no bundled Chromium
- **Leptos over React:** Same language as backend, compiles to WASM, no npm
- **PostgreSQL over SQLite:** User already has pgAdmin4 installed; better for complex queries, JSON arrays, and future scalability
- **Pico CSS over Tailwind:** No build step, classless defaults, one vendored file
- **Askama over Tera:** Compile-time template checking, zero runtime overhead

---

## 5. Project Structure

```
grc-tool/
├── CLAUDE.md                              # Project conventions for AI assistants
├── Cargo.toml                             # Workspace manifest
├── docs/
│   └── TECHNICAL-SPEC.md                  # This document
├── references/                            # Regulatory reference materials
│
├── shared/                                # Shared types crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                         # Re-exports
│       ├── enums.rs                       # All enum types
│       └── models.rs                      # All struct types
│
├── src-tauri/                             # Tauri backend
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/                             # App icons
│   ├── migrations/
│   │   ├── 001_initial_schema.sql         # Full DDL
│   │   └── 002_seed_framework_data.sql    # Framework requirements + cross-refs
│   └── src/
│       ├── main.rs                        # Entry point
│       ├── lib.rs                         # Tauri setup, command registration
│       ├── db/
│       │   ├── mod.rs
│       │   └── pool.rs                    # PgPool initialization
│       ├── models/
│       │   ├── mod.rs
│       │   ├── engagement.rs              # Engagement CRUD queries
│       │   ├── ai_system.rs               # AI System CRUD queries
│       │   ├── requirement.rs             # Requirement queries (read-only + filter)
│       │   ├── assessment.rs              # Assessment upsert/query
│       │   ├── risk.rs                    # Risk register CRUD + matrix queries
│       │   ├── evidence.rs                # Evidence CRUD + linking
│       │   ├── task.rs                    # Task CRUD
│       │   ├── audit.rs                   # Audit log (append-only queries)
│       │   └── cross_reference.rs         # Cross-reference queries
│       ├── commands/
│       │   ├── mod.rs                     # All command registrations
│       │   ├── engagement_cmds.rs
│       │   ├── system_cmds.rs
│       │   ├── assessment_cmds.rs
│       │   ├── risk_cmds.rs
│       │   ├── evidence_cmds.rs
│       │   ├── task_cmds.rs
│       │   ├── report_cmds.rs
│       │   ├── audit_cmds.rs
│       │   ├── llm_cmds.rs
│       │   ├── dashboard_cmds.rs
│       │   └── config_cmds.rs
│       ├── reports/
│       │   ├── mod.rs
│       │   └── templates/
│       │       ├── compliance_report.html # Askama template
│       │       ├── gap_analysis.html
│       │       └── risk_report.html
│       └── llm/
│           ├── mod.rs
│           └── client.rs                  # HTTP client for LLM APIs
│
└── frontend/                              # Leptos WASM frontend
    ├── Cargo.toml
    ├── Trunk.toml
    ├── index.html                         # HTML shell for trunk
    ├── src/
    │   ├── main.rs                        # Leptos mount point
    │   ├── app.rs                         # Root component + router definition
    │   ├── api/
    │   │   ├── mod.rs
    │   │   └── invoke.rs                  # Tauri invoke wrappers
    │   ├── components/
    │   │   ├── mod.rs
    │   │   ├── sidebar.rs                 # Navigation sidebar
    │   │   ├── stat_card.rs               # Dashboard stat card
    │   │   ├── compliance_bar.rs          # Horizontal compliance % bar
    │   │   ├── risk_heatmap.rs            # 5×5 risk matrix grid
    │   │   ├── task_row.rs                # Task list item
    │   │   ├── requirement_row.rs         # Requirement with status badge
    │   │   ├── evidence_card.rs           # Evidence file card
    │   │   ├── audit_row.rs              # Audit log entry
    │   │   ├── chat_bubble.rs             # LLM conversation message
    │   │   ├── status_badge.rs            # Compliance/task status badge
    │   │   └── framework_pill.rs          # Framework identifier pill
    │   └── pages/
    │       ├── mod.rs
    │       ├── dashboard.rs               # Command center overview
    │       ├── engagements.rs             # Engagement list + CRUD
    │       ├── engagement_detail.rs       # Single engagement view
    │       ├── ai_system_detail.rs        # AI system profile
    │       ├── framework_navigator.rs     # Browse requirements by framework
    │       ├── cross_reference.rs         # Cross-framework mapping view
    │       ├── risk_matrix.rs             # Risk assessment heat map
    │       ├── evidence_vault.rs          # Evidence management
    │       ├── gap_analysis.rs            # Gap analysis view
    │       ├── reports.rs                 # Report generation
    │       ├── audit_trail.rs             # Immutable audit log viewer
    │       ├── llm_assistant.rs           # LLM chat interface
    │       └── settings.rs                # App configuration
    └── style/
        ├── pico.min.css                   # Vendored Pico CSS
        └── custom.css                     # App-specific overrides
```

---

## 6. Data Model — Data Structures First

Following Rob Pike's Rule 5: define enums first, then core structs, then let algorithms emerge naturally.

All types live in the `shared` crate. Both frontend and backend depend on `shared`.

### 6.1 Enums

```rust
use serde::{Deserialize, Serialize};

/// Which regulatory/standards framework a requirement belongs to.
/// Extensible: add new variants for future regulations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Framework {
    EuAiAct,            // Regulation (EU) 2024/1689
    Iso42001,           // ISO/IEC 42001:2023
    Iso23894,           // ISO/IEC 23894:2023
    NistAiRmf,          // NIST AI RMF 1.0
    OecdAiPrinciples,   // OECD Recommendation on AI
}

/// EU AI Act risk tier (Articles 5, 6, 50).
/// Determines which obligations apply to an AI system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RiskCategory {
    Unacceptable,   // Art. 5 — prohibited, must not be deployed
    High,           // Art. 6 + Annex I/III — full compliance suite
    Limited,        // Art. 50 — transparency obligations only
    Minimal,        // Residual — voluntary codes of conduct
    Gpai,           // Art. 51–56 — General Purpose AI model rules
}

/// Assessment status for a requirement against a specific AI system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    NotAssessed,    // Default — not yet evaluated
    Met,            // Fully compliant
    Partial,        // Partially met, remediation in progress
    Gap,            // Not met, action required
    NotApplicable,  // Does not apply to this system/role
}

/// Task lifecycle status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Open,
    InProgress,
    Blocked,
    Done,
    Deferred,
}

/// Priority levels for tasks and risks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Engagement lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngagementStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

/// Evidence artifact classification.
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

/// Risk likelihood scale (1–5) for the 5×5 risk matrix.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RiskLikelihood {
    Rare            = 1,
    Unlikely        = 2,
    Possible        = 3,
    Likely          = 4,
    AlmostCertain   = 5,
}

/// Risk impact scale (1–5) for the 5×5 risk matrix.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RiskImpact {
    Negligible      = 1,
    Minor           = 2,
    Moderate        = 3,
    Major           = 4,
    Catastrophic    = 5,
}

/// Role in the AI value chain (EU AI Act Chapter III).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObligationRole {
    Provider,                   // Art. 16 — develops or commissions the AI system
    Deployer,                   // Art. 26 — uses the AI system professionally
    Importer,                   // Art. 23 — places third-country system on EU market
    Distributor,                // Art. 24 — makes system available in supply chain
    AuthorizedRepresentative,   // Art. 22 — acts on provider's behalf in EU
    ProductManufacturer,        // Art. 25 — integrates AI into a product
}

/// Action types for the immutable audit trail.
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

/// Relationship between cross-referenced requirements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CrossRefRelationship {
    Equivalent,     // Functionally the same obligation
    Overlapping,    // Significant shared scope, not identical
    Supports,       // One requirement helps satisfy the other
    Extends,        // One goes beyond the other in scope
}
```

### 6.2 Core Entities

```rust
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

/// A client engagement or project. Top-level organizational unit.
/// One engagement contains multiple AI systems being assessed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engagement {
    pub id: Uuid,
    pub name: String,                       // "Acme Corp Phase 1"
    pub client_name: String,                // "Acme Corp" or "Personal"
    pub description: String,
    pub status: EngagementStatus,
    pub primary_role: ObligationRole,       // Your role in this engagement
    pub frameworks: Vec<Framework>,          // Which frameworks apply
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An AI system being assessed within an engagement.
/// Compliance is measured at this level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSystem {
    pub id: Uuid,
    pub engagement_id: Uuid,                // FK → engagements
    pub name: String,                       // "Customer Chatbot v2"
    pub description: String,
    pub intended_purpose: String,           // EU AI Act Art. 13(3)(b)(i)
    pub risk_category: RiskCategory,
    pub domain: String,                     // "Healthcare", "Finance", "HR", etc.
    pub is_gpai: bool,                      // General Purpose AI? Art. 51
    pub is_high_risk_listed: bool,          // Listed in Annex III?
    pub is_safety_component: bool,          // Safety component per Annex I?
    pub deployment_context: String,         // Where/how it's deployed
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A specific requirement from a regulatory framework.
/// SEED DATA — inserted via migrations, not created by the user.
/// This is the reference backbone that powers all compliance tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkRequirement {
    pub id: Uuid,
    pub framework: Framework,
    pub reference_id: String,               // "EU-AIA-ART9", "ISO42001-6.1"
    pub title: String,
    pub description: String,
    pub article_clause: String,             // "Article 9(2)(a)", "Clause 6.1.2"
    pub category: String,                   // "Risk Management", "Transparency"
    pub subcategory: Option<String>,
    pub applicable_risk_categories: Vec<RiskCategory>,
    pub applicable_roles: Vec<ObligationRole>,
    pub is_mandatory: bool,
    pub guidance_text: String,              // Practitioner-level guidance
    pub implementation_notes: String,       // What "compliance" looks like in practice
    pub sort_order: i32,                    // Display ordering within framework
}

/// Assessment of one requirement for one AI system.
/// This is what the user fills in during compliance review.
/// The combination (ai_system_id, requirement_id) is unique.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementAssessment {
    pub id: Uuid,
    pub ai_system_id: Uuid,                // FK → ai_systems
    pub requirement_id: Uuid,              // FK → framework_requirements
    pub status: ComplianceStatus,
    pub assessor_notes: String,
    pub remediation_plan: String,
    pub target_date: Option<NaiveDate>,
    pub assessed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Mapping between requirements across different frameworks.
/// Shows where obligations overlap, enabling unified compliance.
/// Seeded via migrations. Read-only in normal use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub id: Uuid,
    pub source_requirement_id: Uuid,        // FK → framework_requirements
    pub target_requirement_id: Uuid,        // FK → framework_requirements
    pub relationship: CrossRefRelationship,
    pub notes: String,
}

/// A risk in the risk register for a specific AI system.
/// Supports inherent risk scoring and residual risk (after mitigation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEntry {
    pub id: Uuid,
    pub ai_system_id: Uuid,                // FK → ai_systems
    pub title: String,                      // "Training data bias in hiring model"
    pub description: String,
    pub risk_source: String,                // What causes this risk
    pub affected_rights: Vec<String>,       // Fundamental rights impacted
    pub likelihood: RiskLikelihood,
    pub impact: RiskImpact,
    pub inherent_score: i32,                // likelihood_val × impact_val (1–25)
    pub mitigation_measures: String,
    pub residual_likelihood: Option<RiskLikelihood>,
    pub residual_impact: Option<RiskImpact>,
    pub residual_score: Option<i32>,
    pub related_requirement_ids: Vec<Uuid>, // FK → framework_requirements
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An evidence artifact stored on the local filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub engagement_id: Uuid,                // FK → engagements
    pub file_name: String,                  // Original filename
    pub file_path: String,                  // Absolute local path
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub tags: Vec<String>,
    pub uploaded_at: DateTime<Utc>,
}

/// Links evidence to assessments, risks, or tasks (polymorphic join).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceLink {
    pub id: Uuid,
    pub evidence_id: Uuid,                          // FK → evidence
    pub requirement_assessment_id: Option<Uuid>,     // FK → requirement_assessments
    pub risk_entry_id: Option<Uuid>,                // FK → risk_entries
    pub task_id: Option<Uuid>,                      // FK → tasks
}

/// Action item within an engagement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub engagement_id: Uuid,                // FK → engagements
    pub ai_system_id: Option<Uuid>,         // FK → ai_systems (optional scope)
    pub title: String,
    pub description: String,
    pub framework: Option<Framework>,
    pub related_requirement_id: Option<Uuid>, // FK → framework_requirements
    pub status: TaskStatus,
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Immutable audit log entry. APPEND-ONLY.
/// PostgreSQL triggers prevent UPDATE and DELETE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub entity_type: String,                // "engagement", "ai_system", etc.
    pub entity_id: Uuid,
    pub action: AuditAction,
    pub field_changed: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub details: String,
    pub timestamp: DateTime<Utc>,           // Set once, never changed
}

/// LLM conversation turn (query + response pair).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConversation {
    pub id: Uuid,
    pub engagement_id: Option<Uuid>,        // FK → engagements (optional context)
    pub ai_system_id: Option<Uuid>,         // FK → ai_systems (optional context)
    pub query: String,
    pub response: String,
    pub model_used: String,
    pub created_at: DateTime<Utc>,
}

/// Application configuration. Singleton — one row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub id: Uuid,
    pub llm_provider: String,               // "openai", "anthropic", "ollama"
    pub llm_api_key_encrypted: String,      // Encrypted at rest
    pub llm_model: String,                  // "gpt-4o", "claude-sonnet-4-20250514", etc.
    pub evidence_storage_path: String,      // Local directory for evidence files
    pub db_host: String,
    pub db_port: i32,
    pub db_name: String,
    pub updated_at: DateTime<Utc>,
}
```

### 6.3 Entity Relationship Diagram (Text)

```
Engagement (1) ──── (N) AiSystem
     │                      │
     │                      ├──── (N) RequirementAssessment ──── (1) FrameworkRequirement
     │                      │                                          │
     │                      ├──── (N) RiskEntry                       │
     │                      │                                    CrossReference
     │                      │                                   (M:N self-join)
     ├──── (N) Evidence
     │           │
     │           └──── (N) EvidenceLink ──── RequirementAssessment
     │                                  ──── RiskEntry
     │                                  ──── Task
     ├──── (N) Task
     │
     └──── (N) LlmConversation

AuditLog (standalone, references any entity by type + id)
AppConfig (singleton)
```

---

## 7. Database Schema (PostgreSQL DDL)

This is migration `001_initial_schema.sql`. All tables, constraints, indexes, and triggers.

```sql
-- =============================================
-- GRC Command Center — 001_initial_schema.sql
-- =============================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================
-- TABLES (ordered by dependency)
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

-- IMMUTABLE audit log — append only
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
-- SEED DEFAULT CONFIG
-- =============================================

INSERT INTO app_config (id, evidence_storage_path, db_name)
VALUES (uuid_generate_v4(), '', 'grc_command_center');
```

---

## 8. Framework Requirement Coverage

Framework requirements are **seed data** inserted via migration `002_seed_framework_data.sql`. They are read-only in normal application use. Below is the category structure and representative examples for each framework.

### 8.1 EU AI Act — Requirement Categories

| Category | Articles | Req Count | Applies To |
|----------|----------|-----------|------------|
| Prohibited Practices | Art. 5 | 8 | All |
| Risk Classification | Art. 6, Annex I/III | 5 | All |
| Risk Management System | Art. 9 | 6 | High-risk |
| Data & Data Governance | Art. 10 | 5 | High-risk |
| Technical Documentation | Art. 11, Annex IV | 4 | High-risk |
| Record-Keeping & Logging | Art. 12 | 3 | High-risk |
| Transparency to Deployers | Art. 13 | 4 | High-risk |
| Human Oversight | Art. 14 | 4 | High-risk |
| Accuracy, Robustness, Security | Art. 15 | 4 | High-risk |
| Provider Obligations | Art. 16–21 | 6 | Providers |
| Deployer Obligations | Art. 26 | 5 | Deployers |
| Fundamental Rights Impact | Art. 27 | 3 | Public deployers |
| Conformity Assessment | Art. 43, Annex VI/VII | 4 | High-risk |
| Quality Management System | Art. 17 | 4 | Providers |
| Transparency (Limited Risk) | Art. 50 | 4 | Limited risk |
| GPAI Model Obligations | Art. 51–56 | 6 | GPAI |
| Post-Market Monitoring | Art. 72 | 3 | Providers |
| Serious Incident Reporting | Art. 73 | 3 | Providers |

**Representative seed entries:**

```sql
INSERT INTO framework_requirements (id, framework, reference_id, title, description,
    article_clause, category, applicable_risk_categories, applicable_roles,
    is_mandatory, guidance_text, implementation_notes, sort_order)
VALUES
(uuid_generate_v4(), 'eu_ai_act', 'EU-AIA-ART5-PROHIB',
 'Prohibited AI Practices Screening',
 'Verify the AI system does not employ any of the prohibited techniques listed in Article 5.',
 'Article 5(1)(a)–(h)',
 'Prohibited Practices',
 '{unacceptable,high,limited,minimal,gpai}',
 '{provider,deployer,importer,distributor}',
 TRUE,
 'Screen the AI system against each prohibition: subliminal manipulation, exploitation of vulnerabilities, social scoring, real-time remote biometric identification (with exceptions), emotion inference in workplace/education, untargeted facial image scraping, biometric categorization for sensitive attributes.',
 'Document the screening result for each prohibition. If any apply, the system MUST NOT be placed on the market or put into service.',
 10),

(uuid_generate_v4(), 'eu_ai_act', 'EU-AIA-ART9-RMS',
 'Risk Management System',
 'Establish, implement, document, and maintain a risk management system for the high-risk AI system.',
 'Article 9(1)–(8)',
 'Risk Management System',
 '{high}',
 '{provider}',
 TRUE,
 'The RMS is a continuous iterative process running throughout the entire lifecycle. It must: (a) identify and analyse known/foreseeable risks, (b) estimate and evaluate risks, (c) evaluate risks from post-market data, (d) adopt suitable risk management measures. Testing must be done prior to placing on market.',
 'Create and maintain a documented RMS that is reviewed at least annually. Include risk identification methodology, risk analysis results, risk evaluation criteria, and risk treatment plans.',
 30),

(uuid_generate_v4(), 'eu_ai_act', 'EU-AIA-ART27-FRIA',
 'Fundamental Rights Impact Assessment',
 'Conduct a fundamental rights impact assessment before deploying a high-risk AI system.',
 'Article 27(1)–(4)',
 'Fundamental Rights Impact',
 '{high}',
 '{deployer}',
 TRUE,
 'Required for public bodies and private entities providing public services, plus entities using credit scoring or health/life insurance AI. Assess: deployer processes, affected persons/groups, specific risks, human oversight measures, impact on vulnerable groups, expected societal impact.',
 'Use a FRIA template. Document the assessment before first deployment. Notify the national supervisory authority of the assessment results. Re-assess when circumstances change materially.',
 70);
```

### 8.2 ISO/IEC 42001 — Requirement Categories

| Category | Clauses | Req Count |
|----------|---------|-----------|
| Context of the Organization | 4.1–4.4 | 4 |
| Leadership & Commitment | 5.1–5.3 | 3 |
| Planning — Risks & Opportunities | 6.1 | 3 |
| AI Management System Objectives | 6.2 | 2 |
| Support — Resources & Competence | 7.1–7.3 | 3 |
| Support — Communication | 7.4 | 1 |
| Support — Documented Information | 7.5 | 2 |
| Operational Planning & Control | 8.1–8.4 | 4 |
| Performance Evaluation | 9.1–9.3 | 4 |
| Improvement | 10.1–10.2 | 2 |
| Annex A — AI Policies (A.2–A.4) | Annex A | 3 |
| Annex A — AI System Lifecycle (A.5–A.6) | Annex A | 3 |
| Annex A — Data Management (A.7) | Annex A | 2 |
| Annex A — Transparency & Oversight (A.8–A.9) | Annex A | 3 |
| Annex A — System Lifecycle (A.10) | Annex A | 2 |

### 8.3 ISO/IEC 23894 — Requirement Categories

| Category | Clauses | Req Count |
|----------|---------|-----------|
| Principles | 5.1–5.8 | 8 |
| Framework — Leadership & Commitment | 6.2 | 2 |
| Framework — Integration | 6.3 | 1 |
| Framework — Design | 6.4 | 2 |
| Framework — Implementation | 6.5 | 2 |
| Framework — Evaluation | 6.6 | 1 |
| Framework — Improvement | 6.7 | 1 |
| Process — Communication & Consultation | 7.1 | 1 |
| Process — Scope, Context, Criteria | 7.2 | 2 |
| Process — Risk Assessment | 7.3 (7.3.1–7.3.3) | 4 |
| Process — Risk Treatment | 7.4 | 2 |
| Process — Monitoring & Review | 7.5 | 2 |
| Process — Recording & Reporting | 7.6 | 2 |

### 8.4 NIST AI RMF — Requirement Categories

| Function | Subcategories | Req Count |
|----------|--------------|-----------|
| GOVERN 1 — Policies for AI risk management | 1.1–1.7 | 7 |
| GOVERN 2 — Accountability structures | 2.1–2.3 | 3 |
| GOVERN 3 — Workforce diversity & culture | 3.1–3.2 | 2 |
| GOVERN 4 — Organizational practices | 4.1–4.3 | 3 |
| GOVERN 5 — Processes for engagement | 5.1–5.2 | 2 |
| GOVERN 6 — Policies for oversight | 6.1–6.2 | 2 |
| MAP 1 — Context establishment | 1.1–1.6 | 6 |
| MAP 2 — AI categorization | 2.1–2.3 | 3 |
| MAP 3 — Benefits, costs, risks | 3.1–3.5 | 5 |
| MAP 4 — Risk prioritization | 4.1–4.2 | 2 |
| MAP 5 — Stakeholder engagement | 5.1–5.2 | 2 |
| MEASURE 1 — Metrics identified | 1.1–1.3 | 3 |
| MEASURE 2 — AI system evaluated | 2.1–2.13 | 13 |
| MEASURE 3 — Mechanisms for tracking | 3.1–3.3 | 3 |
| MEASURE 4 — Feedback incorporated | 4.1–4.2 | 2 |
| MANAGE 1 — Risks responded to | 1.1–1.4 | 4 |
| MANAGE 2 — Strategies to maximize benefit | 2.1–2.4 | 4 |
| MANAGE 3 — Risk decisions documented | 3.1–3.2 | 2 |
| MANAGE 4 — Residual risks documented | 4.1–4.2 | 2 |

### 8.5 OECD AI Principles — Requirement Categories

| Principle | Req Count |
|-----------|-----------|
| 1 — Inclusive Growth, Sustainable Development, Well-being | 3 |
| 2 — Human-Centred Values and Fairness | 4 |
| 3 — Transparency and Explainability | 3 |
| 4 — Robustness, Security, and Safety | 3 |
| 5 — Accountability | 3 |

### 8.6 Total Requirement Count Estimate

| Framework | Estimated Requirements |
|-----------|----------------------|
| EU AI Act | ~80 |
| ISO/IEC 42001 | ~45 |
| ISO/IEC 23894 | ~30 |
| NIST AI RMF | ~70 |
| OECD AI Principles | ~16 |
| **Total** | **~241** |

---

## 9. Cross-Reference Map

The cross-reference map shows where requirements **overlap** across frameworks. This enables the user to satisfy multiple frameworks with a single control/action.

### Key Cross-References

| # | EU AI Act | ISO/IEC 42001 | ISO/IEC 23894 | NIST AI RMF | OECD | Relationship |
|---|-----------|---------------|---------------|-------------|------|-------------|
| 1 | Art. 9 — Risk Management System | 6.1 — Risks & opportunities | 7.3 — Risk Assessment | MAP + MEASURE | Principle 4 | Overlapping |
| 2 | Art. 10 — Data Governance | A.7 — Data management | — | MAP 3 | Principle 2 | Overlapping |
| 3 | Art. 11 — Technical Documentation | 7.5 — Documented info | 7.6 — Recording & reporting | GOVERN 4 | Principle 3 | Supports |
| 4 | Art. 12 — Record-Keeping | 7.5 — Documented info | 7.6 — Recording & reporting | GOVERN 1 | Principle 5 | Supports |
| 5 | Art. 13 — Transparency | A.8 — Transparency | — | GOVERN 2 | Principle 3 | Equivalent |
| 6 | Art. 14 — Human Oversight | A.9 — Human oversight | — | MANAGE 3 | Principle 2 | Equivalent |
| 7 | Art. 15 — Accuracy/Robustness | A.10 — System lifecycle | 7.4 — Risk treatment | MEASURE 2 | Principle 4 | Overlapping |
| 8 | Art. 17 — Quality Management | 8.1 — Operational planning | 6.5 — Implementation | GOVERN 1 | Principle 5 | Overlapping |
| 9 | Art. 27 — FRIA | 4.1 — Context of org | 7.2 — Scope & context | MAP 1 | Principle 1+2 | Supports |
| 10 | Art. 43 — Conformity Assessment | 9.2 — Internal audit | 6.6 — Evaluation | MEASURE 1 | Principle 5 | Overlapping |
| 11 | Art. 72 — Post-Market Monitoring | 9.1 — Monitoring/evaluation | 7.5 — Monitoring & review | MANAGE 4 | Principle 4 | Overlapping |
| 12 | Art. 5 — Prohibited Practices | — | 7.3.2 — Risk identification | MAP 2 | Principle 2 | Supports |
| 13 | Art. 51–56 — GPAI Obligations | A.2–A.4 — AI policies | — | GOVERN 1, 6 | Principle 3 | Supports |
| 14 | Art. 73 — Incident Reporting | 10.1 — Nonconformity | 7.5 — Monitoring | MANAGE 1 | Principle 5 | Overlapping |

These cross-references are seeded in the `cross_references` table. The UI allows the user to select any requirement and see all its cross-framework mappings.

---

## 10. Feature Specifications

### 10.1 Dashboard (Command Center)

**Purpose:** At-a-glance overview of all active work. The first screen the user sees.

**Data required:**
- `engagements` WHERE status = 'active'
- Aggregated compliance % per framework (from requirement_assessments)
- Top 10 tasks WHERE status IN ('open','in_progress') ORDER BY priority, due_date
- Count of open risks by severity
- Count of unaddressed gaps

**Computed values:**
- Compliance % = (assessments WHERE status = 'met') / (total assessments - not_applicable) × 100
- Risk summary = COUNT of risk_entries grouped by inherent_score ranges

**Interactions:**
- Click engagement card → navigate to engagement detail
- Click task → navigate to task (inline edit)
- Click framework compliance bar → navigate to framework navigator filtered to that framework

### 10.2 Engagement Manager

**Purpose:** CRUD for engagements. The user manages one engagement per client project.

**Views:**
- List view: all engagements with status, client name, # AI systems, compliance summary
- Create/Edit dialog: name, client_name, description, status, primary_role, frameworks (multi-select)

**Logic:**
- Creating/updating writes to `audit_log`
- Deleting an engagement cascades to AI systems, assessments, risks, tasks, evidence

### 10.3 AI System Profile

**Purpose:** Full details of a single AI system. The heart of compliance work.

**Sections:**
1. **Header:** Name, risk category badge, engagement name, domain
2. **Classification Panel:** Risk category with reasoning, GPAI flag, Annex III flag, safety component flag
3. **Compliance Summary:** Per-framework progress bars showing % of applicable requirements met
4. **Risk Summary:** Mini risk matrix showing this system's risks plotted
5. **Recent Assessments:** Last 5 requirement assessments with status
6. **Linked Evidence:** Evidence artifacts attached to this system's assessments

**Risk Classification Decision Tree** (embedded helper):
- Is it in Annex I (safety component)? → likely High
- Is it in Annex III? → High
- Does Art. 6(3) exception apply? → May be reclassified
- Is it a GPAI model? → GPAI category
- Does it interact with persons? → at least Limited
- Otherwise → Minimal

### 10.4 Framework Navigator

**Purpose:** Browse all requirements grouped by framework and category. Assess each one per AI system.

**Layout:**
- Left: framework selector (tabs or dropdown)
- Main: requirements listed by category, each showing:
  - Reference ID, title, article/clause
  - Current assessment status (badge)
  - Mandatory flag
- Right panel (on click): requirement detail with guidance text, implementation notes, assessment editor

**Assessment Editor (right panel):**
- Status dropdown (NotAssessed/Met/Partial/Gap/NA)
- Assessor notes (textarea)
- Remediation plan (textarea, shown when status = partial or gap)
- Target date (date picker, shown when status = partial or gap)
- Save button

**Filtering:**
- By framework (tabs)
- By category (dropdown)
- By assessment status (dropdown: all, met, partial, gap, not_assessed)
- By risk category applicability (dropdown)
- Text search on title/description

### 10.5 Cross-Reference Engine

**Purpose:** Show where requirements overlap across frameworks. Enables the user to address multiple frameworks with a single control.

**Views:**

**A. Single Requirement View:**
- Select any requirement → see all cross-referenced requirements from other frameworks
- Each cross-reference shows: target requirement title, framework, article, relationship type, notes
- Shows assessment status of the target requirement for the current AI system

**B. Matrix View:**
- Select two frameworks (e.g., EU AI Act × ISO 42001)
- Display a matrix: rows = categories from framework A, cols = categories from framework B
- Cells show count of cross-references between those categories
- Click cell to drill down

### 10.6 Risk Assessment Matrix (Heat Map)

**Purpose:** Visual 5×5 risk matrix for risk assessment and visualization.

**Layout:**
- 5×5 grid (Likelihood vs Impact)
- Cells color-coded:
  - 1–4 = Green (Low)
  - 5–9 = Yellow (Medium)
  - 10–14 = Orange (High)
  - 15–19 = Red (Very High)
  - 20–25 = Dark Red (Critical)
- Risk entries plotted as dots/badges on the grid
- Toggle: Inherent Risk / Residual Risk view

**Scope:**
- Dropdown to select: All engagements / specific engagement / specific AI system
- Filtered risk entries populate the grid

**Interactions:**
- Click a cell → show all risks at that score level
- Click a risk entry → expand details with edit capability
- "Add Risk" button → form to create new risk entry

**Risk Entry Form:**
- Title, description, risk source
- Affected fundamental rights (multi-select)
- Likelihood (5-level dropdown)
- Impact (5-level dropdown)
- Inherent score (auto-calculated)
- Mitigation measures (textarea)
- Residual likelihood + impact (optional)
- Related requirements (multi-select from framework_requirements)
- Status, priority

### 10.7 Evidence Vault

**Purpose:** Upload, store, and link evidence artifacts.

**Layout:**
- Grid/list of evidence cards
- Each card: file name, evidence type badge, file size, upload date, description, tags
- Filter by: engagement, evidence type, tags, search
- Upload button → file picker dialog

**Upload Flow:**
1. User clicks "Upload Evidence"
2. Tauri native file picker opens
3. File is copied to `evidence_storage_path/{engagement_id}/{filename}`
4. Evidence record created in database
5. Audit log entry written

**Linking:**
- From within a requirement assessment → "Attach Evidence" button
- From within a risk entry → "Attach Evidence" button
- From within a task → "Attach Evidence" button
- Evidence can be linked to multiple entities simultaneously

### 10.8 Gap Analysis

**Purpose:** Automatically computed view of all compliance gaps.

**Data source:** requirement_assessments WHERE status IN ('gap', 'partial')

**Layout:**
- Grouped by framework, then by category
- Each gap shows:
  - Requirement reference ID, title
  - Status badge (gap = red, partial = orange)
  - Assessor notes
  - Remediation plan
  - Target date
  - Cross-references (shortcuts to related requirements in other frameworks)

**Filters:**
- By engagement / AI system
- By framework
- By status (gap only, partial only, both)

**Summary stats at top:**
- Total gaps by framework
- % of requirements in gap state
- Upcoming target dates

### 10.9 Report Generator

**Purpose:** Generate HTML compliance reports.

**Available report types:**
1. **Full Compliance Report** — Per engagement: all AI systems, all frameworks, all assessments
2. **Gap Analysis Report** — Per engagement or AI system: all gaps with remediation plans
3. **Risk Assessment Report** — Per AI system: risk register with heat map visualization

**Workflow:**
1. User selects report type
2. User selects scope (engagement / AI system)
3. User clicks "Generate"
4. Backend renders Askama template with queried data
5. HTML file saved to local filesystem (configurable directory)
6. File opened in default browser via Tauri shell

**Report includes:**
- Header with engagement details, generation date, report type
- Framework-by-framework breakdown
- Compliance status summary with visual bars
- Detailed findings (per requirement)
- Risk matrix visualization (CSS grid, inline)
- Evidence references
- Audit trail excerpt

### 10.10 Audit Trail

**Purpose:** View the immutable audit log. Proof of professional diligence.

**Layout:**
- Reverse-chronological table
- Columns: Timestamp, Entity Type, Action, Details, Old Value → New Value

**Filters:**
- Date range
- Entity type (dropdown: engagement, ai_system, requirement_assessment, etc.)
- Action type
- Entity ID (text search)

**Key property:** Read-only. No editing, no deletion from the UI. The database triggers enforce this.

### 10.11 LLM Assistant

**Purpose:** AI-powered regulatory Q&A. Context-aware assistance for the governance professional.

**Layout:**
- Chat interface (messages top-down)
- Input box at bottom
- Context selector: engagement + AI system (optional)
- Model indicator showing which LLM is active

**System prompt template:**
```
You are an AI Governance regulatory assistant for a Certified AI Governance Professional.

Context:
- Engagement: {engagement_name} (Role: {primary_role})
- AI System: {system_name} (Risk: {risk_category}, Domain: {domain})
- Frameworks in scope: {frameworks}
- Current compliance posture: {met}/{total} requirements met ({pct}%)

Provide responses that:
1. Reference specific articles, clauses, and sections
2. Give actionable, practitioner-level guidance
3. Flag cross-framework implications
4. Suggest specific evidence that would demonstrate compliance
5. Be precise — cite the regulation, not just general advice
```

**Conversation stored in** `llm_conversations` for reference.

### 10.12 Settings

**Purpose:** App configuration.

**Sections:**
1. **LLM Configuration:** Provider (dropdown: OpenAI/Anthropic/Ollama), API key (password field), model name
2. **Evidence Storage:** Directory path picker
3. **Database:** Host, port, DB name (display only, configured at install)
4. **About:** Version, build date

---

## 11. Screen Wireframes

### 11.1 Dashboard

```
┌──────────────────────────────────────────────────────────────────────────┐
│  GRC COMMAND CENTER                                        [⚙ Settings] │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  ┌─ ACTIVE ENGAGEMENTS ──────────────────────────────┐    │
│             │  │                                                    │    │
│  ● Dashboard│  │  ┌────────────┐ ┌────────────┐ ┌────────────┐    │    │
│  □ Engage-  │  │  │ Acme Corp  │ │ Beta Inc   │ │ Personal   │    │    │
│    ments    │  │  │ 3 systems  │ │ 1 system   │ │ 2 systems  │    │    │
│  □ Frame-   │  │  │ ██░░ 68%  │ │ ████ 45%  │ │ ████ 88%  │    │    │
│    works    │  │  │ Active     │ │ Active     │ │ Active     │    │    │
│  □ Cross-   │  │  └────────────┘ └────────────┘ └────────────┘    │    │
│    Refs     │  └────────────────────────────────────────────────────┘    │
│  □ Risk     │                                                            │
│    Matrix   │  ┌─ COMPLIANCE POSTURE (all engagements) ────────────┐    │
│  □ Evidence │  │                                                    │    │
│  □ Gap      │  │  EU AI Act       ████████████░░░░░░  68%          │    │
│    Analysis │  │  ISO/IEC 42001   ██████████████████░  83%          │    │
│  □ Reports  │  │  ISO/IEC 23894   ████████████████████  95%         │    │
│  □ Audit    │  │  NIST AI RMF     ██████████░░░░░░░░  50%          │    │
│    Trail    │  │  OECD Principles ███████████████████░  92%         │    │
│  □ AI       │  └────────────────────────────────────────────────────┘    │
│    Assistant│                                                            │
│             │  ┌─ PRIORITY TASKS ──────────────────────────────────┐    │
│             │  │  ⚡ CRITICAL  Complete FRIA — Acme System X       │    │
│             │  │  ⚡ HIGH      Review Art.9 RMS — Beta Inc         │    │
│             │  │  📋 MEDIUM    Update ISO 42001 Annex A controls   │    │
│             │  │  📋 MEDIUM    Attach evidence for Art. 11 docs    │    │
│             │  │  📋 LOW       Review OECD Principle 3 alignment   │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  ┌─ QUICK STATS ─────────────────────────────────────┐    │
│             │  │  6 AI Systems │ 23 Open Tasks │ 8 Open Risks │    │    │
│             │  │  12 Gaps      │ 47 Evidence   │ 241 Reqs     │    │    │
│             │  └────────────────────────────────────────────────────┘    │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.2 Engagement Detail

```
┌──────────────────────────────────────────────────────────────────────────┐
│  ← Engagements                                            [Edit] [Del]  │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  ACME CORP ENGAGEMENT                                     │
│             │  Client: Acme Corporation                                  │
│             │  Role: Provider │ Status: Active                           │
│             │  Frameworks: EU AI Act, ISO 42001, NIST AI RMF            │
│             │                                                            │
│             │  ┌─ AI SYSTEMS ──────────────────────────────────────┐    │
│             │  │                                                    │    │
│             │  │  ┌──────────────────────────────────────────────┐ │    │
│             │  │  │ ● Customer Chatbot v2    [HIGH]  Healthcare │ │    │
│             │  │  │   EU: 72% │ ISO42001: 80% │ NIST: 55%      │ │    │
│             │  │  │   5 risks │ 3 open tasks                     │ │    │
│             │  │  └──────────────────────────────────────────────┘ │    │
│             │  │  ┌──────────────────────────────────────────────┐ │    │
│             │  │  │ ● Fraud Detection Model  [HIGH]  Finance    │ │    │
│             │  │  │   EU: 65% │ ISO42001: 78% │ NIST: 48%      │ │    │
│             │  │  │   7 risks │ 5 open tasks                     │ │    │
│             │  │  └──────────────────────────────────────────────┘ │    │
│             │  │  ┌──────────────────────────────────────────────┐ │    │
│             │  │  │ ● Internal Summarizer    [LIMITED] General   │ │    │
│             │  │  │   EU: 90% │ ISO42001: 92% │ NIST: 70%      │ │    │
│             │  │  │   1 risk  │ 1 open task                      │ │    │
│             │  │  └──────────────────────────────────────────────┘ │    │
│             │  │                                                    │    │
│             │  │  [+ Add AI System]                                │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  ┌─ ENGAGEMENT EVIDENCE (47 files) ──────────────────┐    │
│             │  │  View all →                                        │    │
│             │  └────────────────────────────────────────────────────┘    │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.3 Framework Navigator

```
┌──────────────────────────────────────────────────────────────────────────┐
│  FRAMEWORK NAVIGATOR          AI System: [Customer Chatbot v2 ▾]        │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  [EU AI Act] [ISO 42001] [ISO 23894] [NIST] [OECD]       │
│             │                                                            │
│             │  Filter: [All Statuses ▾] [All Categories ▾] [Search...] │
│             │                                                            │
│             │  ┌─ RISK MANAGEMENT SYSTEM (Art. 9) ─────────────────┐   │
│             │  │                                                    │    │
│             │  │  EU-AIA-ART9-RMS                          [GAP]   │    │
│             │  │  Risk Management System                            │    │
│             │  │  Article 9(1)–(8) │ Mandatory │ High-risk         │    │
│             │  │                                                    │    │
│             │  │  EU-AIA-ART9-2A                        [PARTIAL]  │    │
│             │  │  Risk Identification & Analysis                    │    │
│             │  │  Article 9(2)(a) │ Mandatory │ High-risk          │    │
│             │  │                                                    │    │
│             │  │  EU-AIA-ART9-2B                           [MET]   │    │
│             │  │  Risk Estimation & Evaluation                      │    │
│             │  │  Article 9(2)(b) │ Mandatory │ High-risk          │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  ┌─ DATA & DATA GOVERNANCE (Art. 10) ────────────────┐   │
│             │  │  ...                                               │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
├─────────────┤  ┌─ DETAIL PANEL (on row click) ─────────────────────┐   │
│             │  │                                                    │    │
│             │  │  EU-AIA-ART9-RMS — Risk Management System         │    │
│             │  │  Article 9(1)–(8) │ Applies to: HIGH risk         │    │
│             │  │                                                    │    │
│             │  │  GUIDANCE:                                         │    │
│             │  │  The RMS is a continuous iterative process...      │    │
│             │  │                                                    │    │
│             │  │  IMPLEMENTATION:                                   │    │
│             │  │  Create and maintain a documented RMS that is...   │    │
│             │  │                                                    │    │
│             │  │  CROSS-REFERENCES:                                 │    │
│             │  │  • ISO42001-6.1 (Overlapping)                     │    │
│             │  │  • ISO23894-7.3 (Overlapping)                     │    │
│             │  │  • NIST MAP+MEASURE (Overlapping)                 │    │
│             │  │                                                    │    │
│             │  │  ── ASSESSMENT ───────────────────────             │    │
│             │  │  Status: [Gap ▾]                                   │    │
│             │  │  Notes: [________________________]                 │    │
│             │  │  Remediation: [____________________]              │    │
│             │  │  Target Date: [2026-06-15]                        │    │
│             │  │  Evidence: [+ Attach]                              │    │
│             │  │                                    [Save]          │    │
│             │  └────────────────────────────────────────────────────┘    │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.4 Risk Assessment Matrix (Heat Map)

```
┌──────────────────────────────────────────────────────────────────────────┐
│  RISK ASSESSMENT MATRIX    [Engagement ▾] [AI System ▾] [● Inherent ○ Residual]│
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │   IMPACT ↑                                                │
│             │   ┌────────────┬──────────┬──────────┬──────────┬────────┐│
│             │   │Catastrophic│  5 🟡    │ 10 🟠   │ 15 🔴   │ 20 🔴  ││
│             │   │   (5)      │          │          │  ●R-003  │ 25 🔴  ││
│             │   ├────────────┼──────────┼──────────┼──────────┼────────┤│
│             │   │Major (4)   │  4 🟢    │  8 🟡   │ 12 🟠   │ 16 🔴  ││
│             │   │            │          │          │ ●R-001   │ 20 🔴  ││
│             │   ├────────────┼──────────┼──────────┼──────────┼────────┤│
│             │   │Moderate (3)│  3 🟢    │  6 🟡   │  9 🟡   │ 12 🟠  ││
│             │   │            │          │ ●R-002   │          │ 15 🔴  ││
│             │   ├────────────┼──────────┼──────────┼──────────┼────────┤│
│             │   │Minor (2)   │  2 🟢    │  4 🟢   │  6 🟡   │  8 🟡  ││
│             │   │            │          │          │          │ 10 🟠  ││
│             │   ├────────────┼──────────┼──────────┼──────────┼────────┤│
│             │   │Negligible  │  1 🟢    │  2 🟢   │  3 🟢   │  4 🟢  ││
│             │   │   (1)      │          │          │          │  5 🟡  ││
│             │   └────────────┴──────────┴──────────┴──────────┴────────┘│
│             │                Rare(1)  Unlikely(2) Poss(3)  Likely(4)    │
│             │                                             AlmCert(5)    │
│             │                         LIKELIHOOD →                      │
│             │                                                            │
│             │   ┌─ RISK REGISTER ──────────────────────────────────┐    │
│             │   │                                                   │    │
│             │   │  ●R-001  Training data bias           🟠 12/25  │    │
│             │   │          Likely(4) × Moderate(3)  Status: Open   │    │
│             │   │          Affects: Non-discrimination, Fairness   │    │
│             │   │                                                   │    │
│             │   │  ●R-002  Model opacity / explainability  🟡 6/25│    │
│             │   │          Unlikely(2) × Moderate(3) Status: Open  │    │
│             │   │          Affects: Transparency, Due process      │    │
│             │   │                                                   │    │
│             │   │  ●R-003  Data breach / adversarial attack 🔴 20/25│   │
│             │   │          Likely(4) × Catastrophic(5) Status: Open│    │
│             │   │          Affects: Privacy, Security              │    │
│             │   │                                                   │    │
│             │   │  [+ Add Risk Entry]                              │    │
│             │   └───────────────────────────────────────────────────┘    │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.5 Cross-Reference View

```
┌──────────────────────────────────────────────────────────────────────────┐
│  CROSS-REFERENCE MAP             [AI System ▾] for assessment status    │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  Search: [Art. 9________________]                         │
│             │                                                            │
│             │  ┌─ SOURCE REQUIREMENT ──────────────────────────────┐    │
│             │  │  EU-AIA-ART9-RMS — Risk Management System         │    │
│             │  │  EU AI Act │ Article 9(1)–(8) │ High-risk         │    │
│             │  │  Assessment: [GAP]                                 │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  ┌─ CROSS-REFERENCED REQUIREMENTS ───────────────────┐    │
│             │  │                                                    │    │
│             │  │  ┌ OVERLAPPING ──────────────────────────────────┐│    │
│             │  │  │ ISO42001-6.1  Risks & Opportunities     [MET]││    │
│             │  │  │ ISO/IEC 42001 │ Clause 6.1                   ││    │
│             │  │  │ "Both require systematic identification..."   ││    │
│             │  │  └──────────────────────────────────────────────┘│    │
│             │  │                                                    │    │
│             │  │  ┌ OVERLAPPING ──────────────────────────────────┐│    │
│             │  │  │ ISO23894-7.3  Risk Assessment       [PARTIAL]││    │
│             │  │  │ ISO/IEC 23894 │ Clause 7.3                   ││    │
│             │  │  │ "Both define risk assessment as iterative..." ││    │
│             │  │  └──────────────────────────────────────────────┘│    │
│             │  │                                                    │    │
│             │  │  ┌ OVERLAPPING ──────────────────────────────────┐│    │
│             │  │  │ NIST-MAP-1  Context Establishment    [GAP]   ││    │
│             │  │  │ NIST AI RMF │ MAP 1                          ││    │
│             │  │  │ "NIST MAP function covers similar scope..."  ││    │
│             │  │  └──────────────────────────────────────────────┘│    │
│             │  │                                                    │    │
│             │  │  ┌ SUPPORTS ────────────────────────────────────┐│    │
│             │  │  │ OECD-P4  Robustness, Security, Safety  [MET]││    │
│             │  │  │ OECD │ Principle 4                           ││    │
│             │  │  │ "OECD Principle 4 supports the goals..."     ││    │
│             │  │  └──────────────────────────────────────────────┘│    │
│             │  └────────────────────────────────────────────────────┘    │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.6 Gap Analysis

```
┌──────────────────────────────────────────────────────────────────────────┐
│  GAP ANALYSIS              [Engagement ▾] [AI System ▾] [Framework ▾]  │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  SUMMARY: 12 gaps │ 8 partial │ Target: 2026-08-01       │
│             │                                                            │
│             │  ┌─ EU AI ACT — 5 gaps, 3 partial ──────────────────┐    │
│             │  │                                                    │    │
│             │  │  [GAP] EU-AIA-ART9-RMS  Risk Management System   │    │
│             │  │  Notes: "No formal RMS documented yet"            │    │
│             │  │  Remediation: "Draft RMS per Art.9 template"      │    │
│             │  │  Target: 2026-05-15 │ Cross-refs: ISO42001-6.1   │    │
│             │  │                                                    │    │
│             │  │  [PARTIAL] EU-AIA-ART10-DG  Data Governance      │    │
│             │  │  Notes: "Data inventory started, governance..."   │    │
│             │  │  Remediation: "Complete data quality assessment"  │    │
│             │  │  Target: 2026-06-01                               │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  ┌─ ISO/IEC 42001 — 3 gaps, 2 partial ──────────────┐    │
│             │  │  ...                                               │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  ┌─ NIST AI RMF — 4 gaps, 3 partial ────────────────┐    │
│             │  │  ...                                               │    │
│             │  └────────────────────────────────────────────────────┘    │
│             │                                                            │
│             │  [Generate Gap Analysis Report (HTML)]                     │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.7 LLM Assistant

```
┌──────────────────────────────────────────────────────────────────────────┐
│  AI GOVERNANCE ASSISTANT       Context: [Acme Corp ▾] [Chatbot v2 ▾]   │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  ┌────────────────────────────────────────────────────┐   │
│             │  │  🧑 What specific evidence do I need to            │   │
│             │  │     demonstrate compliance with Article 13          │   │
│             │  │     transparency requirements for a high-risk       │   │
│             │  │     healthcare AI system?                            │   │
│             │  │                                                      │   │
│             │  │  🤖 For a high-risk healthcare AI system under      │   │
│             │  │     Article 13, you need to provide deployers       │   │
│             │  │     with the following documented evidence:          │   │
│             │  │                                                      │   │
│             │  │     1. **Instructions for Use (Art. 13(3)(a))**     │   │
│             │  │        - Identity and contact of provider           │   │
│             │  │        - System characteristics, capabilities,      │   │
│             │  │          limitations                                 │   │
│             │  │                                                      │   │
│             │  │     2. **Performance Metrics (Art. 13(3)(b))**      │   │
│             │  │        - Intended purpose documentation             │   │
│             │  │        - Level of accuracy with validation data     │   │
│             │  │                                                      │   │
│             │  │     3. **Known Limitations (Art. 13(3)(b)(iv))**    │   │
│             │  │        - Foreseeable misuse scenarios               │   │
│             │  │        - Known/foreseeable circumstances affecting  │   │
│             │  │          accuracy                                    │   │
│             │  │                                                      │   │
│             │  │     **Cross-framework note:** This also satisfies   │   │
│             │  │     ISO 42001 Annex A.8 (Transparency) and OECD    │   │
│             │  │     Principle 3 (Transparency & Explainability).    │   │
│             │  │                                                      │   │
│             │  │     Your current status for EU-AIA-ART13 is [GAP]. │   │
│             │  │     Suggested evidence types: Technical Report,     │   │
│             │  │     Assessment Record.                               │   │
│             │  └────────────────────────────────────────────────────┘   │
│             │                                                            │
│             │  ┌────────────────────────────────────────────┐ [Send]    │
│             │  │ Ask about any regulation or requirement... │            │
│             │  └────────────────────────────────────────────┘            │
└─────────────┴────────────────────────────────────────────────────────────┘
```

### 11.8 Audit Trail

```
┌──────────────────────────────────────────────────────────────────────────┐
│  AUDIT TRAIL (Immutable)       [Date Range ▾] [Entity ▾] [Action ▾]    │
├─────────────┬────────────────────────────────────────────────────────────┤
│             │                                                            │
│  NAVIGATION │  Showing 142 entries                                      │
│             │                                                            │
│             │  ┌────────┬──────────┬────────────┬──────────────────┐    │
│             │  │  TIME  │ ENTITY   │  ACTION    │ DETAILS          │    │
│             │  ├────────┼──────────┼────────────┼──────────────────┤    │
│             │  │ 14:32  │ assess-  │ status_    │ EU-AIA-ART9-RMS │    │
│             │  │ Apr 01 │ ment     │ changed    │ not_assessed→gap│    │
│             │  ├────────┼──────────┼────────────┼──────────────────┤    │
│             │  │ 14:28  │ risk_    │ created    │ "Training data  │    │
│             │  │ Apr 01 │ entry    │            │ bias" added     │    │
│             │  ├────────┼──────────┼────────────┼──────────────────┤    │
│             │  │ 14:15  │ evidence │ evidence_  │ "RMS_Policy.pdf"│    │
│             │  │ Apr 01 │          │ attached   │ → EU-AIA-ART9   │    │
│             │  ├────────┼──────────┼────────────┼──────────────────┤    │
│             │  │ 13:50  │ ai_      │ created    │ "Customer Chat- │    │
│             │  │ Apr 01 │ system   │            │ bot v2" created │    │
│             │  ├────────┼──────────┼────────────┼──────────────────┤    │
│             │  │ 13:45  │ engage-  │ created    │ "Acme Corp"     │    │
│             │  │ Apr 01 │ ment     │            │ engagement begin│    │
│             │  └────────┴──────────┴────────────┴──────────────────┘    │
│             │                                                            │
│             │  [< Prev]  Page 1 of 15  [Next >]                        │
└─────────────┴────────────────────────────────────────────────────────────┘
```

---

## 12. Tauri IPC Command Reference

Every backend operation is exposed as a `#[tauri::command]` async function. Frontend calls these via `invoke()`.

### Naming Convention
`{action}_{entity}` — e.g., `create_engagement`, `list_ai_systems`, `update_task`

### Complete Command List

| Command | Args | Returns | Audit? |
|---------|------|---------|--------|
| **Engagements** | | | |
| `create_engagement` | CreateEngagementDto | Engagement | ✓ |
| `list_engagements` | status_filter? | Vec\<Engagement\> | |
| `get_engagement` | id: Uuid | Engagement | |
| `update_engagement` | id, UpdateEngagementDto | Engagement | ✓ |
| `delete_engagement` | id: Uuid | () | ✓ |
| **AI Systems** | | | |
| `create_ai_system` | CreateAiSystemDto | AiSystem | ✓ |
| `list_ai_systems` | engagement_id: Uuid | Vec\<AiSystem\> | |
| `get_ai_system` | id: Uuid | AiSystem | |
| `update_ai_system` | id, UpdateAiSystemDto | AiSystem | ✓ |
| `delete_ai_system` | id: Uuid | () | ✓ |
| **Requirements** | | | |
| `list_requirements` | framework?, category? | Vec\<FrameworkRequirement\> | |
| `get_requirement` | id: Uuid | FrameworkRequirement | |
| `search_requirements` | query: String | Vec\<FrameworkRequirement\> | |
| **Assessments** | | | |
| `upsert_assessment` | UpsertAssessmentDto | RequirementAssessment | ✓ |
| `list_assessments` | ai_system_id, framework? | Vec\<RequirementAssessment\> | |
| `get_assessment` | ai_system_id, requirement_id | RequirementAssessment | |
| **Cross-References** | | | |
| `get_cross_references` | requirement_id: Uuid | Vec\<CrossReferenceExpanded\> | |
| `get_cross_ref_matrix` | fw_a: Framework, fw_b: Framework | CrossRefMatrix | |
| **Risk Entries** | | | |
| `create_risk_entry` | CreateRiskDto | RiskEntry | ✓ |
| `list_risk_entries` | ai_system_id? | Vec\<RiskEntry\> | |
| `update_risk_entry` | id, UpdateRiskDto | RiskEntry | ✓ |
| `delete_risk_entry` | id: Uuid | () | ✓ |
| `get_risk_matrix_data` | engagement_id?, ai_system_id? | RiskMatrixData | |
| **Evidence** | | | |
| `upload_evidence` | UploadEvidenceDto | Evidence | ✓ |
| `list_evidence` | engagement_id, type_filter? | Vec\<Evidence\> | |
| `delete_evidence` | id: Uuid | () | ✓ |
| `link_evidence` | LinkEvidenceDto | EvidenceLink | ✓ |
| `unlink_evidence` | link_id: Uuid | () | ✓ |
| **Tasks** | | | |
| `create_task` | CreateTaskDto | Task | ✓ |
| `list_tasks` | engagement_id?, status?, priority? | Vec\<Task\> | |
| `update_task` | id, UpdateTaskDto | Task | ✓ |
| `delete_task` | id: Uuid | () | ✓ |
| **Audit Log** | | | |
| `list_audit_log` | AuditFilterDto | Vec\<AuditLog\> | |
| **Reports** | | | |
| `generate_report` | ReportRequest | ReportResult (file_path) | ✓ |
| **LLM** | | | |
| `query_llm` | LlmQueryDto | LlmConversation | ✓ |
| `list_conversations` | engagement_id? | Vec\<LlmConversation\> | |
| **Dashboard** | | | |
| `get_dashboard_stats` | — | DashboardStats | |
| **Gap Analysis** | | | |
| `get_gap_analysis` | engagement_id?, ai_system_id? | GapAnalysisData | |
| **Config** | | | |
| `get_config` | — | AppConfig | |
| `update_config` | UpdateConfigDto | AppConfig | |

### DTO Examples

```rust
#[derive(Deserialize)]
pub struct CreateEngagementDto {
    pub name: String,
    pub client_name: String,
    pub description: String,
    pub primary_role: ObligationRole,
    pub frameworks: Vec<Framework>,
}

#[derive(Deserialize)]
pub struct UpsertAssessmentDto {
    pub ai_system_id: Uuid,
    pub requirement_id: Uuid,
    pub status: ComplianceStatus,
    pub assessor_notes: String,
    pub remediation_plan: String,
    pub target_date: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct DashboardStats {
    pub active_engagements: i64,
    pub total_ai_systems: i64,
    pub compliance_by_framework: Vec<FrameworkCompliance>,
    pub open_tasks: i64,
    pub open_risks: i64,
    pub total_gaps: i64,
    pub total_evidence: i64,
    pub priority_tasks: Vec<Task>,
}

#[derive(Serialize)]
pub struct FrameworkCompliance {
    pub framework: Framework,
    pub total_applicable: i64,
    pub met: i64,
    pub partial: i64,
    pub gap: i64,
    pub not_assessed: i64,
    pub pct: f64,
}

#[derive(Serialize)]
pub struct RiskMatrixData {
    pub entries: Vec<RiskEntry>,
    pub matrix: [[Vec<Uuid>; 5]; 5],   // [likelihood][impact] → list of risk IDs
}

#[derive(Serialize)]
pub struct GapAnalysisData {
    pub frameworks: Vec<FrameworkGaps>,
    pub total_gaps: i64,
    pub total_partial: i64,
}

#[derive(Serialize)]
pub struct FrameworkGaps {
    pub framework: Framework,
    pub gaps: Vec<GapEntry>,
}

#[derive(Serialize)]
pub struct GapEntry {
    pub requirement: FrameworkRequirement,
    pub assessment: RequirementAssessment,
    pub cross_references: Vec<CrossReference>,
}
```

---

## 13. LLM Integration

### Architecture

```
Frontend (chat UI)
    │
    ▼ invoke("query_llm", { query, engagement_id, ai_system_id })
Tauri Command
    │
    ├── Build system prompt (from DB context)
    ├── Append user query
    ├── Call LLM API via reqwest
    ├── Store conversation in llm_conversations
    ├── Write audit_log entry
    └── Return response
```

### Supported Providers

| Provider | Endpoint | Auth |
|----------|----------|------|
| OpenAI | `https://api.openai.com/v1/chat/completions` | Bearer token |
| Anthropic | `https://api.anthropic.com/v1/messages` | x-api-key header |
| Ollama (local) | `http://localhost:11434/api/generate` | None |

### System Prompt Construction

The system prompt is dynamically built from database context:

1. **Base instruction:** Regulatory assistant persona
2. **Engagement context:** Name, role, frameworks (from `engagements` table)
3. **AI System context:** Name, risk category, domain (from `ai_systems` table)
4. **Compliance snapshot:** Summary counts (from `requirement_assessments`)
5. **Open gaps:** List of current gap requirement titles (from assessments WHERE status = 'gap')

This gives the LLM relevant context to provide specific, actionable guidance.

### Safety

- API key encrypted at rest using `ring` or `aes-gcm`
- Key decrypted only in memory when making API calls
- No conversation data sent to any service except the configured LLM provider
- Ollama option enables fully offline LLM usage

---

## 14. Report Generation

### Technology

Reports are generated using **Askama** templates — Rust compile-time HTML templating. The backend:
1. Queries all relevant data
2. Builds a Rust struct matching the Askama template
3. Renders to an HTML string
4. Writes the HTML to a file in the configured reports directory
5. Returns the file path to the frontend
6. Frontend calls `tauri::api::shell::open()` to open in the default browser

### Template Structure

Each template is a self-contained HTML file with inline CSS (no external dependencies). This ensures reports look correct when opened in any browser.

```html
<!-- compliance_report.html (Askama template) -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Compliance Report — {{ engagement.name }}</title>
    <style>
        /* Inline CSS for portability */
        body { font-family: 'Segoe UI', system-ui, sans-serif; max-width: 1000px; margin: 0 auto; }
        .status-met { color: #16a34a; } .status-gap { color: #dc2626; }
        .status-partial { color: #d97706; } .status-na { color: #6b7280; }
        .compliance-bar { height: 20px; border-radius: 4px; }
        .risk-grid { display: grid; grid-template-columns: repeat(5, 1fr); gap: 2px; }
        /* ... more styles ... */
    </style>
</head>
<body>
    <header>
        <h1>AI Governance Compliance Report</h1>
        <p>Engagement: {{ engagement.name }} | Client: {{ engagement.client_name }}</p>
        <p>Generated: {{ generated_at }} | Role: {{ engagement.primary_role }}</p>
    </header>

    {% for system in ai_systems %}
    <section>
        <h2>{{ system.name }} ({{ system.risk_category }})</h2>
        {% for fw in framework_results %}
        <h3>{{ fw.framework_name }}</h3>
        <div class="compliance-bar">...</div>
        {% for req in fw.requirements %}
        <div class="requirement">
            <span class="ref">{{ req.reference_id }}</span>
            <span class="title">{{ req.title }}</span>
            <span class="status-{{ req.status }}">{{ req.status }}</span>
        </div>
        {% endfor %}
        {% endfor %}
    </section>
    {% endfor %}

    <footer>
        <p>Generated by GRC Command Center v{{ version }}</p>
    </footer>
</body>
</html>
```

---

## 15. Security Considerations

### Data at Rest
- PostgreSQL with trust auth is acceptable for single-user local deployment
- LLM API keys encrypted using AES-256-GCM before storage in `app_config`
- Evidence files stored on local filesystem — protected by OS-level permissions

### Data in Transit
- All LLM API calls use HTTPS (enforced by reqwest)
- No other network communication — fully local otherwise

### Input Validation
- All user inputs sanitized before SQL queries (SQLx parameterized queries handle this)
- File uploads validated: check file size limits, allowed MIME types
- No dynamic SQL construction — all queries use SQLx macros or parameterized queries

### Audit Integrity
- `audit_log` table has PostgreSQL triggers preventing UPDATE and DELETE
- Timestamps are server-side (PostgreSQL NOW()), not client-provided
- Audit entries include old and new values for change tracking

### Tauri Security
- Tauri capabilities configured to allow only necessary APIs (filesystem, shell, dialog)
- No `dangerousRemoteDomainIpcAccess`
- CSP configured in `tauri.conf.json` to prevent XSS

---

## 16. Extensibility

### Adding a New Framework

To add a new regulatory framework (e.g., Canada's AIDA or Brazil's AI Bill):

1. **Add enum variant:** Add to `Framework` enum in `shared/src/enums.rs`
2. **Create migration:** New SQL file inserting requirements into `framework_requirements`
3. **Seed cross-references:** Add `cross_references` entries mapping to existing frameworks
4. **No code changes needed** in:
   - Frontend (framework pills, navigator tabs render dynamically)
   - Backend (queries are framework-agnostic)
   - Reports (templates iterate over all frameworks)

The architecture is **data-driven**, not code-driven. New frameworks are configuration, not features.

### Adding New Entity Types

1. Define struct in `shared/src/models.rs`
2. Create migration for new table
3. Add model queries in `src-tauri/src/models/`
4. Add Tauri commands in `src-tauri/src/commands/`
5. Add frontend page/component
6. Add route

---

## 17. Build & Development

### Prerequisites

| Tool | Install Command |
|------|----------------|
| Rust (stable) | `rustup install stable` |
| Tauri CLI | `cargo install tauri-cli` |
| trunk | `cargo install trunk` |
| wasm32 target | `rustup target add wasm32-unknown-unknown` |
| PostgreSQL 16+ | Already installed (pgAdmin4) |
| SQLx CLI | `cargo install sqlx-cli --features postgres` |

### Database Setup

```bash
# Create the database
createdb grc_command_center

# Or via psql
psql -U postgres -c "CREATE DATABASE grc_command_center;"

# Run migrations
cd src-tauri
sqlx migrate run --database-url "postgresql://postgres@localhost/grc_command_center"
```

### Development Workflow

```bash
# Terminal 1: Run the full app in dev mode
cargo tauri dev

# This internally:
#   1. trunk builds the Leptos frontend to WASM
#   2. Tauri compiles the backend
#   3. Opens the app window with hot-reload
```

### Production Build

```bash
cargo tauri build
# Produces: src-tauri/target/release/bundle/
#   - .exe installer (Windows)
#   - .msi installer (Windows)
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://postgres@localhost/grc_command_center` |
| `GRC_EVIDENCE_PATH` | Local evidence storage directory | `~/grc-evidence/` |

### Cargo Workspace Layout

```toml
# Root Cargo.toml
[workspace]
members = ["shared", "src-tauri", "frontend"]
resolver = "2"
```

---

## Appendix A: Frontend Routing Table

| Path | Page Component | Description |
|------|---------------|-------------|
| `/` | Dashboard | Command center overview |
| `/engagements` | Engagements | List + create engagements |
| `/engagements/:id` | EngagementDetail | Single engagement with systems |
| `/systems/:id` | AiSystemDetail | AI system profile + compliance |
| `/frameworks` | FrameworkNavigator | Browse requirements (all frameworks) |
| `/frameworks/:fw` | FrameworkNavigator | Requirements for one framework |
| `/cross-references` | CrossReference | Cross-framework mapping engine |
| `/risk-matrix` | RiskMatrix | Risk heat map (all) |
| `/risk-matrix/:sys_id` | RiskMatrix | Risk heat map (one system) |
| `/evidence` | EvidenceVault | Evidence management |
| `/gap-analysis` | GapAnalysis | Gap analysis view |
| `/reports` | Reports | Report generation |
| `/audit-trail` | AuditTrail | Immutable audit log |
| `/assistant` | LlmAssistant | AI regulatory Q&A |
| `/settings` | Settings | App configuration |

## Appendix B: Risk Score Color Bands

| Score Range | Label | Color | Hex |
|------------|-------|-------|-----|
| 1–4 | Low | Green | `#16a34a` |
| 5–9 | Medium | Yellow | `#ca8a04` |
| 10–14 | High | Orange | `#ea580c` |
| 15–19 | Very High | Red | `#dc2626` |
| 20–25 | Critical | Dark Red | `#991b1b` |

## Appendix C: Compliance Status Color Coding

| Status | Color | Hex |
|--------|-------|-----|
| Met | Green | `#16a34a` |
| Partial | Amber | `#d97706` |
| Gap | Red | `#dc2626` |
| Not Assessed | Gray | `#6b7280` |
| Not Applicable | Light Gray | `#9ca3af` |

---

*End of Technical Specification*
