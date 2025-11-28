# Variable States: Technical Specification

**Version:** 1.0.0
**Last Updated:** 2025-11-24
**Status:** Specification
**Audience:** Language implementers, compiler engineers, runtime developers

---

## Table of Contents

1. [Overview](#overview)
2. [Foundational Principles](#foundational-principles)
3. [Complete State Model](#complete-state-model)
4. [Assignment Operators](#assignment-operators)
5. [State Lifecycle](#state-lifecycle)
6. [Reserved Fields](#reserved-fields)
7. [Reserved Enumerations](#reserved-enumerations)
8. [State Transitions](#state-transitions)
9. [Runtime Semantics](#runtime-semantics)
10. [Implementation Requirements](#implementation-requirements)
11. [Edge Cases](#edge-cases)
12. [Appendix](#appendix)

---

## Overview

### Purpose

This specification defines the complete variable state model for the Polyglot programming language. It provides implementation requirements for compilers, runtimes, and tooling.

### Scope

- **In Scope:** Variable states, assignment operators, state transitions, runtime semantics
- **Out of Scope:** Pipeline states (see Pipeline States Specification), queue control syntax

### Key Insight

Polyglot is **async-centric**. Variables don't have "values" immediately - they transition through states as async operations complete. This specification defines those states and transitions.

---

## Foundational Principles

### Principle 1: Async-Centric by Design

Polyglot is not a synchronous language with async features bolted on. **Async is the foundation.**

**Implications:**
- Variables may not be immediately available
- State transitions are first-class concerns
- Automatic waiting is core runtime behavior
- No explicit `await` keyword

### Principle 2: State-Aware Coordination

Traditional "mutability" concepts don't apply. Polyglot uses **state transitions** for async coordination.

**Implications:**
- Variables transition: Declared → Pending → Ready/Faulted
- Once Ready, variables are immutable (consequence, not design goal)
- States are queryable via `.state` field
- Compiler/runtime manage transitions

### Principle 3: Serialization Foundation

ALL Polyglot data is serialized for cross-language coordination.

**Implications:**
- Variables are serialized strings
- Dot notation is primary syntax
- Type information is metadata
- Cross-language compatibility required

### Principle 4: Automatic Waiting

Pipelines automatically wait for variables to be ready. No explicit synchronization.

**Implications:**
- Pipeline boundaries trigger waits
- Developers never write await
- `[i]` blocks expect Ready variables
- Runtime handles blocking/unblocking

---

## Complete State Model

Polyglot variables have **9 distinct states** organized into two categories:

### Core States (5)

States required for basic variable lifecycle:

| State | Description | Can Read Value? | Mutable? | Triggers Pipeline? |
|-------|-------------|----------------|----------|-------------------|
| **Declared** | Schema defined, no value, no default | ❌ No | N/A | ❌ No ([i] requires Ready) |
| **DefaultReady** | Has default value, allows ONE override | ✅ Yes | ⚠️ Once | ✅ Yes |
| **Pending** | Async operation in progress | ❌ No | N/A | ⚠️ Waits |
| **Ready** | Value available, immutable | ✅ Yes | ❌ No | ✅ Yes |
| **Faulted** | Operation failed, has error info | ❌ No* | ❌ No | ❌ No (error path) |

*Can read `.errors` field

### Queue Management States (4)

Advanced states for resilience and performance:

| State | Description | Purpose | Transition |
|-------|-------------|---------|------------|
| **Retrying** | Automatic retry attempt in progress | Transient failure recovery | → Ready/Faulted |
| **Paused** | Waiting for external trigger | Human approval, scheduled events | → Pending (when triggered) |
| **Cached** | Cached result, may be stale | Performance optimization | → Dirty (on invalidation) |
| **Dirty** | Cache invalid, needs refresh | Cache invalidation | → Pending (on refresh) |

---

## Assignment Operators

Polyglot has **three bidirectional operator pairs** for variable assignment:

### Operator Summary

| Operator Pair | Direction | Purpose | Resulting State |
|---------------|-----------|---------|-----------------|
| None | N/A | Schema-only declaration | Declared |
| `<~` / `~>` | Bidirectional | Default assignment | DefaultReady |
| `<<` / `>>` | Bidirectional | Constant/Async assignment | Ready or Pending |

---

### 1. Schema-Only (No Operator)

**Syntax:**
```polyglot
[<] .field: Type
```

**Semantics:**
- Declares field schema without value
- No default provided
- Field starts in **Declared** state
- Must be explicitly populated before use

**Use Cases:**
- Fields populated by pipelines
- Required parameters
- Data from external sources

**Example:**
```polyglot
[#] UserProfile
[<] .id: pg\string          # Declared (must populate)
[<] .name: pg\string        # Declared (must populate)
[<] .email: pg\string       # Declared (must populate)
[X]
```

---

### 2. Default Assignment `<~` / `~>`

**Syntax:**
```polyglot
[<] .field: Type <~ default_value    # Left direction
[>] .field: Type ~> .var             # Right direction
```

**Semantics:**
- Provides default value
- Field starts in **DefaultReady** state
- Allows **ONE override** during instantiation
- After first use or override → transitions to **Ready**
- Default kicks in at `[i]` blocks if not overridden

**Override-Once Behavior:**
```polyglot
[#] Config
[<] .timeout: pg\int <~ 30
[X]

[i] .config1: #Config << #Config
# .config1.timeout = 30 (default used)

[i] .config2: #Config << #Config{.timeout: 60}
# .config2.timeout = 60 (overridden)
# After this, .config2.timeout is immutable (Ready state)
```

**Use Cases:**
- Configuration with sensible defaults
- Optional parameters
- Fallback values

**Implementation Requirements:**
- Track override count per field
- Enforce single override constraint
- Transition DefaultReady → Ready after override or first use
- Default application at `[i]` block entry

---

### 3. Constant/Async Assignment `<<` / `>>`

**Syntax:**
```polyglot
[<] .field: Type << constant_value   # Constant (left)
[>] .field: Type >> .var             # Async assignment (right)
```

**Semantics:**

#### Constant Assignment (`<<`)
- Immediate **Ready** state
- Value is immutable
- Cannot be overridden

#### Async Assignment (`>>`)
- Variable starts in **Pending** state
- Transitions to **Ready** or **Faulted** when pipeline completes
- Immutable once Ready

**Use Cases:**
- Constants: Version numbers, fixed config
- Async: Pipeline outputs, API responses

**Example:**
```polyglot
[#] AppInfo
[<] .version: pg\string << "1.0.0"    # Ready immediately (constant)
[X]

[r] |FetchData
[>] .result: pg\string >> .data       # Pending → Ready/Faulted (async)
```

---

## State Lifecycle

### Basic Lifecycle Flow

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
(no value) (has default) (const)  (waiting)
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

### Extended Lifecycle with Queue States

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

---

## Reserved Fields

### `.state` Field

**Type:** `#Variables.States.*` (Reserved Enumeration)

**Purpose:** Query current variable state

**Semantics:**
- Available on ALL variables
- Read-only
- Compiler-managed
- Updated atomically on state transitions

**Usage:**
```polyglot
[?] .var.state =? #Variables.States.Ready
[~][r] |ProcessData
[~][<] .input << .var

[?] .var.state =? #Variables.States.Faulted
[~][r] |HandleError
[~][<] .errors << .var.errors
```

**Implementation Requirements:**
- Thread-safe reads
- Atomic updates on transitions
- Available for nested fields (`.user.address.state`)

---

### `.errors` Field

**Type:** `pg\array{!}` (Array of error objects)

**Purpose:** Store error details when Faulted

**Semantics:**
- Available on ALL variables
- Populated on Faulted state
- Empty array when Ready
- Compiler-managed

**Error Object Structure:**
```polyglot
[#] ErrorObject
[<] .type: pg\string              # Error type (e.g., "!pg.Network.Timeout")
[<] .message: pg\string           # Human-readable message
[<] .code: pg\int                 # Error code (optional)
[<] .timestamp: pg\string         # ISO 8601 timestamp
[<] .context: pg\map{pg\string,pg\string}  # Additional context
[X]
```

**Usage:**
```polyglot
[r] |RiskyOperation
[>] .result: pg\string >> .data
[>] .errors: pg\array{!} >> .operation_errors

[?] .data.state =? #Variables.States.Faulted
[~][r] |LogErrors
[~][<] .errors: pg\array{!} << .operation_errors
```

**Implementation Requirements:**
- Support multiple errors (array)
- Preserve error order (chronological)
- Include stack traces in context
- Thread-safe access

---

## Reserved Enumerations

### `#Variables.States.*`

Complete enumeration of variable states:

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

**Implementation Requirements:**
- Pre-compile time constants
- Immutable
- Available globally
- Type-safe comparisons

---

## State Transitions

### Valid Transitions

| From State | To State(s) | Trigger | Notes |
|------------|-------------|---------|-------|
| Declared | Pending | Pipeline assignment | Field populated via `>>` |
| Declared | Ready | Direct assignment | Explicit value provided |
| DefaultReady | Pending | Override with async | Override with pipeline result |
| DefaultReady | Ready | First use or override | Default used or overridden |
| Pending | Ready | Pipeline success | Value fulfilled |
| Pending | Faulted | Pipeline failure | Error occurred |
| Pending | Retrying | Transient failure | Auto-retry triggered |
| Faulted | Retrying | Retry attempt | Manual or auto retry |
| Retrying | Ready | Retry success | Operation succeeded |
| Retrying | Faulted | Retry exhausted | All retries failed |
| Ready | Cached | Cache enabled | Result cached |
| Cached | Dirty | Invalidation event | Cache invalidated |
| Dirty | Pending | Refresh triggered | Re-fetch data |
| Pending | Paused | External trigger needed | Wait for approval |
| Paused | Pending | Trigger received | Resume execution |

### Invalid Transitions

**Forbidden transitions (compiler/runtime must prevent):**

- Ready → Declared (cannot "un-ready" a variable)
- Ready → Pending (cannot make Ready async again)
- Faulted → Ready (must go through Retrying)
- DefaultReady → Declared (cannot remove default)
- Any state → DefaultReady (DefaultReady is initial state only)

---

## Runtime Semantics

### Automatic Waiting Behavior

**Rule:** Pipelines automatically wait for Pending variables.

**Implementation:**
```
When pipeline P references variable V:
  IF V.state == Pending:
    Block P until V.state ∈ {Ready, Faulted}
  ELSE IF V.state == Declared:
    Throw CompileError: "Variable must be Ready at pipeline boundary"
  ELSE IF V.state ∈ {Ready, DefaultReady, Cached}:
    Proceed with pipeline execution
  ELSE IF V.state == Faulted:
    Propagate error to error handler
```

**Blocking Mechanism:**
- Non-busy wait (yield CPU)
- Timeout support (configurable)
- Cancellation support (abort pipeline)

---

### `[i]` Block Entry Semantics

**Rule:** All `[i]` variables MUST be Ready (or DefaultReady) before pipeline triggers.

**Implementation:**
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

---

### DefaultReady Override Semantics

**Rule:** DefaultReady fields allow ONE override before becoming immutable.

**Implementation:**
```
Enumeration field F with default D:

Instantiation without override:
  F.value = D
  F.state = DefaultReady

  On first read:
    F.state = Ready
    F.value remains D (immutable)

Instantiation with override O:
  F.value = O
  F.override_count = 1
  F.state = Ready  (immediate transition)
  F.value is immutable

  On second override attempt:
    Throw RuntimeError: "Cannot override field twice"
```

**Concurrency Safety:**
- Override count atomic increment
- State transition atomic with value update
- Race condition prevention via locking

---

### Error Propagation

**Rule:** Faulted variables propagate errors through pipeline chains.

**Implementation:**
```
Pipeline P outputs variable V to pipeline Q:

IF V.state == Faulted:
  Q receives Faulted variable
  Q can:
    - Check V.state explicitly
    - Handle via error block [!]
    - Propagate to downstream pipeline

Error propagation modes:
  1. Explicit handling: [!] error blocks
  2. State checking: [?] .var.state =? #Variables.States.Faulted
  3. Automatic propagation: If unhandled, abort pipeline Q
```

---

## Implementation Requirements

### Compiler Requirements

#### 1. Type Checking
- Enforce type safety for all operators
- Validate Default value matches field type
- Check state comparisons use correct enumerations

#### 2. State Analysis
- Track state flow through pipeline graph
- Detect unreachable code after Faulted checks
- Warn on unused `.errors` fields

#### 3. Optimization
- Eliminate redundant state checks
- Inline constant assignments
- Optimize away DefaultReady → Ready transitions when no override

#### 4. Error Detection
- Detect second override attempts on DefaultReady fields
- Warn when Declared fields reach `[i]` blocks
- Flag invalid state transitions

---

### Runtime Requirements

#### 1. State Management
- Atomic state transitions
- Thread-safe state reads
- State history for debugging (optional)

#### 2. Waiting Mechanism
- Non-busy wait for Pending variables
- Timeout handling (configurable)
- Cancellation support
- Deadlock detection

#### 3. Memory Management
- Free Pending variable resources on Ready/Faulted
- Clean up error objects after handling
- Cache eviction for Cached state

#### 4. Observability
- State transition logging (debug mode)
- Metrics: state distribution, wait times
- Tracing: variable lifecycle visualization

---

### Debugging Requirements

#### 1. State Inspection
- Runtime API to query variable state
- State history (last N transitions)
- Current value (if Ready)

#### 2. Error Details
- Full `.errors` array access
- Stack traces for Faulted states
- Context propagation

#### 3. Performance Analysis
- Wait time metrics per variable
- State transition bottlenecks
- Cache hit rates (Cached state)

---

## Edge Cases

### Edge Case 1: Nested Field States

**Scenario:** Accessing state of nested field

```polyglot
[#] Address
[<] .city: pg\string
[X]

[#] User
[<] .address: #Address
[X]

[r] |FetchUser
[>] .user: #User >> .user_data

# Is this valid?
[?] .user_data.address.city.state =? #Variables.States.Ready
```

**Resolution:** YES, all serialized fields have `.state` introspection.

**Implementation:**
- Nested fields inherit parent state initially
- Can have independent state after assignment
- `.user_data.address.state` reflects inner enumeration state

---

### Edge Case 2: DefaultReady at `[i]` Block

**Scenario:** DefaultReady variable at pipeline entry

```polyglot
[#] Config
[<] .timeout: pg\int <~ 30
[X]

[|] MyPipeline
[i] .config: #Config << #Config
# What is .config.timeout.state here?
```

**Resolution:** **Ready** (default kicked in at `[i]`)

**Implementation:**
- At `[i]` block entry, DefaultReady → Ready transition occurs
- Default value applied if not overridden
- Transition is atomic

---

### Edge Case 3: Faulted Variable Passed to Pipeline

**Scenario:** Passing Faulted variable as input

```polyglot
[r] |FailingOperation
[>] .result: pg\string >> .data

[r] |ProcessData
[<] .input: pg\string << .data  # What if .data is Faulted?
```

**Resolution:** Pipeline waits for Ready/Faulted, then:
- If Faulted: Error propagates to ProcessData
- ProcessData can handle via error blocks
- If unhandled, ProcessData aborts

**Implementation:**
- Wait completes on Faulted state
- Error propagation via pipeline chain
- Unhandled errors abort pipeline

---

### Edge Case 4: Concurrent State Access

**Scenario:** Multiple threads reading/writing same variable state

```polyglot
[p] |BackgroundTask1
[>] .result: pg\string >> .shared_var

[p] |BackgroundTask2
[<] .input: pg\string << .shared_var
```

**Resolution:** Thread-safe state transitions

**Implementation:**
- State reads: atomic, lock-free
- State writes: mutex-protected
- State transitions: atomic compare-and-swap
- Waiting: condition variable or similar

---

### Edge Case 5: Second Override Attempt

**Scenario:** Trying to override DefaultReady field twice

```polyglot
[#] Config
[<] .timeout: pg\int <~ 30
[X]

[i] .config1: #Config << #Config{.timeout: 60}  # First override: OK
# Can we create another instance with different override?
[i] .config2: #Config << #Config{.timeout: 90}  # Second instance: OK

# But within same instance:
# .config1.timeout = 120  # ERROR: Already overridden
```

**Resolution:**
- Each instance tracks its own override count
- Second override on SAME INSTANCE is error
- Different instances are independent

**Implementation:**
- Per-instance override tracking
- Runtime error on second override
- Clear error message with field name

---

## Appendix

### A. State Comparison Table

| State | Has Value? | Can Override? | Blocks Pipeline? | Error Info? |
|-------|-----------|---------------|------------------|-------------|
| Declared | ❌ | N/A (no value) | ✅ Yes (not Ready) | ❌ |
| DefaultReady | ✅ (default) | ✅ (once) | ❌ No (Ready-like) | ❌ |
| Pending | ❌ | ❌ | ✅ Yes (waits) | ❌ |
| Ready | ✅ | ❌ | ❌ No | ❌ |
| Faulted | ❌ | ❌ | ✅ Yes (error path) | ✅ (.errors) |
| Retrying | ❌ | ❌ | ✅ Yes (waits) | ✅ (.errors) |
| Paused | ❌ | ❌ | ✅ Yes (waits) | ❌ |
| Cached | ✅ (cached) | ❌ | ❌ No | ❌ |
| Dirty | ❌ (invalid) | ❌ | ✅ Yes (needs refresh) | ❌ |

---

### B. Operator Precedence

Assignment operators have equal precedence, evaluated left-to-right:

```polyglot
[<] .a: Type << value1
[>] .b: Type ~> .var1
[<] .c: Type <~ default1
```

All three are independent assignments.

---

### C. Memory Layout (Informative)

Conceptual variable memory layout:

```
Variable {
  state: enum Variables.States  (4 bytes)
  override_count: u8             (1 byte)
  value: SerializedData          (variable size)
  errors: Array<ErrorObject>     (variable size)
  metadata: TypeInfo             (variable size)
}
```

**Note:** Actual layout is implementation-defined.

---

### D. Performance Considerations

#### State Checks
- Use fast-path for common case (Ready state)
- Branch prediction hints for error paths
- Inline state comparisons when possible

#### Waiting
- Use futex/condition variables for blocking
- Avoid busy-waiting
- Batch wake-ups for multiple waiters

#### Caching
- LRU eviction for Cached state
- Configurable TTL
- Memory limits

---

### E. Related Specifications

- [Pipeline States Specification](./pipeline-states-specification.md)
- [Error Handling Specification](./error-handling-specification.md)
- [Queue Control Specification](./queue-control-specification.md)
- [Serialization Specification](./serialization-specification.md)

---

### F. Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-11-24 | Initial specification based on brainstorming session 2025-11-23 |

---

## Glossary

- **Async-centric:** Architecture where async operations are the default, not exceptions
- **State transition:** Change from one variable state to another
- **Automatic waiting:** Runtime behavior where pipelines wait for Pending variables without explicit await
- **Override-once:** Semantic allowing exactly one modification before immutability
- **Reserved field:** Compiler-managed field available on all variables (`.state`, `.errors`)
- **Reserved enumeration:** Pre-defined enumeration that cannot be altered (`#Variables.States.*`)
- **Serialization tree:** Hierarchical representation of all Polyglot data as serialized strings
- **Dot notation:** Syntax for accessing nested fields (`.user.address.city`)
- **[i] block:** Input block where variables must be Ready before pipeline triggers

---

**End of Specification**

---

**For questions or clarifications, contact:**
- Language Design Team
- Runtime Implementation Team
- Compiler Team

**Document Maintained By:** Product Management (PM)
