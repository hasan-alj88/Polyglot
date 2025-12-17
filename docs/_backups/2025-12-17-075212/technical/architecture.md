# Polyglot Architecture

**Last Updated:** 2025-12-06
**Version:** 1.1 (Regenerated)
**Project:** Polyglot - Asynchronous Automation Language

---

## Executive Summary

Polyglot is implemented as a Rust workspace with three backend services (Trigger Monitor, Queue Manager, Runner) that communicate via PostgreSQL and Redis. The architecture prioritizes async performance, runtime type resolution, and AI agent consistency through explicit patterns and conventions.

**Core Architecture:**
- **Language:** Rust 2021 Edition
- **Pattern:** 3 async backend services + 6 library crates + CLI
- **Databases:** PostgreSQL (IR + metadata), Redis (queues), InfluxDB (time-series)
- **Async Runtime:** Tokio 1.x

**Key Innovation:** Runtime type resolution enabling cross-language FFI without manual setup.

---

## Project Initialization

**First Implementation Story: Initialize Cargo Workspace**

```bash
cargo init polyglot
```

Then configure `Cargo.toml` as a workspace:

```toml
[workspace]
members = [
    "crates/polyglot-cli",
    "crates/polyglot-lexer",
    "crates/polyglot-parser",
    "crates/polyglot-ir",
    "crates/polyglot-trigger-monitor",
    "crates/polyglot-queue-manager",
    "crates/polyglot-runner",
    "crates/polyglot-runtime-wrappers",
    "crates/polyglot-db",
]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1.41", features = ["full"] }
sqlx = { version = "0.8.6", features = ["runtime-tokio", "postgres", "json"] }
redis = { version = "0.32.7", features = ["tokio-comp"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.140"
thiserror = "2.0.17"
anyhow = "1.0.99"
tracing = "0.1.41"
tracing-subscriber = "0.3.19"
clap = { version = "4.5", features = ["derive"] }
config = "0.15.15"
toml = "0.9.8"
```

**Rationale:** Manual workspace setup provides precise control over service boundaries, shared library organization, and dependency management for Polyglot's specialized language implementation architecture.

---

## Decision Summary

| Category | Decision | Version | Affects FR Categories | Rationale |
| -------- | -------- | ------- | --------------------- | --------- |
| Project Setup | Manual Cargo Workspace | Rust 2021 Edition | All | Specialized architecture requires custom workspace structure |
| Database Client | SQLx | 0.8.6 | FR10-18, FR27-40, FR95-102 | Async-first design, compile-time query verification, lightweight |
| Database Migrations | sqlx-cli | (bundled) | FR10-18 | Version-controlled SQL migrations |
| Queue Client | redis | 0.32.7 | FR27-40 | Standard Rust Redis client with tokio-comp async support |
| IR Serialization | JSON (serde_json) | 1.0.140 | FR1-9 | Human-readable for MVP debugging, PostgreSQL JSONB native support |
| IR Storage | PostgreSQL JSONB | - | FR3-5, FR10-18 | Hybrid: document storage for IR + relational power for metadata |
| CLI Framework | clap (derive API) | 4.5 | FR54-74 | Standard Rust CLI framework, auto-generated help, type-safe parsing |
| Error Handling (Libs) | thiserror | 2.0.17 | All library crates | Structured error types with custom variants |
| Error Handling (Bins) | anyhow | 1.0.99 | All binary crates | Simple propagation with context chaining, async-safe (Send+Sync) |
| Logging Framework | tracing + tracing-subscriber | 0.1.41 + 0.3.19 | FR95-102 | Async-native structured logging, OpenTelemetry ready |
| Configuration Format | TOML | 0.9.8 | FR75-83 | Rust ecosystem standard, human-readable |
| Configuration Library | config | 0.15.15 | FR75-83 | Layered config (defaults → file → env vars), 12-factor app support |
| Testing Organization | Rust standard | - | All | Unit tests inline with #[cfg(test)], integration tests in tests/, E2E separate |
| IR Type Definitions | Rust structs + serde | - | FR1-9 | .pg types map to Rust enums/structs, serde for JSON serialization |
| Time-Series Database | InfluxDB | 2.x | FR19-26, FR95-102 | Stores time-based triggers, trigger results, resource metrics (CPU/RAM/GPU) |
| IR Structure | 3 Separate IRs | - | FR1-9 | Polyglot code → {Trigger IR, Queue IR, Runner IR} as separate JSONB columns |
| Lexer Generator | logos | 0.14 | FR1-2, FR6-7 | Declarative token definitions, compile-time DFA generation, 45+ token types, <100ms performance |

