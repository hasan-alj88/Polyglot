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

