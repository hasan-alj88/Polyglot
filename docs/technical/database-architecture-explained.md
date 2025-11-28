# Database Architecture: Complete Guide

**Date:** 2025-11-24
**Audience:** Implementers, architects, and developers

---

## Overview

Polyglot uses a **hybrid database architecture** combining three database technologies:

1. **PostgreSQL** - Primary database (relational + document storage)
2. **Redis** - Queue management (fast in-memory operations)
3. **InfluxDB** - Time-series data (metrics, trigger schedules)

---

## Technology Stack

### PostgreSQL (Primary Database)

**Purpose:** Store pipeline definitions, execution state, and logs

**Why PostgreSQL?**
- Hybrid approach: **JSONB columns** for IR storage (document-like) + **relational columns** for metadata
- ACID guarantees for state management
- GIN indexes on JSONB for efficient querying
- LISTEN/NOTIFY for real-time updates

**Key Features Used:**
- JSONB columns for 3 IRs (Trigger, Queue, Runner)
- Foreign keys with cascading deletes
- Partial indexes for performance
- Database triggers for notifications

---

### Redis (Queue System)

**Purpose:** Fast queue operations for pipeline dispatch

**Why Redis?**
- O(1) push/pop operations (`RPUSH`, `BLPOP`)
- Blocking operations (wait for queue items)
- Multiple queues: default, pause, user-defined (future)

**Fallback:** If Redis is down, Queue Manager polls PostgreSQL (`pipeline_instances WHERE status='queued'`)

---

### InfluxDB (Time-Series Database)

**Purpose:** Store time-based triggers, execution results, resource metrics

**Why InfluxDB?**
- Optimized for temporal data (timestamps, intervals)
- Efficient storage for metrics (CPU, RAM, GPU, Network)
- Better query performance for time-based triggers

**Data Stored:**
- Time-based trigger schedules (cron expressions)
- Trigger execution results (when did trigger fire?)
- Resource metrics (CPU usage over time)

---

## UML / Entity Relationship Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         POSTGRESQL                              │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────┐
│     pipelines        │
├──────────────────────┤
│ id (PK)              │ UUID
│ name                 │ TEXT (UNIQUE)
│ version              │ INTEGER
│ trigger_ir           │ JSONB ◄─── Trigger Monitor reads this
│ queue_ir             │ JSONB ◄─── Queue Manager reads this
│ runner_ir            │ JSONB ◄─── Runner reads this
│ created_at           │ TIMESTAMPTZ
│ updated_at           │ TIMESTAMPTZ
│ activated            │ BOOLEAN (default: false)
└──────────┬───────────┘
           │
           │ 1:N (one pipeline, many triggers)
           │
           ├──────────────────────────────────────┐
           │                                      │
           ▼                                      ▼
┌──────────────────────┐           ┌──────────────────────┐
│      triggers        │           │  pipeline_instances  │
├──────────────────────┤           ├──────────────────────┤
│ id (PK)              │           │ id (PK)              │
│ pipeline_id (FK) ────┼───────────┤ pipeline_id (FK)     │
│ trigger_type         │           │ status               │ ('created', 'queued', 'running', 'exited')
│ config (JSONB)       │           │ exit_code            │ INTEGER (NULL if not exited)
│ created_at           │           │ created_at           │ TIMESTAMPTZ
└──────────────────────┘           │ queued_at            │ TIMESTAMPTZ (NULL until queued)
                                   │ started_at           │ TIMESTAMPTZ (NULL until running)
                                   │ exited_at            │ TIMESTAMPTZ (NULL until exited)
                                   └──────────┬───────────┘
                                              │
                                              │ 1:N (one instance, many logs)
                                              │
                                              ▼
                                   ┌──────────────────────┐
                                   │   execution_logs     │
                                   ├──────────────────────┤
                                   │ id (PK)              │ BIGSERIAL
                                   │ instance_id (FK)     │
                                   │ timestamp            │ TIMESTAMPTZ
                                   │ level                │ TEXT ('debug', 'info', 'warn', 'error')
                                   │ message              │ TEXT
                                   │ context (JSONB)      │ Structured log data
                                   └──────────────────────┘


┌─────────────────────────────────────────────────────────────────┐
│                            REDIS                                │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────┐
│  dispatch_queue      │ ◄─── Queue Manager pushes (RPUSH)
│  (Redis List)        │ ◄─── Runner pops (BLPOP)
│                      │
│  Items: instance_ids │ (UUIDs as strings)
└──────────────────────┘