---

## Project Structure

```
polyglot/
├── Cargo.toml                          # Workspace root
├── Cargo.lock
├── polyglot.toml.example               # Example configuration
├── README.md
├── LICENSE
│
├── crates/
│   ├── polyglot-cli/                   # CLI binary (FR54-FR74)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # CLI entry point
│   │   │   ├── commands/               # Subcommands
│   │   │   │   ├── mod.rs
│   │   │   │   ├── compile.rs
│   │   │   │   ├── register.rs
│   │   │   │   ├── activate.rs
│   │   │   │   ├── trigger.rs
│   │   │   │   ├── status.rs
│   │   │   │   └── services.rs
│   │   │   └── config.rs               # Config loading
│   │   └── tests/
│   │
│   ├── polyglot-lexer/                 # Lexer library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── token.rs                # Token types (111 tokens)
│   │   │   ├── lexer.rs                # Lexer implementation (logos-based)
│   │   │   └── error.rs                # LexerError (thiserror)
│   │   └── tests/
│   │       └── lexer_tests.rs
│   │
│   ├── polyglot-parser/                # Parser library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs               # Recursive descent parser
│   │   │   ├── ast.rs                  # AST types
│   │   │   └── error.rs                # ParserError (thiserror)
│   │   └── tests/
│   │       └── parser_tests.rs
│   │
│   ├── polyglot-ir/                    # Intermediate Representation (FR3-FR5)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs                # IR type definitions
│   │   │   ├── pipeline.rs             # Pipeline IR
│   │   │   ├── trigger.rs              # Trigger IR
│   │   │   ├── validation.rs           # IR validation
│   │   │   └── error.rs                # IrError (thiserror)
│   │   └── tests/
│   │
│   ├── polyglot-db/                    # Database layer (FR10-FR18)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── models.rs               # DB models
│   │   │   ├── pipelines.rs            # Pipeline queries
│   │   │   ├── instances.rs            # Instance queries
│   │   │   ├── triggers.rs             # Trigger queries
│   │   │   └── error.rs                # DbError (thiserror)
│   │   ├── migrations/                 # sqlx migrations
│   │   │   ├── 20250116_001_create_pipelines.sql
│   │   │   ├── 20250116_002_create_instances.sql
│   │   │   └── 20250116_003_create_triggers.sql
│   │   └── tests/
│   │
│   ├── polyglot-trigger-monitor/       # Service: Trigger Monitor (FR19-FR26)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # Service entry point
│   │   │   ├── monitor.rs              # TriggerMonitor struct
│   │   │   ├── handlers/               # Trigger handlers
│   │   │   │   ├── mod.rs
│   │   │   │   ├── trait.rs            # TriggerHandler trait
│   │   │   │   ├── time.rs             # TimeTrigger
│   │   │   │   ├── webhook.rs          # WebhookTrigger
│   │   │   │   ├── file_watch.rs       # FileWatchTrigger
│   │   │   │   └── manual.rs           # ManualTrigger
│   │   │   ├── registry.rs             # Dynamic trigger registry
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   ├── polyglot-queue-manager/         # Service: Queue Manager (FR27-FR40)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── manager.rs              # QueueManager struct
│   │   │   ├── queue.rs                # Queue operations (Redis)
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   ├── polyglot-runner/                # Service: Runner (FR30-FR53)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── runner.rs               # Runner struct
│   │   │   ├── executor.rs             # Pipeline execution
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   └── polyglot-runtime-wrappers/      # Runtime integration (FR41-FR53)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── trait.rs                # RuntimeWrapper trait
│       │   ├── python.rs               # Python runtime wrapper
│       │   └── error.rs                # WrapperError (thiserror)
│       └── tests/
│
├── docs/                               # Documentation (FR84-FR94)
│   ├── technical/
│   │   └── architecture.md             # This document
│   ├── user/
│   └── project/
│       ├── prd.md
│       └── epics.md
│
├── examples/                           # Example .pg files (FR84-FR94)
│   ├── hello_world.pg
│   ├── python_integration.pg
│   └── automation_workflow.pg
│
└── .github/
    └── workflows/
        └── ci.yml                      # CI/CD pipeline
```

