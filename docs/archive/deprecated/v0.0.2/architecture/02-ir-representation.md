---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: none
---
<!-- no current replacement -->
> **Deprecated:** No current replacement exists for this content.

# IR Representation & Type System

**Document Type:** Architecture Design Document
**Version:** 0.0.2
**Last Updated:** 2025-11-14
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
- [BNF Grammar](../language/bnf/aljam3 grammer.md) - Complete grammar
- [Type System](../language/02-type-system.md) - Type specifications
- [Complete Syntax](../language/01-syntax-complete.md) - Full syntax reference

---

## IR Design Philosophy

### Database-Centric Storage

<!-- @c:reference/glossary#pipeline -->
Rather than storing IR as JSON blobs, Aljam3 uses **normalized relational tables** to represent the AST structure. This enables:

1. **Efficient Querying:** Components can query IR using SQL
2. **Indexing & Optimization:** Database can optimize common queries
3. **Relational Integrity:** Foreign keys enforce IR structure validity
4. **SQL-Based Analysis:** Monitor and analyze IR without custom parsers

### Type System Principles

**Strongly Typed:**
- No implicit type conversions across language boundaries
- All type conversions explicit in `.aj3` code
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
- `pg\int` - Aljam3 integer
- `pg\string` - Aljam3 string
- `pg\bool` - Aljam3 boolean
- `pg\dt` - Aljam3 datetime
- `pg\path` - Aljam3 path type
- `py\int` - Python integer
- `py\str` - Python string
- `py\dict` - Python dictionary
- `rs\i32` - Rust i32
- `rs\String` - Rust String
- `js\number` - JavaScript number

### Mutable Types

**Syntax:** `language.mutable\type`

**Examples:**
- `pg.mutable\int` - Mutable Aljam3 integer
- `py.mutable\list` - Mutable Python list

### Collection Types

**Syntax:** `language\collection{element_type}`

**Examples:**
- `pg\array{pg\int}` - Array of integers
- `pg\set{pg\string}` - Set of strings
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
.aj3 Source Files
      ↓
   Lexer (tokenization)
      ↓
   Parser (AST generation)
      ↓
   Type Checker (validation)
      ↓
   IR Generator
      ↓
PostgreSQL Tables (normalized IR)
```

---

## IR Tree Structure

The `ir_nodes` table stores a tree structure using parent-child relationships:

### Example Pipeline IR Tree

```
Pipeline (root)
├── Input (.data: pg\string)
├── Input (.timeout: pg\int)
├── Trigger (|T.Daily)
├── Wrapper (|W.Python3.11)
├── Parallel Group
│   ├── Run (|ProcessPartA)
│   └── Run (|ProcessPartB)
├── Join (|Y.Join)
├── Run (|CombineResults)
└── Output (.result: pg\string)
```

### SQL Representation

```sql
-- Root: Pipeline Definition
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label)
VALUES ('uuid-1', NULL, 1, 'pipeline', 'DataProcessor');

-- Input Nodes
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, input_type, attributes)
VALUES
  ('uuid-1', 'uuid-root', 1, 'input', 'pg\string', '{"name": ".data", "required": true}'),
  ('uuid-1', 'uuid-root', 2, 'input', 'pg\int', '{"name": ".timeout", "default": 30}');

-- Trigger Node
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label, attributes)
VALUES
  ('uuid-1', 'uuid-root', 3, 'trigger', '|T.Daily', '{"time": "16:30:00"}');

-- Wrapper Node
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label)
VALUES
  ('uuid-1', 'uuid-root', 4, 'wrapper', '|W.Python3.11');

-- Parallel Group
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, is_parallel, join_group_id)
VALUES
  ('uuid-1', 'uuid-root', 5, 'parallel', TRUE, 'group-1');

-- Parallel Steps
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label, join_group_id)
VALUES
  ('uuid-1', 'uuid-parallel', 1, 'run', '|ProcessPartA', 'group-1'),
  ('uuid-1', 'uuid-parallel', 2, 'run', '|ProcessPartB', 'group-1');

-- Join Node
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label, join_group_id)
VALUES
  ('uuid-1', 'uuid-root', 6, 'join', '|Y.Join', 'group-1');

-- Output Node
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, output_type, attributes)
VALUES
  ('uuid-1', 'uuid-root', 7, 'output', 'pg\string', '{"name": ".result"}');
```

---

## Component IR Usage

### Trigger Monitor

<!-- @c:reference/glossary#Trigger Monitor -->
**Reads:**
- Trigger definitions from `triggers` table
- Trigger configuration (cron, file paths, webhooks)

**Writes:**
- Updates `last_triggered_at` timestamp
- Creates new `pipeline_instances`

<!-- @c:reference/glossary#Queue Manager -->
### Queue Manager

**Reads:**
- Queue definitions from `queue_definitions`
- Priority, resource requirements, timeouts
- Current queue entries from `queue_entries`

**Writes:**
- Creates/updates queue entries
- Moves instances between queues (Pending → Dispatch → Pause)
- Updates priority and resource allocation

### Runner Pool

**Reads:**
- Pipeline structure from `ir_nodes`
- Step order, types, and attributes
- Input/output type information

**Writes:**
- Updates `execution_steps` status
- Records stdout/stderr, exit codes
- Captures resource usage metrics

---

## Type Conversion Examples

### Aljam3 → Python

```aljam3
[i] .count: pg\int << 42
[r] |U.Python.Print
[<] .value: py\int << .count  // Type conversion: pg\int → py\int
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

```aljam3
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

Enumerations are stored in `ir_nodes` with special handling:

```sql
-- Enumeration Definition
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label)
VALUES ('uuid-pkg', NULL, 1, 'enumeration', 'Config.Database');

-- Enumeration Fields
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label, input_type, attributes)
VALUES
  ('uuid-pkg', 'uuid-enum', 1, 'enum_field', '.host', 'pg\string', '{"value": "localhost"}'),
  ('uuid-pkg', 'uuid-enum', 2, 'enum_field', '.port', 'pg\int', '{"value": 5432}'),
  ('uuid-pkg', 'uuid-enum', 3, 'enum_field', '.database', 'pg\string', '{"value": "aljam3_db"}');
```

---

## Error Type IR Storage

Error types (e.g., `!ValidationError`) are stored similarly to enumerations:

```sql
-- Error Definition
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label)
VALUES ('uuid-pkg', NULL, 1, 'error', '!ValidationError');

-- Required Error Fields (per v0.0.2 spec)
INSERT INTO ir_nodes (pipeline_definition_id, parent_node_id, node_order, node_type, node_label, input_type, attributes)
VALUES
  ('uuid-pkg', 'uuid-error', 1, 'error_field', '.message', 'pg\string', '{"value": "Validation failed"}'),
  ('uuid-pkg', 'uuid-error', 2, 'error_field', '.code', 'pg\int', '{"value": 4000}'),
  ('uuid-pkg', 'uuid-error', 3, 'error_field', '.trace', 'pg\string', '{"value": ""}');
```

---

## See Also

- [System Architecture](./00-overview.md) - Component design
- [Database Schema](./01-database-schema.md) - Complete schema
- [Type System](../language/02-type-system.md) - Language type specs
- [Enumerations](../language/03-enumerations.md) - Enumeration system
- [Error Handling](../language/04-error-handling.md) - Error types

---

**Note:** This IR design is based on v0.0.1 architecture planning with v0.0.2 syntax compliance. All type strings use v0.0.2 backslash separator: `pg\int`, `py\dict`, `pg\dt`.
