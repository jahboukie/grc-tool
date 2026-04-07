# GRC Command Center

## Project Description
Personal AI Governance, Risk & Compliance command center for a Certified AI Governance Professional.
Single-user, local-first desktop app. PostgreSQL-backed. Zero npm.

## Tech Stack
- **Shell:** Tauri 2 (desktop window, IPC, filesystem)
- **Frontend:** Leptos (CSR mode, compiled to WASM via trunk)
- **Backend:** Rust (Tauri commands, SQLx async Postgres driver)
- **Database:** PostgreSQL 16+ (local, trust auth, pgAdmin4 managed)
- **Templating:** Askama (HTML report generation)
- **Styling:** Pico CSS (vendored) + custom CSS overrides
- **Serialization:** serde / serde_json throughout
- **HTTP:** reqwest (LLM API calls only)

## Build Commands
```bash
cargo build                    # Build all workspace crates
cd frontend && trunk build     # Build WASM frontend
cargo tauri dev                # Run app in development mode
cargo tauri build              # Produce release binary
sqlx migrate run               # Apply database migrations
sqlx database create           # Create grc_command_center database
```

## Project Structure
```
grc-tool/
├── CLAUDE.md              # This file
├── Cargo.toml             # Workspace manifest
├── docs/                  # Full technical specification
├── shared/                # Shared types crate (enums, models)
├── src-tauri/             # Tauri backend (commands, db, reports, llm)
├── frontend/              # Leptos WASM frontend (pages, components)
└── references/            # Regulatory reference materials
```

See `docs/TECHNICAL-SPEC.md` for the complete specification.

## Architecture
```
Leptos (WASM) → tauri::invoke() → Tauri Commands (Rust) → SQLx → PostgreSQL
```

## Database
- Name: `grc_command_center`
- Host: `localhost:5432`
- Auth: trust (local pgAdmin4)
- Migrations: `src-tauri/migrations/`
- The `audit_log` table is **immutable** — triggers prevent UPDATE and DELETE.

## Conventions

### Rust
- Edition 2021, stable toolchain
- `shared` crate holds all types used by both frontend and backend
- Enums: PascalCase in Rust, snake_case in DB (serde rename_all = "snake_case")
- All primary keys are UUID v4
- All timestamps UTC via chrono
- SQLx compile-time checked queries where possible

### Data
- Data structures first — design structs/enums before writing logic (Rob Pike Rule 5)
- Framework requirements are seed data in migrations, not hardcoded in Rust
- Every entity mutation MUST write to `audit_log` (append-only)
- Cross-references between frameworks stored in `cross_references` table

### Frontend
- No npm, no Node.js — Leptos compiled via trunk
- Pico CSS for base styling, custom.css for overrides
- No dynamic CSS class construction — use static classes or inline styles
- Component files stay under ~150 lines; extract when exceeded
- Tauri invoke calls wrapped in `frontend/src/api/` module

### Reports
- HTML only (no PDF dependencies)
- Rendered server-side via Askama templates
- Written to local filesystem, opened in default browser

## Common Mistakes to Avoid
- Do NOT add npm or Node.js dependencies
- Do NOT use SQLite — this project uses PostgreSQL
- Do NOT modify audit_log records after creation (triggers will reject it)
- Do NOT skip audit logging on any entity mutation
- Do NOT hardcode framework requirements — they come from the database
- Do NOT construct dynamic CSS classes (e.g., `format!("bg-{}", color)`)
- Do NOT use Context providers in Leptos — use signals and props
- Do NOT add features, refactor, or "improve" beyond what was requested
- Do NOT create a sub-shell or spawn Node processes

## Frameworks Covered
1. EU AI Act (Regulation 2024/1689)
2. ISO/IEC 42001:2023 (AI Management System)
3. ISO/IEC 23894:2023 (AI Risk Management)
4. NIST AI Risk Management Framework 1.0
5. OECD AI Principles (2019, updated 2024)

## Auth
Auth is deferred — single-user local desktop app, no authentication layer needed.
