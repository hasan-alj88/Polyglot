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

**variable_state_transitions table:**
```sql
CREATE TABLE variable_state_transitions (
    id BIGSERIAL PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id) ON DELETE CASCADE,
    variable_name TEXT NOT NULL,              -- Fully qualified variable path (e.g., ".user_data.address.city")
    from_state TEXT NOT NULL CHECK (from_state IN ('Declared', 'DefaultReady', 'Pending', 'Ready', 'Faulted', 'Retrying', 'Paused', 'Cached', 'Dirty')),
    to_state TEXT NOT NULL CHECK (to_state IN ('Declared', 'DefaultReady', 'Pending', 'Ready', 'Faulted', 'Retrying', 'Paused', 'Cached', 'Dirty')),
    transition_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    context JSONB,                            -- Additional context: trigger type, error details, etc.
    CONSTRAINT fk_instance FOREIGN KEY (instance_id) REFERENCES pipeline_instances(id)
);

CREATE INDEX idx_state_transitions_instance ON variable_state_transitions(instance_id, transition_timestamp DESC);
CREATE INDEX idx_state_transitions_variable ON variable_state_transitions(variable_name, transition_timestamp DESC);
CREATE INDEX idx_state_transitions_timestamp ON variable_state_transitions(transition_timestamp DESC);
```

**Purpose:** Audit trail for variable state transitions, useful for debugging, performance analysis, and understanding variable lifecycle in production.

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
                                          (1) ──< (N) variable_state_transitions
