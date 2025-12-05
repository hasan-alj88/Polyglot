# Database Schema Design

**Document Type:** Architecture Design Document
**Version:** 0.0.2
**Last Updated:** 2025-11-20
**Status:** Active Development - Design Phase

---

## Overview

Complete PostgreSQL schema for IR storage and workflow state management.

This schema stores the Intermediate Representation (IR) as JSONB columns in PostgreSQL, enabling efficient queries by all system components (Trigger Monitor, Queue Manager, Runner Pool). Each pipeline has 3 separate IRs stored as JSONB: Trigger IR, Queue IR, and Runner IR.

---

## Related Documents

### Architecture
- [System Architecture Overview](./00-overview.md) - Component interaction
- [IR Representation](./02-ir-representation.md) - Type system & IR structure

### Language Specification
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Language syntax
- [Type System](../language/02-type-system.md) - Data types

---

## Schema Philosophy

**Hybrid Storage Design:**
- **IR as JSONB:** Compiled IR stored in 3 JSONB columns (trigger_ir, queue_ir, runner_ir)
- **Metadata as Relational:** Package info, state, queues stored in relational tables
- **Best of Both Worlds:** Document flexibility + relational power
- **Efficient Queries:** GIN indexes on JSONB enable fast IR queries
- **Service Separation:** Each service reads only its relevant IR column

**IR Structure (ADR-008: 3-IR Architecture):**
- **Trigger IR:** Trigger conditions and event logic
- **Queue IR:** Queue management and priority logic
- **Runner IR:** Execution flow and runtime orchestration

**Type Storage:**
- Type information embedded in IR JSON using v0.0.2 syntax
- Examples: `'py\int'`, `'pg\path'`, `'pg\dt'`
- Enables type validation and cross-language conversion

---

## Core Tables

### 1. `packages`
Stores package metadata and versioning information.

```sql
CREATE TABLE packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    registry_type VARCHAR(50) NOT NULL, -- 'local', 'community', 'company'
    version VARCHAR(50) NOT NULL,       -- major.minor.patch[.alpha|.beta]
    author VARCHAR(255),
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_committed BOOLEAN DEFAULT FALSE, -- false = .alpha, true = committed
    is_testing BOOLEAN DEFAULT FALSE,   -- true = .beta

    UNIQUE(name, version)
);

CREATE INDEX idx_packages_name ON packages(name);
CREATE INDEX idx_packages_version ON packages(version);
```

**Versioning Rules:**
- Committed: `major.minor.patch`
- Uncommitted: `major.minor.patch.alpha`
- Testing: `major.minor.patch.beta`

---

### 2. `pipelines`
Stores the compiled pipeline IR in 3 separate JSONB columns.

```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID REFERENCES packages(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    file_number INTEGER DEFAULT 1,  -- For multi-file packages

    -- 3-IR Architecture (ADR-008): Separate IRs for each service
    trigger_ir JSONB NOT NULL,    -- Trigger Monitor reads this
    queue_ir JSONB NOT NULL,      -- Queue Manager reads this
    runner_ir JSONB NOT NULL,     -- Runner Pool reads this

    -- State Management
    activated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(package_id, name, version)
);

-- Indexes for efficient queries
CREATE INDEX idx_pipelines_package ON pipelines(package_id);
CREATE INDEX idx_pipelines_activated ON pipelines(activated) WHERE activated = TRUE;

-- GIN indexes for JSONB query performance
CREATE INDEX idx_pipelines_trigger_ir_gin ON pipelines USING GIN(trigger_ir);
CREATE INDEX idx_pipelines_queue_ir_gin ON pipelines USING GIN(queue_ir);
CREATE INDEX idx_pipelines_runner_ir_gin ON pipelines USING GIN(runner_ir);
```

---

### 3. `pipeline_instances`
Tracks individual workflow execution instances.