---

## FR Category to Architecture Mapping

| FR Category | Architecture Components |
| ----------- | ----------------------- |
| Pipeline Development & Compilation (FR1-FR9) | polyglot-lexer, polyglot-parser, polyglot-ir, polyglot-cli |
| Pipeline Registry & Lifecycle (FR10-FR18) | polyglot-db, polyglot-cli |
| Trigger System (FR19-FR26) | polyglot-trigger-monitor, polyglot-db |
| Queue Management & Execution (FR27-FR40) | polyglot-queue-manager, polyglot-runner, polyglot-db |
| Runtime Integration & FFI (FR41-FR53) | polyglot-runner, polyglot-runtime-wrappers |
| CLI & Developer Tools (FR54-FR74) | polyglot-cli |
| Installation & Configuration (FR75-FR83) | All crates |
| Documentation & Examples (FR84-FR94) | docs/, examples/ |
| Observability & Monitoring (FR95-FR102) | All services (tracing) |

---

## Technology Stack Details

### Core Technologies

**Language & Runtime:**
- Rust 2021 Edition
- Tokio 1.41 (async runtime with "full" features)

**Database Stack:**
- **PostgreSQL:** Primary data store for IR, metadata, registry
  - JSONB columns for 3 separate IRs (Trigger IR, Queue IR, Runner IR)
  - Relational tables for pipeline metadata, instances, triggers
- **SQLx 0.8.6:** Async database client
  - Features: `runtime-tokio`, `postgres`, `json`, `macros`
  - Compile-time query verification
  - Connection pooling built-in
- **sqlx-cli:** Database migration management

**Time-Series Database:**
- **InfluxDB 2.x:** Stores time-based trigger schedules, trigger execution results, resource metrics (CPU/RAM/GPU)
- Used by: Trigger Monitor, Resource Monitor (post-MVP)

**Queue System:**
- **Redis 0.32.7:** Queue operations for dispatch
  - Feature: `tokio-comp` for async support
  - Default Queue (MVP), User-defined queues (post-MVP)
- **Fallback:** PostgreSQL polling when Redis unavailable

**Serialization:**
- **serde 1.0:** Rust struct serialization with derive macros
- **serde_json 1.0.140:** JSON IR format
- **JSONB:** PostgreSQL native storage for IR

**CLI Framework:**
- **clap 4.5:** Command-line argument parsing
  - Derive API for type-safe CLI definitions
  - Auto-generated help and version info

**Error Handling:**
- **Library crates:** `thiserror 2.0.17` for structured error types
- **Binary crates:** `anyhow 1.0.99` for simple error propagation
- All errors: `Send + Sync` for async safety

**Logging & Observability:**
- **tracing 0.1.41:** Async-native structured logging
- **tracing-subscriber 0.3.19:** Log collection and formatting
- **Future:** OpenTelemetry integration (post-MVP)