┌──────────────────────┐
│    pause_queue       │ ◄─── Paused instances go here
│  (Redis List)        │
│                      │
│  Items: instance_ids │
└──────────────────────┘


┌─────────────────────────────────────────────────────────────────┐
│                          INFLUXDB                               │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────┐
│  trigger_schedules   │ ◄─── Trigger Monitor reads
│  (Time-series)       │
│                      │
│  Fields:             │
│  - pipeline_id       │
│  - trigger_type      │
│  - schedule (cron)   │
│  - next_run_time     │
└──────────────────────┘

┌──────────────────────┐
│   trigger_results    │ ◄─── Trigger Monitor writes
│  (Time-series)       │
│                      │
│  Fields:             │
│  - pipeline_id       │
│  - trigger_type      │
│  - fired_at          │
│  - result (success)  │
└──────────────────────┘

┌──────────────────────┐
│  resource_metrics    │ ◄─── Resource Monitor subservice writes
│  (Time-series)       │
│                      │
│  Fields:             │
│  - timestamp         │
│  - cpu_percent       │
│  - ram_available_gb  │
│  - gpu_percent       │
│  - network_mbps      │
└──────────────────────┘
```

---

## Detailed Table Descriptions

### 1. `pipelines` Table

**Purpose:** Store compiled pipeline definitions

**Key Columns:**

| Column | Type | Purpose |
|--------|------|---------|
| `id` | UUID | Primary key, globally unique pipeline ID |
| `name` | TEXT | Human-readable name (unique constraint) |
| `version` | INTEGER | Version number (for future versioning support) |
| `trigger_ir` | JSONB | Trigger IR - defines WHEN pipeline runs |
| `queue_ir` | JSONB | Queue IR - defines HOW pipeline is queued (`[t]`, `[Q]` logic) |
| `runner_ir` | JSONB | Runner IR - defines WHAT pipeline does (execution steps) |
| `activated` | BOOLEAN | Is this pipeline active? (triggers listening) |

**JSONB Columns Explained:**

The 3 IR columns store JSON documents with variable state metadata:

**Example `runner_ir` with Variable States:**
```json
{
  "execution_mode": "sequential",
  "steps": [...],
  "variables": {
    ".user_data": {
      "type": "#UserProfile",
      "state": "Pending",
      "override_count": 0,
      "errors": [],
      "assignment_operator": ">+",
      "source": "pipeline_output"
    },
    ".config": {
      "type": "#Config",
      "state": "DefaultReady",
      "override_count": 0,
      "errors": [],
      "default_values": {
        ".timeout": 30,
        ".retries": 3
      },
      "assignment_operator": "<~"
    }
  }
}
```

**Why 3 Separate IRs?**
- Separation of concerns: Trigger Monitor only reads `trigger_ir`, Queue Manager only reads `queue_ir`, Runner only reads `runner_ir`
- Easier to query (GIN indexes on each column)
- Clear responsibility boundaries

---

### 2. `triggers` Table

**Purpose:** Store trigger configurations for pipelines

**Key Columns:**

| Column | Type | Purpose |
|--------|------|---------|
| `pipeline_id` | UUID (FK) | Which pipeline does this trigger belong to? |
| `trigger_type` | TEXT | Type: 'time', 'resource', 'webhook', 'manual', 'file_watch' |
| `config` | JSONB | Trigger-specific configuration (schedules, thresholds, etc.) |

**Example Configurations:**

**Time Trigger:**
```json
{
  "schedule": "0 0 * * *",  // Daily at midnight (cron)
  "timezone": "UTC"
}
```

**Resource Trigger:**
```json
{
  "conditions": {
    "cpu_threshold": 50,        // Trigger when CPU < 50%
    "ram_available_gb": 4,      // AND RAM > 4GB available
    "operator": "AND"
  }
}
```

**Webhook Trigger:**
```json
{
  "endpoint": "/webhooks/my-pipeline",
  "method": "POST",
  "auth": "bearer_token"
}
```

---

### 3. `pipeline_instances` Table

**Purpose:** Track individual pipeline executions

**Key Columns:**

| Column | Type | Purpose |
|--------|------|---------|
| `id` | UUID | Unique execution ID |
| `pipeline_id` | UUID (FK) | Which pipeline is this an instance of? |
| `status` | TEXT | Current state: 'created', 'queued', 'running', 'exited' |
| `exit_code` | INTEGER | Exit status (0 = success, non-zero = failure), NULL if not exited |
| `created_at` | TIMESTAMPTZ | When was this instance created? |
| `queued_at` | TIMESTAMPTZ | When was it added to dispatch queue? |
| `started_at` | TIMESTAMPTZ | When did execution start? |
| `exited_at` | TIMESTAMPTZ | When did it finish? |

**Instance Lifecycle:**

```
Trigger fires
    ↓
