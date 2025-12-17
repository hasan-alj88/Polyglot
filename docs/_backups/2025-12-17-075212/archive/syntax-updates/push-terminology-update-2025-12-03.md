# Push Terminology Update - Summary

**Date:** 2025-12-03
**Task:** Clarify distinction between operator duality and operand states
**Status:** ✅ **COMPLETE**

---

## Key Clarification

### Operators vs Operands

**Push/Pull Operators (Dual/Bidirectional):**
- The **operators** (`<<`, `>>`, `<~`, `~>`) have dual nature
- Can be read from BOTH perspectives simultaneously
- Example: `.x >> .y` = "x pushed into y" AND "y pulled from x"

**Variables/Operands (Specific States):**
- The **variables** (operands) are in specific states
- Variables **await push** (not "await assignment")
- Can await **normal push** or **default push**

---

## Terminology Changes

### ❌ Old (Incorrect):
- "Awaiting assignment"
- "Assignment operators"
- "Constant assignment"
- "Async assignment"

### ✅ New (Correct):
- **"Awaiting push"** (variables wait for push operations)
- **"Normal push"** (`<<` or `>>` - standard push operation)
- **"Default push"** (`<~` or `~>` - default value push)
- **"Sync push"** (immediate Final)
- **"Async push"** (Pending → Final when complete)

---

## Updated Terminology

### State Descriptions

| State | Old Description | New Description |
|-------|----------------|-----------------|
| Pending | "Awaiting assignment" | **"Awaiting push"** (normal or async) |
| Default | "Has default value" | **"Has default push"**, can override with push |
| Final | "Value available" | **"Value available, no more pushes"** |

### Operator Types

| Operators | Old Name | New Name |
|-----------|----------|----------|
| `<<` / `>>` | "Constant/Async Assignment" | **"Normal Push"** (sync or async) |
| `<~` / `~>` | "Default Assignment" | **"Default Push"** (override once) |

---

## Examples Updated

### Example 1: Pending State (Awaiting Push)

**Before:**
```polyglot
[r] .temp: pg\int  // Pending (awaiting assignment)
```

**After:**
```polyglot
[r] .temp: pg\int  // Pending (awaiting push)
```

### Example 2: Normal Push

**Before:**
```polyglot
[r] .x << 42  // Constant assignment
```

**After:**
```polyglot
[r] .x << 42  // Normal push (sync): Pending → Final
```

### Example 3: Default Push

**Before:**
```polyglot
[i] .timeout <~ 30  // Default assignment
```

**After:**
```polyglot
[i] .timeout <~ 30  // Default push: Creates Default
```

### Example 4: Async Push

**Before:**
```polyglot
[r] .data << py\fetch()  // Async assignment
```

**After:**
```polyglot
[r] .data << py\fetch()  // Normal push (async): Pending state
```

---

## Push Types Classification

### Normal Push (`<<` or `>>`)

**Two subtypes:**

1. **Sync Push** - Immediate Final
   ```polyglot
   [r] .x << 42                    // Sync push: literal value
   [r] .y << .x                    // Sync push: Final variable
   ```

2. **Async Push** - Pending → Final/Faulted
   ```polyglot
   [r] .data << py\fetch()         // Async push: operation
   [r] .result >> .pipeline_output // Async push: pipeline result
   ```

### Default Push (`<~` or `~>`)

Creates **Default** state, allows one override push:

```polyglot
[i] .timeout <~ 30           // Default push (literal)
[i] .config ~> .default      // Default push (from source)

[r] .timeout << 60           // Override push: Default → Final
```

---

## State Lifecycle with Push Operations

```
DECLARATION
     │
     ├─ No operator        → Pending (awaiting push)
     ├─ Default push <~ ~> → Default (can override with push)
     └─ Normal push << >>  → Final (sync) or Pending (async)

PUSH OPERATIONS
Pending      ──(normal push)────> Final
Pending      ──(async push fail)─> Faulted
Default ──(override push)──> Final

SCOPE END
Any State    ──([X] cleanup)────> Cleared
```

---

## Files Updated

### Technical Documentation

**`docs/technical/variable-states-specification.md`:**
- Core states table: "Awaiting assignment" → "Awaiting push"
- Section 1: "Declaration Without Value" → uses "awaiting push"
- Section 2: "Default Assignment" → "Default Push"
- Section 3: "Constant/Async Assignment" → "Normal Push"
- Operator summary table: Updated with push terminology
- All examples updated

### User Documentation

**`docs/user/variable-state-system.md`:**
- Six core states list updated
- State 1: Pending description uses "awaiting push"
- State 2: Default uses "default push"
- All examples clarified with "normal push" or "default push"

---

## Consistency Rules

### When to Use Each Term:

**"Push" (general):**
- Referring to the operation itself
- "Variables await push"
- "Push operations trigger state transitions"

**"Normal Push":**
- `<<` or `>>` operators
- Standard push operation
- Can be sync or async

**"Default Push":**
- `<~` or `~>` operators
- Creates Default state
- Can override once

**"Sync Push":**
- Normal push with immediate Final state
- Literals, Final variables

**"Async Push":**
- Normal push that creates/maintains Pending
- Operations, pipelines, external calls

**"Override Push":**
- Push to Default variable
- Transitions to Final

---

## Benefits of Precise Terminology

### 1. Clear Distinction

**Operators** (dual/bidirectional):
- `<<` = Push/Pull Left
- `>>` = Push/Pull Right

**Operations** (specific):
- Normal push
- Default push
- Override push

### 2. Aligned with Philosophy

The terminology now matches Polyglot's data-flow philosophy:
- Not "assignment" (storage-centric)
- But "push" (flow-centric)

### 3. Precise State Descriptions

Variables are in specific states:
- "Awaiting push" (clear expectation)
- "Has default push" (clear source)
- "No more pushes" (clear constraint)

---

## Summary Table

| Concept | Correct Term | Usage |
|---------|-------------|--------|
| Operators | Push/Pull Left/Right | Dual, bidirectional |
| Operation (general) | Push | "Variables await push" |
| `<<` / `>>` | Normal Push | Sync or async |
| `<~` / `~>` | Default Push | Override once |
| Literal/Final | Sync Push | Immediate Final |
| Pending source | Async Push | Maintains/creates Pending |
| To Default | Override Push | → Final |
| Variable state | Awaiting Push | Not "awaiting assignment" |

---

## Validation Checklist

- [x] "Awaiting assignment" → "Awaiting push" (all instances)
- [x] "Assignment operators" → "Push operators" (all instances)
- [x] "Constant/Async assignment" → "Normal push (sync/async)"
- [x] "Default assignment" → "Default push"
- [x] Examples clarify push type (normal/default/sync/async)
- [x] State descriptions use push terminology
- [x] Operator sections use push terminology
- [x] Technical spec updated
- [x] User guide updated

---

## Conclusion

**Status:** ✅ **COMPLETE**

Documentation now consistently distinguishes:

1. **Operators** = Dual/bidirectional (can read both ways)
2. **Operands** = Specific states (await specific push types)
3. **Push types** = Normal, Default, Sync, Async, Override

This aligns terminology with Polyglot's data-flow philosophy and provides precise language for describing variable states and operations.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Philosophy:** Operators are dual; operands await push
