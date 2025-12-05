# IR Representation & Type System

**Document Type:** Architecture Design Document
**Version:** 0.0.2
**Last Updated:** 2025-11-20
**Status:** Active Development - Design Phase

---

## Overview

Intermediate Representation (IR) design, type system architecture, and cross-language type conversion strategy.

---

## Related Documents

### Architecture
- [System Architecture Overview](./00-overview.md) - High-level design
- [Database Schema](./01-database-schema.md) - IR storage tables

### Language Specification
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Complete grammar
- [Type System](../language/02-type-system.md) - Type specifications
- [Complete Syntax](../language/01-syntax-complete.md) - Full syntax reference

---

## IR Design Philosophy

### JSONB Storage (ADR-003 & ADR-008)

Polyglot stores IR as **3 separate JSONB columns** in PostgreSQL, combining document flexibility with relational power:

1. **Trigger IR:** Trigger conditions and event logic (read by Trigger Monitor)
2. **Queue IR:** Queue management and priority logic (read by Queue Manager)
3. **Runner IR:** Execution flow and runtime orchestration (read by Runner Pool)

**Benefits:**
- **Service Separation:** Each service reads only its relevant IR column
- **Efficient Querying:** GIN indexes enable fast JSONB queries
- **Flexible Schema:** IR can evolve without database migrations
- **SQL-Based Analysis:** Query IR structure using PostgreSQL's JSONB operators
- **Human Readable:** JSON format easier to debug than relational rows

### Type System Principles

**Strongly Typed:**
- No implicit type conversions across language boundaries
- All type conversions explicit in `.pg` code
- Type checking at compile time (IR generation)

**IDE-Friendly:**
- Deterministic type inference for autocomplete
- Type annotations provide IntelliSense hints
- Clear error messages for type mismatches

---

## Type System Specification

### Type Format

**Syntax:** `language\type`

**Examples:**
- `:pg.int` - Polyglot integer
- `:pg.string` - Polyglot string
- `:pg.bool` - Polyglot boolean
- `:pg.dt` - Polyglot datetime
- `:pg.path` - Polyglot path type
- `py\int` - Python integer
- `py\str` - Python string
- `py\dict` - Python dictionary
- `rs\i32` - Rust i32
- `rs\String` - Rust String
- `js\number` - JavaScript number

### Mutable Types

**Syntax:** `language.mutable\type`

**Examples:**
- `pg.mutable\int` - Mutable Polyglot integer
- `py.mutable\list` - Mutable Python list

### Collection Types

**Syntax:** `language\collection{element_type}`

**Examples:**
- `pg.array.pg.int` - Array of integers
- `:pg.set{pg\string}` - Set of strings
- `py\list{py\int}` - Python list of integers

### Type Conversion Strategy

Data is translated through intermediate representation:

```
Source Language Type → JSON/Binary → Target Language Type
```

**Example:**
```
Python dict → JSON string → Rust HashMap
pg\dt → ISO 8601 string → py\datetime
```

---

## IR Generation Process

```
.pg Source Files
      ↓
   Lexer (tokenization)
      ↓
   Parser (AST generation)
      ↓
   Type Checker (validation)
      ↓
   IR Generator (3-IR split)
      ↓
PostgreSQL JSONB Columns
   ├── trigger_ir (Trigger Monitor)
   ├── queue_ir (Queue Manager)
   └── runner_ir (Runner Pool)
```

---

## 3-IR JSONB Structure

Each pipeline is stored with 3 separate JSONB columns representing different concerns:

### Example: DataProcessor Pipeline

**Trigger IR** (trigger_ir column):
```json
{
  "type": "schedule",
  "trigger": "Daily",
  "config": {
    "time": "16:30:00",
    "timezone": "UTC"
  }
}
```

**Queue IR** (queue_ir column):
```json
{
  "priority": 5,
  "max_retries": 3,
  "timeout_seconds": 600,
  "resources": {
    "cpu": 2,
    "memory_mb": 4096,
    "gpu": 0
  },
  "pause_conditions": []
}
```

**Runner IR** (runner_ir column):
```json
{
  "inputs": [
    {"name": ".data", "type": "pg\\string", "required": true},
    {"name": ".timeout", "type": "pg\\int", "default": 30}
  ],
  "wrapper": {
    "runtime": "python",
    "version": "3.11"
  },
  "execution": [
    {
      "type": "parallel",
      "steps": [
        {"type": "run", "pipeline": "ProcessPartA"},
        {"type": "run", "pipeline": "ProcessPartB"}
      ],
      "join": {"type": "join", "operation": "Y.Join"}
    },
    {"type": "run", "pipeline": "CombineResults"}
  ],
  "outputs": [
    {"name": ".result", "type": "pg\\string"}
  ]
}
```