**Configuration:**
- **TOML 0.9.8:** Configuration file format
- **config 0.15.15:** Layered configuration library
  - Defaults → file → environment variables
  - 12-factor app support

**Lexer:**
- **logos 0.14:** Declarative lexer generator
  - Compile-time DFA generation
  - 111 token types for v0.0.2 syntax
  - Performance: <100ms for 1000-line files

### Integration Points

**Service-to-Service Communication:**
- No direct HTTP/RPC between services
- **Database-driven:** All communication via PostgreSQL and Redis
  - Trigger Monitor → PostgreSQL: Create pipeline instances
  - Queue Manager → Redis: Queue operations
  - Runner → PostgreSQL: Update instance state

**External Dependencies:**
- PostgreSQL connection (required)
- Redis connection (required for MVP)
- InfluxDB connection (required for time-series features)

---

## Implementation Patterns

These patterns ensure AI agents write compatible code across all workspace crates.

### Naming Conventions

**Module/File Naming:**
- Snake_case for file names: `trigger_monitor.rs`, `queue_manager.rs`
- Match crate names: `polyglot-trigger-monitor` → `trigger_monitor/`

**Type Naming:**
- PascalCase for structs/enums: `TriggerMonitor`, `PipelineInstance`, `LexerError`
- Trait names describe capability: `TriggerHandler`, `RuntimeWrapper`

**Function Naming:**
- Snake_case: `load_triggers()`, `execute_pipeline()`, `connect_db()`
- Async functions: No special prefix (`.await` makes it clear)

**Database Naming:**
- Tables: Plural snake_case: `pipelines`, `pipeline_instances`, `triggers`
- Columns: Snake_case: `pipeline_id`, `created_at`, `activated`
- Foreign keys: `<table>_id` format: `pipeline_id`, `instance_id`
- JSONB columns: `trigger_ir`, `queue_ir`, `runner_ir`

**Error Variants:**
- PascalCase with descriptive names: `LexerError::UnexpectedChar`, `DbError::ConnectionFailed`
- Include context in variant: `InvalidTrigger { trigger_id: Uuid, reason: String }`

### Structure Patterns

**Crate Organization:**
```
crate-name/
├── Cargo.toml
├── src/
│   ├── lib.rs or main.rs       # Entry point
│   ├── <domain>.rs             # Core logic (e.g., lexer.rs, monitor.rs)
│   ├── error.rs                # Error types (thiserror)
│   ├── config.rs               # Configuration structs
│   └── <sub>/                  # Sub-modules if complex
│       ├── mod.rs
│       └── <feature>.rs
└── tests/                      # Integration tests
    └── <crate>_tests.rs
```

**Test Organization:**
- Unit tests: Inline with `#[cfg(test)] mod tests { ... }`
- Integration tests: `tests/` directory
- Test file naming: `<feature>_tests.rs`

**Error Module Pattern (All Library Crates):**
```rust
// error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrateNameError {
    #[error("Specific error: {0}")]
    SpecificError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

**Binary Main Pattern (All Services + CLI):**
```rust
// main.rs
use anyhow::{Context, Result};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize tracing
    tracing_subscriber::fmt::init();

    // 2. Load config
    let config = Config::load()
        .context("Failed to load configuration")?;

    // 3. Connect to database
    let pool = connect_db(&config.database_url).await
        .context("Failed to connect to database")?;

    // 4. Run service/CLI
    run(pool, config).await
}
```

### Format Patterns

**Database Model Serialization:**
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub trigger_ir: sqlx::types::Json<TriggerIr>,
    pub queue_ir: sqlx::types::Json<QueueIr>,
    pub runner_ir: sqlx::types::Json<RunnerIr>,
    pub created_at: DateTime<Utc>,
    pub activated: bool,
}
```

**Date/Time Format:**
- Internal: `chrono::DateTime<Utc>`
- Database: PostgreSQL `TIMESTAMPTZ`
- Logs: ISO 8601 via tracing JSON formatter
- User-facing: ISO 8601 strings