```

- One pipeline has many triggers
- One pipeline can have many instances
- One instance has many log entries
- One instance has many state transition records
- Cascading delete: Deleting pipeline removes all triggers, instances, logs, and state transitions

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

### Variable State Model

**Status:** Core language feature (v1.0.0, 2025-11-24)
**Specification:** `docs/technical/variable-states-specification.md`

#### Overview

Polyglot variables are **state-aware** entities that transition through explicit states as async operations complete. This is not an implementation detail—it's the foundational semantic model for all Polyglot data.

Variables don't simply "have values" or "don't have values." They progress through a lifecycle:
- `Declared` → Schema defined, no value yet
- `Pending` → Async operation in progress
- `Ready` → Value available, immutable
- `Faulted` → Operation failed

This state model enables Polyglot's **automatic waiting** behavior: pipelines block on `Pending` variables without explicit `await` keywords.

#### The Nine Variable States

Polyglot defines 9 distinct variable states organized into two categories:

**Core States (5) - MVP Implementation - Required for basic variable lifecycle:**

| State | Description | Can Read Value? | Triggers Pipeline? | Transition Triggers |
|-------|-------------|----------------|-------------------|-------------------|
| `Declared` | Schema defined, no value, no default | ❌ No | ❌ No (must be Ready) | Pipeline assignment, explicit population |
| `DefaultReady` | Has default value, allows ONE override | ✅ Yes (default) | ✅ Yes | First use, override → `Ready` |
| `Pending` | Async operation in progress | ❌ No | ⚠️ Waits | Pipeline completion → `Ready`/`Faulted` |
| `Ready` | Value available, immutable | ✅ Yes | ✅ Yes | None (terminal state) |
| `Faulted` | Operation failed, has error info | ❌ No (check `.errors`) | ❌ No (error path) | Retry → `Retrying` |

**Queue Management States (4) - Post-MVP - Advanced resilience and performance:**

| State | Description | Purpose | Transition |
|-------|-------------|---------|------------|
| `Retrying` | Automatic retry attempt in progress | Transient failure recovery | → `Ready`/`Faulted` |
| `Paused` | Waiting for external trigger | Human approval, scheduled events | → `Pending` (when triggered) |
| `Cached` | Cached result, may be stale | Performance optimization | → `Dirty` (on invalidation) |
| `Dirty` | Cache invalid, needs refresh | Cache invalidation | → `Pending` (on refresh) |

**Implementation Priority:** The 5 core states are required for MVP (v0.1.0). The 4 queue management states will be implemented post-MVP as advanced features for enterprise use cases requiring retry logic, caching, and pause/resume capabilities.

#### Assignment Operators and State Control

**Critical Concept:** Polyglot's `<<` and `>>` operators are **NOT** like `=` in other languages. They are **directional push/pull operators** that define data flow and represent **final assignment** (immutability once Ready).

Polyglot has **three ways** to declare fields, each with different **push count** allowances:

**1. Schema-Only Declaration (No Operator) → 1 Push Allowed**
```polyglot
[#] UserProfile
[<] .id: pg\string          # Declared state - must populate later
[<] .name: pg\string        # No default, no value
[X]

[i] .user: #UserProfile << #UserProfile
[<] .user.id << "user_123"       # ← 1st push: FINAL (now Ready)
[<] .user.id << "user_456"       # ← ERROR: Already had final push
```
- **Initial State:** `Declared`
- **Push Count:** 1 final push with `<<` or `>>`
- **Semantics:** Field has no value until explicitly pushed to
- **After first push:** Transitions to `Ready` (immutable)
- **Use case:** Fields populated by pipelines, required parameters

**2. Default Assignment (`<~` / `~>`) → 2 Pushes Allowed**
```polyglot
[#] Config
[<] .timeout: pg\int <~ 30      # DefaultReady - can override once
[<] .retries: pg\int <~ 3        # Default value provided
[X]

[i] .config: #Config << #Config{.timeout: 60}  # ← 1st push: Override default
[<] .config.timeout << 90                       # ← 2nd push: FINAL (now Ready)
[<] .config.timeout << 120                      # ← ERROR: Already had final push
```
- **Initial State:** `DefaultReady`
- **Push Count:** 2 pushes with `<<` or `>>`
  - **1st push:** Override the default (stays DefaultReady temporarily)
  - **2nd push:** Final assignment (transitions to Ready, immutable)
- **Alternative:** If pulled from without override, uses default and becomes Ready
- **Use case:** Configuration with sensible defaults, optional parameters
- **Discouraged anti-pattern:** Using `<~`/`~>` to pull from constants (`<<`/`>>`)

**3. Constant/Async Assignment (`<<` / `>>`) → 0 or 1 Push**
```polyglot
[#] AppInfo
[<] .version: pg\string << "1.0.0"    # Ready immediately (constant)
[X]

[<] .version << "2.0.0"               # ← ERROR: Already Ready (0 additional pushes)

[r] |FetchData
[>] .result: pg\string >> .data       # Pending → Ready/Faulted (async)
```
- **Initial State:** `Ready` (constant) or `Pending` (async)
- **Push Count:**
  - **Constant:** 0 additional pushes (already final at declaration)
  - **Async:** 1 push when pipeline completes (transitions to Ready/Faulted)
- **Semantics:**
  - `<<` (constant): Immutable, always `Ready`, no further pushes allowed
  - `>>` (async): Starts `Pending`, receives final push from pipeline
- **Use case:** Version numbers, API keys (constant), pipeline outputs (async)

**Operator Summary (Push/Pull Semantics):**

| Declaration | Direction | Purpose | Initial State | Pushes Allowed | Final State |
|-------------|-----------|---------|---------------|----------------|-------------|
| (none) | N/A | Schema-only | `Declared` | **1 push** | `Ready` |
| `<~` / `~>` | `<~` push left, `~>` push right | Default value | `DefaultReady` | **2 pushes** | `Ready` |
| `<<` / `>>` | `<<` push left, `>>` push right | Constant/Async | `Ready` or `Pending` | **0 (const) or 1 (async)** | `Ready` or `Faulted` |

**Key Insight:** `<<` and `>>` are **directional push/pull operators**, not simple assignment (`=`). They define data flow direction and represent final assignment.

---

#### Enumeration vs Serial Fields: The Type Distinction

**Critical Concept:** Within `[#]` enumeration definitions, fields can be one of two types:

1. **Enum Fields** - Declaration only (no type, no value)
2. **Serial Fields** - Type-specified with value

This distinction is fundamental to understanding Polyglot's type system.

---

**1. Enum Fields (Declaration Only)**

**Syntax:** Field declared WITHOUT type, WITHOUT value assignment

```polyglot
[#] Colors
[<] .Red              # Enum field (declaration only)
[<] .Blue             # Enum field (declaration only)
[<] .Green            # Enum field (declaration only)
[X]
```

**Characteristics:**
- No type specified (no `pg\type`)
- No value assigned (no `<<`, `<~`, etc.)
- Pure declaration: defines that this field exists
- Like traditional enums in other languages

---

**2. Serial Fields (Type + Value)**

**Syntax:** Field declared WITH type, WITH value assignment

```polyglot
[#] Config
[<] .timeout: pg\int <~ 30        # Serial field (has type pg\int)
[<] .retries: pg\int <~ 3         # Serial field (has type pg\int)
[<] .version: pg\string << "1.0"  # Serial field (has type pg\string)
[X]
```

**Characteristics:**
- Type specified (`pg\int`, `pg\string`, etc.)
- Value assigned (via `<<`, `<~`, `~>`, or `>>`)
- Hierarchical key-value structure
- Serialized as nested data

---

**3. Polyglot's Unique Feature: Mixing Both in One Definition**

**Rule:** Enum fields and Serial fields **CANNOT be siblings** (same hierarchy level), but **CAN be uncles** (different hierarchy levels).

**❌ INVALID - Siblings (same level):**
```polyglot
[#] Bad
[<] .enumField              # Enum field
[<] .serialField: pg\string # Serial field - ERROR: Can't be siblings!
[X]
```

**✅ VALID - Uncles (different levels):**
```polyglot
[#] Good
[<] .category.option1              # Enum field (nested under .category)
[<] .category.option2              # Enum field (nested under .category)
[<] .metadata: pg\string <~ "info" # Serial field (sibling to .category)
[X]
```

**Another valid example:**
```polyglot
[#] MixedExample
[<] .enumField.constant1           # Enum nested
[<] .enumField.constant2           # Enum nested
[<] .serialField: pg\string        # Serial (uncle to .constant1/.constant2)
[X]
```

**Visual Hierarchy:**
```
MixedExample (root)
├─ enumField (parent node)
│   ├─ constant1 (enum field - child)
│   └─ constant2 (enum field - child)
└─ serialField (serial field - sibling to enumField parent)
```

**Why This Rule Exists:** Enables **exhaustive condition checking** in `[?]` switch blocks. When enum and serial are at different levels, the compiler can verify all enum cases are handled.

---

**4. How to Distinguish Enum vs Serial in Code**

| Feature | Enum Field | Serial Field |
|---------|-----------|--------------|
| Type specified? | ❌ No | ✅ Yes (`pg\type`) |
| Value assigned? | ❌ No | ✅ Yes (`<<`, `<~`, etc.) |
| Purpose | Define constants/options | Store hierarchical data |
| Like traditional... | `enum` in other languages | `struct` or `dictionary` |
| Example | `.Red`, `.Blue` | `.timeout: pg\int <~ 30` |

---

#### Collection Types and Constraints

**Collection Types in Polyglot:**
- `pg\array{T}` - Ordered collection of items
- `pg\set{T}` - Unordered collection of unique items
- `pg\map{K,V}` - Key-value pairs

---

**Critical Constraint: Collections Cannot Contain Collections Directly**

**Rule:** Collection types cannot have another collection as their item type.

**Invalid (Compile Error):**
```polyglot
pg\array{pg\array{pg\int}}           # ❌ Array of arrays - NOT allowed
pg\set{pg\array{pg\string}}          # ❌ Set of arrays - NOT allowed
pg\array{pg\set{pg\int}}             # ❌ Array of sets - NOT allowed
pg\map{pg\string, pg\array{pg\int}} # ❌ Map with array values - NOT allowed
```

**Valid (Allowed):**
```polyglot
pg\array{#UserProfile}               # ✅ Array of enumerations
pg\array{pg\serial}                  # ✅ Array of serial (if needed)
pg\set{#Status}                      # ✅ Set of enumerations
pg\map{pg\string, #Config}           # ✅ Map with enumeration values
```

---

**Why This Constraint Exists:**

1. **Serialization Consistency:** Ensures predictable serialization structure
2. **Type Safety:** Prevents deeply nested type complexity
3. **Performance:** Simplifies runtime type checking
4. **Clarity:** Forces explicit structure via enumerations

---

**Solution: Wrap Collections in Enumerations**

**To create nested collection structures, wrap the inner collection in an enumeration:**

```polyglot
# Define enumeration wrapper for inner collection
[#] Row
[<] .cells: pg\array{pg\int}              # Inner collection wrapped
[X]

# Use outer collection with enumeration
[i] .matrix: pg\array{#Row}               # ✅ Valid: Array of Row enumerations
```

**Benefits of Wrapping:**
- Named structures (`.cells` is more descriptive than `[0]`)
- Additional metadata possible (`.row_id`, `.created_at`, etc.)
- Type-safe access (`.row.cells` vs `[i][j]`)
- Enables recursive structures (trees)

---

**Collection Type Reference:**

**1. `pg\array{T}` - Ordered Array**

**Valid element types:**
- ✅ Primitive types: `pg\int`, `pg\string`, `pg\bool`, `pg\dt`
- ✅ Enumerations: `#UserProfile`, `#Config`
- ✅ Serial: `pg\serial` (if needed)
- ❌ Collections: `pg\array{T}`, `pg\set{T}`, `pg\map{K,V}`

**Example:**
```polyglot
[i] .numbers: pg\array{pg\int}            # ✅ Array of integers
[i] .users: pg\array{#User}               # ✅ Array of User enumerations
[i] .invalid: pg\array{pg\array{pg\int}} # ❌ Compile error
```

---

**2. `pg\set{T}` - Unordered Set (Unique Values)**

**Valid element types:** Same as `pg\array{T}`

**Example:**
```polyglot
[i] .unique_ids: pg\set{pg\string}        # ✅ Set of unique strings
[i] .statuses: pg\set{#Status}            # ✅ Set of Status enumerations
```

**Behavior:**
- Automatically deduplicates values
- No guaranteed order
- Use with `~Y.IntoSet` to collect unique values

---

**3. `pg\map{K,V}` - Key-Value Map**

**Valid key types:**
- ✅ Primitive types that are hashable: `pg\string`, `pg\int`, `pg\uint`
- ❌ Collections: Cannot use collections as keys

**Valid value types:**
- ✅ Primitive types: `pg\int`, `pg\string`, `pg\bool`, `pg\dt`
- ✅ Enumerations: `#Config`, `#User`
- ❌ Collections: Cannot use collections as values directly

**Example:**
```polyglot
[i] .user_ages: pg\map{pg\string, pg\int}     # ✅ Name → Age
[i] .configs: pg\map{pg\string, #Config}      # ✅ Key → Config enum
[i] .invalid: pg\map{pg\string, pg\array{pg\int}}  # ❌ Compile error
```

**Workaround for map with collection values:**
```polyglot
[#] UserData
[<] .scores: pg\array{pg\int}
[X]

[i] .user_data: pg\map{pg\string, #UserData}  # ✅ Map → UserData wrapper
```

---

**Recursive Structures (Trees, Graphs)**

**Enumerations can reference themselves**, enabling recursive structures:

**Example: Binary Tree**
```polyglot
[#] TreeNode
[<] .value: pg\int
[<] .left: #TreeNode                      # ✅ Recursive reference
[<] .right: #TreeNode                     # ✅ Recursive reference
[X]
```

**Example: Graph Node**
```polyglot
[#] GraphNode
[<] .id: pg\string
[<] .neighbors: pg\array{#GraphNode}      # ✅ Array of same type
[X]
```

**Example: Nested Menu**
```polyglot
[#] MenuItem
[<] .label: pg\string
[<] .children: pg\array{#MenuItem}        # ✅ Recursive nested menu
[X]
```

---

#### State Lifecycle Diagram

```
ENUMERATION FIELD DECLARATION
         |
         ↓
    ┌────┴────┬──────────┬──────────┐
    |         |          |          |
 Schema    Default   Constant    Async
  Only      <~ ~>     << >>     Pipeline
    |         |          |          |
    ↓         ↓          ↓          ↓
Declared  DefaultReady Ready    Pending
(no value) (has default)(const)  (waiting)
    |         |          |          |
    |         ↓          |          ↓
    |    [i] block       |      Ready/Faulted
    |   (expected        |          |
    |    Ready)          |          |
    |         |          |          |
    |    ┌────┴────┐     |          |
    |    |         |     |          |
    |    ↓         ↓     |          |
    | Override   Use     |          |
    |    or    Default   |          |
    | Populate           |          |
    |    |      |        |          |
    └────┴──────┴────────┴──────────┘
              |
              ↓
       Ready (immutable)
```

**Extended Lifecycle with Queue States:**
```
Declared ────────┐
                 │
DefaultReady ────┤
                 │
                 ↓
             Pending ──────→ Ready (success)
                 ↓              ↑
                 ↓              │
            Faulted ─→ Retrying ┘
                 │         ↑
                 │         │
            [Retry Logic]  │
                 │         │
                 └─────────┘

Ready ──────→ Cached ──────→ Dirty ──────→ Pending
(result)   (performance) (invalidated)  (refresh)
   │
   ↓
Paused ──────→ [External Trigger] ──────→ Pending
(waiting)      (human approval, etc.)     (resume)
```

#### Reserved Schema: `.*.pgvar.*` (Always Ready)

**Critical Concept:** Every variable in Polyglot has a reserved namespace `.*.pgvar` that provides metadata access.

**Key Property:** ALL fields under `.*.pgvar` are **ALWAYS Ready** (no wait time) because they are database-tracked metadata, not async operations.

---

**Reserved Schema Structure:**

```polyglot
.*                                    # The variable's value itself
.*.pgvar.state: #PgVar.States.*      # Current state (enum)
.*.pgvar.history.{state}.at: pg\dt   # State transition timestamps
```

---

**1. `.*` - Variable Value Access**

Access the variable's actual value:

```polyglot
[<] .result << .user_data.*  # Pull the value of .user_data
```

**Note:** `.*` triggers auto-await if variable is `Pending`.

---

**2. `.*.pgvar.state` - Current State**

**Type:** `#PgVar.States.*` (Reserved Enumeration)

**Purpose:** Query current variable state

**Always Ready:** ✅ No wait time (database-tracked)

**Example:**
```polyglot
[?] .user_data.pgvar.state =? #PgVar.States.Ready
[~][r] |ProcessUser
[~][<] .user << .user_data.*

[?] .user_data.pgvar.state =? #PgVar.States.Faulted
[~][r] |HandleError

[?] *?
[~][r] |U.Log.Warn
[~][<] .msg << "Variable not ready"
```

**State Enumeration:** `#PgVar.States.*` includes:
- `Declared`, `DefaultReady`, `Pending`, `Ready`, `Faulted`
- (Post-MVP: `Retrying`, `Paused`, `Cached`, `Dirty`)

---

**3. `.*.pgvar.history.{state}.at` - State Transition History**

**Type:** `pg\dt` (DateTime timestamp)

**Purpose:** Track when variable entered each state

**Always Ready:** ✅ No wait time (database-tracked)

**Example:**
```polyglot
# When did .user_data become Pending?
[<] .pending_time: pg\dt << .user_data.pgvar.history.Pending.at

# How long in Pending state?
[<] .duration: pg\dt << DT.ToNow"{.user_data.pgvar.history.Pending.at}"

# When did it become Ready?
[<] .ready_time: pg\dt << .user_data.pgvar.history.Ready.at
```

**Structure:**
```
.var.pgvar.history
  ├─ .Declared.at: pg\dt
  ├─ .Pending.at: pg\dt
  ├─ .Ready.at: pg\dt
  └─ .Faulted.at: pg\dt  (if applicable)
```

---

**Why Always Ready?**

The `.*.pgvar` namespace is **metadata tracked in the database**, not async operations. When you access `.var.pgvar.state` or `.var.pgvar.history`, you're querying the variable tracking system, which is **always available** without async wait.

**Contrast:**
- `.var.*` (value) → May be `Pending` (triggers auto-await)
- `.var.pgvar.state` (metadata) → Always `Ready` (no wait)

---

**Error Handling:**

Errors are stored in a separate field:

**`.errors` Field:**
- **Type:** `pg\array{!}` (Array of error objects)
- **Purpose:** Store error details when `Faulted`
- **Availability:** ALL variables
- **Always Ready:** ✅ (metadata)

**Error Object Structure:**
```polyglot
[#] ErrorObject
[<] .type: pg\string              # Error type (e.g., "!pg.Network.Timeout")
[<] .message: pg\string           # Human-readable message
[<] .code: pg\int                 # Error code (optional)
[<] .timestamp: pg\dt             # Timestamp
[<] .context: pg\map{pg\string,pg\string}  # Additional context
[X]
```

**Example:**
```polyglot
[?] .var.pgvar.state =? #PgVar.States.Faulted
[~][r] |U.Log.Error
[~][<] .error_details << .var.errors
```

---

#### DateTime System: `pg\dt` and String Literal Pipelines

**Core Type:** Polyglot uses **ONLY `pg\dt`** for ALL datetime operations (timestamps, durations, differences).

---

**1. `DT.Now` - Current Timestamp**

**Syntax:** `DT.Now` (no arguments)

**Returns:** `pg\dt` (current timestamp)

**Example:**
```polyglot
[r] .start_time: pg\dt << DT.Now  # Capture current time
```

**Note:** Use `[r]` to pull from `DT.Now` pipeline (not `[<]`).

---

**2. `DT.ToNow"{.var}"` - Time Difference**

**Syntax:** `DT.ToNow"{.datetime_var}"` (string literal with variable reference)

**Returns:** `pg\dt` (duration between `.datetime_var` and now)

**String Literal = Inline Pipeline:**
```polyglot
DT.ToNow"{.started}"

# Equivalent to:
[<] .args: pg\serial << {.started: pg\dt}  # ← PULLS from .started (auto-await)
[r] |DT.ToNow
```

**Key:** String literals `"{.var}"` create inline pipelines that **pull from variables**, triggering auto-await if `Pending`.

**Example:**
```polyglot
[r] .started: pg\dt << DT.Now

# Later...
[<] .elapsed: pg\dt << DT.ToNow"{.started}"  # Time since .started
```

---

**3. Duration Literals**

**Syntax:** `DT.Minutes"3"`, `DT.Seconds"30"`, `DT.Hours"2"`

**Returns:** `pg\dt` (duration value)

**Examples:**
```polyglot
[<] .timeout: pg\dt << DT.Minutes"5"   # 5 minute duration
[<] .delay: pg\dt << DT.Seconds"30"    # 30 second duration
[<] .window: pg\dt << DT.Hours"2"      # 2 hour duration
```

**Note:** String literal syntax (`"3"` not `(3)`), not function calls.

---

**4. DateTime Comparisons**

**Operators:** `=?` (equal), `>?` (greater), `<?` (less), `>=?` (greater or equal), `=<?` (less or equal)

**Example:**
```polyglot
[?] DT.ToNow"{.started}" >? DT.Minutes"5"
[~][r] |U.Log.Warn
[~][<] .msg << "Operation took longer than 5 minutes"
```

---

**5. Timeout Pattern Using Switch Blocks**

**Idiomatic way to handle timeouts in Polyglot:**

```polyglot
[r] .timeout_start: pg\dt << DT.Now  # Start timeout timer

# Switch block waits until ONE branch becomes true:
[?] DT.ToNow"{.timeout_start}" >? DT.Minutes"3"  # Timeout branch
[~][r] |U.Log.Error
[~][<] .msg << "Timeout after 3 minutes"
[~][r] |HandleTimeout

[?] DT.ToNow"{.timeout_start}" =<? DT.Minutes"3"  # Within timeout
[&] .var.pgvar.state =? #PgVar.States.Ready       # AND variable Ready
[~][<] .result << .var.*                          # Process result
[~][r] |ProcessResult
```

**How it works:**
1. `[?]` switch block continuously re-evaluates branches
2. First branch: Check if 3 minutes elapsed
3. Second branch with `[&]` AND: Check if within timeout AND variable Ready
4. Whichever branch becomes true first executes
5. `[&]` combines multiple conditions (all must be true)

**Key:** Switch blocks provide built-in timeout mechanism without explicit `await` or sleep.

---

#### String Literal Processing Architecture

**CRITICAL CONCEPT:** String literals in Polyglot are NOT primitive values - they are **inline pipeline calls**.

---

##### Overview

Every string literal in Polyglot syntax is actually syntactic sugar for a pipeline call. This architectural decision enables:
1. **Type flexibility** - String literal syntax can return ANY type (not just strings)
2. **Unified formatting** - All value-to-string conversions go through pipelines
3. **Extensibility** - Developers can define custom format pipelines
4. **Consistency** - No "magic" string formatting - everything is explicit pipelines

---

##### Syntax Forms

**1. Plain String Literal**
```polyglot
"hello world"
```
**Desugars to:**
```polyglot
U.String"hello world"
```
**Pipeline called:** `|U.String`
**Returns:** `pg\string`

---

**2. Explicit Pipeline String Literal**
```polyglot
DT.Now""
```
**Pipeline called:** `|DT.Now`
**Parameter:** Empty string `""`
**Returns:** `pg\dt` (NOT a string!)

---

**3. Parameterized String Literal**
```polyglot
DT.Minutes"5"
```
**Pipeline called:** `|DT.Minutes`
**Parameter:** `"5"` (passed as `.formatted_argument_string`)
**Returns:** `pg\dt` (duration)

---

**4. Interpolated String Literal**
```polyglot
"Count: {.count:Hex}"
```
**Processing (5 steps):**
1. Extract `{.count:Hex}` placeholder
2. Infer type of `.count` → `pg\int`
3. Call format pipeline: `|U.String.Polyglot.Int.Hex`
4. Get result: `"FF"` (if `.count` is 255)
5. Substitute: `"Count: FF"`
6. Call `|U.String` with final string

---

##### Pipeline Signature Requirements

**MANDATORY for all string literal pipelines:**

```polyglot
[|] PipelineName
[i] .formatted_argument_string: pg\string    # MANDATORY input name
[t] |T.String.Call                           # MANDATORY trigger type
[W] RT.SomeWrapper"..." or |W.Polyglot.Scope # MANDATORY wrapper
[o] .result: AnyType                         # Can be ANY type!
[X]
```

**Key constraints:**
- Input MUST be named `.formatted_argument_string`
- Trigger MUST be `|T.String.Call`
- Single output (can be any type, not limited to strings)
- Standard pipeline structure with `[i]`, `[t]`, `[W]`, `[o]`

---

##### Example: DateTime Pipeline Definition

```polyglot
[|] DT.Now
[i] .formatted_argument_string: pg\string  # Required (even though unused)
[t] |T.String.Call                         # Required trigger
[W] RT.Rust"chrono::Utc::now"             # Rust wrapper for datetime
[o] .timestamp: pg\dt                      # Returns pg\dt, not string!
[X]
```

**Usage:**
```polyglot
[r] .now: pg\dt << DT.Now""   # Empty string required
```

---

##### Format Pipeline Resolution Algorithm

**Given:** `"{.variable:FormatIdentifier}"`

**Step 1: Type Inference**
```
Infer type of .variable → T (e.g., pg\int)
```

**Step 2: Language Context**
```
Determine language context → L (default: Polyglot)
```

**Step 3: Pipeline Name Construction**
```
Construct pipeline name: |U.String.{L}.{T}.{FormatIdentifier}
Example: |U.String.Polyglot.Int.Hex
```

**Step 4: Pipeline Lookup**
```
Check if pipeline exists in registry
- Found: Call pipeline with variable value
- Not found: Compile error with suggestion
```

**Step 5: Result Substitution**
```
Replace {.variable:FormatIdentifier} with pipeline output
```

---

##### Processing Workflow (Interpolated Strings)

**Input:** `"Total: {.price:Currency}, Items: {.count}"`

**Step 1: Extract Placeholders**
```rust
placeholders = [
    {variable: ".price", format: Some("Currency")},
    {variable: ".count", format: None}
]
```

**Step 2: Infer Types**
```rust
.price → pg\float
.count → pg\int
```

**Step 3: Construct Format Pipeline Names**
```rust
.price → |U.String.Polyglot.Float.Currency
.count → |U.String.Polyglot.Int.Default  # No format specified
```

**Step 4: Pack into Serial Array**
```rust
pg\array{pg\serial} = [
    {variable: .price, format_pipeline: |U.String.Polyglot.Float.Currency},
    {variable: .count, format_pipeline: |U.String.Polyglot.Int.Default}
]
```

**Step 5: Call Format Pipelines (Parallel)**
```rust
results = [
    call |U.String.Polyglot.Float.Currency with .price → "$1,234.56"
    call |U.String.Polyglot.Int.Default with .count → "42"
]
```

**Step 6: Substitute Back**
```rust
"Total: {.price:Currency}, Items: {.count}"
→ "Total: $1,234.56, Items: 42"
```

**Step 7: Pass to Target Pipeline**
```rust
Call |U.String with .formatted_argument_string = "Total: $1,234.56, Items: 42"
Returns: pg\string
```

---

##### Auto-Await Triggers

**CRITICAL:** When interpolation accesses a variable (`{.var}`), it triggers **auto-await** on that variable.

```polyglot
[r] |FetchPrice          # Async operation
[>] .price >> .result    # .result is Pending

[r] .msg: pg\string << "Price: {.result:Currency}"
                       # ↑ Auto-await triggered here!
                       # Runtime blocks until .result is Ready/Faulted
```

**Why this matters:**
- Interpolation PULLS from variables
- Pulling triggers auto-await (if variable is Pending)
- No explicit `await` needed - happens automatically
- Consistent with Polyglot's async-centric model

---

##### Type Mismatch Error Handling

**Scenario:** Developer uses incorrect format for type

```polyglot
[r] .name: pg\string << "Alice"
[r] .msg: pg\string << "{.name:Hex}"  # ERROR!
```

**Compiler behavior:**

**Step 1: Infer type**
```
.name → pg\string
```

**Step 2: Construct pipeline name**
```
|U.String.Polyglot.String.Hex
```

**Step 3: Lookup fails**
```
Error: Pipeline not found: |U.String.Polyglot.String.Hex
Note: Format identifier 'Hex' expects type 'pg\int', but variable '.name' is 'pg\string'
Suggestion: Available formats for 'pg\string': [ToUpper, ToLower, Trim, ...]
```

**Why this works elegantly:**
- No special type checking needed
- Format pipelines are namespaced by type
- Missing pipeline = compile error
- Natural error messages

---

##### Bootstrap Formatters (Base Case)

**Problem:** Format pipelines need to format values, but formatting requires pipelines - circular dependency?

**Solution:** Base case formatters implemented directly in Rust (not Polyglot)

**Base formatters (Rust implementations):**
```
|U.String.Polyglot.Int.Default       → Rust: value.to_string()
|U.String.Polyglot.Float.Default     → Rust: format!("{}", value)
|U.String.Polyglot.Bool.Default      → Rust: if value {"true"} else {"false"}
|U.String.Polyglot.DateTime.Default  → Rust: value.to_rfc3339()
|U.String.Polyglot.Path.Default      → Rust: path.to_string_lossy()
```

**Higher-level formatters (can be Polyglot):**
```polyglot
[|] U.String.Polyglot.Int.Hex
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope
[r] .value: pg\int << .formatted_argument_string
[r] .result: pg\string << // Rust code: format!("{:X}", value)
[o] .result: pg\string
[X]
```

**Hierarchy:**
1. Base formatters: Rust implementations (no dependencies)
2. Standard formatters: Polyglot pipelines calling base formatters
3. Custom formatters: User-defined Polyglot pipelines

**No circular dependency:** Base formatters don't use string literals internally (use Rust directly).

---

##### Language-Specific Formatters

**Pattern:** `|U.String.{language}.{type}.{format}`

**Languages supported:**
- `Polyglot` - Language-agnostic (default)
- `Python` - Python-specific formatting (future)
- `Rust` - Rust-specific formatting (future)
- `JavaScript` - JS-specific formatting (future)

**Example use case:**
```polyglot
[r] .timestamp: pg\dt << DT.Now""

# Polyglot format (ISO8601)
[r] .msg1: pg\string << "{.timestamp:ISO8601}"  # "2024-01-15T14:30:00Z"

# Python format (future - uses Python's strftime)
[r] .msg2: pg\string << "{.timestamp:Python.Strftime}"  # Locale-aware
```

---

##### Custom Format Pipeline Example

**Define custom format:**
```polyglot
[|] U.String.Polyglot.Int.PhoneUS
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope

[r] .number: pg\int << .formatted_argument_string

# Extract parts (555-123-4567)
[r] .area: pg\int << (.number / 10000000)
[r] .exchange: pg\int << ((.number / 10000) % 1000)
[r] .line: pg\int << (.number % 10000)

[r] .result: pg\string << "({.area}) {.exchange}-{.line}"

[o] .result: pg\string
[X]
```

**Usage:**
```polyglot
[r] .phone: pg\int << 5551234567
[r] .msg: pg\string << "Call: {.phone:PhoneUS}"
// Result: "Call: (555) 123-4567"
```

---

##### Compiler Implementation Requirements

**Lexer:**
1. Recognize `Pipeline.Name"..."` as inline pipeline call
2. Tokenize string content (extract interpolations)
3. Parse `{.variable:format}` patterns
4. Handle escape sequences

**Parser:**
1. Build AST for string literal nodes
2. Extract placeholder expressions
3. Validate format identifier syntax
4. Generate pipeline call nodes

**Type Checker:**
1. Infer types of interpolated variables
2. Construct format pipeline names
3. Verify pipelines exist in registry
4. Check output types match expected types

**Code Generator:**
1. Generate format pipeline calls
2. Generate string substitution logic
3. Generate final pipeline call with `.formatted_argument_string`
4. Insert auto-await for interpolated variables

---

##### Runtime Implementation Requirements

**String Literal Handler:**
1. Receive string with placeholders
2. Extract placeholder metadata
3. Call format pipelines (can parallelize)
4. Wait for all results (auto-await)
5. Substitute results into string
6. Return formatted string

**Format Pipeline Registry:**
1. Register all available format pipelines
2. Support dynamic lookup by name
3. Cache pipeline handles for performance
4. Provide error messages for missing formatters

---

##### Performance Considerations

**Optimization strategies (future):**
1. **Compile-time formatting** - If all variables are constants, format at compile time
2. **Format pipeline caching** - Cache pipeline handles to avoid repeated lookups
3. **Lazy formatting** - Only format when string is actually used
4. **String pooling** - Reuse identical formatted strings
5. **Parallel formatting** - Call multiple format pipelines concurrently

**Initial implementation:**
- **Correctness first** - Make it work, then optimize
- **Naive execution** - Sequential formatting, no caching
- **Profiling-driven** - Measure before optimizing

**Quote from user:**
> "will start using pipelines then will think of way to optimize it. Make it work first optimize later."

---

##### Why This Architecture?

**1. Consistency**
- No "magic" formatting - everything is explicit pipelines
- Same mechanism for all types
- Predictable behavior

**2. Extensibility**
- Developers can define custom formatters
- No language changes needed for new formats
- Community can share formatter libraries

**3. Type Safety**
- Format pipelines namespaced by type
- Type mismatches = compile errors
- Clear error messages

**4. Flexibility**
- String literal syntax can return ANY type
- Not limited to string outputs
- Unified mechanism for literals

**5. Separation of Concerns**
- Lexer: Tokenize syntax
- Parser: Build AST
- Compiler: Resolve types and pipelines
- Runtime: Execute formatting

---

##### Edge Cases

**1. Nested Interpolation**
```polyglot
"{.name} at {DT.Now\"\"}"
```
**Status:** To be determined (likely disallowed or requires explicit nesting)

**2. Escape Sequences**
```polyglot
"Literal \{.var\} not interpolated"
```
**Status:** Standard escape rules apply (`\{` = literal brace)

**3. Empty Format Identifier**
```polyglot
"{.var:}"  # Format identifier empty
```
**Status:** Compile error (format identifier required if `:` present)

**4. Multiple Formats**
```polyglot
"{.var:Hex,Padded}"  # Two formats?
```
**Status:** Not supported (single format per placeholder)

---

##### Related Documentation

- **Canonical Reference:** [String Literals Internals](string-literals-internals.md)
- **Format Catalog:** [Standard Library - Format Identifiers](../user/standard-library/utilities-catalog.md#format-identifier-pipelines)
- **Type System:** [Type System](../user/language/type-system.md)
- **Pipeline Definitions:** [Pipeline Syntax](../user/language/syntax-complete.md)

---

#### Valid State Transitions

| From State | To State(s) | Trigger | Notes |
|------------|-------------|---------|-------|
| `Declared` | `Pending` | Pipeline assignment | Field populated via `>>` |
| `Declared` | `Ready` | Direct assignment | Explicit value provided |
| `DefaultReady` | `Pending` | Override with async | Override with pipeline result |
| `DefaultReady` | `Ready` | First use or override | Default used or overridden |
| `Pending` | `Ready` | Pipeline success | Value fulfilled |
| `Pending` | `Faulted` | Pipeline failure | Error occurred |
| `Pending` | `Retrying` | Transient failure | Auto-retry triggered |
| `Faulted` | `Retrying` | Retry attempt | Manual or auto retry |
| `Retrying` | `Ready` | Retry success | Operation succeeded |
| `Retrying` | `Faulted` | Retry exhausted | All retries failed |
| `Ready` | `Cached` | Cache enabled | Result cached |
| `Cached` | `Dirty` | Invalidation event | Cache invalidated |
| `Dirty` | `Pending` | Refresh triggered | Re-fetch data |
| `Pending` | `Paused` | External trigger needed | Wait for approval |
| `Paused` | `Pending` | Trigger received | Resume execution |

**Forbidden Transitions (compiler/runtime must prevent):**
- `Ready` → `Declared` (cannot "un-ready" a variable)
- `Ready` → `Pending` (cannot make Ready async again)
- `Faulted` → `Ready` (must go through `Retrying`)
- `DefaultReady` → `Declared` (cannot remove default)
- Any state → `DefaultReady` (DefaultReady is initial state only)

#### Runtime Semantics: Automatic Waiting

**Core Behavior:** Automatic waiting occurs when **pulling from** a variable (via `<<`, `>>`, `<~`, `~>`, comparison operators `=?`, `>?`, etc., or `[o]` output blocks). No explicit `await` keyword.

**Algorithm (When Pulling From Variable V):**
```
When operation pulls from variable V:
  IF V.state == Pending:
    Block until V.state ∈ {Ready, Faulted}
    Use non-busy wait (yield CPU, condition variable)
    Support timeout and cancellation
  ELSE IF V.state == Declared:
    Throw CompileError: "Variable must be Ready when pulled from"
  ELSE IF V.state ∈ {Ready, DefaultReady, Cached}:
    Proceed (return value)
  ELSE IF V.state == Faulted:
    Propagate error to error handler or abort pipeline
```

**Triggers for Auto-Await (Pulling From Variable):**
1. **Assignment operators:** `<< .var`, `>> .var`, `<~ .var`, `~> .var`
2. **Comparison operators:** `[?] .var =? value`, `[?] .var >? threshold`
3. **Output blocks:** `[o] .result >> .var`
4. **Any value access:** Reading variable's value requires it to be Ready

**`[i]` Block Entry Semantics:**

All `[i]` (input) variables MUST be `Ready` (or `DefaultReady`) before pipeline triggers.

```
On pipeline trigger:
  FOR EACH variable V in [i] block:
    IF V.state == DefaultReady:
      Apply default value
      Transition V.state to Ready
    ELSE IF V.state == Declared:
      Throw RuntimeError: "[i] variable not ready"
    ELSE IF V.state == Pending:
      Wait until V.state ∈ {Ready, Faulted}
    ELSE IF V.state == Faulted:
      Invoke error handler

  IF ALL [i] variables Ready:
    Execute pipeline body
  ELSE:
    Abort pipeline
```

**DefaultReady Push Count Semantics:**

`DefaultReady` fields allow **2 pushes** with `<<` or `>>` operators before becoming immutable `Ready`.

```
Enumeration field F with default D (declared with <~ or ~>):

Scenario A: No override (default used)
  F.value = D
  F.state = DefaultReady
  push_count = 0

  On first pull (value accessed):
    F.state = Ready
    F.value = D (immutable)
    push_count = 0 (no pushes, just default applied)

Scenario B: Override with pushes
  1st push: << new_value
    F.value = new_value
    F.push_count = 1
    F.state = DefaultReady (still allows one more push)

  2nd push: << final_value
    F.value = final_value
    F.push_count = 2
    F.state = Ready (now immutable)

  3rd push attempt:
    Throw RuntimeError: "Cannot push to field (already had 2 pushes)"
```

**Concurrency Safety:**
- Push count: atomic increment
- State transition: atomic with value update
- Race conditions prevented via locking

#### IR Representation of Variable States

Variables in IR include state metadata:

**Conceptual Variable Structure (informative):**
```
Variable {
  state: enum Variables.States     // Current state (4 bytes)
  push_count: u8                    // Track push count (1 byte)
  value: SerializedData             // Actual value (variable size)
  errors: Array<ErrorObject>        // Error details if Faulted (variable size)
  metadata: TypeInfo                // Type information (variable size)
}
```

**Note:** `push_count` tracks how many times `<<` or `>>` operators have pushed to this field. Max: 1 for schema-only, 2 for defaults, 0 for constants.

**IR JSON Representation (Runner IR example):**
```json
{
  "runner_ir": {
    "execution_mode": "sequential",
    "steps": [...],
    "variables": {
      ".user_data": {
        "type": "#UserProfile",
        "state": "Pending",
        "push_count": 0,
        "errors": [],
        "assignment_operator": ">+",
        "source": "pipeline_output",
        "max_pushes": 1
      },
      ".config": {
        "type": "#Config",
        "state": "DefaultReady",
        "push_count": 0,
        "errors": [],
        "default_values": {
          ".timeout": 30,
          ".retries": 3
        },
        "assignment_operator": "<~",
        "max_pushes": 2
      }
    }
  }
}
```

**Note:** `max_pushes` indicates how many pushes allowed based on declaration type (1 for schema-only, 2 for defaults, 0 for constants).

**State Metadata in JSONB:**
- Stored in `trigger_ir`, `queue_ir`, `runner_ir` columns as JSONB
- Each IR tracks state information relevant to its phase
- State transitions logged to `execution_logs` table

#### Compiler Requirements

**Type Checking:**
- Enforce type safety for all operators
- Validate default value matches field type
- Check state comparisons use correct enumerations (`#Variables.States.*`)

**State Analysis:**
- Track state flow through pipeline graph
- Detect unreachable code after `Faulted` checks
- Warn on unused `.errors` fields

**Optimization:**
- Eliminate redundant state checks
- Inline constant assignments (`<<`)
- Optimize away `DefaultReady` → `Ready` transitions when no override

**Error Detection:**
- Detect second override attempts on `DefaultReady` fields (compile-time if possible)
- Warn when `Declared` fields reach `[i]` blocks without population
- Flag invalid state transitions

#### Runtime Requirements

**State Management:**
- Atomic state transitions (use mutexes or atomic operations)
- Thread-safe state reads
- Optional: State history for debugging

**Waiting Mechanism:**
- Non-busy wait for `Pending` variables (condition variables)
- Timeout handling (configurable per pipeline)
- Cancellation support (abort waiting pipeline)
- Deadlock detection (optional, post-MVP)

**Memory Management:**
- Free `Pending` variable resources on transition to `Ready`/`Faulted`
- Clean up error objects after handling
- Cache eviction for `Cached` state (LRU policy)

**Observability:**
- State transition logging in debug mode
- Metrics: state distribution, wait times, cache hit rates
- Tracing: variable lifecycle visualization (future)

#### Performance Considerations

**Goal:** Variable state operations must be fast enough to not impact pipeline execution (NFR-P1, NFR-P2).

**State Check Performance:**
- **Target:** State lookup < 1μs (nanosecond-level for hot path)
- **Strategy:**
  - Use fast-path for common case (Ready state)
  - Inline state comparisons where possible
  - Branch prediction hints for error paths (Faulted is rare)
- **Benchmark:** Measure state checks per second in tight loop

**Waiting Mechanism Performance:**
- **Target:** Context switch overhead < 100μs
- **Strategy:**
  - Use futex/condition variables (not busy-waiting)
  - Batch wake-ups for multiple waiters on same variable
  - Lock-free state reads (only writes need mutex)
- **Benchmark:** Measure wait/wake latency with concurrent pipelines

**State Transition Performance:**
- **Target:** Transition time < 10μs (excluding async operation time)
- **Strategy:**
  - Atomic state updates (compare-and-swap)
  - Minimize lock hold time
  - Defer audit logging to background thread
- **Benchmark:** Measure transitions per second under load

**Memory Footprint:**
- **Target:** Variable metadata < 64 bytes overhead per variable
- **Layout:**
  ```
  Variable {
    state: u8           (1 byte)
    override_count: u8  (1 byte)
    padding: u16        (2 bytes)
    value_ptr: *mut     (8 bytes)
    errors_ptr: *mut    (8 bytes)
    metadata_ptr: *mut  (8 bytes)
    -- Total: ~32 bytes + value size
  }
  ```
- **Strategy:**
  - Pack state and override_count into single cache line
  - Use heap allocation for errors (rare case)
  - Share type metadata across instances

**Cache Performance (Post-MVP):**
- **Target:** Cache hit rate > 80% for repeated queries
- **Strategy:**
  - LRU eviction policy
  - Configurable TTL (default: 5 minutes)
  - Memory limits (default: 256MB per pipeline)
- **Benchmark:** Monitor cache hit rate, eviction frequency

**NFR Alignment:**
- **NFR-P1:** Compilation < 1s for 1000-line files (state model doesn't impact compilation speed)
- **NFR-P2:** Trigger to execution < 2s (state checks must be sub-millisecond)
- **NFR-P3:** Type conversion < 10ms for <1MB data (state transitions add negligible overhead)
- **NFR-P4:** Queue throughput 100+ instances/second (state management must not bottleneck)
- **NFR-P5:** Database queries < 100ms (state transition audit logs must be async)

**Optimization Checklist:**
- [ ] Profile state check hot paths
- [ ] Minimize allocations in state transitions
- [ ] Use atomic operations where possible
- [ ] Defer expensive operations (logging) to background threads
- [ ] Monitor lock contention on state mutations
- [ ] Benchmark wait/wake latency under load

#### Database Persistence Strategy

**Decision:** Store variable states in IR JSONB columns (Option A)

**Rationale:**
- Simpler schema (no new tables)
- Variable state is part of IR definition
- Can query using PostgreSQL JSONB operators if needed

**Alternative (Future):** If querying variable states across pipelines becomes critical, migrate to separate `variable_states` table:
```sql
CREATE TABLE variable_states (
    id UUID PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id),
    variable_name TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('Declared', 'DefaultReady', 'Pending', 'Ready', 'Faulted', 'Retrying', 'Paused', 'Cached', 'Dirty')),
    value JSONB,
    errors JSONB,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Implementation Patterns for AI Agents

**State Transition Pattern (Rust):**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableState {
    Declared,
    DefaultReady,
    Pending,
    Ready,
    Faulted,
    Retrying,
    Paused,
    Cached,
    Dirty,
}

pub struct Variable {
    pub state: VariableState,
    pub override_count: u8,
    pub value: Option<serde_json::Value>,
    pub errors: Vec<ErrorObject>,
}

impl Variable {
    pub async fn transition_to_ready(&mut self, value: serde_json::Value) -> Result<()> {
        match self.state {
            VariableState::Pending | VariableState::Retrying => {
                self.state = VariableState::Ready;
                self.value = Some(value);
                Ok(())
            }
            VariableState::Declared => {
                Err(RuntimeError::InvalidTransition {
                    from: "Declared",
                    to: "Ready",
                    reason: "Must go through Pending".to_string(),
                })
            }
            VariableState::Ready => {
                Err(RuntimeError::AlreadyReady)
            }
            _ => Err(RuntimeError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Ready".to_string(),
                reason: "Invalid transition".to_string(),
            }),
        }
    }
}
```

**Automatic Waiting Pattern (Rust):**
```rust
use tokio::sync::Notify;
use std::sync::Arc;

pub struct VariableWaiter {
    notify: Arc<Notify>,
}

impl VariableWaiter {
    pub async fn wait_for_ready(&self, var: &Variable, timeout_ms: u64) -> Result<()> {
        if matches!(var.state, VariableState::Ready | VariableState::Cached) {
            return Ok(());
        }

        if matches!(var.state, VariableState::Declared) {
            return Err(CompileError::VariableNotReady {
                var_name: var.name.clone(),
            });
        }

        // Wait with timeout
        tokio::select! {
            _ = self.notify.notified() => {
                if matches!(var.state, VariableState::Ready) {
                    Ok(())
                } else if matches!(var.state, VariableState::Faulted) {
                    Err(RuntimeError::VariableFaulted {
                        var_name: var.name.clone(),
                        errors: var.errors.clone(),
                    })
                } else {
                    Err(RuntimeError::UnexpectedState {
                        expected: "Ready or Faulted",
                        actual: format!("{:?}", var.state),
                    })
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(timeout_ms)) => {
                Err(RuntimeError::WaitTimeout {
                    var_name: var.name.clone(),
                    timeout_ms,
                })
            }
        }
    }
}
```

#### Edge Cases

**1. Nested Field States:**
- Nested fields (`.user.address.city`) have independent `.state` introspection
- Accessing `.user.address.state` is valid
- All serialized fields have state metadata

**2. Faulted Variable Passed to Pipeline:**
- Pipeline waits for `Ready`/`Faulted`, then propagates error
- If unhandled, downstream pipeline aborts
- Error blocks `[!]` can catch specific error types

**3. Concurrent State Access:**
- State reads: atomic, lock-free
- State writes: mutex-protected
- State transitions: atomic compare-and-swap or mutex
- Waiting: condition variable or similar

**4. Second Override Attempt:**
- Each instance tracks its own override count
- Second override on SAME INSTANCE is runtime error
- Different instances are independent

#### Future Innovations

**Status:** Exploratory concepts for post-MVP consideration

**1. Partial/Streaming State**

**Concept:** Variable that receives data incrementally as it becomes available.

**Motivation:**
- Large dataset processing (logs, file downloads)
- Streaming data pipelines
- Progressive rendering/display

**Design Challenges:**
- How does streaming fit with "all variables are serialized strings"?
- Is partial data queryable mid-stream?
- What guarantees exist about data completeness?
- How to handle partial failures?

**Possible Approaches:**

**Option A: Streaming as Iterator Pattern**
- Variable doesn't hold full data, holds iterator/cursor
- Each iteration yields Ready chunk
- Terminal state when iterator exhausted
```polyglot
[r] |StreamLogs
[>] .logs: pg\stream{pg\string} >> .log_stream  # Hypothetical stream type

# Consumer iterates:
[r] |ProcessLogChunk
[<] .chunk: pg\string << .log_stream.next()
```

**Option B: Partial State with Progress Metadata**
- New state: `Partial` (between Pending and Ready)
- Variable holds accumulated data + progress indicator
- Transition: Declared → Pending → Partial (N%) → Ready (100%)
```polyglot
[?] .download.state =? #Variables.States.Partial
[~][r] |U.Log.Info
[~][<] .msg << "Download {.download.progress}% complete"
```

**Option C: No Special State (Defer to Application)**
- Use Ready state with array that grows over time
- Application-level chunking via pipeline chaining
- No language-level streaming support

**Recommendation:** Further research needed. Streaming conflicts with serialization model. Consider Option C (application-level) for MVP, Option A (iterator) for future if demand exists.

**2. Reactive State System**
- Automatic re-execution when dependencies change
- Dirty state triggers downstream pipeline refresh
- DAG-based dependency tracking

**3. Distributed State Coordination**
- Variables that span multiple machines
- Pipeline states synchronized across cluster
- Distributed caching and retry logic

#### References

- **Technical Specification:** `docs/technical/variable-states-specification.md` (v1.0.0)
- **User Guide:** `docs/user/language/variables-user-guide.md`
- **Brainstorming Session:** `docs/brainstorming-session-results-2025-11-23.md`
- **Reserved Enumeration:** `#Variables.States.*` (defined in next section)

---

### Reserved Enumerations

**Status:** Core language feature (v0.0.2)

#### Overview

Reserved Enumerations are **pre-compiled, immutable constants** defined by the Polyglot language itself. Unlike user-defined enumerations, Reserved Enumerations:
- Cannot be altered or redefined
- Are available globally without imports
- Are part of the serialization tree
- Exist at compile time (not runtime-created)

They serve as the foundation for Polyglot's introspection capabilities and standard library.

#### Purpose

Reserved Enumerations provide:
1. **Language Introspection:** Query variable/pipeline states at runtime
2. **Type-Safe Constants:** Enum-based comparisons prevent typo bugs
3. **Standard Library Foundation:** Pre-defined types for common operations
4. **Cross-Language Consistency:** Same enumerations across all runtime wrappers

#### Core Reserved Enumerations

**1. `#Variables.States.*` - Variable State Model**

Defines all 9 variable states as immutable constants:

```polyglot
[#] Variables.States
[<] .Declared: pg\string << "Declared"
[<] .DefaultReady: pg\string << "DefaultReady"
[<] .Pending: pg\string << "Pending"
[<] .Ready: pg\string << "Ready"
[<] .Faulted: pg\string << "Faulted"
[<] .Retrying: pg\string << "Retrying"
[<] .Paused: pg\string << "Paused"
[<] .Cached: pg\string << "Cached"
[<] .Dirty: pg\string << "Dirty"
[X]
```

**Usage:**
```polyglot
[?] .var.state =? #Variables.States.Ready
[~][r] |ProcessData

[?] .var.state =? #Variables.States.Faulted
[~][r] |HandleError
```

**Implementation:** Compiler ensures these are available without declaration.

---

**2. `#Pipelines.States.*` - Pipeline Execution States**

**Status:** Epic 2 Future Work - Will be finalized during IR Generation implementation

Defines 10 pipeline execution states (draft):

```polyglot
[#] Pipelines.States
[<] .Registered: pg\string << "Registered"       # Pipeline defined, not triggered
[<] .Awaiting: pg\string << "Awaiting"           # Waiting for trigger condition
[<] .Triggered: pg\string << "Triggered"         # Trigger fired, ready for dispatch
[<] .DispatchQueue: pg\string << "DispatchQueue" # Queued for execution
[<] .Executing: pg\string << "Executing"         # Currently running
[<] .Paused: pg\string << "Paused"               # Execution paused (queue control)
[<] .Cached: pg\string << "Cached"               # Result cached for reuse
[<] .Retry: pg\string << "Retry"                 # Retry attempt in progress
[<] .Failed: pg\string << "Failed"               # Execution failed
[<] .Completed: pg\string << "Completed"         # Successfully finished
[X]
```

**Usage (future):**
```polyglot
[?] |SomePipeline.state =? #Pipelines.States.Executing
[~][r] |U.Log.Info
[~][<] .msg: pg\string << "Pipeline is running"
```

**Note:** Pipeline state introspection syntax is not yet finalized. This enumeration will be fully specified and validated during Epic 2 (IR Generation). The 10-state model shown here is a draft from brainstorming session 2025-11-23 and may be refined based on implementation requirements.

---

**3. `#Boolean` - True/False Constants**

**Status:** Core language feature

Standard boolean enumeration with aliases:

```polyglot
[#] Boolean
[<] .True: pg\string << "True"    # Alias: #True
[<] .False: pg\string << "False"  # Alias: #False
[X]
```

**Usage:**
```polyglot
[#] Config
[<] .debug_mode: #Boolean <~ #False
[<] .verbose: #Boolean <~ #True
[X]

[?] .config.debug_mode =? #True
[~][r] |EnableDebugLogging
```

**Aliases:** `#True` and `#False` are global aliases for `#Boolean.True` and `#Boolean.False`.

---

**4. Template Pattern Reserved Enumerations**

Some Reserved Enumerations follow a **template pattern** for systematic organization.

**Example:** `DT.Business.Week.*` (Datetime business week constants)

```polyglot
[#] DT.Business.Week
[<] .Monday: pg\int << 1
[<] .Tuesday: pg\int << 2
[<] .Wednesday: pg\int << 3
[<] .Thursday: pg\int << 4
[<] .Friday: pg\int << 5
[X]
```

**Pattern Rationale:**
- Namespaced organization (`DT` for datetime)
- Hierarchical structure (`Business.Week` for business week days)
- Consistent naming across related constants

**Other Template Patterns (future):**
- `DT.Month.*` - Month constants (1-12)
- `DT.Quarter.*` - Quarter constants (Q1-Q4)
- `HTTP.Status.*` - HTTP status codes (200, 404, 500, etc.)
- `Encoding.*` - Character encodings (UTF8, ASCII, etc.)

---

#### Reserved vs User-Defined Enumerations

| Aspect | Reserved Enumerations | User-Defined Enumerations |
|--------|----------------------|---------------------------|
| **Mutability** | Immutable (compiler-enforced) | Immutable once instantiated |
| **Availability** | Global, no imports needed | Must be defined in code |
| **Definition Time** | Pre-compile time (language-provided) | Compile time (user-provided) |
| **Namespace** | Special syntax (`#Variables.States.*`, `#Boolean`, etc.) | User-defined (`#MyEnum.*`) |
| **Purpose** | Language introspection, standard library | Application-specific data structures |
| **Override** | Cannot be redefined | Cannot conflict with reserved names |

**Critical Rule:** User-defined enumerations **cannot use reserved names**. Compiler will reject:
```polyglot
[#] Variables.States  # ERROR: Cannot redefine reserved enumeration
[<] .Custom: pg\string << "Custom"
[X]
```

---

#### Compiler Implementation

**Reserved Enumeration Handling:**

1. **Pre-Compilation Phase:**
   - Load all reserved enumerations into symbol table
   - Mark as immutable and globally available
   - Register reserved names (prevent user redefinition)

2. **Type Checking Phase:**
   - Validate comparisons use correct enum types
   - Ensure reserved fields (`.state`, `.errors`) reference correct enumerations
   - Reject attempts to redefine reserved names

3. **Code Generation Phase:**
   - Reserved enum access is constant folding (no runtime lookup)
   - Direct value substitution in IR

**Example Compiler Pattern (Rust):**
```rust
pub struct ReservedEnumerations {
    enums: HashMap<String, Enumeration>,
}

impl ReservedEnumerations {
    pub fn new() -> Self {
        let mut enums = HashMap::new();

        // Variables.States
        enums.insert(
            "Variables.States".to_string(),
            Enumeration {
                name: "Variables.States".to_string(),
                fields: vec![
                    ("Declared".to_string(), json!("Declared")),
                    ("DefaultReady".to_string(), json!("DefaultReady")),
                    ("Pending".to_string(), json!("Pending")),
                    ("Ready".to_string(), json!("Ready")),
                    ("Faulted".to_string(), json!("Faulted")),
                    ("Retrying".to_string(), json!("Retrying")),
                    ("Paused".to_string(), json!("Paused")),
                    ("Cached".to_string(), json!("Cached")),
                    ("Dirty".to_string(), json!("Dirty")),
                ],
                reserved: true,
            },
        );

        // Boolean
        enums.insert(
            "Boolean".to_string(),
            Enumeration {
                name: "Boolean".to_string(),
                fields: vec![
                    ("True".to_string(), json!("True")),
                    ("False".to_string(), json!("False")),
                ],
                reserved: true,
            },
        );

        // Add aliases
        // #True -> #Boolean.True
        // #False -> #Boolean.False

        Self { enums }
    }

    pub fn is_reserved(&self, name: &str) -> bool {
        self.enums.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&Enumeration> {
        self.enums.get(name)
    }
}
```

---

#### Runtime Behavior

**Reserved Enumerations at Runtime:**

1. **No Runtime Allocation:**
   - Values are compile-time constants
   - Direct substitution in generated code
   - Zero runtime overhead

2. **Serialization:**
   - Reserved enum values are serialized as strings
   - Cross-language consistency maintained
   - Example: `#Variables.States.Ready` → `"Ready"` (JSON string)

3. **Comparison:**
   - String equality checks in serialized form
   - Runtime wrappers match string values

**Example Runtime Check (Python wrapper):**
```python
# Polyglot code:
# [?] .var.state =? #Variables.States.Ready

# Runtime wrapper (Python):
if var_state == "Ready":
    # Process ready variable
    pass
```

---

#### IR Representation

Reserved enumerations appear in IR as string constants:

**Before Compilation (Polyglot):**
```polyglot
[?] .user_data.state =? #Variables.States.Ready
[~][r] |ProcessUser
```

**After Compilation (IR JSON):**
```json
{
  "condition": {
    "type": "comparison",
    "left": {
      "type": "field_access",
      "object": ".user_data",
      "field": "state"
    },
    "operator": "=?",
    "right": {
      "type": "constant",
      "value": "Ready",
      "enum_type": "Variables.States"
    }
  },
  "then_block": {
    "type": "pipeline_call",
    "pipeline": "|ProcessUser"
  }
}
```

**Note:** `enum_type` metadata preserves type information for validation.

---

#### Future Reserved Enumerations

**Planned (Post-MVP):**

1. **`#HTTP.Status.*`** - HTTP status codes
   ```polyglot
   [#] HTTP.Status
   [<] .OK: pg\int << 200
   [<] .NotFound: pg\int << 404
   [<] .InternalServerError: pg\int << 500
   [X]
   ```

2. **`#Encoding.*`** - Character encodings
   ```polyglot
   [#] Encoding
   [<] .UTF8: pg\string << "UTF-8"
   [<] .ASCII: pg\string << "ASCII"
   [X]
   ```

3. **`#DT.Format.*`** - Datetime format patterns
   ```polyglot
   [#] DT.Format
   [<] .ISO8601: pg\string << "YYYY-MM-DDTHH:mm:ss.sssZ"
   [<] .USDate: pg\string << "MM/DD/YYYY"
   [X]
   ```

4. **`#Queue.Priority.*`** - Queue priority levels
   ```polyglot
   [#] Queue.Priority
   [<] .Critical: pg\int << 1
   [<] .High: pg\int << 2
   [<] .Normal: pg\int << 3
   [<] .Low: pg\int << 4
   [X]
   ```

---

#### Documentation Requirements

**For Each Reserved Enumeration:**

1. **Language Specification:** Formal definition with all fields
2. **Standard Library Docs:** Usage examples and patterns
3. **Compiler Implementation:** How it's loaded and validated
4. **Runtime Behavior:** Serialization format and cross-language handling

**Current Documentation Status:**

| Enumeration | Spec Defined? | Documented? | Implemented? |
|-------------|---------------|-------------|--------------|
| `#Variables.States.*` | ✅ Yes (v1.0.0) | ✅ Yes | ⚠️ Pending (Epic 2) |
| `#Pipelines.States.*` | ⚠️ Draft (brainstorming) | ⚠️ Partial | ❌ No |
| `#Boolean` | ✅ Yes (v0.0.2) | ✅ Yes | ⚠️ Pending (Epic 1) |
| `DT.Business.Week.*` | ✅ Yes (v0.0.2) | ✅ Yes | ⚠️ Pending (Epic 4) |

---

#### References

- **Brainstorming Session:** `docs/brainstorming-session-results-2025-11-23.md` (Lines 300-310, 432-448)
- **Variable States Spec:** `docs/technical/variable-states-specification.md` (Lines 377-402)
- **Syntax Spec:** `docs/v0.0.2/language/enumerations.md` (Reserved enumeration semantics)

---