### Querying JSONB IR

```sql
-- Find all pipelines with daily triggers
SELECT name, trigger_ir FROM pipelines
WHERE trigger_ir->>'type' = 'schedule'
  AND trigger_ir->>'trigger' = 'Daily';

-- Find high-priority pipelines
SELECT name, queue_ir FROM pipelines
WHERE (queue_ir->>'priority')::int > 7;

-- Find pipelines using Python runtime
SELECT name, runner_ir FROM pipelines
WHERE runner_ir->'wrapper'->>'runtime' = 'python';
```

---

## Component IR Usage

### Trigger Monitor

**Reads:**
- **Trigger IR** from `pipelines.trigger_ir` column
- Trigger type, configuration (cron, file paths, webhooks)
- Extracted trigger definitions in `triggers` table (for monitoring)

**Writes:**
- Updates `last_triggered_at` timestamp in `triggers`
- Creates new `pipeline_instances` when triggered

**Example Query:**
```sql
SELECT id, name, trigger_ir
FROM pipelines
WHERE activated = TRUE;
```

### Queue Manager

**Reads:**
- **Queue IR** from `pipelines.queue_ir` column
- Priority, resource requirements, max retries, timeouts
- Current queue entries from `queue_entries`

**Writes:**
- Creates/updates queue entries
- Moves instances between queues (Pending → Dispatch → Pause)
- Updates priority and resource allocation

**Example Query:**
```sql
SELECT queue_ir->>'priority' as priority,
       queue_ir->'resources' as resources
FROM pipelines
WHERE id = ?;
```

### Runner Pool

**Reads:**
- **Runner IR** from `pipelines.runner_ir` column
- Execution steps, wrapper configuration, input/output types
- Execution state from `pipeline_instances` and `execution_steps`

**Writes:**
- Updates execution step status
- Stores checkpoint data
- Records results and errors

**Example Query:**
```sql
SELECT runner_ir->'execution' as execution_steps,
       runner_ir->'wrapper' as runtime_config
FROM pipelines
WHERE id = ?;
```

---

## Type Conversion Examples

### Polyglot → Python

```polyglot
[i] .count:pg.int << 42
[r] |U.Python.Print
[<] .value: py\int << .count  // Type conversion:pg.int → py\int
```

**IR Storage:**
```json
{
  "source_type": "pg\\int",
  "target_type": "py\\int",
  "conversion": "direct",  // Simple copy for compatible types
  "value": 42
}
```

### Python → Rust

```polyglot
[r] |ProcessData
[>] .result: py\dict >> .data

[r] |RustProcessor
[<] .input: rs\HashMap << .data  // Type conversion: py\dict → rs\HashMap
```

**IR Storage:**
```json
{
  "source_type": "py\\dict",
  "target_type": "rs\\HashMap",
  "conversion": "json_serialize",  // Serialize to JSON, then parse
  "intermediate": "json"
}
```

---

## Enumeration IR Storage

Enumerations are embedded in JSONB IR where used:

```json
{
  "type": "enumeration",
  "name": "Config.Database",
  "fields": [
    {"name": ".host", "type": "pg\\string", "value": "localhost"},
    {"name": ".port", "type": "pg\\int", "value": 5432},
    {"name": ".database", "type": "pg\\string", "value": "polyglot_db"}
  ]
}
```

**Note:** Enumerations are typically embedded inline where referenced in the Runner IR, rather than stored as separate entities.

---

## Error Type IR Storage

Error types (e.g., `!ValidationError`) are embedded in JSONB IR where handled:

```json
{
  "type": "error",
  "name": "!ValidationError",
  "fields": [
    {"name": ".message", "type": "pg\\string", "value": "Validation failed"},
    {"name": ".code", "type": "pg\\int", "value": 4000},
    {"name": ".trace", "type": "pg\\string", "value": ""}
  ]
}
```

**Note:** Error handlers in Runner IR reference error types; the actual error definitions are resolved at compile time.

---

## See Also

- [System Architecture](./00-overview.md) - Component design
- [Database Schema](./01-database-schema.md) - Complete schema
- [Type System](../language/02-type-system.md) - Language type specs
- [Enumerations](../language/03-enumerations.md) - Enumeration system
- [Error Handling](../language/04-error-handling.md) - Error types

---

**Note:** This IR design is based on v0.0.1 architecture planning with v0.0.2 syntax compliance. All type strings use v0.0.2 backslash separator: `:pg.int`, `py\dict`, `:pg.dt`.
