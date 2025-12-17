# Architecture Impact Analysis: Variable States System

**Date:** 2025-11-24
**Analyst:** PM Agent (John)
**Status:** Awaiting Approval

---

## Executive Summary

Recent brainstorming sessions (2025-11-23) and technical documentation (2025-11-24) introduced a **complete variable state model** that represents a fundamental shift in Polyglot's core language semantics. The current architecture document (last updated 2025-11-17) predates this work and **does not reflect** the variable states system.

**Key Finding:** The variable states system is not an incremental feature—it's a foundational language characteristic that affects multiple architecture layers:
- IR representation (variable metadata)
- Runtime semantics (state transitions, automatic waiting)
- Database schema (state tracking, reserved fields)
- Compiler implementation (state analysis, operator handling)

**Recommendation:** Update architecture document to incorporate variable states system as a first-class architectural component.

---

## What Changed: Summary of Variable States Work

### 1. New Variable State Model (9 States)

**Core States (5):**
- `Declared` - Schema defined, no default value
- `DefaultReady` - Has default, allows ONE override
- `Pending` - Async operation in progress
- `Ready` - Value available, immutable
- `Faulted` - Operation failed

**Queue Management States (4):**
- `Retrying` - Automatic retry in progress
- `Paused` - Waiting for external trigger
- `Cached` - Cached result (may be stale)
- `Dirty` - Cache invalid, needs refresh

**Source:**
- Brainstorming session: `docs/brainstorming-session-results-2025-11-23.md`
- Technical spec: `docs/technical/variable-states-specification.md` v1.0.0

---

### 2. Three Assignment Operators with State Implications

| Operator | Purpose | Resulting State | Example |
|----------|---------|-----------------|---------|
| (none) | Schema-only | `Declared` | `[<] .field: Type` |
| `<~` / `~>` | Default | `DefaultReady` | `[<] .field: Type <~ value` |
| `<<` / `>>` | Constant/Async | `Ready` or `Pending` | `[<] .field: Type << value` |

**Impact:** Operators are not just syntax—they control state lifecycle and mutability semantics.

**Source:**
- Brainstorming: Lines 118-161, 362-410
- Technical spec: Lines 118-238

---

### 3. Reserved Fields for All Variables

**`.state` field:**
- Type: `#Variables.States.*` (Reserved Enumeration)
- Purpose: Query current variable state
- Available on ALL variables (including nested fields)
- Example: `[?] .var.state =? #Variables.States.Ready`

**`.errors` field:**
- Type: `pg\array{!}` (Array of error objects)
- Purpose: Store error details when Faulted
- Compiler-managed

**Source:**
- Brainstorming: Lines 342-351
- Technical spec: Lines 302-374

---

### 4. Foundational Principles

**Four core principles defining Polyglot's async-centric nature:**

1. **Async-Centric by Design** - Async is the foundation, not a bolt-on
2. **State-Aware Coordination** - Variables transition through states (not "mutable vs immutable")
3. **Serialization Foundation** - ALL data is serialized for cross-language coordination
4. **Automatic Waiting** - Pipelines automatically wait for variables to be ready (no explicit `await`)

**Source:** Technical spec, Lines 43-85

---

## Architecture Gaps Identified

### Gap 1: No Variable State Model Documentation

**Current State:** Architecture document mentions "IR type definitions" and "Rust structs + serde" but does NOT describe variable states.

**Missing Content:**
- Variable state lifecycle diagrams
- State transition rules
- Reserved fields (`.state`, `.errors`)
- Assignment operator semantics

**Impact:** Implementers don't know:
- How to represent variable state in IR
- When state transitions occur
- How to implement automatic waiting
- What metadata to track

**Recommendation:** Add new section: **"Variable State Model"** under "Data Architecture"

---

### Gap 2: IR Structure Doesn't Account for Variable Metadata

**Current State:** IR structure shows 3 separate IRs (Trigger, Queue, Runner) but doesn't specify how variable states are represented.

**Missing Content:**
- Variable metadata in IR (state, override_count, errors)
- Reserved field serialization
- State tracking across pipeline boundaries

**Example from spec (Lines 786-791):**
```
Variable {
  state: enum Variables.States  (4 bytes)
  override_count: u8             (1 byte)
  value: SerializedData          (variable size)
  errors: Array<ErrorObject>     (variable size)
  metadata: TypeInfo             (variable size)
}
```

**Recommendation:** Update "IR Structure" section to show variable metadata representation

---

### Gap 3: Database Schema Missing Variable State Tracking

**Current State:** Database schema shows `pipeline_instances` table with `status` column but no variable state tracking.

**Missing Content:**
- How variable states are persisted
- Reserved fields storage (`.state`, `.errors`)
- State transition logging

**Questions to Address:**
- Are variable states stored in IR JSONB columns?
- Do we need a separate `variable_states` table?
- How are state transitions audited?

**Recommendation:** Clarify variable state persistence strategy in database schema

---

