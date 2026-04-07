# GRC Command Center — Codebase vs Technical Spec Gap Analysis

**Date:** April 7, 2026
**Scope:** Full comparison of implemented code against `docs/TECHNICAL-SPEC.md`
**Method:** Exhaustive file-by-file inventory of backend (44 commands), frontend (17 pages, 12 components), shared types (20 enums, 36 structs), and 4 migrations.

---

## Executive Summary

The project is substantially built. The database schema, shared types, backend command layer, and all frontend pages exist and compile. The architecture matches the spec. However, there are **functional gaps** — places where the UI is read-only when the spec calls for CRUD, where scope selectors are missing, and one confirmed runtime bug on the Risk Matrix page.

| Area | Spec Coverage | Status |
|------|--------------|--------|
| Database Schema | 100% | All 13 tables + triggers + indexes |
| Seed Data | 100% | 238 requirements + 21 cross-references |
| Shared Types | 100%+ | All spec types plus FRIA + intake additions |
| Backend Commands | ~97% | 44 commands; 1 spec command missing |
| Frontend Pages | 100% exist | All 17 routes; functional completeness varies |
| Reports | 100% | 3 templates implemented |
| LLM Integration | 100% | 3 providers (OpenAI, Anthropic, Ollama) |

---

## 1. CONFIRMED BUG — Risk Matrix Page

**Symptom:** `Error: Error: missing field "matrix"`

**Root Cause:** Field name and type mismatch between backend and frontend.

| Layer | Struct Field | Type |
|-------|-------------|------|
| Backend (`shared/src/models.rs:688`) | `cells: Vec<RiskMatrixCell>` | Flat list of 25 cells |
| Frontend (`frontend/src/pages/risk_matrix.rs:19`) | `matrix: [[Vec<Uuid>; 5]; 5]` | 5×5 nested array |

The frontend defines a **local** `RiskMatrixData` struct expecting a `matrix` field with type `[[Vec<Uuid>; 5]; 5]`, but the backend serializes a struct with a `cells` field of type `Vec<RiskMatrixCell>`. The `RiskHeatmap` component also takes `matrix: [[Vec<Uuid>; 5]; 5]` as a prop.

**Fix required:** Either:
- (A) Change the frontend to use the shared `RiskMatrixData` struct and transform `cells` into a 5×5 array locally, or
- (B) Change the backend `RiskMatrixData` to serialize as a `matrix: [[Vec<Uuid>; 5]; 5]` to match the spec's `RiskMatrixData` definition.

Option (B) aligns with the spec (Section 12: `RiskMatrixData { entries, matrix: [[Vec<Uuid>; 5]; 5] }`).

---

## 2. Missing Backend Command

| Spec Command | Status | Impact |
|-------------|--------|--------|
| `get_cross_ref_matrix(fw_a, fw_b) → CrossRefMatrix` | **NOT IMPLEMENTED** | Cross-Reference "Matrix View" (Spec §10.5 View B) cannot work |

The spec defines a matrix view where the user selects two frameworks and sees a category×category grid showing cross-reference counts. The backend command, the `CrossRefMatrix` return type, and the frontend matrix view are all absent. Only the "Single Requirement View" (View A) is implemented.

---

## 3. Frontend Functional Gaps

### 3.1 Missing CRUD Operations

Several pages are **read-only** where the spec requires **create/update/delete** capabilities:

| Page | Missing Operations | Spec Reference |
|------|-------------------|----------------|
| **Dashboard** | Click engagement card → navigate; click task → navigate; click compliance bar → navigate | §10.1 Interactions |
| **Engagement Detail** | Delete engagement button | §10.2 (Delete shown in wireframe header) |
| **AI System Detail** | Edit AI system, delete AI system | §10.3 (wireframe shows classification editing) |
| **Risk Matrix** | Add/edit/delete risk entries; cell click → filter; inherent/residual toggle | §10.6 |
| **Evidence Vault** | Upload evidence (file picker flow); link/unlink evidence to assessments/risks/tasks | §10.7 |
| **Gap Analysis** | Filter by engagement/AI system/framework; "Generate Gap Analysis Report" button | §10.8 |
| **Audit Trail** | Date range filter; action type filter; entity ID text search | §10.10 |