[created] ──→ Insert into pipeline_instances with status='created'
    ↓
Queue Manager picks up
    ↓
[queued] ──→ Update status='queued', set queued_at, push to Redis
    ↓
Runner dequeues
    ↓
[running] ──→ Update status='running', set started_at
    ↓
Execution completes
    ↓
[exited] ──→ Update status='exited', set exited_at, set exit_code
```

---

### 4. `execution_logs` Table

**Purpose:** Store execution logs for debugging and monitoring

**Key Columns:**

| Column | Type | Purpose |
|--------|------|---------|
| `instance_id` | UUID (FK) | Which instance generated this log? |
| `timestamp` | TIMESTAMPTZ | When was this logged? |
| `level` | TEXT | Log level: 'debug', 'info', 'warn', 'error' |
| `message` | TEXT | Log message |
| `context` | JSONB | Structured context (variables, stack traces, etc.) |

**Example Log Entry:**
```json
{
  "id": 12345,
  "instance_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2025-11-24T10:30:00Z",
  "level": "error",
  "message": "Variable .user_data transitioned to Faulted state",
  "context": {
    "variable_name": ".user_data",
    "state": "Faulted",
    "errors": [
      {
        "type": "!pg.Network.Timeout",
        "message": "API request timed out after 30s"
      }
    ]
  }
}
```

**Why JSONB for `context`?**
- Flexible structure (different log types have different context)
- Can query inside context: `WHERE context->>'variable_name' = '.user_data'`
- Structured logs enable better debugging

---

## Relationships & Cascading Deletes

### One-to-Many Relationships

```
pipelines (1) ──< (N) triggers
pipelines (1) ──< (N) pipeline_instances (1) ──< (N) execution_logs
```

**Cascading Delete Behavior:**

```sql
DELETE FROM pipelines WHERE id = 'some-uuid';
```

**Effect:**
1. All `triggers` for that pipeline are deleted
2. All `pipeline_instances` for that pipeline are deleted
3. All `execution_logs` for those instances are deleted

**Why?** When a pipeline is deleted, its execution history is no longer relevant.

---

## Database Triggers & Real-Time Updates

### PostgreSQL NOTIFY Trigger

**Purpose:** Notify Trigger Monitor when pipeline activation changes

**How It Works:**

1. User runs `polyglot activate my-pipeline`
2. CLI updates: `UPDATE pipelines SET activated = TRUE WHERE name = 'my-pipeline'`
3. PostgreSQL trigger fires: `notify_trigger_updated()`
4. PostgreSQL sends notification: `NOTIFY trigger_updated, '{"pipeline_id": "...", "action": "added"}'`
5. Trigger Monitor receives notification (via `LISTEN trigger_updated`)
6. Trigger Monitor loads trigger IR and spawns appropriate handler

**SQL Implementation:**
```sql
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

---

## Variable States in Database

### Where Are Variable States Stored?

**Answer:** Inside the 3 JSONB IR columns (`trigger_ir`, `queue_ir`, `runner_ir`)

**Example: Variable State in `runner_ir`**

```json
{
  "runner_ir": {
    "execution_mode": "sequential",
    "steps": [
      {
        "id": "step1",
        "type": "wrapper",
        "runtime": "Python3.11",
        "code": "result = fetch_user(user_id)"
      }
    ],
    "variables": {
      ".user_id": {
        "type": "pg\\string",
        "state": "Ready",
        "value": "user123",
        "override_count": 0,
        "errors": []
      },
      ".user_data": {
        "type": "#UserProfile",
        "state": "Pending",
        "override_count": 0,
        "errors": [],
        "assignment_operator": ">+",
        "source": "step1_output"
      }
    }
  }
}
```

### Querying Variable States

**PostgreSQL JSONB Operators:**