```sql
CREATE TABLE pipeline_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID REFERENCES pipelines(id) ON DELETE CASCADE,

    -- Execution State
    status VARCHAR(50) NOT NULL,  -- 'pending', 'running', 'paused', 'completed', 'failed'
    current_step INTEGER,

    -- Checkpointing Data
    checkpoint_data JSONB,  -- Serialized execution state
    checkpoint_at TIMESTAMP,

    -- Timing
    started_at TIMESTAMP,
    completed_at TIMESTAMP,

    -- Results
    exit_code INTEGER,
    error_message TEXT,

    -- Metadata
    triggered_by VARCHAR(100),  -- 'schedule', 'file', 'webhook', 'manual', 'pipeline'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_instance_status ON pipeline_instances(status);
CREATE INDEX idx_instance_pipeline ON pipeline_instances(pipeline_id);
```

---

### 4. `triggers`
Stores trigger definitions extracted from Trigger IR.

```sql
CREATE TABLE triggers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID REFERENCES pipelines(id) ON DELETE CASCADE,

    -- Trigger Configuration
    trigger_type VARCHAR(50) NOT NULL,  -- 'schedule', 'file', 'webhook', 'resource', 'pipeline'
    trigger_config JSONB NOT NULL,      -- Type-specific config

    -- Examples of trigger_config:
    -- Schedule: {"cron": "0 16 30 * * *", "timezone": "UTC"}
    -- File: {"path": "/data/*.csv", "events": ["create", "modify"]}
    -- Resource: {"metric": "cpu_usage", "threshold": 50, "operator": "<"}
    -- Webhook: {"endpoint": "/api/trigger/xyz", "method": "POST"}

    is_active BOOLEAN DEFAULT TRUE,
    last_triggered_at TIMESTAMP,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_triggers_type ON triggers(trigger_type);
CREATE INDEX idx_triggers_pipeline ON triggers(pipeline_id);
```

---

### 5. `queue_definitions`
Stores user-defined queue behavior from IR.

```sql
CREATE TABLE queue_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID REFERENCES pipelines(id),

    -- Priority Configuration
    base_priority INTEGER DEFAULT 5,  -- 1-10 scale, user-defined
    priority_boost_rules JSONB,       -- Dynamic priority adjustments

    -- Resource Requirements
    required_resources JSONB,  -- {"cpu": 2, "memory_mb": 4096, "gpu": 0}

    -- Timeout Settings
    max_execution_time_seconds INTEGER,

    -- Retry Configuration
    max_retries INTEGER DEFAULT 0,
    retry_delay_seconds INTEGER DEFAULT 60,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

---

### 6. `queue_entries`
Active queue entries (Pending, Dispatch, Pause).

```sql
CREATE TABLE queue_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_instance_id UUID REFERENCES pipeline_instances(id),

    -- Queue Management
    queue_type VARCHAR(50) NOT NULL,  -- 'pending', 'dispatch', 'pause'
    priority INTEGER NOT NULL,

    -- Resource Tracking
    allocated_resources JSONB,  -- Resources assigned to this execution

    -- Scheduling
    enqueued_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    dequeued_at TIMESTAMP,

    -- Pause/Resume
    pause_reason VARCHAR(255),  -- 'resource_wait', 'checkpoint', 'manual', 'error'
    resume_condition JSONB,     -- Condition for automatic resume

    CONSTRAINT chk_queue_type CHECK (queue_type IN ('pending', 'dispatch', 'pause'))
);

CREATE INDEX idx_queue_type ON queue_entries(queue_type);
CREATE INDEX idx_queue_priority ON queue_entries(priority DESC, enqueued_at ASC);
```

---

### 7. `execution_steps`
Individual step execution within a pipeline instance.

```sql
CREATE TABLE execution_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_instance_id UUID REFERENCES pipeline_instances(id),

    -- Step Definition
    step_order INTEGER NOT NULL,
    step_type VARCHAR(50) NOT NULL,  -- 'run', 'parallel', 'join', 'log', etc.
    language VARCHAR(50),            -- 'python', 'rust', 'javascript', etc.

    -- Execution
    status VARCHAR(50) NOT NULL,     -- 'pending', 'running', 'completed', 'failed'
    started_at TIMESTAMP,
    completed_at TIMESTAMP,

    -- Input/Output
    inputs JSONB,
    outputs JSONB,

    -- Results
    exit_code INTEGER,
    stdout TEXT,
    stderr TEXT,
    error_message TEXT,

    -- Resource Usage (captured post-execution)
    cpu_usage_percent DECIMAL(5,2),
    memory_usage_mb INTEGER,
    duration_ms BIGINT
);