### 3.2 Missing Scope/Context Selectors

| Page | What's Missing | Spec Requirement |
|------|---------------|------------------|
| **Risk Matrix** | No engagement/AI system dropdown selectors | §10.6: "Dropdown to select: All / specific engagement / specific AI system" |
| **Gap Analysis** | No engagement/AI system/framework filter dropdowns | §10.8: "By engagement/AI system, by framework, by status" |
| **Reports** | No engagement/AI system scope picker — hardcoded `Uuid::nil()` | §10.9: "User selects scope (engagement/AI system)" |
| **LLM Assistant** | No engagement/AI system context selector | §10.11: "Context selector: engagement + AI system (optional)" |
| **LLM Assistant** | No conversation history persistence (no `list_conversations` call) | §10.11: Chat history should be loadable |
| **LLM Assistant** | No model indicator showing which LLM is active | §10.11 wireframe |

### 3.3 Missing UI Features per Page

#### Dashboard (§10.1)
- [ ] Engagement cards are not clickable (should navigate to engagement detail)
- [ ] Compliance bars are not clickable (should navigate to framework navigator filtered)
- [ ] Task rows are not clickable (should navigate/inline-edit)
- [ ] Quick stats row not fully interactive

#### Engagement Detail (§10.2)
- [ ] No delete engagement button/confirmation
- [ ] No compliance summary per AI system (per-framework % bars)
- [ ] No risk count / open task count per AI system listed
- [ ] No "Engagement Evidence" section linking to evidence vault filtered

#### AI System Detail (§10.3)
- [ ] No edit form for AI system fields
- [ ] No compliance summary (per-framework progress bars)
- [ ] No risk summary (mini risk matrix)
- [ ] No recent assessments section
- [ ] No linked evidence section

#### Framework Navigator (§10.4)
- [ ] No "text search on title/description" filter
- [ ] No "risk category applicability" dropdown filter
- [ ] No evidence attachment from within assessment editor
- [ ] Category filter within framework tab

#### Cross-Reference (§10.5)
- [ ] No Matrix View (View B) — framework × framework category grid
- [ ] No assessment status display on cross-referenced requirements for current AI system

#### Risk Matrix (§10.6)
- [ ] No "Add Risk Entry" form/button
- [ ] No click-to-expand risk detail with edit capability
- [ ] No inherent/residual risk toggle
- [ ] No cell click → filter to that risk level
- [ ] Scope selectors (engagement/system) missing

#### Evidence Vault (§10.7)
- [ ] No file upload via Tauri native file picker
- [ ] No evidence linking UI (attach to assessments/risks/tasks)
- [ ] No tag-based filtering
- [ ] No search capability

#### Gap Analysis (§10.8)
- [ ] No scope filtering (engagement, AI system, framework, status)
- [ ] No "Generate Gap Analysis Report" button
- [ ] No summary stats (total gaps by framework, % in gap state, upcoming target dates)
- [ ] No cross-reference shortcuts on gap entries

#### Reports (§10.9)
- [ ] Scope selector missing — reports use hardcoded nil UUID
- [ ] Generated report not auto-opened in browser (just shows file path)
- [ ] No engagement/AI system picker before generating

#### Audit Trail (§10.10)
- [ ] No date range filter
- [ ] No action type filter
- [ ] No entity ID text search
- [ ] No pagination controls (only entity_type filter + page nav exists)

#### LLM Assistant (§10.11)
- [ ] No engagement/AI system context selector dropdowns
- [ ] No conversation history (no `list_conversations` call)
- [ ] No model indicator
- [ ] Messages not persisted across page navigations

#### Settings (§10.12)
- [ ] No "About" section (version, build date)
- [ ] Database settings shown are incomplete (no host/port display)

---

## 4. Backend Gaps