```sql
-- Find all pipelines with Faulted variables
SELECT id, name
FROM pipelines
WHERE runner_ir->'variables' @> '{"state": "Faulted"}';

-- Find pipelines using a specific variable
SELECT id, name
FROM pipelines
WHERE runner_ir->'variables' ? '.user_data';

-- Get variable state for specific pipeline
SELECT runner_ir->'variables'->'.user_data'->>'state' AS var_state
FROM pipelines
WHERE name = 'my-pipeline';
```

### State Transition Logging

State transitions are logged to `execution_logs`:

```sql
INSERT INTO execution_logs (instance_id, level, message, context)
VALUES (
  'instance-uuid',
  'info',
  'Variable transitioned to Ready state',
  '{"variable_name": ".user_data", "from_state": "Pending", "to_state": "Ready"}'::jsonb
);
```

---

## Data Flow Through The System

### 1. Pipeline Compilation & Registration

```
User: polyglot compile my_pipeline.pg
    ↓
CLI: Lexer + Parser → AST
    ↓
CLI: AST → {Trigger IR, Queue IR, Runner IR}
    ↓
PostgreSQL: INSERT INTO pipelines (trigger_ir, queue_ir, runner_ir)
    ↓
Result: Pipeline registered, but NOT activated
```

---

### 2. Pipeline Activation

```
User: polyglot activate my_pipeline
    ↓
CLI: UPDATE pipelines SET activated = TRUE WHERE name = 'my_pipeline'
    ↓
PostgreSQL Trigger: notify_trigger_updated() fires
    ↓
PostgreSQL: NOTIFY trigger_updated, '{"pipeline_id": "...", "action": "added"}'
    ↓
Trigger Monitor: LISTEN trigger_updated receives notification
    ↓
Trigger Monitor: Reads trigger_ir from pipelines table
    ↓
Trigger Monitor: Spawns appropriate trigger handler (Time, Resource, Webhook, etc.)
    ↓
Result: Trigger handler now monitoring for trigger condition
```

---

### 3. Trigger Fires & Instance Creation

```
Trigger Condition Met (e.g., cron schedule fires)
    ↓
Trigger Handler: INSERT INTO pipeline_instances (pipeline_id, status='created')
    ↓
PostgreSQL: Returns instance_id
    ↓
Trigger Handler: Writes to InfluxDB (trigger_results: fired_at, pipeline_id)
    ↓
Result: Instance created, waiting for queue manager
```

---

### 4. Queue Manager Picks Up Instance

```
Queue Manager: Polls pipeline_instances WHERE status='created'
    ↓
Queue Manager: Reads queue_ir from pipelines table
    ↓
Queue Manager: Applies [t] timing logic (immediate, delayed, scheduled)
    ↓
Queue Manager: Applies [Q] queue selection (default, pause, user-defined)
    ↓
Queue Manager: UPDATE pipeline_instances SET status='queued', queued_at=NOW()
    ↓
Redis: RPUSH dispatch_queue, instance_id
    ↓
Result: Instance queued, waiting for runner
```

---

### 5. Runner Executes Instance

```
Runner: BLPOP dispatch_queue (blocking wait for queue items)
    ↓
Redis: Returns instance_id
    ↓
Runner: Reads pipeline_instances to get pipeline_id
    ↓
Runner: Reads runner_ir from pipelines table
    ↓
Runner: UPDATE pipeline_instances SET status='running', started_at=NOW()
    ↓
Runner: Executes steps from runner_ir
    ├─ Variable state transitions (Pending → Ready/Faulted)
    ├─ Logs written to execution_logs table
    └─ Error handling via [!] blocks
    ↓
Runner: UPDATE pipeline_instances SET status='exited', exited_at=NOW(), exit_code=0
    ↓
Result: Execution complete
```

---

### 6. Fallback: Redis Unavailable

```
Queue Manager: Attempt RPUSH dispatch_queue
    ↓
Redis: Connection refused / timeout
    ↓
Queue Manager: Switch to PostgreSQL polling mode
    ↓
Queue Manager: Continuously poll:
    SELECT id FROM pipeline_instances WHERE status='queued' ORDER BY created_at LIMIT 10
    ↓
Runner: Polls PostgreSQL for queued instances instead of BLPOP Redis
    ↓
Runner: UPDATE pipeline_instances SET status='running'
    ↓
Result: Degraded performance (polling vs push/pop) but system continues
```

---

## Indexes & Performance

### Why These Indexes?

**`idx_pipelines_activated` (Partial Index):**
```sql
CREATE INDEX idx_pipelines_activated ON pipelines(activated) WHERE activated = TRUE;
```
- **Purpose:** Trigger Monitor only queries activated pipelines
- **Benefit:** Smaller index (only active pipelines), faster lookups

