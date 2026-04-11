---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/technical/brainstorming/marker-declarations.md
---

# Architecture

## Executive Summary

Polyglot is implemented as a Rust workspace with three backend services (Trigger Monitor, Queue Manager, Runner) that communicate via PostgreSQL and Redis. The architecture prioritizes async performance, runtime type resolution, and AI agent consistency through explicit patterns and conventions.

## Project Initialization

**First Implementation Story: Initialize Cargo Workspace**

```bash
cargo init polyglot
```

Then configure `Cargo.toml` as a workspace with the following structure:

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
```

**Rationale:** Manual workspace setup provides precise control over service boundaries, shared library organization, and dependency management for Polyglot's specialized language implementation architecture.

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

## Project Structure

```
polyglot/
├── Cargo.toml                          # Workspace root
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
│   │   │   ├── token.rs                # Token types
│   │   │   ├── lexer.rs                # Lexer implementation
│   │   │   └── error.rs                # LexerError (thiserror)
│   │   └── tests/
│   │       └── lexer_tests.rs
│   │
│   ├── polyglot-parser/                # Parser library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs               # Parser implementation
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
│   │   │   ├── queue.rs                # Queue operations
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
│   ├── v0.0.2/                         # Existing v0.0.2 docs
│   ├── architecture.md                 # This document
│   ├── prd.md
│   └── product-brief-Polyglot-2025-11-15.md
│
├── examples/                           # Example .pg files (FR84-FR94)
│   ├── hello_world.pg
│   ├── python_integration.pg
│   └── automation_workflow.pg
│
└── migrations/                         # Global migrations (symlink to polyglot-db/migrations)
```

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
| Observability & Monitoring (FR95-FR102) | All services (logging) |
| IDE & Tooling Integration (FR103-FR106) | Future: LSP server |
| Package Ecosystem (FR107-FR111) | Future: registry service |
| Advanced Features (FR112-FR120) | Future enhancements |

## Technology Stack Details

### Core Technologies

**Async Runtime:**
- Tokio 1.x (latest stable)
- All services and I/O operations use Tokio async runtime

**Database Stack:**
- PostgreSQL (3 JSONB columns for Trigger/Queue/Runner IRs, relational for metadata)
- SQLx 0.8.6 with `tokio-comp`, `postgres`, `macros` features
- sqlx-cli for migrations
- Connection pooling via SQLx built-in pool

**Time-Series Database:**
- InfluxDB 2.x (MVP requirement)
- Stores: Time-based trigger schedules, trigger execution results, resource metrics
- Used by: Trigger Monitor, Resource Monitor subservice

**Queue System:**
- Redis 0.32.7 with `tokio-comp` feature
- Queue operations: Default Queue, Pause Queue (MVP), User-defined queues (post-MVP)
- Fallback: PostgreSQL polling when Redis is down (`pipeline_instances WHERE status='queued'`)

**Serialization:**
- serde 1.x for Rust struct serialization
- serde_json 1.0.140 for IR JSON format
- PostgreSQL JSONB native storage

**CLI:**
- clap 4.5 with derive API
- Auto-generated help and version info

**Error Handling:**
- thiserror 2.0.17 for library error types (Send + Sync for async)
- anyhow 1.0.99 for binary error propagation with context

**Logging:**
- tracing 0.1.41 (async-native structured logging)
- tracing-subscriber 0.3.19 with `env-filter` and `json` features
- Format: Structured JSON logs with spans

**Configuration:**
- config 0.15.15 (layered config system)
- TOML 0.9.8 (file format)
- Environment variable overrides

**Date/Time:**
- chrono (latest) for timestamp handling
- UTC timestamps, ISO 8601 format in logs
- PostgreSQL TIMESTAMPTZ columns

### Integration Points

**Service Communication:**
- Services communicate via PostgreSQL (state) and Redis (queues)
- Trigger Monitor → Queue Manager: Via Redis
- No direct HTTP/RPC between services (database-driven architecture)

**Database Interactions:**
- PostgreSQL: Pipeline registry, instance state, activation status, triggers table
- InfluxDB: Time-based trigger schedules, trigger program results, resource metrics
- Redis: Dispatch queues (default, pause), trigger events

**Trigger Monitor Architecture:**
- **Reads PostgreSQL**: Which trigger programs to activate (including Resource Monitor)
- **Reads InfluxDB**: Results from activated trigger programs
- **Spawns Trigger Programs**: Time-based, resource-based, webhooks (multiple instances if settings differ)
- **Resource Monitor Subservice**: Monitors CPU, RAM, GPU, Network at fixed intervals (selective monitoring)

**PostgreSQL LISTEN/NOTIFY:**
- Trigger IR updates: `NOTIFY trigger_updated, '{"pipeline_id": "uuid"}'`
- Trigger Monitor listens and reloads trigger programs

**Queue Manager Logic:**
- Receives trigger events via Redis
- Applies Queue IR `[t]` logic (timing/dispatch rules)
- Applies Queue IR `[Q]` logic (queue selection: default, pause, user-defined)
- Fallback: Polls `pipeline_instances WHERE status='queued'` if Redis down

**Runner Execution Modes (from Runner IR):**
- Sequential: Steps execute one after another
- Parallel: Steps execute concurrently
- Background: Fire-and-forget execution
- Join: Combine parallel execution results

## Novel Architectural Patterns

### Dynamic Trigger Loading System

**Pattern Name:** Dynamic Trigger Registry with Hybrid Monitoring

**Purpose:** Enable runtime-configurable pipeline triggers without service restarts

**Components:**

1. **Trigger IR Storage** (PostgreSQL JSONB)
   - Trigger configuration stored alongside pipeline IR
   - Supports multiple trigger types per pipeline

2. **PostgreSQL LISTEN/NOTIFY**
   - Database publishes `trigger_updated` notifications
   - Trigger Monitor subscribes and reacts to changes

3. **Dynamic Handler Registry**
   ```rust
   struct TriggerMonitor {
       handlers: HashMap<TriggerId, Box<dyn TriggerHandler>>,
       db_pool: PgPool,
   }

   trait TriggerHandler: Send + Sync {
       async fn start(&mut self) -> Result<()>;
       async fn stop(&mut self) -> Result<()>;
   }
   ```

4. **Hybrid Trigger Types:**
   - **Async Listening**: Webhook (HTTP server), File Watch (fs events), DB Events
   - **Sync Loop**: Time-based (cron schedules), Manual (CLI triggered)

**Data Flow:**
1. User activates pipeline → CLI updates database → PostgreSQL NOTIFY
2. Trigger Monitor receives notification → Parses trigger IR → Spawns appropriate handler
3. Handler monitors for trigger condition → Detects event → Creates pipeline instance in DB
4. Queue Manager picks up instance → Queues to Redis → Runner executes

**Implementation Guide for AI Agents:**
- All trigger handlers implement `TriggerHandler` trait
- Handlers are spawned as separate tokio tasks
- On IR update, old handler is stopped, new handler started
- Each handler type lives in `crates/polyglot-trigger-monitor/src/handlers/<type>.rs`

**Affects FR Categories:**
- FR19-FR26 (Trigger System)
- FR27-FR40 (Queue Management - trigger creates instances)

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
- JSONB columns: `ir` (for intermediate representation)

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

// Ensure Send + Sync for async
impl std::error::Error for CrateNameError {}
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
    pub ir: sqlx::types::Json<PipelineIr>,  // JSONB column
    pub created_at: DateTime<Utc>,
    pub activated: bool,
}
```

