# Variable States: Technical Specification

**Version:** 1.2.0
**Last Updated:** 2025-12-03
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
- Variables transition through 6 core states: Pending → Default/Final/Faulted → Cleared
- Once Final, variables are immutable (consequence, not design goal)
- States are queryable via `.state` field
- Compiler/runtime manage transitions automatically
- All variables end in Cleared state when scope ends

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
- `[i]` blocks expect Final variables
- Runtime handles blocking/unblocking

---

## Complete State Model

Polyglot variables have **10 distinct states** organized into two categories:

### Core States (6)

States required for basic variable lifecycle:

| State | Description | Can Read Value? | Mutable? | Triggers Pipeline? |
|-------|-------------|----------------|----------|-------------------|
| **Pending** | Declared without value, awaiting push | ❌ No | N/A | ❌ No ([i] requires Final) |
| **Default** | Has default push, allows ONE override push | ✅ Yes | ⚠️ Once | ✅ Yes |
| **Final** | Value available, immutable | ✅ Yes | ❌ No | ✅ Yes |
| **Faulted** | Operation failed, has error info | ❌ No* | ❌ No | ❌ No (error path) |
| **Cleared** | Scope ended, memory freed | ❌ No | ❌ No | ❌ No (terminal state) |

*Can read `.errors` field

**Note:** When a variable is declared without value (`.var: type`), it starts in **Pending** state (awaiting push). When declared with push/pull operators (`<<`, `>>`, `<~`, `~>`), it transitions directly to the appropriate state (Default, Final, or Pending if async).

### Queue Management States (4)

Advanced states for resilience and performance:

| State | Description | Purpose | Transition |
|-------|-------------|---------|------------|
| **Retrying** | Automatic retry attempt in progress | Transient failure recovery | → Final/Faulted |
| **Paused** | Waiting for external trigger | Human approval, scheduled events | → Pending (when triggered) |
| **Cached** | Cached result, may be stale | Performance optimization | → Dirty (on invalidation) |
| **Dirty** | Cache invalid, needs refresh | Cache invalidation | → Pending (on refresh) |

---

## Assignment Operators

Polyglot has **three bidirectional operator pairs** for variable assignment:

### Operator Summary

| Operator Pair | Direction | Purpose | Resulting State |
|---------------|-----------|---------|-----------------|
| None | N/A | Declaration without value (awaits push) | Pending |
| `<~` / `~>` | Bidirectional | Default push (override once) | Default |
| `<<` / `>>` | Bidirectional | Normal push (sync or async) | Final or Pending (if async) |

**Key Points:**
- **Declaration without operator** (`.var: type`) → **Pending** state (awaiting push)
- **Declaration with `<~` or `~>`** (`.var: type <~ value` or `.var: type ~> .source`) → **Default** state (default push)
- **Declaration with `<<` or `>>`** (`.var: type << value` or `.var: type >> .source`) → **Final** (sync push) or **Pending** (async push)
- **All variables** → **Cleared** state at end of scope (via `|W.Polyglot.Scope`)

---

### 1. Declaration Without Value (No Operator)

**Syntax:**
```polyglot
[<] .field: Type
```

**Semantics:**
- Declares field schema without value
- No default provided
- Field starts in **Pending** state
- Must receive push before use
- Awaits push via `<<` or `>>` operators (normal push)

**Use Cases:**
- Fields populated by pipelines
- Required parameters that will be assigned later
- Data from external sources

**Example:**
```polyglot
[#] UserProfile
[<] .id: pg\string          # Pending (awaiting push)
[<] .name: pg\string        # Pending (awaiting push)
[<] .email: pg\string       # Pending (awaiting push)
[X]

[r] .user: #UserProfile << #UserProfile
[r] .user.id << "user-123"      # Normal push: Pending → Final
[r] .user.name << "John Doe"    # Normal push: Pending → Final
[r] .user.email << "john@example.com"  # Normal push: Pending → Final
```

---

### 2. Default Push `<~` / `~>`

**Syntax:**
```polyglot
[<] .field: Type <~ default_value    # Default push left
[>] .field: Type ~> .var             # Default push right
```

