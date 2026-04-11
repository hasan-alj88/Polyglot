---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/user/concepts/collections.md
---
<!-- @d:user/concepts/collections -->
> **Deprecated:** This document is superseded. See the current spec for up-to-date content.

# Expansion Operator

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

<!-- @u:user/concepts/collections#expand-collect -->
The expansion operator `[~]` controls nesting and scope in Polyglot. It indicates that an operation runs WITHIN a parent context rather than at the same level. Understanding implicit vs explicit expansion is crucial for writing correct Polyglot code.

**Key Concepts:**
- Block element hierarchy (parent-child relationships)
- Implicit expansion (automatic)
- Explicit expansion (requires `[~]`)
- Nesting levels
- Scope rules
- Race condition prevention

---

## Table of Contents

1. [Block Element Hierarchy](#block-element-hierarchy)
2. [Implicit Expansion](#implicit-expansion)
3. [Explicit Expansion](#explicit-expansion)
4. [Nesting Levels](#nesting-levels)
5. [Scope Rules](#scope-rules)
6. [When to Use `[~]`](#when-to-use-)
7. [Parallel Blocks and `[~]`](#parallel-blocks-and-)
8. [Unpack Operations and `[~]`](#unpack-operations-and-)
9. [Race Condition Prevention](#race-condition-prevention)
10. [Best Practices](#best-practices)

---

## Block Element Hierarchy

### Parent-Child Relationships

<!-- @u:user/syntax/blocks#block-markers -->
All block elements have hierarchical relationships where certain elements are automatically children of others.

---

### Top-Level Parent: `[|]` Pipeline

**`[|]` is the parent of:**
- `[i]` Input declaration
- `[o]` Output declaration
- `[t]` Trigger
- `[Q]` Queue control
- `[r]` Run/operation
- `[p]` Parallel execution
- `[w]` Wrapper context
- `[\]` Setup block
- `[/]` Cleanup block

```polyglot
[|] MyPipeline          // Parent
[i] .input: pg\string   // Child - implicit expansion
[t] |T.Daily            // Child - implicit expansion
[r] |Operation          // Child - implicit expansion
[X]
```

---

### Operation Parents

**Any block with operation/pipeline call is parent of:**
- `[<]` Input assignment
- `[>]` Output assignment

```polyglot
[r] |SomeOperation      // Parent
[<] .input << value     // Child - implicit expansion
[>] .output >> result   // Child - implicit expansion
```

---

### Hierarchy Visualization

```
[|] Pipeline (Level 0)
├── [i] Input (Level 1, implicit)
├── [t] Trigger (Level 1, implicit)
│   ├── [<] Trigger param (Level 2, implicit)
│   └── [>] Trigger output (Level 2, implicit)
├── [r] Operation (Level 1, implicit)
│   ├── [<] Operation input (Level 2, implicit)
│   └── [>] Operation output (Level 2, implicit)
└── [p] Parallel (Level 1, implicit)
    ├── [<] Parallel input (Level 2, implicit)
    ├── [>] Parallel output (Level 2, implicit)
    └── [~][r] Nested operation (Level 2, EXPLICIT with [~])
        ├── [~][<] Nested input (Level 3, implicit from [~][r])
        └── [~][>] Nested output (Level 3, implicit from [~][r])
```

---

## Implicit Expansion

### What is Implicit Expansion?

**Implicit expansion** occurs automatically when a block element is the expected child of its parent. No `[~]` is needed.

---

### Example: Pipeline Children

```polyglot
[|] Pipeline
[i] .input: pg\string   // Implicit - expected child of [|]
[t] |T.Daily            // Implicit - expected child of [|]
[r] |Operation          // Implicit - expected child of [|]
[X]
```

**No `[~]` needed** - these are all direct children of `[|]`.

---

### Example: Operation Children

```polyglot
[r] |ProcessData
[<] .input: pg\string << "value"    // Implicit - child of [r]
[<] .size: pg\int << 1024           // Implicit - child of [r]
[>] .result: pg\string >> output    // Implicit - child of [r]
```

**No `[~]` needed** - `[<]` and `[>]` are expected children of `[r]`.

---

### Why Implicit Expansion Exists

**Benefits:**
- **Cleaner syntax** - Less visual clutter
- **Readability** - Clear parent-child relationships
- **Less typing** - Common patterns don't need `[~]`

**When it works:**
- Parent-child relationship is defined in language
- Child is direct descendant (not nested)

---

## Explicit Expansion

### What is Explicit Expansion?

**Explicit expansion** with `[~]` indicates an operation runs WITHIN a parent context, not just as a sibling.

---

### When Explicit Expansion is Required

Use `[~]` when:
1. Nesting operations inside parallel blocks
2. Nesting operations inside unpack iterations
3. Adding operations within expanded contexts
4. Any operation that runs WITHIN (not AFTER) parent

---

### Example: Operations Inside Parallel

```polyglot
[p] |ParallelBlock
[<] .data: pg\string << input
[~][r] |NestedOperation         // [~] means: runs WITHIN parallel block
[~][<] .input: pg\string << .data  // Implicit child of [~][r]
[~][>] .result >> temp             // Implicit child of [~][r]
[>] .output >> result              // Direct child of [p] - no [~]
```

**Why `[~]` is needed:**
- `[~][r]` runs WITHIN `[p]` block
- Without `[~]`, it would run AFTER `[p]` completes

---

### Example: Operations Inside Unpack

```polyglot
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}

[r] ~.items                         // Unpack array
[~][r] |ProcessItem                 // [~] means: runs WITHIN iteration
[~][<] .item: pg\string << .current_item  // Implicit child of [~][r]
```

**Why `[~]` is needed:**
- `[~][r]` runs for EACH item (within iteration)
- Without `[~]`, it would run once after iteration completes

---

### Explicit vs Implicit Summary

```polyglot
// Implicit - direct child
[r] |Operation
[<] .input << value     // Implicit

// Explicit - nested within
[p] |Parallel
[~][r] |Operation       // Explicit - WITHIN parallel
[~][<] .input << value  // Implicit - child of [~][r]
```

---

## Nesting Levels

### Counting Nesting Depth

Each `[~]` adds one level of nesting:
- No `[~]` = Level 0 (same level as parent)
- `[~]` = Level 1 (one level deep)
- `[~][~]` = Level 2 (two levels deep)
- `[~][~][~]` = Level 3 (three levels deep)

---

### Single Level Nesting

```polyglot
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}

// Level 0 - outer scope
[r] ~.items

// Level 1 - within iteration
[~][r] |ProcessItem
[~][<] .item: pg\string << .current_item
```

---

### Two Level Nesting

```polyglot
[r] .matrix: pg\array{pg\array{pg\int}} << array{
[^]  array{1, 2, 3},
[^]  array{4, 5, 6}
[^]}

// Level 0 - outer scope
[r] ~.matrix

// Level 1 - iterate rows
[~][r] .row: pg\array{pg\int} << .current_row
[~][r] ~.row

// Level 2 - iterate columns within rows
[~][~][r] |ProcessCell
[~][~][<] .cell: pg\int << .current_cell
```

---

### Three Level Nesting

```polyglot
// Level 0
[r] .data: pg\array{pg\array{pg\array{pg\string}}}

[r] ~.data
// Level 1
[~][r] ~.current_2d

// Level 2
[~][~][r] ~.current_1d

// Level 3
[~][~][~][r] |ProcessElement
[~][~][~][<] .element: pg\string << .current_element
```

---

### Visual Nesting

```
Level 0: [r] Operation
Level 1: [~][r] Operation          (within Level 0)
Level 2: [~][~][r] Operation       (within Level 1)
Level 3: [~][~][~][r] Operation    (within Level 2)
```

---

## Scope Rules

### Scope in Sequential Operations

Sequential operations (`[r]`) have **direct access** to outer scope:

```polyglot
[r] .x: pg\int << 10

[r] |Operation1
[<] .value: pg\int << .x  // Direct access

[r] |Operation2
[<] .value: pg\int << .x  // Direct access
```

**No `[~]` needed** - sequential operations share scope.

---

### Scope in Parallel Operations

<!-- @u:user/concepts/pipelines#structure -->
Parallel operations (`[p]`) use **copy semantics**:

```polyglot
[r] .x: pg\int << 10

[p] |Parallel1
[<] .value: pg\int << .x  // COPY of .x

[p] |Parallel2
[<] .value: pg\int << .x  // COPY of .x (independent)
```

**No `[~]` needed for inputs** - but modifications don't affect outer scope.

---

### Scope in Nested Operations

Nested operations with `[~]` have access to **parent's scope**:

```polyglot
[p] |Parallel
[<] .shared_data: pg\string << input
[~][r] |NestedOperation
[~][<] .data: pg\string << .shared_data  // Access to parallel block's scope
```

---

### Scope Isolation

Each `[~]` level creates a scope boundary:

```polyglot
// Level 0
[r] .level0_var: pg\int << 0

[r] ~Array
// Level 1 - can access level0_var
[~][r] .level1_var: pg\int << .level0_var

[~][r] ~NestedArray
// Level 2 - can access both level0_var and level1_var
[~][~][r] .level2_var: pg\int << .level1_var + .level0_var
```

---

## When to Use `[~]`

### Use `[~]` For:

1. **Operations inside parallel blocks**
```polyglot
[p] |Parallel
[~][r] |NestedOp  // ✓ Needs [~]
```

2. **Operations inside unpack iterations**
```polyglot
[r] ~Array
[~][r] |ProcessItem  // ✓ Needs [~]
```

3. **Operations inside expanded contexts**
```polyglot
[r] ~SomeExpansion
[~][r] |NestedOp  // ✓ Needs [~]
```

---

### Don't Use `[~]` For:

1. **Direct children of pipeline**
```polyglot
[|] Pipeline
[r] |Operation  // ✗ No [~] - direct child
```

2. **Input/output of operations**
```polyglot
[r] |Operation
[<] .input << value  // ✗ No [~] - implicit child
```

3. **Top-level pipeline declarations**
```polyglot
[i] .input: pg\string  // ✗ No [~] - direct child of pipeline
[t] |T.Daily           // ✗ No [~] - direct child of pipeline
```

---

### Decision Flow

**Question:** Do I need `[~]`?

```
Is the operation running WITHIN a parent context?
├─ YES → Use [~]
│  Examples:
│  - Operation inside [p] block
│  - Operation inside unpack iteration
│  - Nested expansion
│
└─ NO → Don't use [~]
   Examples:
   - Direct child of [|] pipeline
   - Input/output of operation ([<], [>])
   - Sequential operations at same level
```

---

## Parallel Blocks and `[~]`

### Basic Parallel - No `[~]` Needed

```polyglot
[p] |Parallel
[<] .input << data     // No [~] - direct child
[>] .output >> result  // No [~] - direct child
```

---

### Operations Inside Parallel - `[~]` Required

```polyglot
[p] |Parallel
[<] .input << data
[~][r] |Transform         // [~] required - WITHIN parallel
[~][<] .data << .input    // No [~] - child of [~][r]
[~][>] .result >> temp    // No [~] - child of [~][r]
[>] .output >> result
```

---

### Multiple Nested Operations

```polyglot
[p] |Parallel
[<] .input << data
[~][r] |Step1             // [~] - WITHIN parallel
[~][<] .in << .input      // Implicit - child of [~][r]
[~][>] .out >> temp1      // Implicit - child of [~][r]

[~][r] |Step2             // [~] - WITHIN parallel
[~][<] .in << temp1       // Implicit - child of [~][r]
[~][>] .out >> temp2      // Implicit - child of [~][r]

[>] .output >> result     // No [~] - direct child of [p]
```

---

### Nested Parallel Blocks

```polyglot
[p] |OuterParallel
[<] .input << data
[~][r] |Prepare
[~][<] .data << .input

// Nested parallel blocks
[~][p] |InnerParallel1         // [~] - WITHIN outer parallel
[~][<] .in << prepared_data

[~][p] |InnerParallel2         // [~] - WITHIN outer parallel
[~][<] .in << prepared_data

[~][Y] |Y.Join                 // [~] - WITHIN outer parallel
[~][>] inner_result1
[~][>] inner_result2

[>] .output >> outer_result
```

---

## Unpack Operations and `[~]`

### Basic Unpack

```polyglot
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}

[r] ~.items                     // Unpack
[~][r] |ProcessItem             // [~] - WITHIN iteration
[~][<] .item << .current_item   // Implicit - child of [~][r]
```

---

### Nested Unpack

```polyglot
[r] .matrix: pg\array{pg\array{pg\int}}

[r] ~.matrix                    // Level 1
[~][r] .row: pg\array{pg\int} << .current_row
[~][r] ~.row                    // Level 2 - unpack within unpack

[~][~][r] |ProcessCell          // [~][~] - two levels deep
[~][~][<] .cell << .current_cell
```

---

### Unpack with Standard Library

```polyglot
[r] ~Array.ForEach              // Unpack operation
[~][r] |ProcessElement          // [~] - WITHIN foreach
[~][<] .element << .current
```

---

## Race Condition Prevention

### Why Copy Semantics?

The combination of `[~]` expansion and copy semantics prevents race conditions:

---

### Race Condition Example (Hypothetical)

**If Polyglot used shared state (it doesn't):**
```
Thread 1: Read .counter (value: 10)
Thread 2: Read .counter (value: 10)
Thread 1: Increment → 11
Thread 2: Increment → 11
Write back: .counter = 11 (WRONG! Should be 12)
```

---

### How Polyglot Prevents This

**Copy semantics ensure safety:**
```polyglot
[r] .counter: pg\int << 10

[p] |Thread1
[<] .value: pg\int << .counter  // COPY (value: 10)
// Modify .value locally
[>] .result >> result1

[p] |Thread2
[<] .value: pg\int << .counter  // COPY (value: 10, independent)
// Modify .value locally
[>] .result >> result2

[Y] |Y.Join
[>] result1  // result1 has Thread1's value
[>] result2  // result2 has Thread2's value
```

**No race condition** - each thread has independent copy.

---

### Scope Isolation Prevents Races

```polyglot
[p] |Parallel1
[~][r] .temp: pg\int << 0  // Local to Parallel1

[p] |Parallel2
[~][r] .temp: pg\int << 0  // Different .temp, local to Parallel2
```

**Each parallel block has isolated scope** - no shared mutable state.

---

### Sequential Access is Direct

```polyglot
[r] .counter: pg\int << 0
[r] .counter << .counter + 1  // Direct access - safe
[r] .counter << .counter + 1  // Sequential - no race
```

**Sequential operations are safe** - no concurrent access.

---

## Best Practices

### 1. Use `[~]` Only When Needed

```polyglot
// ✓ CORRECT - [~] only for nested operations
[p] |Parallel
[<] .input << data        // No [~] - direct child
[~][r] |Nested           // [~] - within parallel
[>] .output >> result     // No [~] - direct child

// ✗ WRONG - Unnecessary [~]
[p] |Parallel
[~][<] .input << data     // Don't need [~]
[~][>] .output >> result  // Don't need [~]
```

---

### 2. Be Consistent with Nesting Depth

```polyglot
// ✓ CORRECT - Consistent depth
[r] ~Array
[~][r] |Process
[~][r] |Transform
[~][r] |Validate

// ✗ CONFUSING - Inconsistent depth
[r] ~Array
[~][r] |Process
[r] |Transform  // Wrong - missing [~]
[~][r] |Validate
```

---

### 3. Understand Copy vs Direct Access

```polyglot
// Sequential - direct access
[r] .x: pg\int << 10
[r] |Operation
[<] .value: pg\int << .x  // Direct access to .x

// Parallel - copy semantics
[p] |Parallel
[<] .value: pg\int << .x  // Copy of .x
```

---

### 4. Use Explicit Nesting for Clarity

```polyglot
// ✓ GOOD - Clear nesting structure
[p] |Parallel
[<] .input << data
[~][r] |Step1
[~][<] .in << .input
[~][>] .out >> temp
[~][r] |Step2
[~][<] .in << temp
[>] .output >> result

// ✗ HARDER TO READ - Missing visual hierarchy
[p] |Parallel
[<] .input << data
[~][r] |Step1
[~][<] .in << .input
[~][>] .out >> temp
[~][r] |Step2
[~][<] .in << temp
[>] .output >> result
```

---

### 5. Document Complex Nesting

```polyglot
// ✓ GOOD - Comments explain nesting
[r] .matrix: pg\array{pg\array{pg\int}}

// Iterate rows
[r] ~.matrix
[~][r] .row: pg\array{pg\int} << .current_row

// Iterate cells within each row
[~][r] ~.row
[~][~][r] |ProcessCell
[~][~][<] .cell << .current_cell
```

---

### 6. Avoid Deep Nesting When Possible

```polyglot
// ✓ BETTER - Flatten structure
[|] ProcessMatrix
[i] .matrix: pg\array{pg\array{pg\int}}

[r] ~.matrix
[~][r] |ProcessRow
[~][<] .row: pg\array{pg\int} << .current_row
[~][>] .processed_row >> result_row

[X]

[|] ProcessRow
[i] .row: pg\array{pg\int}
[r] ~.row
[~][r] |ProcessCell
[~][<] .cell: pg\int << .current_cell
[X]

// ✗ HARDER TO MAINTAIN - Deep nesting
[r] ~.matrix
[~][r] ~.current_row
[~][~][r] |ProcessCell
[~][~][<] .cell << .current_cell
```

---

### 7. Test Parallel Blocks Independently

```polyglot
// ✓ GOOD - Each parallel block is testable
[p] |ProcessPartA
[<] .input << data
[~][r] |TransformA
[~][<] .in << .input
[>] .result >> result_a

[p] |ProcessPartB
[<] .input << data
[~][r] |TransformB
[~][<] .in << .input
[>] .result >> result_b
```

---

### 8. Use Scope Isolation Intentionally

```polyglot
// ✓ CORRECT - Intentional isolation
[p] |Worker1
[~][r] .local_state: pg\int << 0  // Isolated
[~][r] |ProcessWithState

[p] |Worker2
[~][r] .local_state: pg\int << 0  // Independent isolation
[~][r] |ProcessWithState
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - `[~]` overview
- [Block Markers](06-block-markers.md) - `[~]` marker details
- [Parallel Execution](08-parallel-execution.md) - `[~]` in parallel blocks

### Examples
- [Parallel Execution Examples](../examples/parallel-execution.md) - Nesting patterns
- [Data Processing Examples](../examples/data-processing.md) - Unpack patterns

### Planning
- [Decision Log](../decision-log.md) - Expansion operator decisions (#15)

---

**End of Expansion Operator Reference**