**API Response Format (Future HTTP endpoints):**
```rust
#[derive(Serialize)]
struct ApiResponse<T> {
    data: Option<T>,
    error: Option<ApiError>,
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

### Communication Patterns

**Service-to-Service (Via Database):**
- No direct HTTP/RPC calls between services
- State changes via PostgreSQL transactions
- Queue operations via Redis
- Event notifications via PostgreSQL LISTEN/NOTIFY

**Database Transaction Pattern:**
```rust
async fn create_and_queue_instance(
    pool: &PgPool,
    pipeline_id: Uuid,
) -> Result<Uuid> {
    let mut tx = pool.begin().await?;

    // 1. Create instance in database
    let instance_id = sqlx::query_scalar!(
        "INSERT INTO pipeline_instances (pipeline_id, status)
         VALUES ($1, 'created') RETURNING id",
        pipeline_id
    )
    .fetch_one(&mut *tx)
    .await?;

    // 2. Update status to queued
    sqlx::query!(
        "UPDATE pipeline_instances SET status = 'queued' WHERE id = $1",
        instance_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(instance_id)
}
```

**Redis Queue Pattern:**
```rust
use redis::AsyncCommands;

async fn queue_instance(
    redis: &mut RedisConnection,
    instance_id: Uuid,
) -> Result<()> {
    redis.rpush("dispatch_queue", instance_id.to_string()).await?;
    Ok(())
}

async fn dequeue_instance(
    redis: &mut RedisConnection,
) -> Result<Option<Uuid>> {
    let result: Option<String> = redis.blpop("dispatch_queue", 0.0).await?;
    Ok(result.map(|s| Uuid::parse_str(&s).unwrap()))
}
```

### Lifecycle Patterns

**Service Startup Pattern:**
1. Initialize tracing
2. Load configuration
3. Connect to database (with retry logic)
4. Connect to Redis (if needed)
5. Run main service loop
6. Handle graceful shutdown

**Graceful Shutdown Pattern:**
```rust
use tokio::signal;

async fn run_with_shutdown(service: Service) -> Result<()> {
    tokio::select! {
        result = service.run() => result,
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
            service.shutdown().await
        }
    }
}
```

**Pipeline Instance State Transitions:**
- Created → Queued → Running → Exited (success or failure)
- All transitions logged with tracing
- State changes via database updates with timestamps

### Location Patterns

**Configuration File:**
- Location: `polyglot.toml` in project root or `~/.config/polyglot/polyglot.toml`
- Environment override: `POLYGLOT_CONFIG` env var
- Per-service sections: `[database]`, `[redis]`, `[services]`

**Database Migrations:**
- Location: `crates/polyglot-db/migrations/`
- Naming: `YYYYMMDD_NNN_description.sql` (e.g., `20250116_001_create_pipelines.sql`)
- Run via: `sqlx migrate run`

**Logs:**
- Stdout (JSON structured) for containerized deployment
- File logging (optional) via `tracing-appender`

**Example `.pg` Files:**
- Location: `examples/` directory
- Naming: `<use_case>.pg` (e.g., `hello_world.pg`, `python_integration.pg`)

### Consistency Patterns (Cross-Cutting)

**UUID Generation:**
```rust
use uuid::Uuid;