**Logging Format:**
```rust
use tracing::{info, error, warn, debug, instrument};

#[instrument(skip(pool))]  // Auto-trace function entry/exit
async fn load_pipelines(pool: &PgPool) -> Result<Vec<Pipeline>> {
    info!("Loading active pipelines");

    match query_pipelines(pool).await {
        Ok(pipelines) => {
            info!(count = pipelines.len(), "Loaded pipelines");
            Ok(pipelines)
        }
        Err(e) => {
            error!(error = %e, "Failed to load pipelines");
            Err(e.into())
        }
    }
}
```

### Cross-Cutting Concerns

**Error Handling Strategy:**
- **Library crates:** Use `thiserror` for domain-specific errors
- **Binary crates:** Use `anyhow` for easy propagation
- **Async safety:** All errors must be `Send + Sync`
- **Context:** Use `.context()` to add meaningful error messages

**Logging Approach:**
- **Structured logging:** Use tracing with structured fields
- **Log levels:** `error!`, `warn!`, `info!`, `debug!`, `trace!`
- **Instrumentation:** Use `#[instrument]` for automatic span tracking
- **Format:** JSON output for production, pretty-print for development

**Date/Time Handling:**
- **Library:** `chrono` for date/time operations
- **Timezone:** Always UTC internally
- **Format:** ISO 8601 for all external representations

**Testing Strategy:**
- **Unit tests:** Test individual functions inline
- **Integration tests:** Test crate APIs in `tests/`
- **E2E tests:** Separate test suite for full system (post-MVP)
- **Coverage target:** 80% for library crates, 60% for services

---

## Data Architecture

### Database Schema

**pipelines Table:**
```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    trigger_ir JSONB NOT NULL,
    queue_ir JSONB NOT NULL,
    runner_ir JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    activated BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_pipelines_name ON pipelines(name);
CREATE INDEX idx_pipelines_activated ON pipelines(activated);
```

**pipeline_instances Table:**
```sql
CREATE TABLE pipeline_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id),
    status VARCHAR(50) NOT NULL, -- Created, Queued, Running, Exited, Failed
    trigger_data JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT
);

CREATE INDEX idx_instances_pipeline ON pipeline_instances(pipeline_id);
CREATE INDEX idx_instances_status ON pipeline_instances(status);
CREATE INDEX idx_instances_created ON pipeline_instances(created_at DESC);
```

**triggers Table:**
```sql
CREATE TABLE triggers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id),
    trigger_type VARCHAR(50) NOT NULL, -- Manual, Time, Webhook, FileWatch
    trigger_config JSONB NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_triggers_pipeline ON triggers(pipeline_id);
CREATE INDEX idx_triggers_type ON triggers(trigger_type);
CREATE INDEX idx_triggers_enabled ON triggers(enabled);
```

### IR Structure

**Three Separate IRs:**

1. **Trigger IR:** Defines when pipeline should execute
2. **Queue IR:** Defines queue routing and priority
3. **Runner IR:** Defines execution logic and runtime wrappers

**Stored as separate JSONB columns** for service isolation and independent evolution.

---

## Security Architecture

**Database Security:**
- Connection string in environment variables (never in code)
- PostgreSQL user with least privilege (no superuser)
- Prepared statements via SQLx (SQL injection prevention)

**Service Security:**
- No HTTP endpoints in MVP (database-driven only)
- Service-to-service: Shared database credentials (trusted network)
- Future: Mutual TLS for service communication (post-MVP)

**Configuration Security:**
- Sensitive values in environment variables
- `.env` file in `.gitignore`
- Example config file with placeholders

---

## Performance Considerations

**Async-First Design:**
- All I/O operations use Tokio async runtime
- No blocking operations in hot paths
- Database queries use connection pooling