CREATE INDEX idx_step_instance ON execution_steps(pipeline_instance_id);
CREATE INDEX idx_step_status ON execution_steps(status);
```

---

### 8. `resources`
Current system resource snapshot (Resource Monitor data).

```sql
CREATE TABLE resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Resource Metrics
    cpu_usage_percent DECIMAL(5,2),
    cpu_cores_available INTEGER,
    memory_total_mb BIGINT,
    memory_used_mb BIGINT,
    memory_available_mb BIGINT,

    gpu_count INTEGER DEFAULT 0,
    gpu_usage_percent JSONB,  -- Array of GPU usage per device

    disk_usage_percent DECIMAL(5,2),

    -- Custom Metrics (user-defined)
    custom_metrics JSONB,

    -- Network Metrics (future)
    network_in_mbps DECIMAL(10,2),
    network_out_mbps DECIMAL(10,2),

    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_resources_timestamp ON resources(timestamp DESC);

-- Cleanup old resource snapshots (keep last 24 hours by default)
-- Handled by background task
```

---

## Table Relationships

```
packages (1) ──┐
               ├──> pipelines (N) ──┐
               │                    ├──> triggers (N)
               │                    ├──> queue_definitions (1)
               │                    │
               │                    └──> pipeline_instances (N) ──┐
               │                                                   ├──> execution_steps (N)
               │                                                   └──> queue_entries (N)
               │
               └──> (multiple pipelines per package)

resources (independent snapshot table)

Note: IR is stored in pipelines table as 3 JSONB columns (trigger_ir, queue_ir, runner_ir)
```

---

## Component Query Patterns

### Trigger Monitor Queries

```sql
-- Find all active triggers for monitoring
SELECT t.trigger_type, t.trigger_config, pd.name as pipeline_name, p.name as package_name
FROM triggers t
JOIN pipelines pd ON t.pipeline_id = pd.id
JOIN packages p ON pd.package_id = p.id
WHERE t.is_active = TRUE;

-- Query trigger IR for specific pipeline
SELECT trigger_ir FROM pipelines WHERE id = ?;

-- Find pipelines with specific trigger type (using JSONB query)
SELECT name, trigger_ir FROM pipelines
WHERE trigger_ir @> '{"type": "schedule"}';
```

### Queue Manager Queries

```sql
-- Get queue behavior for pipeline
SELECT base_priority, required_resources, max_retries
FROM queue_definitions
WHERE pipeline_id = ?;

-- Get pending queue entries ordered by priority
SELECT qe.*, pi.pipeline_id
FROM queue_entries qe
JOIN pipeline_instances pi ON qe.pipeline_instance_id = pi.id
WHERE qe.queue_type = 'pending'
ORDER BY qe.priority DESC, qe.enqueued_at ASC;
```

### Runner Queries

```sql
-- Get runner IR for pipeline execution
SELECT runner_ir FROM pipelines WHERE id = ?;

-- Query specific execution mode from runner IR
SELECT runner_ir->'execution_mode' FROM pipelines WHERE id = ?;

-- Track step execution
UPDATE execution_steps
SET status = 'running', started_at = CURRENT_TIMESTAMP
WHERE id = ?;

-- Find all pipelines using specific runtime (using JSONB query)
SELECT name, runner_ir FROM pipelines
WHERE runner_ir @> '{"runtime": "python"}';
```

---

## See Also

- [System Architecture](./00-overview.md) - Component design
- [IR Representation](./02-ir-representation.md) - IR structure details
- [Type System](../language/02-type-system.md) - Type specifications
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Language syntax

---

**Note:** This schema implements ADR-003 (PostgreSQL JSONB for IR) and ADR-008 (3-IR Architecture). All type strings use v0.0.2 syntax: `pg\int`, `py\dict`, `pg\dt`, etc.