let id = Uuid::new_v4();  // Always v4 UUIDs
```

**Timestamp Creation:**
```rust
use chrono::Utc;

let now = Utc::now();  // Always UTC
```

**Configuration Loading:**
```rust
use config::Config;

let settings = Config::builder()
    .add_source(config::File::with_name("polyglot"))
    .add_source(config::Environment::with_prefix("POLYGLOT"))
    .build()?;
```

**Database Connections:**
```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(config.pool_size)
    .connect(&config.database_url)
    .await?;
```

**All Agents MUST Follow These Patterns**

These implementation patterns are the consistency contract. Any deviation will cause integration failures or agent conflicts.

## Data Architecture

### Database Schema

**pipelines table:**
```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT UNIQUE NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    trigger_ir JSONB NOT NULL,          -- Trigger IR (JSON)
    queue_ir JSONB NOT NULL,            -- Queue IR (JSON) - contains [t] and [Q] logic
    runner_ir JSONB NOT NULL,           -- Runner IR (JSON) - execution modes
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activated BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_pipelines_activated ON pipelines(activated) WHERE activated = TRUE;
CREATE INDEX idx_pipelines_trigger_ir_gin ON pipelines USING GIN(trigger_ir);
CREATE INDEX idx_pipelines_queue_ir_gin ON pipelines USING GIN(queue_ir);
CREATE INDEX idx_pipelines_runner_ir_gin ON pipelines USING GIN(runner_ir);
```

**triggers table:**
```sql
CREATE TABLE triggers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    trigger_type TEXT NOT NULL CHECK (trigger_type IN ('time', 'resource', 'webhook', 'manual', 'file_watch')),
    config JSONB NOT NULL,              -- Trigger-specific configuration
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_pipeline FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE INDEX idx_triggers_pipeline ON triggers(pipeline_id);
CREATE INDEX idx_triggers_type ON triggers(trigger_type);
CREATE INDEX idx_triggers_config_gin ON triggers USING GIN(config);
```

**pipeline_instances table:**
```sql
CREATE TABLE pipeline_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('created', 'queued', 'running', 'exited')),
    exit_code INTEGER,                  -- NULL if not exited
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    queued_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    exited_at TIMESTAMPTZ,
    CONSTRAINT fk_pipeline FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE INDEX idx_instances_pipeline ON pipeline_instances(pipeline_id);