| Issue | Severity | Details |
|-------|----------|---------|
| `get_cross_ref_matrix` not implemented | Medium | Needed for Cross-Reference Matrix View (§10.5 View B) |
| `update_config` does not write audit_log | Low | Spec implies all mutations audited; config changes escape the trail |
| API key encryption | Low | Spec calls for AES-256-GCM encryption via `ring` or `aes-gcm`; current implementation stores key as-is in `llm_api_key_encrypted` field (encryption not verified) |
| Evidence upload doesn't use Tauri file picker | Medium | Backend `upload_evidence` expects DTO; no native file dialog integration for actual file copy to evidence storage path |
| Report auto-open in browser | Low | Backend returns file path but no `tauri::api::shell::open()` call to launch browser |

---

## 5. LLM Provider Expansion

### Current State

The backend (`src-tauri/src/llm/client.rs`) supports 3 providers:

| Provider | Endpoint | Auth | Status |
|----------|----------|------|--------|
| OpenAI | `https://api.openai.com/v1/chat/completions` | Bearer token | Implemented |
| Anthropic | `https://api.anthropic.com/v1/messages` | x-api-key header | Implemented |
| Ollama | `http://localhost:11434/api/generate` | None | Implemented |

The Settings page dropdown offers: None, OpenAI, Anthropic, Ollama.

### Required Changes

**Add LM Studio as the bespoke local provider:**
- LM Studio exposes an OpenAI-compatible API at `http://localhost:1234/v1/chat/completions`
- Default model: `gemma-4-e2b-it` (downloaded locally)
- No API key required (local inference)
- This should be the **recommended default** — queries become deterministic from the embedded regulatory documentation used to build the system prompt
- Uses the same OpenAI request/response format, so the implementation reuses the existing `call_openai()` function with a different base URL and no auth

**Add Google Gemini as a cloud provider:**
- Endpoint: `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent`
- Auth: API key as query parameter (`?key=`) or Bearer token
- Default model: `gemini-2.5-flash` or user-specified
- New request/response structs needed (Gemini uses a different JSON schema than OpenAI)

### Target Provider Matrix

| Provider | Type | Endpoint | Auth | Default Model |
|----------|------|----------|------|---------------|
| LM Studio | Local (bespoke) | `http://localhost:1234/v1/chat/completions` | None | `gemma-4-e2b-it` |
| Ollama | Local | `http://localhost:11434/api/generate` | None | User-specified |
| OpenAI | Cloud | `https://api.openai.com/v1/chat/completions` | Bearer token | `gpt-4o` |
| Anthropic | Cloud | `https://api.anthropic.com/v1/messages` | x-api-key | `claude-sonnet-4-20250514` |
| Gemini | Cloud | `https://generativelanguage.googleapis.com/v1beta/...` | API key | `gemini-2.5-flash` |

### Files to Change

| File | Change |
|------|--------|
| `src-tauri/src/llm/client.rs` | Add `call_lm_studio()` (reuse OpenAI format, local URL, no auth) and `call_gemini()` (new request/response types) to `call_llm()` match |
| `frontend/src/pages/settings.rs` | Add "LM Studio (local)" and "Gemini" to provider dropdown; auto-fill model name on provider change; conditionally hide API key field for local providers |
| `shared/src/models.rs` | No struct changes needed (provider is a plain string in `AppConfig`) |
| DB `app_config` | No schema change needed (provider stored as TEXT) |

### Design Notes

- LM Studio's OpenAI-compatible mode means minimal new code — the existing `call_openai()` logic works, only the URL and auth header differ
- The Settings UI should group providers into "Local" (LM Studio, Ollama — no API key needed) and "Cloud" (OpenAI, Anthropic, Gemini — API key required)
- When a local provider is selected, the API key field should be hidden or disabled
- LM Studio should be presented as the recommended option in the UI since it uses the local regulatory knowledge base for deterministic responses

---

## 6. Spec Additions (Implemented Beyond Spec)

These are implemented features **not in the original spec** but valuable additions:

