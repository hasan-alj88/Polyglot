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