### Gap 4: Runtime Semantics Not Documented

**Current State:** Architecture mentions "async runtime" (Tokio) but doesn't explain automatic waiting behavior.

**Missing Content from spec (Lines 439-487):**
- **Automatic Waiting:** Pipelines wait for Pending variables
- **`[i]` Block Entry Semantics:** All inputs must be Ready before execution
- **DefaultReady Override Semantics:** Enforce single override constraint
- **Error Propagation:** How Faulted states propagate through pipeline chains

**Example (Lines 446-456):**
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

**Recommendation:** Add new section: **"Runtime Semantics"** describing state-aware coordination

---

### Gap 5: Compiler Requirements Not Specified

**Current State:** Architecture describes lexer and parser but doesn't specify compiler responsibilities for state management.

**Missing Content from spec (Lines 547-570):**
- **Type Checking:** Validate default values match field types
- **State Analysis:** Track state flow through pipeline graph
- **Optimization:** Eliminate redundant state checks, inline constants
- **Error Detection:** Detect second override attempts, warn on unused `.errors` fields

**Recommendation:** Add compiler requirements to "Implementation Patterns" section

---

### Gap 6: Reserved Enumerations Not Documented

**Current State:** Architecture doesn't mention Reserved Enumerations at all.

**Missing Content:**
- `#Variables.States.*` - 9 variable states
- `#Pipelines.States.*` - 10 pipeline states (from brainstorming)
- `#Boolean` - True/False (mentioned in brainstorming)
- Reserved vs user-defined enumeration distinction

**Example (Lines 383-395):**
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

**Recommendation:** Add section on Reserved Enumerations in language model

---

### Gap 7: Terminology: "Immutability" vs "State Transitions"

**Current State:** Architecture doesn't address variable mutability semantics.

**Missing Clarification:**
The brainstorming session (Lines 11-16, 59-93) revealed that "immutability" is the **wrong framing** for Polyglot:

> Traditional "immutable" thinking - Since Polyglot just passes values without manipulating them, variables seemed immutable... but this was the WRONG FRAMING.
>
> Key Insight: "Immutability" assumes a sync world where variables are "changed" vs "not changed." But in Polyglot's async-centric coordination model, variables don't get "changed" - they **transition through states** as async operations complete.

**Recommendation:** Add conceptual section explaining async-centric coordination vs traditional mutability

---

## Required Architecture Updates

### Priority 1: Add "Variable State Model" Section

**Location:** After "Data Architecture" → "IR Structure"

**Content to Include:**
1. **Overview:** Async-centric coordination, state transitions
2. **9 Variable States:** Table with descriptions, transitions
3. **Assignment Operators:** Three operators with state implications
4. **Reserved Fields:** `.state` and `.errors` specifications
5. **State Lifecycle Diagram:** Visual flow from Declared → Ready/Faulted
6. **Runtime Semantics:** Automatic waiting, `[i]` block behavior

**Estimated Length:** ~800-1000 lines (similar to Technical Specification length)

**References:**
- Technical spec: `docs/technical/variable-states-specification.md`
- User guide: `docs/user/language/variables-user-guide.md`
- Brainstorming: `docs/brainstorming-session-results-2025-11-23.md`

---

### Priority 2: Update "IR Structure" Section

**Current Location:** Lines 745-821 in architecture.md

**Changes Needed:**
1. Add variable metadata to IR JSON examples
2. Show how `.state` and `.errors` fields are serialized
3. Document state tracking across Trigger/Queue/Runner IRs

**Example Addition:**
```json
{
  "runner_ir": {
    "steps": [...],
    "variables": {
      ".user_data": {
        "type": "#UserProfile",
        "state": "Pending",
        "override_count": 0,
        "errors": []
      }
    }
  }
}
```

---

### Priority 3: Update Database Schema

**Current Location:** Lines 637-728 in architecture.md

**Decision Required:** How to persist variable states?

**Option A: Store in IR JSONB columns**
- ✅ Simpler schema (no new tables)
- ✅ Variable state is part of IR
- ❌ Harder to query variable states across pipelines