| Feature | Files | Value |
|---------|-------|-------|
| FRIA Assessment workflow | Migration 003, full CRUD, dedicated page | Implements EU AI Act Art. 27 as a first-class entity |
| Engagement Intake Scoping | Migration 004, 7 new fields, framework suggestion engine | Deterministic framework recommendation from scoping inputs |
| 8 new enums | `IndustrySector`, `Jurisdiction`, `AssuranceObjective`, `AiUseCase`, `PersonalDataProfile`, `FriaStatus`, `FriaNotificationStatus`, `ReportType` | Richer engagement context and FRIA workflow support |
| `suggest_frameworks_for_scope()` | `shared/src/models.rs` | Intelligent framework selection based on intake parameters |

---

## 7. Data Model Differences

| Item | Spec (§6) | Implementation | Notes |
|------|-----------|---------------|-------|
| `Engagement` fields | 8 fields | 15 fields | +7 from engagement intake (migration 004) |
| `RiskMatrixData` | `matrix: [[Vec<Uuid>; 5]; 5]` | `cells: Vec<RiskMatrixCell>` | **Mismatch causing runtime bug** |
| `DashboardStats` | 8 fields | 10 fields | +`fria_in_scope`, `fria_completed` |
| `FriaAssessment` | Not in spec §6.2 | 18-field struct | New entity |
| `CrossRefMatrix` | Defined in spec §12 | Not implemented | Missing backend type |

---

## 8. Routing Table Comparison

| Spec Route | Implementation | Match? |
|-----------|----------------|--------|
| `/` | `DashboardPage` | ✓ |
| `/engagements` | `EngagementsPage` | ✓ |
| `/engagements/:id` | `EngagementDetailPage` | ✓ |
| `/systems/:id` | `AiSystemDetailPage` | ✓ |
| `/frameworks` | `FrameworkNavigatorPage` | ✓ |
| `/frameworks/:fw` | `FrameworkNavigatorPage` | ✓ |
| `/cross-references` | `CrossReferencePage` | ✓ |
| `/risk-matrix` | `RiskMatrixPage` | ✓ |
| `/risk-matrix/:sys_id` | `RiskMatrixPage` | ✓ |
| `/evidence` | `EvidenceVaultPage` | ✓ |
| `/gap-analysis` | `GapAnalysisPage` | ✓ |
| `/reports` | `ReportsPage` | ✓ |
| `/audit-trail` | `AuditTrailPage` | ✓ |
| `/assistant` | `LlmAssistantPage` | ✓ |
| `/settings` | `SettingsPage` | ✓ |
| — | `/fria` and `/fria/:sys_id` (FriaPage) | ✓ (bonus) |

All spec routes exist. FRIA routes are a bonus addition.

---

## Implementation Checklist

Priority key: **P0** = blocking/broken, **P1** = core workflow gap, **P2** = important UX, **P3** = polish

### P0 — Bugs / Broken Features

- [x] **Fix Risk Matrix deserialization bug** — align `RiskMatrixData` between backend (`cells`) and frontend (`matrix`) so the page renders without error
  - Files: `shared/src/models.rs`, `frontend/src/pages/risk_matrix.rs`, `frontend/src/components/risk_heatmap.rs`, `src-tauri/src/models/risk.rs`

### P1 — Core Workflow Gaps (spec-required CRUD/functionality)