**`idx_pipelines_trigger_ir_gin` (GIN Index):**
```sql
CREATE INDEX idx_pipelines_trigger_ir_gin ON pipelines USING GIN(trigger_ir);
```
- **Purpose:** Enable fast queries inside JSONB (e.g., `trigger_ir @> '{"type": "time"}'`)
- **Benefit:** Find pipelines by trigger type without full table scan

**`idx_instances_status`:**
```sql
CREATE INDEX idx_instances_status ON pipeline_instances(status);
```
- **Purpose:** Queue Manager queries `WHERE status='created'` or `WHERE status='queued'`
- **Benefit:** Fast filtering by status

**`idx_logs_instance` (Composite Index):**
```sql
CREATE INDEX idx_logs_instance ON execution_logs(instance_id, timestamp DESC);
```
- **Purpose:** Fetch logs for specific instance, sorted by time (newest first)
- **Benefit:** CLI command `polyglot logs <instance_id>` is fast

---

## Database Connection Patterns

### Connection Pooling

**All services use SQLx connection pooling:**

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(10)  // Default pool size
    .connect(&database_url)
    .await?;
```

**Why?**
- Reuse connections (avoid connection overhead)
- Limit concurrent connections to PostgreSQL
- Handle connection failures gracefully

---

### Transaction Patterns

**Atomic Operations:**

```rust
async fn create_and_queue_instance(
    pool: &PgPool,
    pipeline_id: Uuid,
) -> Result<Uuid> {
    let mut tx = pool.begin().await?;

    // 1. Create instance
    let instance_id = sqlx::query_scalar!(
        "INSERT INTO pipeline_instances (pipeline_id, status)
         VALUES ($1, 'created') RETURNING id",
        pipeline_id
    )
    .fetch_one(&mut *tx)
    .await?;

    // 2. Update status
    sqlx::query!(
        "UPDATE pipeline_instances SET status = 'queued', queued_at = NOW()
         WHERE id = $1",
        instance_id
    )
    .execute(&mut *tx)
    .await?;

    // 3. Commit transaction (both operations succeed or both fail)
    tx.commit().await?;

    Ok(instance_id)
}
```

**Why Transactions?**
- ACID guarantees: Both operations succeed or both fail
- Prevents inconsistent state (instance created but not queued)

---

## Future Considerations

### When to Add `variable_states` Table?

**Current:** Variable states stored in `runner_ir` JSONB column

**Future (if needed):** Separate table for queryability:

```sql
CREATE TABLE variable_states (
    id UUID PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id),
    variable_name TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('Declared', 'DefaultReady', ...)),
    value JSONB,
    errors JSONB,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_var_states_instance ON variable_states(instance_id);
CREATE INDEX idx_var_states_name ON variable_states(variable_name);
```

**When to Migrate?**
- Need to query variable states across many pipelines
- Analytics: "Show all Faulted variables in last 30 days"
- Debugging: "Find instances where .user_data was Pending for >5 minutes"

**For Now:** JSONB is sufficient (simpler schema, fewer tables)

---

## Summary

### Database Stack

| Database | Purpose | Key Features |
|----------|---------|--------------|
| **PostgreSQL** | Primary storage | JSONB for IRs, LISTEN/NOTIFY, transactions, cascading deletes |
| **Redis** | Queue management | Fast push/pop, blocking operations, fallback to PostgreSQL |
| **InfluxDB** | Time-series data | Trigger schedules, metrics, resource monitoring |

### Key Tables

| Table | Purpose | Relationships |
|-------|---------|---------------|
| `pipelines` | Store compiled pipelines | 1:N with triggers, 1:N with instances |
| `triggers` | Trigger configurations | N:1 with pipelines |
| `pipeline_instances` | Track executions | N:1 with pipelines, 1:N with logs |
| `execution_logs` | Execution logs | N:1 with instances |

### Variable States Storage

- Stored in `runner_ir` JSONB column
- Includes: state, override_count, errors, assignment_operator
- Queryable via PostgreSQL JSONB operators
- State transitions logged to `execution_logs`

### Performance Optimizations

- Connection pooling (reuse connections)
- Partial indexes (smaller, faster)
- GIN indexes on JSONB (fast queries inside JSON)
- Composite indexes (multi-column queries)

---

**Architecture designed for async-centric coordination with state-aware variables!** 🚀