**Option B: Add `variable_states` table**
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
```
- ✅ Easy to query variable states
- ✅ Separate concern from IR
- ❌ More complex schema
- ❌ Need to keep in sync with IR

**Recommendation:** Start with Option A (IR JSONB), migrate to Option B if querying becomes critical.

---

### Priority 4: Add "Runtime Semantics" Section

**Location:** After "Data Architecture"

**Content to Include:**
1. **Automatic Waiting:** How pipelines block on Pending variables
2. **`[i]` Block Entry:** Ready state enforcement
3. **DefaultReady Override:** Single override constraint
4. **Error Propagation:** Faulted state handling
5. **Blocking Mechanism:** Non-busy wait, timeout support

**Source:** Technical spec, Lines 439-544

---

### Priority 5: Update "Implementation Patterns"

**Current Location:** Lines 342-634 in architecture.md

**Additions Needed:**

**Compiler Patterns:**
- State analysis pattern
- Type checking with defaults pattern
- Override detection pattern

**Runtime Patterns:**
- Automatic waiting implementation
- State transition atomicity
- Error propagation pattern

**Example Code:**
```rust
// State transition pattern
async fn transition_to_ready(var: &mut Variable, value: Value) -> Result<()> {
    match var.state {
        VariableState::Pending => {
            var.state = VariableState::Ready;
            var.value = Some(value);
            Ok(())
        }
        VariableState::Declared => {
            Err(RuntimeError::InvalidTransition { from: "Declared", to: "Ready" })
        }
        _ => Err(RuntimeError::AlreadyReady)
    }
}
```

---

### Priority 6: Add Reserved Enumerations

**Location:** New section under "Technology Stack Details"

**Content:**
1. **Definition:** Pre-compiled constants, immutable, globally available
2. **`#Variables.States.*`** - 9 states
3. **`#Pipelines.States.*`** - 10 states (if finalized)
4. **`#Boolean`** - True/False
5. **Reserved vs User-Defined:** Distinction and constraints

**Source:** Technical spec, Lines 377-402

---

### Priority 7: Add Conceptual Section on Async-Centric Design

**Location:** Near beginning, after "Executive Summary"

**Purpose:** Frame Polyglot's unique approach to variable semantics

**Key Points:**
- Traditional languages: "mutable vs immutable"
- Polyglot: "state-aware async coordination"
- Variables transition through states (not "changed")
- Immutability is a **consequence**, not the design goal
- Automatic waiting = no explicit `await`

**Quote from brainstorming (Lines 59-63):**
> "Immutability" assumes a sync world where variables are "changed" vs "not changed." But in Polyglot's async-centric coordination model, variables don't get "changed" - they **transition through states** as async operations complete.

---

## Implementation Notes for AI Agents

**Critical:** Variable states system affects multiple Epic 1 stories:

**Story 1.2 (Lexer Token Definitions):**
- ✅ Already completed, operators defined

**Story 1.3 (Lexer Implementation):**
- ✅ Logos handles operators correctly

**Story 1.4 (Parser Type Definitions):**
- ⚠️ Must include variable state metadata in AST
- ⚠️ Assignment operators must be parsed with state intent

**Story 1.5 (Parser Implementation):**
- ⚠️ Parser must validate operator usage (schema-only vs default vs constant)
- ⚠️ Reserved fields (`.state`, `.errors`) must be recognized

**Epic 2 (IR Generation - Future):**
- 🔴 IR must represent variable states
- 🔴 State transition rules must be encoded
- 🔴 Reserved fields must be in IR

**Epic 3 (CLI Tool - Future):**
- 🔴 `polyglot compile` must validate state transitions
- 🔴 Error messages must reference states

---

## Approval Checklist

Before proceeding with architecture updates, please confirm:

- [ ] **Variable state model is accurate** - Does the 9-state model match your vision?
- [ ] **Assignment operators are correct** - Three operators with state implications?
- [ ] **Reserved fields are final** - `.state` and `.errors` as specified?
- [ ] **Database persistence strategy** - Option A (IR JSONB) vs Option B (separate table)?
- [ ] **Priority order is acceptable** - Should any section be done first?
- [ ] **Terminology change approved** - "State-aware coordination" vs "immutability"?

---

## Recommended Next Steps

After approval:

1. **Update architecture.md** with 7 priority changes (estimated 2-3 hours)
2. **Create architecture diagrams** - Variable state lifecycle, state transitions
3. **Review with implementation team** - Validate feasibility
4. **Update Epic 1 stories** - Add state-aware acceptance criteria
5. **Document state handling patterns** - For AI agent consistency

---

## Questions for Clarification

1. **Pipeline States:** Brainstorming mentions 10 pipeline states. Should these be in architecture now, or wait until Epic 2?

2. **Queue Management States:** The 4 queue states (Retrying, Paused, Cached, Dirty) are marked for "Future" in brainstorming. Should architecture include them as "post-MVP" features?

3. **State Transition Logging:** Should state transitions be logged to `execution_logs` table, or separate audit trail?

4. **Performance Implications:** State checking on every variable access. Should we add performance notes for state lookups?

5. **Partial/Streaming State:** Brainstorming lists this as "under consideration." Should architecture mention it as "future exploration"?

---

## References

**Primary Sources:**
- `docs/brainstorming-session-results-2025-11-23.md` - Variable state model design
- `docs/technical/variable-states-specification.md` - Technical specification v1.0.0
- `docs/user/language/variables-user-guide.md` - User-facing documentation
- `docs/technical/architecture.md` - Current architecture (2025-11-17)

**Related Work:**
- PRD: `docs/project/prd.md` - Product requirements
- Workflow Status: `docs/project/bmm-workflow-status.yaml` - Project progress

---

**Report prepared by:** PM Agent (John)
**Date:** 2025-11-24
**Status:** ⚠️ AWAITING YOUR APPROVAL ⚠️
