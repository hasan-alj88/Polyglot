# IR Specification - TODO

**Status:** 📋 Backlog (Scheduled after Epic 1, before Epic 2)
**Priority:** Critical
**Estimated Effort:** 3-5 days
**Dependencies:** Epic 1 (Lexer) must be complete

---

## When to Create This

**Timeline Position:**
```
Epic 1: Lexer ✅ Complete
    ↓
📋 THIS DOCUMENT ← You are here (not started)
    ↓
Epic 2: Parser & IR Generation
```

**Why This Order?**
1. **After Lexer:** Need to know what tokens exist before defining how to represent them in IR
2. **Before Compiler:** Need IR structure defined before writing parser/compiler that generates it
3. **Before Epic 2 Stories:** Story 2.1 (Parser) and 2.2 (IR Generation) both need this spec

---

## What This Document Must Define

### 1. Complete JSON Schema for All 3 IRs

**Trigger IR:**
- All trigger types (time, resource, webhook, manual, file_watch)
- Trigger configuration schemas
- Enable/disable flags
- Trigger priority/ordering

**Queue IR:**
- `[t]` timing block logic (immediate, delayed, scheduled)
- `[Q]` queue selection logic (default, pause, user-defined)
- Priority system
- Rate limiting
- Resource limits

**Runner IR:**
- Pipeline structure (`[|]` blocks)
- Enumeration definitions (`[#]` blocks)
- Variable declarations and metadata
- Input/output blocks (`[i]`, `[o]`)
- All execution blocks (`[r]`, `[p]`, `[b]`, `[s]`, `[Y]`)
- Control flow (`[?]` conditionals, `[!]` error handlers)
- Wrapper configuration (`[W]` blocks)
- Execution graph (DAG of dependencies)

---

### 2. Syntax Mapping Examples

**For Every Polyglot Feature, Show:**
- Polyglot source code
- Corresponding IR JSON
- Explanation of transformation

**Minimum Examples Needed:**
1. Simple pipeline with one step
2. Pipeline with variables (all 3 operators: `<~`, `<<`, `>>`)
3. Enumeration definition
4. Conditional logic (`[?]` block)
5. Error handling (`[!]` block)
6. Parallel execution (`[p]` block)
7. Join operation (`[Y]` block)
8. Wrapper call (`[W]` + `[r]`)
9. Complex nested pipeline
10. Variable state transitions

---

### 3. Validation Rules

**What makes IR valid?**
- Required fields vs optional fields
- Type constraints (e.g., `state` must be one of 9 valid states)
- Referential integrity (step IDs must exist in execution graph)
- Variable references (all used variables must be declared)
- Type safety (variable types match usage)

---

### 4. Block Marker → IR Mapping

**Complete mapping table for all 27+ block markers:**

| Block Marker | IR Representation | Example |
|--------------|-------------------|---------|
| `[|]` | Pipeline definition | Top-level object |
| `[#]` | Enumeration definition | `enumerations` object |
| `[i]` | Input block | `input_block` with variables |
| `[o]` | Output block | `output_block` with variables |
| `[t]` | Timing block | Queue IR `timing_block` |
| `[Q]` | Queue selection | Queue IR `queue_block` |
| `[W]` | Wrapper | Runner IR wrapper config |
| `[r]` | Pipeline call | Step with type "pipeline_call" |
| `[p]` | Parallel execution | Step with mode "parallel" |
| `[b]` | Background execution | Step with mode "background" |
| `[s]` | Serial load | Step with type "serial_load" |
| `[Y]` | Join operation | Step with type "join" |
| `[?]` | Conditional | Step with type "conditional" |
| `[!]` | Error handler | Step with type "error_handler" |
| `[<]` | Input field | Enumeration field (input) |
| `[>]` | Output field | Enumeration field (output) |
| `[~]` | Nested block | Nested steps array |
| `[\]` | Setup block | `setup_block` in Runner IR |
| `[/]` | Cleanup block | `cleanup_block` in Runner IR |
| `[X]` | End marker | N/A (structure boundary) |

---

### 5. Type System Encoding

**How types are represented in IR:**

| Polyglot Type | IR Representation | Example |
|---------------|-------------------|---------|
| `pg\string` | `{"type": "primitive", "name": "string"}` | Basic type |
| `pg\int` | `{"type": "primitive", "name": "int"}` | Basic type |
| `pg\array{pg\string}` | `{"type": "array", "element": {"type": "primitive", "name": "string"}}` | Array |
| `#UserProfile` | `{"type": "enumeration", "name": "UserProfile"}` | User-defined |
| `#Variables.States.Ready` | `{"type": "reserved_enum", "path": ["Variables", "States", "Ready"]}` | Reserved |

---

### 6. Variable State Metadata

**Complete variable object structure:**

```json
{
  "name": ".user_data",
  "type": {
    "type": "enumeration",
    "name": "UserProfile"
  },
  "state": "Pending",
  "assignment_operator": ">+",
  "override_count": 0,
  "default_value": null,
  "errors": [],
  "source": {
    "type": "pipeline_output",
    "pipeline": "|FetchUser",
    "output_field": ".user"
  },
  "location": {
    "file": "my_pipeline.pg",
    "line": 12,
    "column": 5
  }
}
```

---

### 7. Error Information Encoding

**Error objects in IR:**

```json
{
  "type": "!pg.Network.Timeout",
  "message": "API request timed out after 30s",
  "code": 408,
  "timestamp": "2025-11-24T10:30:00Z",
  "context": {
    "endpoint": "https://api.example.com/users",
    "timeout_ms": 30000,
    "retry_count": 3
  },
  "stack_trace": [
    {
      "pipeline": "|FetchUser",
      "step": "step_uuid",
      "line": 15
    }
  ]
}
```