**Semantics:**
- Provides default value via default push
- Field starts in **Default** state
- Allows **ONE override push** during instantiation
- After first use or override push → transitions to **Final**
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
# After this, .config2.timeout is immutable (Final state)
```

**Use Cases:**
- Configuration with sensible defaults
- Optional parameters
- Fallback values

**Implementation Requirements:**
- Track override count per field
- Enforce single override constraint
- Transition Default → Final after override or first use
- Default application at `[i]` block entry

---

### 3. Normal Push `<<` / `>>`

**Syntax:**
```polyglot
[<] .field: Type << value   # Normal push left
[>] .field: Type >> .var    # Normal push right
```

**Semantics:**

#### Synchronous Push (`<<` or `>>` with literal/Final value)
- Immediate **Final** state
- Value is immutable
- No more pushes accepted

#### Asynchronous Push (`<<` or `>>` with Pending source)
- Variable starts in **Pending** state
- Transitions to **Final** or **Faulted** when operation completes
- Immutable once Final

**Use Cases:**
- Sync push: Literals, Final variables, pure functions
- Async push: Pipeline outputs, API responses, external operations

**Example:**
```polyglot
[#] AppInfo
[<] .version: pg\string << "1.0.0"    # Sync push: Final immediately
[X]

[r] |FetchData
[>] .result: pg\string >> .data       # Async push: Pending → Final/Faulted
```

---

## State Lifecycle

### Basic Lifecycle Flow

```
VARIABLE DECLARATION
         |
         ↓
    ┌────┴────┬──────────┬──────────┐
    |         |          |          |
 No Value  Default   Constant    Async
  (type)    <~ ~>     << value  Pipeline >>
    |         |          |          |
    ↓         ↓          ↓          ↓
 Pending  Default Final    Pending
(awaiting) (override-1) (immut)  (async wait)
    |         |          |          |
    ↓         ↓          |          ↓
Assignment [i] block     |      Final/Faulted
<< or >>  (defaults      |          |
    |     applied)       |          |
    ↓         |          |          |
    |    ┌────┴────┐     |          |
    |    |         |     |          |
    |    ↓         ↓     |          |
    | Override   Use     |          |
    |   Once    Default  |          |
    |    |      |        |          |
    └────┴──────┴────────┴──────────┘
              |
              ↓
       Final (immutable)
              |
              ↓
      [Scope Ends - Pipeline [X]]
              |
              ↓
       |W.Polyglot.Scope Cleanup
              |
              ↓
         Cleared
      (memory freed)
```

### Extended Lifecycle with Queue States

```
Pending ─────────┐
                 │
Default ────┤
                 │
                 ↓
             Pending ──────→ Final (success)
                 ↓              ↑
                 ↓              │
            Faulted ─→ Retrying ┘
                 │         ↑
                 │         │
            [Retry Logic]  │
                 │         │
                 └─────────┘

Final ──────→ Cached ──────→ Dirty ──────→ Pending
(result)   (performance) (invalidated)  (refresh)
   │
   ↓
Paused ──────→ [External Trigger] ──────→ Pending
(waiting)      (human approval, etc.)     (resume)

All Paths Eventually Lead To:
   │
   ↓
[Pipeline Ends]
   │
   ↓
Cleared (scope cleanup, memory freed)
```

---

### Variable Scope and Memory Cleanup

**Complete Variable Lifecycle (Birth to Death):**

```
DECLARATION (Birth)
      ↓
   [State Transitions: Pending/Default → ... → Final/Faulted]
      ↓
PIPELINE END (Death)
      ↓
SCOPE CLEANUP via |W.Polyglot.Scope
      ↓
   Cleared State
      ↓
MEMORY FREED
```

**Scope Rules:**
1. **Variables are born** when declared (`[i]`, `[r]`, `[o]`) - start in **Pending** or **Default** state
2. **Variables transition** through states during pipeline execution
3. **Variables die** when pipeline ends (reaches `[X]`)
4. **Variables enter Cleared state** via `|W.Polyglot.Scope` wrapper
5. **Memory is freed** immediately, variable no longer accessible

---

### The `|W.Polyglot.Scope` Wrapper

**Purpose:** Automatic memory management and resource cleanup

**Behavior:**
- **Implicitly present** in all pipelines (even when not written)
- **Manages variable lifecycle** from declaration to cleanup
- **Transitions all variables to Cleared state** when pipeline ends
- **Frees memory** when variables go out of scope (pipeline ends)
- **Handles cleanup** for ALL variable states (Final, Faulted, Pending, etc.)

**Example:**
```polyglot
[|] ProcessData
[i] .input: pg\string
[t] |T.Call
// [W] |W.Polyglot.Scope ← IMPLICIT!

// Variable lifecycle:
[r] .temp1: pg\string << .input + "A"    // .temp1 BORN (Pending → Final)
[r] .temp2: pg\string << .temp1 + "B"    // .temp2 BORN (Pending → Final)
[r] .result: pg\string << .temp2 + "C"   // .result BORN (Pending → Final)

[o] .result: pg\string
[X]
// Pipeline ends - |W.Polyglot.Scope cleanup:
// 1. .input → Cleared (freed)
// 2. .temp1 → Cleared (freed)
// 3. .temp2 → Cleared (freed)
// 4. .result → Cleared (freed)
```

---

### Nested Pipeline Scopes

**Each pipeline creates its own scope:**

```polyglot
[|] OuterPipeline
[i] .outer_input: pg\string
[t] |T.Call
// Outer scope starts

[r] .outer_temp: pg\string << .outer_input + "X"

// Call inner pipeline
[r] |InnerPipeline
[<] .data: pg\string << .outer_temp
[>] .processed: pg\string >> .inner_result

[o] .inner_result: pg\string
[X]
// Outer scope cleanup:
// - .outer_input freed
// - .outer_temp freed
// - .inner_result freed

[|] InnerPipeline
[i] .data: pg\string
[t] |T.Call
// Inner scope starts (independent of outer!)

[r] .inner_temp: pg\string << .data + "Y"
[o] .inner_temp: pg\string
[X]
// Inner scope cleanup (happens BEFORE returning to outer):
// - .data freed
// - .inner_temp freed
```

**Key Point:** Variables in nested pipelines are cleaned up **immediately** when that pipeline ends, before control returns to the calling pipeline.

---

### State Transitions After Scope End

**Important:** Once a pipeline ends and scope cleanup occurs, all variables transition to **Cleared** state and are then freed. No further state transitions are possible after cleanup.

```
Final ──→ [Pipeline Ends] ──→ [Scope Cleanup] ──→ Cleared ──→ FREED (memory released)
Faulted ──→ [Pipeline Ends] ──→ [Scope Cleanup] ──→ Cleared ──→ FREED (memory released)
Any State ──→ [Pipeline Ends] ──→ [Scope Cleanup] ──→ Cleared ──→ FREED (memory released)
```

**This is the terminal state:** Cleared is the final state before memory deallocation. All variables, regardless of their state during execution, transition to Cleared when the pipeline scope ends.

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
[?] .var.state =? #Variables.States.Final
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
- Empty array when Final
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

### `#Variables.States.*` or `#PgVar.States.*`

Complete enumeration of variable states:

```polyglot
[#] Variables.States  // Also aliased as #PgVar.States
[<] .Pending: pg\string << "Pending"
[<] .Default: pg\string << "Default"
[<] .Final: pg\string << "Final"
[<] .Faulted: pg\string << "Faulted"
[<] .Cleared: pg\string << "Cleared"
[<] .Retrying: pg\string << "Retrying"
[<] .Paused: pg\string << "Paused"
[<] .Cached: pg\string << "Cached"
[<] .Dirty: pg\string << "Dirty"
[X]
```

**Core States (6):**
- **Pending**: Declared without value, awaiting assignment
- **Default**: Has default value, allows one override
- **Final**: Value available, immutable
- **Faulted**: Operation failed, has error info
- **Cleared**: Scope ended, memory freed (terminal state)

**Queue Management States (4):**
- **Retrying**: Automatic retry in progress
- **Paused**: Waiting for external trigger
- **Cached**: Cached result
- **Dirty**: Cache invalidated

**Implementation Requirements:**
- Pre-compile time constants
- Immutable
- Available globally
- Type-safe comparisons
- Cleared state is terminal (cannot transition from Cleared to any other state)

---

## State Transitions

### Valid Transitions

| From State | To State(s) | Trigger | Notes |
|------------|-------------|---------|-------|
| Pending | Final | Direct assignment `<<` or `>>` | Value assigned |
| Pending | Faulted | Pipeline failure | Error occurred |
| Pending | Retrying | Transient failure | Auto-retry triggered |
| Default | Pending | Override with async `>>` | Override with pipeline result |
| Default | Final | First use or override `<<` | Default used or overridden once |
| Final | Cached | Cache enabled | Result cached |
| Final | Cleared | Pipeline ends `[X]` | Scope cleanup |
| Faulted | Retrying | Retry attempt | Manual or auto retry |
| Faulted | Cleared | Pipeline ends `[X]` | Scope cleanup |
| Retrying | Final | Retry success | Operation succeeded |
| Retrying | Faulted | Retry exhausted | All retries failed |
| Cached | Dirty | Invalidation event | Cache invalidated |
| Cached | Cleared | Pipeline ends `[X]` | Scope cleanup |
| Dirty | Pending | Refresh triggered | Re-fetch data |
| Paused | Pending | Trigger received | Resume execution |
| **Any State** | **Cleared** | **Pipeline ends `[X]`** | **Scope cleanup (final state)** |

### Invalid Transitions

**Forbidden transitions (compiler/runtime must prevent):**

- Final → Pending (cannot make Final async again)
- Faulted → Final (must go through Retrying)
- Any state → Default (Default is initial state only)
- **Cleared → Any state (Cleared is terminal, memory freed)**

---

## Runtime Semantics

### Automatic Waiting Behavior

**Rule:** Pipelines automatically wait for Pending variables.

**Implementation:**
```
When pipeline P references variable V:
  IF V.state == Pending:
    Block P until V.state ∈ {Final, Faulted}
  ELSE IF V.state ∈ {Final, Default, Cached}:
    Proceed with pipeline execution
  ELSE IF V.state == Faulted:
    Propagate error to error handler
  ELSE IF V.state == Cleared:
    Throw RuntimeError: "Variable accessed after scope cleanup"
```

**Blocking Mechanism:**
- Non-busy wait (yield CPU)
- Timeout support (configurable)
- Cancellation support (abort pipeline)

---

### `[i]` Block Entry Semantics

**Rule:** All `[i]` variables MUST be Final (or Default) before pipeline triggers.

**Implementation:**
```
On pipeline trigger:
  FOR EACH variable V in [i] block:
    IF V.state == Default:
      Apply default value
      Transition V.state to Final
    ELSE IF V.state == Pending:
      Wait until V.state ∈ {Final, Faulted}
      IF after wait, still Pending:
        Throw RuntimeError: "[i] variable not ready (timeout or deadlock)"
    ELSE IF V.state == Faulted:
      Invoke error handler
    ELSE IF V.state == Final:
      Proceed

  IF ALL [i] variables Final:
    Execute pipeline body
  ELSE:
    Abort pipeline
```

---

### Default Override Semantics

**Rule:** Default fields allow ONE override before becoming immutable.

**Implementation:**
```
Enumeration field F with default D:

Instantiation without override:
  F.value = D
  F.state = Default

  On first read:
    F.state = Final
    F.value remains D (immutable)

Instantiation with override O:
  F.value = O
  F.override_count = 1
  F.state = Final  (immediate transition)
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
- Optimize away Default → Final transitions when no override

#### 4. Error Detection
- Detect second override attempts on Default fields
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
- Free Pending variable resources on Final/Faulted
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
- Current value (if Final)

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
[?] .user_data.address.city.state =? #Variables.States.Final
```

**Resolution:** YES, all serialized fields have `.state` introspection.

**Implementation:**
- Nested fields inherit parent state initially
- Can have independent state after assignment
- `.user_data.address.state` reflects inner enumeration state

---

### Edge Case 2: Default at `[i]` Block

**Scenario:** Default variable at pipeline entry

```polyglot
[#] Config
[<] .timeout: pg\int <~ 30
[X]

[|] MyPipeline
[i] .config: #Config << #Config
# What is .config.timeout.state here?
```

**Resolution:** **Final** (default kicked in at `[i]`)

**Implementation:**
- At `[i]` block entry, Default → Final transition occurs
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

**Resolution:** Pipeline waits for Final/Faulted, then:
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

**Scenario:** Trying to override Default field twice

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

| State | Has Value? | Can Override? | Blocks Pipeline? | Error Info? | Terminal? |
|-------|-----------|---------------|------------------|-------------|-----------|
| Pending | ❌ | N/A (no value) | ✅ Yes (waits) | ❌ | ❌ |
| Default | ✅ (default) | ✅ (once) | ❌ No (Final-like) | ❌ | ❌ |
| Final | ✅ | ❌ | ❌ No | ❌ | ❌ |
| Faulted | ❌ | ❌ | ✅ Yes (error path) | ✅ (.errors) | ❌ |
| Cleared | ❌ | ❌ | ✅ Yes (freed) | ❌ | ✅ |
| Retrying | ❌ | ❌ | ✅ Yes (waits) | ✅ (.errors) | ❌ |
| Paused | ❌ | ❌ | ✅ Yes (waits) | ❌ | ❌ |
| Cached | ✅ (cached) | ❌ | ❌ No | ❌ | ❌ |
| Dirty | ❌ (invalid) | ❌ | ✅ Yes (needs refresh) | ❌ | ❌ |

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
- Use fast-path for common case (Final state)
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
| 1.1.0 | 2025-12-03 | **Major update**: Removed Declared state, added Cleared state, clarified 6-step lifecycle, documented ~> default pull operator, updated all state transitions to include Cleared as terminal state |
| 1.2.0 | 2025-12-03 | **Terminology update**: Renamed states to remove traditional sync programming associations: `Ready` → `Final`, `DefaultReady` → `Default`. Emphasizes async-centric, data-flow paradigm. |

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
- **[i] block:** Input block where variables must be Final before pipeline triggers

---

**End of Specification**

---

**For questions or clarifications, contact:**
- Language Design Team
- Runtime Implementation Team
- Compiler Team

**Document Maintained By:** Product Management (PM)