**Lexer Performance:**
- Target: <100ms for 1000-line files
- logos compile-time DFA generation
- Zero-copy string handling where possible

**Database Performance:**
- JSONB indexing for IR queries (post-MVP)
- Connection pooling (SQLx default)
- Query optimization via compile-time verification

---

## Deployment Architecture

**MVP Deployment:**
- Single-node deployment
- All services on one machine
- Shared PostgreSQL/Redis/InfluxDB

**Post-MVP Scaling:**
- Horizontal scaling of Runner instances
- Queue Manager as coordination point
- Trigger Monitor remains single-instance

---

## Development Environment

### Prerequisites

- Rust 1.75+ (2021 edition)
- PostgreSQL 14+
- Redis 7+
- InfluxDB 2.x

### Setup Commands

```bash
# 1. Clone repository
git clone <repo-url>
cd polyglot

# 2. Install Rust toolchain
rustup update stable

# 3. Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# 4. Set up environment
cp .env.example .env
# Edit .env with database URLs

# 5. Run migrations
sqlx migrate run --database-url $DATABASE_URL

# 6. Build workspace
cargo build

# 7. Run tests
cargo test

# 8. Run CLI
cargo run --bin polyglot-cli -- --help
```

---

## Architecture Decision Records (ADRs)

### ADR-001: Manual Cargo Workspace Setup

**Decision:** Use manual Cargo workspace configuration instead of starter templates

**Rationale:**
- Specialized architecture (3 services + 6 libraries + CLI)
- Custom dependency management per crate
- No template matches our service pattern

**Alternatives Considered:**
- cargo-generate with rust-cli template (too generic)
- Monolithic single crate (poor separation of concerns)

**Consequences:**
- More initial setup work
- Full control over structure
- Clear service boundaries

---

### ADR-002: Logos for Lexer Generation

**Decision:** Use logos crate for lexer implementation

**Rationale:**
- Compile-time DFA generation (fast)
- Declarative token definitions
- 111 token types for v0.0.2 syntax
- Performance target: <100ms for 1000-line files

**Alternatives Considered:**
- Manual lexer (too much code)
- nom parser combinators (runtime overhead)

**Consequences:**
- Compile-time cost acceptable
- Runtime performance excellent
- Clear token definitions

---

### ADR-003: Three Separate IRs

**Decision:** Store Trigger IR, Queue IR, and Runner IR as separate JSONB columns

**Rationale:**
- Service isolation (each service reads only its IR)
- Independent evolution (IR versions can diverge)
- Query efficiency (no need to load full IR)

**Alternatives Considered:**
- Single monolithic IR (tight coupling)
- Separate tables (complex joins)

**Consequences:**
- Three columns instead of one
- Clear service boundaries
- Easier to version independently

---

### ADR-004: thiserror for Libraries, anyhow for Binaries

**Decision:** Use thiserror in library crates, anyhow in binary crates

**Rationale:**
- Libraries need structured, typed errors (thiserror)
- Binaries need simple propagation (anyhow)
- All errors must be Send + Sync for async

**Alternatives Considered:**
- Only anyhow (loses type safety in libraries)
- Only thiserror (verbose in binaries)

**Consequences:**
- Two error handling approaches
- Clear separation of concerns
- Optimal ergonomics for each use case

---

### ADR-005: Database-Driven Service Communication

**Decision:** Services communicate via PostgreSQL and Redis (no direct HTTP/RPC)

**Rationale:**
- Simpler than HTTP endpoints
- Natural transaction boundaries
- Redis for fast queue operations
- PostgreSQL for durable state

**Alternatives Considered:**
- gRPC (added complexity)
- HTTP REST (unnecessary overhead)

**Consequences:**
- Database becomes coordination point
- No network overhead for MVP
- Easier to reason about state

---

_Generated by BMAD Decision Architecture Workflow v1.0_
_Date: 2025-12-06_
_For: hhj_