---

### 8. Execution Graph

**DAG representation of step dependencies:**

```json
{
  "execution_graph": {
    "nodes": [
      {
        "id": "step_1",
        "type": "pipeline_call",
        "can_run_parallel": true
      },
      {
        "id": "step_2",
        "type": "conditional",
        "can_run_parallel": false
      }
    ],
    "edges": [
      {
        "from": "step_1",
        "to": "step_2",
        "condition": "success",
        "variable_dependencies": [".user_data"]
      }
    ],
    "entry_points": ["step_1"],
    "exit_points": ["step_3"]
  }
}
```

---

### 9. Reserved Fields Handling

**How `.state` and `.errors` are encoded:**

```json
{
  "variables": {
    ".user_data": {
      "type": "...",
      "reserved_fields": {
        ".state": {
          "type": "reserved_enum",
          "path": ["Variables", "States"],
          "current_value": "Ready",
          "read_only": true,
          "compiler_managed": true
        },
        ".errors": {
          "type": "array",
          "element_type": "ErrorObject",
          "current_value": [],
          "read_only": true,
          "compiler_managed": true
        }
      }
    }
  }
}
```

---

### 10. Versioning Strategy

**IR version compatibility:**

```json
{
  "ir_version": "1.0.0",
  "polyglot_version": "0.0.2",
  "schema_url": "https://polyglot-lang.org/schemas/ir/v1.0.0.json",
  "backward_compatible_with": ["0.9.0"],
  "breaking_changes": []
}
```

---

## Document Structure

**Proposed outline for `docs/technical/ir-specification.md`:**

```markdown
# Intermediate Representation (IR) Specification

## 1. Overview
   - What is IR?
   - Why 3 separate IRs?
   - Compilation flow

## 2. Trigger IR
   - Complete JSON schema
   - All trigger types
   - Configuration schemas
   - Examples

## 3. Queue IR
   - Complete JSON schema
   - Timing logic ([t] blocks)
   - Queue selection ([Q] blocks)
   - Examples

## 4. Runner IR
   - Complete JSON schema
   - Pipeline structure
   - Enumeration definitions
   - Variable metadata
   - Execution graph
   - Examples

## 5. Block Marker Mapping
   - Table: All 27+ markers → IR
   - Examples for each marker

## 6. Type System
   - Primitive types
   - Collections (array, map)
   - User-defined enumerations
   - Reserved enumerations
   - Type encoding in IR

## 7. Variable States
   - State metadata structure
   - Reserved fields (.state, .errors)
   - Default values and operators

## 8. Control Flow
   - Conditionals ([?])
   - Error handlers ([!])
   - Joins ([Y])
   - Execution graph

## 9. Validation Rules
   - Required fields
   - Type constraints
   - Referential integrity
   - Variable scoping

## 10. Syntax → IR Examples
   - 20+ side-by-side examples
   - Simple to complex
   - All major features covered

## 11. Versioning & Compatibility
   - IR version format
   - Breaking changes policy
   - Migration guide

## Appendix
   - Complete JSON Schema (JSON Schema format)
   - Type definitions (TypeScript/Rust)
   - Validation checklist
```

---

## Success Criteria

**This document is complete when:**

✅ Every Polyglot syntax feature has IR representation defined
✅ JSON schema for all 3 IRs is complete and validated
✅ Validation rules are explicit and testable
✅ Epic 2 implementers can generate IR without guessing
✅ Side-by-side examples cover all block markers
✅ Type system is fully specified
✅ Variable state metadata is complete
✅ Execution graph format is defined

---

## Who Should Write This?

**Recommended:** Collaborative effort between:
1. **Language Designer** - Define semantics and syntax mapping
2. **Architect** - Ensure structure supports all 3 services (Trigger Monitor, Queue Manager, Runner)
3. **Epic 2 Implementer** - Validate that spec is implementable

**Alternative:** AI agent with **bmm-technical-evaluator** + **bmm-requirements-analyst** for comprehensive spec generation

---

## Dependencies on Other Documentation

**Must Read Before Writing:**
1. `docs/technical/variable-states-specification.md` - Variable state model
2. `docs/user/language/syntax-complete.md` - Complete Polyglot syntax
3. `docs/user/language/block-markers.md` - All block markers defined
4. `docs/technical/architecture.md` - How 3 IRs are used by services

---

## Estimated Timeline

**If done comprehensively:**
- Research & planning: 1 day
- Trigger IR specification: 0.5 day
- Queue IR specification: 0.5 day
- Runner IR specification: 2 days (most complex)
- Examples & validation: 1 day
- Review & iteration: 0.5 day

**Total: 5-6 days**

**Can be accelerated with AI agents or parallel work**

---

## Notes

- This is **blocking work** for Epic 2
- Without this, Epic 2 implementers will make inconsistent decisions
- IR is the "contract" between compiler and runtime
- Get this right → rest of implementation is straightforward
- Get this wrong → technical debt and refactoring

---

## Next Steps (When Ready)

1. ✅ Complete Epic 1 (Lexer) - **Story 1.5 (Parser Implementation) must pass**
2. 📋 Prioritize this document - **Block all Epic 2 work until complete**
3. ✍️ Write IR specification - **Use this TODO as outline**
4. ✅ Review with team - **Validate completeness**
5. 🚀 Begin Epic 2 - **Stories 2.1-2.4 now have clear target**

---

**Status:** Waiting for Epic 1 completion (Stories 1.1-1.5)

**Reminder:** Do NOT start Epic 2 without this specification!