CREATE INDEX idx_instances_status ON pipeline_instances(status);
CREATE INDEX idx_instances_created ON pipeline_instances(created_at DESC);
```

**execution_logs table:**
```sql
CREATE TABLE execution_logs (
    id BIGSERIAL PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    level TEXT NOT NULL CHECK (level IN ('debug', 'info', 'warn', 'error')),
    message TEXT NOT NULL,
    context JSONB,                      -- Structured log context
    CONSTRAINT fk_instance FOREIGN KEY (instance_id) REFERENCES pipeline_instances(id)
);

CREATE INDEX idx_logs_instance ON execution_logs(instance_id, timestamp DESC);
CREATE INDEX idx_logs_timestamp ON execution_logs(timestamp DESC);
```

**PostgreSQL NOTIFY Trigger:**
```sql
-- Notify when pipeline activation changes
CREATE OR REPLACE FUNCTION notify_trigger_updated()
RETURNS trigger AS $$
BEGIN
    IF (TG_OP = 'UPDATE' AND OLD.activated != NEW.activated) OR (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('trigger_updated', json_build_object(
            'pipeline_id', NEW.id,
            'action', CASE WHEN NEW.activated THEN 'added' ELSE 'removed' END
        )::text);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_updated_notify
AFTER INSERT OR UPDATE ON pipelines
FOR EACH ROW EXECUTE FUNCTION notify_trigger_updated();
```

### Data Relationships

```
pipelines (1) ──< (N) triggers
pipelines (1) ──< (N) pipeline_instances (1) ──< (N) execution_logs
```

- One pipeline has many triggers
- One pipeline can have many instances
- One instance has many log entries
- Cascading delete: Deleting pipeline removes all triggers, instances, and logs

### IR Structure

**Compilation Flow:**
```
Polyglot code (.pg file)
    ↓ polyglot compile
{Trigger IR, Queue IR, Runner IR}
    ↓ Stored in PostgreSQL
pipelines table (3 JSONB columns)
```

**Trigger IR (JSONB):**
```json
{
  "triggers": [
    {
      "type": "time",
      "schedule": "0 0 * * *",
      "enabled": true
    },
    {
      "type": "resource",
      "conditions": {
        "cpu_threshold": 50,
        "ram_available_gb": 4
      }
    },
    {
      "type": "manual"
    }
  ]
}
```

**Queue IR (JSONB) - Contains [t] and [Q] logic:**
```json
{
  "timing_logic": {
    "type": "immediate",
    "delay_seconds": 0
  },
  "queue_selector": "default",
  "priority": 5,
  "rate_limit": {
    "max_per_minute": 10
  },
  "resource_limits": {
    "max_cpu_percent": 80,
    "max_ram_gb": 8
  }
}
```

**Runner IR (JSONB) - Execution modes:**
```json
{
  "execution_mode": "sequential",
  "steps": [
    {
      "id": "step1",
      "type": "wrapper",
      "runtime": "Python3.11",
      "code": "print('Hello from Python')",
      "mode": "sequential"
    },
    {
      "id": "step2",
      "type": "wrapper",
      "runtime": "Python3.11",
      "code": "print('Step 2')",
      "mode": "parallel",
      "depends_on": ["step1"]
    }
  ]
}
```

---

## Security Architecture

### Authentication & Authorization (Future)

**MVP:** No authentication (local development only)

**Post-MVP:**
- CLI: API key or token-based auth
- Services: mTLS for inter-service communication (if distributed)
- Database: PostgreSQL role-based access control

### Data Protection

**At Rest:**
- PostgreSQL: Enable encryption at rest (OS-level or managed database)
- Redis: Persistence encryption if enabled

**In Transit:**
- PostgreSQL: TLS connections (`sslmode=require`)
- Redis: TLS enabled (`tls-port` configuration)

### Input Validation

**CLI:**
- clap validates argument types
- Path traversal prevention for `.pg` file paths
- Sanitize user input before database queries (SQLx parameterized queries prevent SQL injection)

**Lexer/Parser:**
- Reject malformed `.pg` files
- Limit file size (prevent DoS via massive files)
- Timeout for compilation (prevent infinite loops)

### Process Isolation

**Services:**
- Each service runs as separate process
- Failures isolated (one service crash doesn't affect others)
- Database connection pooling prevents connection exhaustion

**Runtime Wrappers:**
- Future: Sandbox Python/Node/Rust execution (namespaces, cgroups, or containers)
- MVP: Subprocess isolation only

### Secrets Management

**Configuration:**
- Never hardcode credentials in code
- Database URL via environment variable `DATABASE_URL`
- Redis URL via environment variable `REDIS_URL`
- Future: Integration with HashiCorp Vault or AWS Secrets Manager

---

## Performance Considerations

### Compilation Speed (NFR-P1)

**Target:** <1s compilation for 1000-line `.pg` files

**Strategies:**
- Efficient lexer (single-pass)
- Parser uses zero-copy where possible
- IR generation avoids unnecessary clones
- Benchmark compilation in CI

### Pipeline Execution Latency (NFR-P2)

**Target:** <2s from trigger to execution start

**Strategies:**
- PostgreSQL connection pooling (reuse connections)
- Redis pipelining for queue operations
- Minimize database roundtrips
- Index optimization on `pipelines(activated)` and `pipeline_instances(status)`

### Type Conversion Overhead (NFR-P3)

**Target:** <10ms for typical data sizes (<1MB)

**Strategies:**
- JSON serialization (serde_json is highly optimized)
- Streaming deserialization for large payloads
- Benchmark runtime wrapper performance
- Future: Upgrade to bincode for production if needed

### Queue Throughput (NFR-P4)

**Target:** 100+ instances/second

**Strategies:**
- Redis `RPUSH`/`BLPOP` are O(1) operations
- Batch queue operations where possible
- Monitor queue depth and lag

### Database Query Performance (NFR-P5)

**Target:** <100ms for registry queries, <500ms for logs

**Strategies:**
- Indexes on frequently queried columns
- Use `EXPLAIN ANALYZE` to optimize slow queries
- Connection pooling (default: 10 connections)
- Prepared statements via SQLx (cache query plans)

---

## Deployment Architecture

### MVP Deployment (Local/Single-Node)

**All services on one machine:**
```
┌─────────────────────────────────────┐
│         Local Machine               │
│                                     │
│  ┌──────────────┐  ┌──────────────┐│
│  │ PostgreSQL   │  │    Redis     ││
│  └──────────────┘  └──────────────┘│
│                                     │
│  ┌──────────────────────────────┐  │
│  │  Polyglot Services           │  │
│  │  - Trigger Monitor           │  │
│  │  - Queue Manager             │  │
│  │  - Runner                    │  │
│  └──────────────────────────────┘  │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  polyglot CLI                │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

**Setup:**
```bash
# Install dependencies
brew install postgresql redis  # macOS
sudo apt install postgresql redis  # Linux

# Start services
brew services start postgresql redis  # macOS
systemctl start postgresql redis  # Linux

# Build Polyglot
cargo build --release

# Run migrations
sqlx migrate run

# Start services (3 terminals)
./target/release/polyglot-trigger-monitor
./target/release/polyglot-queue-manager
./target/release/polyglot-runner
```

### Production Deployment (Future: Docker + Orchestration)

**Docker Compose:**
```yaml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: polyglot
      POSTGRES_USER: polyglot
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

  trigger-monitor:
    build: .
    command: /usr/local/bin/polyglot-trigger-monitor
    depends_on:
      - postgres
      - redis
    environment:
      DATABASE_URL: postgresql://polyglot:${DB_PASSWORD}@postgres/polyglot
      REDIS_URL: redis://redis:6379

  queue-manager:
    build: .
    command: /usr/local/bin/polyglot-queue-manager
    depends_on:
      - postgres
      - redis

  runner:
    build: .
    command: /usr/local/bin/polyglot-runner
    depends_on:
      - postgres
      - redis
```

### Scalability (Post-MVP)

**Horizontal Scaling:**
- Multiple Runner instances (stateless, can run in parallel)
- Queue Manager can be sharded by queue
- Trigger Monitor: Single instance (LISTEN/NOTIFY limitation), future: leader election

**Managed Services:**
- AWS RDS PostgreSQL
- AWS ElastiCache Redis
- Deploy services to ECS/EKS or Railway

---

## Development Environment

### Prerequisites

**Required:**
- Rust 1.84+ (2024 edition)
- PostgreSQL 14+
- InfluxDB 2.x
- Redis 7+
- SQLx CLI: `cargo install sqlx-cli`
- Python 3.11+ with uv: `pip install uv`

**Optional:**
- Docker & Docker Compose (for containerized development)
- rust-analyzer (LSP for IDE)

### Setup

```bash
# 1. Clone repository
git clone https://github.com/yourusername/polyglot.git
cd polyglot

# 2. Install Rust dependencies
cargo build

# 3. Set up PostgreSQL
createdb polyglot  # or use Docker
export DATABASE_URL="postgresql://localhost/polyglot"

# 4. Set up InfluxDB
influxd  # or Docker
# Create org, bucket, and token via UI (http://localhost:8086)
export INFLUX_URL="http://localhost:8086"
export INFLUX_TOKEN="your-token"

# 5. Set up Redis
redis-server  # or Docker

# 6. Run migrations
sqlx migrate run

# 7. Install Python with uv
pip install uv

# 8. Copy example config
cp polyglot.toml.example polyglot.toml
# Edit polyglot.toml with your database/InfluxDB/Redis URLs

# 9. Run tests
cargo test --workspace

# 10. Run services (in separate terminals)
cargo run --bin polyglot-trigger-monitor
cargo run --bin polyglot-queue-manager
cargo run --bin polyglot-runner

# 11. Use CLI
cargo run --bin polyglot-cli -- compile examples/hello_world.pg  # Validates + converts + registers
cargo run --bin polyglot-cli -- activate hello_world
cargo run --bin polyglot-cli -- trigger hello_world  # Test/debug: bypass trigger logic
cargo run --bin polyglot-cli -- status <instance_id>
cargo run --bin polyglot-cli -- logs <instance_id>
```

### Environment Variables

```bash
# Required
export DATABASE_URL="postgresql://localhost/polyglot"
export INFLUX_URL="http://localhost:8086"
export INFLUX_TOKEN="your-influxdb-token"
export INFLUX_ORG="polyglot"
export INFLUX_BUCKET="metrics"
export REDIS_URL="redis://localhost:6379"

# Optional
export POLYGLOT_CONFIG="./polyglot.toml"
export RUST_LOG="info,polyglot=debug"  # Logging level
export RUST_BACKTRACE=1  # Stack traces on panic
```

### IDE Configuration

**VSCode (.vscode/settings.json):**
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.rustfmt.extraArgs": ["--edition", "2024"]
}
```

**IntelliJ IDEA / RustRover:**
- Enable Rust plugin
- Set Rust toolchain to 1.84+
- Enable Clippy lints

---

## Architecture Decision Records (ADRs)

### ADR-001: Manual Cargo Workspace Over Starter Template

**Status:** Accepted

**Context:** Polyglot's architecture (3 backend services + CLI + shared libraries) is specialized for a language implementation.

**Decision:** Use manual Cargo workspace setup instead of generic Rust starter templates.

**Rationale:**
- Starter templates don't support multi-service architecture
- Custom workspace gives precise control over crate boundaries
- Polyglot's dependencies (PostgreSQL, Redis, Tokio, serde) are specific

**Consequences:**
- More initial setup work
- Full control over project structure
- Clear separation of concerns

---

### ADR-002: SQLx Over Diesel

**Status:** Accepted

**Context:** Need async-first database client for PostgreSQL.

**Decision:** Use SQLx 0.8.6 with `tokio-comp` feature.

**Rationale:**
- Async-first design (Diesel added async later)
- Compile-time query verification against actual schema
- Lightweight, no heavy ORM abstraction
- Direct SQL queries clearer for complex IR storage

**Consequences:**
- More SQL knowledge required (less abstraction than ORM)
- Better performance for async workloads
- sqlx-cli provides migration tooling

---

### ADR-003: PostgreSQL JSONB for IR Storage

**Status:** Accepted

**Context:** IR is serialized data, but metadata is relational.

**Decision:** Use PostgreSQL with JSONB column for IR, relational columns for metadata.

**Rationale:**
- Best of both worlds: document storage + relational queries
- JSONB supports indexing and querying
- One database instead of PostgreSQL + MongoDB
- ACID guarantees for state management

**Consequences:**
- IR stored as JSON (human-readable for debugging)
- Can query inside IR if needed: `WHERE ir->>'trigger_type' = 'manual'`
- PostgreSQL remains single source of truth

---

### ADR-004: thiserror + anyhow Error Handling

**Status:** Accepted

**Context:** Need error handling strategy for libraries + binaries in async context.

**Decision:**
- thiserror 2.0.17 for library error types
- anyhow 1.0.99 for binary error propagation
- All errors must be `Send + Sync` for async

**Rationale:**
- Rust community standard pattern
- Structured errors in libraries enable caller matching
- Anyhow simplifies context chaining in binaries
- Both support async error propagation

**Consequences:**
- Library errors define custom types with variants
- Binaries use `anyhow::Result<T>` and `.context()`
- Errors cross async boundaries safely

---

### ADR-005: Dynamic Trigger Loading with PostgreSQL LISTEN/NOTIFY

**Status:** Accepted

**Context:** Triggers must reload when pipeline IR changes without service restarts.

**Decision:** Use PostgreSQL LISTEN/NOTIFY for trigger updates, dynamic handler registry in Trigger Monitor.

**Rationale:**
- PostgreSQL NOTIFY is real-time (no polling lag)
- Dynamic loading enables runtime configuration
- Hybrid trigger types (async listening + sync loop) support diverse use cases

**Consequences:**
- Novel architectural pattern (documented in this architecture)
- Trigger Monitor maintains in-memory registry
- Handlers are spawned/stopped on IR changes
- Scalability limited to single Trigger Monitor instance (future: leader election)

---

### ADR-006: Database-Driven Service Communication

**Status:** Accepted

**Context:** 3 backend services need to communicate and share state.

**Decision:** Services communicate via PostgreSQL (state) and Redis (queues), no direct HTTP/RPC.

**Rationale:**
- Simplifies deployment (no service discovery needed)
- Database provides strong consistency
- Redis provides fast queue operations
- Failure isolation (services can restart independently)

**Consequences:**
- All services depend on PostgreSQL/Redis availability
- No need for HTTP servers in services (except future webhooks)
- State transitions visible in database
- Future: May need distributed tracing for debugging

---

### ADR-007: InfluxDB for Time-Series Data

**Status:** Accepted

**Context:** Time-based triggers, trigger execution results, and resource metrics are time-series data.

**Decision:** Add InfluxDB 2.x as MVP requirement for time-series storage.

**Rationale:**
- Time-series database optimized for temporal data
- Efficient storage for resource metrics (CPU, RAM, GPU, Network)
- Better query performance for time-based trigger schedules
- Separate concern from relational data (PostgreSQL)

**Consequences:**
- Additional dependency for MVP (InfluxDB required)
- Trigger Monitor reads from both PostgreSQL and InfluxDB
- Resource Monitor writes metrics to InfluxDB at fixed intervals
- More complex deployment (3 databases: PostgreSQL, InfluxDB, Redis)

---

### ADR-008: 3-IR Structure (Trigger, Queue, Runner)

**Status:** Accepted

**Context:** Pipeline IR has distinct concerns: triggering, queuing, and execution.

**Decision:** Split compiled IR into 3 separate IRs stored as separate JSONB columns.

**Rationale:**
- Separation of concerns (trigger logic ≠ queue logic ≠ execution logic)
- Each service reads only its relevant IR (Trigger Monitor → trigger_ir, etc.)
- Easier to query and index (GIN indexes on each IR column)
- Polyglot syntax `[t]` and `[Q]` map cleanly to Queue IR

**Consequences:**
- Database schema has 3 JSONB columns instead of 1
- Compilation produces 3 IRs from single .pg file
- Each IR has its own schema and validation rules
- Clear boundaries for AI agents implementing services

---

### ADR-009: Resource Monitor as Trigger Monitor Subservice

**Status:** Accepted

**Context:** Resource monitoring (CPU, RAM, GPU, Network) is needed for resource-based triggers.

**Decision:** Implement Resource Monitor as a subservice of Trigger Monitor, not a 4th separate service.

**Rationale:**
- Resource Monitor is triggered by Trigger Monitor
- Tight coupling: Trigger Monitor activates resource monitoring based on pipeline needs
- Reduces complexity (3 services instead of 4)
- Selective monitoring: Only monitor resources specified in pipeline IRs

**Consequences:**
- Trigger Monitor crate has resource monitoring submodule
- Resource metrics written to InfluxDB
- Continuous polling at fixed interval (configurable)
- Single service failure affects both trigger checking and resource monitoring

---

### ADR-010: Compile = Validate + Convert + Register

**Status:** Accepted

**Context:** User workflow for getting .pg files into the system.

**Decision:** `polyglot compile <file>.pg` performs validation, IR conversion, AND database registration in one command.

**Rationale:**
- Simplifies user workflow (one command instead of two)
- Compilation without storage is rarely useful (can compile but not use)
- After compile, pipelines are available for activation
- Matches user mental model: "compile" means "make it ready"

**Consequences:**
- No separate `polyglot register` command needed
- Users must have database connection to compile
- "Compile without register" means validation only (future flag: `--dry-run`)
- After compile, users run `polyglot activate <name>` to enable pipelines

---

### ADR-011: Pause Types (Process Pause + Checkpoints)

**Status:** Accepted

**Context:** Pipelines may need to pause execution for various reasons.

**Decision:** Support 2 pause types:
1. **Process Pause** (OS-level): Freeze memory or cache RAM in Redis
2. **Checkpoints** (Programmatic): User-defined pause points in Polyglot code

**Rationale:**
- Process pause handles resource constraints (reduce CPU load, free RAM)
- Checkpoints enable user-defined pause/resume logic based on conditions
- Pause queue separates paused instances from active dispatch queue
- Flexibility for different pause scenarios (system resources vs. business logic)

**Consequences:**
- Pause queue added to Redis (MVP)
- Runner must support pause/resume operations
- Process pause: OS-level signals (SIGSTOP/SIGCONT) or Redis caching
- Checkpoint pause: Save execution state, resume from checkpoint

---

### ADR-012: PostgreSQL Fallback for Redis Queues

**Status:** Accepted

**Context:** Redis failure would halt all pipeline queuing and execution.

**Decision:** Use PostgreSQL as fallback when Redis is unavailable. Queue Manager polls `pipeline_instances WHERE status='queued'`.

**Rationale:**
- High availability: System continues functioning if Redis fails
- PostgreSQL already stores instance state
- No data loss (instances already in database)
- Graceful degradation (slower but functional)

**Consequences:**
- Queue Manager checks Redis health
- On Redis failure, switch to PostgreSQL polling
- Performance degradation during fallback (polling vs. push/pop)
- When Redis recovers, switch back to Redis-based queuing

---

_Generated by BMAD Decision Architecture Workflow v1.0_
_Date: 2025-11-16_
_For: hhj_