- [x] **Risk Matrix: Add risk entry form** — "Add Risk Entry" button + form with all spec fields (title, description, likelihood, impact, mitigation, etc.)
- [x] **Risk Matrix: Edit/delete risk entries** — click risk → expand details with edit capability
- [x] **Risk Matrix: Scope selectors** — engagement and AI system dropdown filters
- [x] **Risk Matrix: Inherent/Residual toggle** — switch heatmap between inherent and residual scores
- [x] **Evidence Vault: Upload evidence** — integrate Tauri native file picker dialog, copy file to storage path, call `upload_evidence`
- [x] **Evidence Vault: Link/unlink evidence** — UI to attach evidence to assessments, risks, or tasks
- [x] **Reports: Scope selector** — engagement/AI system picker before generating (replace hardcoded nil UUID)
- [x] **Reports: Auto-open in browser** — call Tauri shell open on generated file path
- [x] **LLM Assistant: Context selector** — engagement + AI system dropdowns to scope queries
- [x] **LLM Assistant: Conversation history** — call `list_conversations`, persist/display past conversations
- [x] **Cross-Reference: Matrix View** — implement `get_cross_ref_matrix` backend command + frontend matrix grid (§10.5 View B)
- [x] **LLM: Add LM Studio provider** — `call_lm_studio()` in backend (reuse OpenAI format, `http://localhost:1234/v1/...`, no auth), default model `gemma-4-e2b-it`, add to Settings dropdown as recommended local option
- [x] **LLM: Add Google Gemini provider** — `call_gemini()` in backend (new request/response structs for Gemini API), add to Settings dropdown as cloud option
- [x] **Settings: Provider UX** — group local vs cloud providers, auto-fill default model on provider change, hide API key for local providers
- [x] **AI System Detail: Edit form** — allow editing system fields (name, risk category, domain, GPAI flags, etc.)
- [x] **AI System Detail: Compliance summary** — per-framework progress bars
- [x] **AI System Detail: Risk summary** — mini risk matrix for this system
- [x] **Gap Analysis: Scope filters** — engagement, AI system, framework, and status dropdown filters
- [x] **Engagement Detail: Delete engagement** — button with confirmation dialog

### P2 — Important UX Enhancements

- [x] **Dashboard: Clickable engagement cards** — navigate to `/engagements/:id`
- [x] **Dashboard: Clickable compliance bars** — navigate to `/frameworks/:fw`
- [x] **Dashboard: Clickable task rows** — navigate to relevant context or inline edit
- [x] **Engagement Detail: Per-system compliance/risk/task summary** — show % bars, risk count, task count per AI system
- [x] **Engagement Detail: Evidence section** — link to evidence vault filtered by engagement
- [x] **Framework Navigator: Text search filter** — search on requirement title/description
- [x] **Framework Navigator: Risk category applicability filter** — dropdown to filter by applicable risk tier
- [ ] **Framework Navigator: Attach evidence from assessment editor** — "Attach Evidence" button in detail panel
- [x] **Evidence Vault: Tag filter and search** — filter by tags, text search on file name/description
- [x] **Gap Analysis: Generate report button** — trigger gap analysis report from the page
- [x] **Gap Analysis: Summary stats** — total gaps by framework, % in gap state, upcoming target dates
- [x] **Gap Analysis: Cross-reference shortcuts** — show related requirements from other frameworks per gap entry
- [x] **Audit Trail: Date range filter** — from/to date pickers
- [x] **Audit Trail: Action type filter** — dropdown for audit action types
- [x] **Audit Trail: Entity ID search** — text input to filter by entity UUID
- [x] **AI System Detail: Recent assessments section** — last 5 requirement assessments
- [x] **AI System Detail: Linked evidence section** — artifacts attached to this system's assessments
- [x] **AI System Detail: Delete button** — with cascade confirmation
- [x] **LLM Assistant: Model indicator** — show active LLM provider/model name
- [x] **Risk Matrix: Cell click filter** — click a heatmap cell to filter risk register to that score level

### P3 — Polish & Completeness

- [x] **Settings: About section** — version number, build date
- [x] **Settings: Database display** — show host and port (read-only)
- [x] **Backend: Audit log for config updates** — `update_config` should write to audit_log
- [x] **Backend: API key encryption** — implement AES-256-GCM encryption for `llm_api_key_encrypted` field
- [x] **Cross-Reference: Assessment status on cross-ref cards** — show current AI system's assessment status for each cross-referenced requirement
- [x] **Task CRUD from UI** — currently tasks only shown read-only on dashboard; spec implies create/update/delete from various contexts
- [x] **Framework Navigator: Category filter dropdown** — within each framework tab

---

**Total items: 1 bug + 19 P1 + 17 P2 + 7 P3 = 44 items**
