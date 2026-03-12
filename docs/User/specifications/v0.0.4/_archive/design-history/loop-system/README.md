<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

---

> ⚠️ **HISTORICAL DOCUMENT - DEPRECATED SYNTAX**
>
> This document contains **v0.0.3 syntax** with significant differences from v0.0.4:
>
> **Critical Syntax Changes:**
> - `[V]` (uppercase) → `[v]` (lowercase) for join marker
> - Additional prefix system refinements
> - Reserved indication using semicolon (`;`)
>
> **For current v0.0.4 syntax, see:**
> - [Main Documentation](../../README.md)
> - [v0.0.4 Grammar](../../reference/grammar.md)
> - [Markers Reference](../../language/syntax/markers.md)

---

# v0.0.4 Loop System Specifications

**Original Version:** v0.0.3.1 (absorbed into v0.0.4)
**Status:** ✅ Specification Complete - Ready for Implementation
**Date:** December 2025

---

## 📋 Overview

The loop system introduces a revolutionary approach to iteration in Polyglot: **mini-pipeline iterations**. Each loop iteration is treated as an independent mini-pipeline with explicit input/output, scope isolation, and state management.

**Key Innovation:** Three-operator system for unpack-process-pack pattern
- `[~]` - Unpack marker (collection → iterations)
- `[*]` - Pack marker (iterations → collection)
- `[v]` - Join/sync marker (lowercase!)

---

## 📁 Specification Files

### Core Specifications

#### [v0.0.3.1 Loop System Specification](v0.0.3.1-loop-system-specification.md) ⭐
**Purpose:** Complete specification for the loop unpack/pack system
**Status:** ✅ Finalized and approved

**Contents:**
- Three-operator system: `[~]` `[*]` `[v]`
- Mini-pipeline iteration model
- Execution modes: `[r]` `[p]` `[b]`
- Standard unpack operators (ForEach, Enumerate, Range, Zip, etc.)
- Standard pack operators (Collect, Join, Partition, etc.)
- Variable state integration
- Scope isolation semantics
- Error handling patterns

#### [v0.0.3.1 Blind Spots Analysis](v0.0.3.1-blind-spots-analysis.md)
**Purpose:** Comprehensive review identifying and resolving potential issues
**Status:** ✅ Complete - All blockers cleared

**Coverage:**
- 20 potential issues identified
- All blockers resolved
- Edge cases documented
- Design validated against constraints

### Design Evolution

#### [Loop Unpack/Pack Final Design](loop-unpack-pack-final-design.md)
**Purpose:** Final design with complete semantics for the three-operator system
**Contents:**
- Operator semantics and relationships
- Scope boundary definitions
- Unpack → Process → Pack flow
- Integration with existing pipeline model

#### [Loop Pack/Unpack Improvements](loop-pack-unpack-improvements.md)
**Purpose:** Design improvements, patterns, and refinements
**Contents:**
- Common patterns and idioms
- Performance considerations
- Best practices
- Integration with error handling

#### [Loop I/O Mini-Pipelines](loop-io-mini-pipelines.md)
**Purpose:** Initial exploration of loop I/O concepts
**Contents:**
- Early design discussions
- Conceptual foundations
- Evolution toward final design

### Related Decisions

#### [Variable Reassignment & Pack/Unpack](variable-reassignment-pack-unpack.md)
**Purpose:** Exploration of mutability (rejected) and functional solutions
**Decision:** Rejected mutable variables in favor of functional pack/unpack patterns

**Rationale:**
- Maintains immutability principle
- Provides functional alternative (pack/unpack)
- Consistent with async-first architecture

#### [Pipelines as Variables](pipelines-as-variables.md)
**Purpose:** `:pg.pipeline` type proposal
**Status:** ⏸️ On Hold - Deferred to post-MVP

---

## 🎯 Key Features

### Mini-Pipeline Iteration Model

Each iteration is an independent mini-pipeline:
```polyglot
[r] ~ForEach
[~] <array << $items
[~] >item >> $item

   // Mini-pipeline scope - isolated from main pipeline
   [r] $doubled << $item * 2
   [f] $doubled >? 100
      [r] $large << #;Boolean;True

   [v] *Collect.Into.Array
   [*] <item << $doubled
   [*] >array >> $results
```

### Three-Operator System

**Unpack `[~]`:** Collection → Iterations
- Takes collection from main scope
- Outputs iteration variables
- Defines iteration boundaries
```polyglot
[~] <array << $items     // Input: collection
[~] >item >> $item       // Output: iteration variable
```

**Pack `[*]`:** Iterations → Collection
- Takes iteration results
- Outputs to main scope
- Aggregates results
```polyglot
[*] <item << $doubled    // Input: iteration result
[*] >array >> $results   // Output: aggregated collection
```

**Join/Sync `[v]`:** Coordination Point
- Marks transition from process to pack
- Specifies pack strategy
- Lowercase marker (not `[V]`)
```polyglot
[v] *Collect.Into.Array  // Strategy: collect all into array
[v] *Join.All.Success    // Strategy: join only successful
```

### Execution Modes

**Sequential `[r] ~*`:** Blocking, ordered execution
```polyglot
[r] ~ForEach
// Iterations run one after another
```

**Parallel `[p] ~*`:** Concurrent execution
```polyglot
[p] ~ForEach
// Iterations run concurrently
```

**Fire-and-Forget `[b] ~*`:** Non-blocking execution
```polyglot
[b] ~ForEach
// Main pipeline continues immediately
```

### Standard Unpack Operators

- `~ForEach` - Iterate collection elements
- `~Enumerate` - Iterate with index
- `~Range` - Integer ranges
- `~Zip` - Combine multiple collections
- `~Window` - Sliding windows
- `~Chunk` - Fixed-size chunks
- `~ForEach.Chained` - Chain multiple loops
- `~Reduce` - Fold/reduce pattern
- `~While` - Conditional iteration
- `~Until` - Conditional iteration (inverse)

### Standard Pack Operators

- `*Collect.Into.Array` - Collect to array
- `*Collect.Into.Set` - Collect to set
- `*Collect.Last` - Only last value
- `*Join.All.Success` - Only successful iterations
- `*Join.All.Failures` - Only failed iterations
- `*Partition.Status` - Partition by state
- `*Chain` - Chain for next loop
- `*Reduce` - Reduce to single value

### Variable State Integration

Check iteration variable states:
```polyglot
[f] $item;state =? #;Variables;States;Final
   // Process successful iteration

[f] $item;state =? #;Variables;States;Faulted
   [*] <item << $item      // Collect failure
```

---

## 🔑 Key Design Principles

1. **Explicit Scope Boundaries** - Clear separation between main and iteration scopes
2. **Mini-Pipeline Model** - Each iteration is a complete pipeline with I/O
3. **Functional Patterns** - No mutation, pack/unpack instead of reassignment
4. **State Awareness** - Integration with variable state system
5. **Execution Flexibility** - Sequential, parallel, or fire-and-forget modes
6. **Standard Library** - Rich set of unpack/pack operators
7. **Error Handling** - Pack strategies for success/failure partitioning

---

## 📖 Reading Order

**First Time:**
1. [v0.0.3.1 Loop System Specification](v0.0.3.1-loop-system-specification.md) - Complete specification
2. [Loop Unpack/Pack Final Design](loop-unpack-pack-final-design.md) - Design semantics
3. [v0.0.3.1 Blind Spots Analysis](v0.0.3.1-blind-spots-analysis.md) - Validation and edge cases

**Understanding Evolution:**
1. [Loop I/O Mini-Pipelines](loop-io-mini-pipelines.md) - Initial concepts
2. [Loop Pack/Unpack Improvements](loop-pack-unpack-improvements.md) - Refinements
3. [Loop Unpack/Pack Final Design](loop-unpack-pack-final-design.md) - Final design

**Related Decisions:**
1. [Variable Reassignment & Pack/Unpack](variable-reassignment-pack-unpack.md) - Why no mutation
2. [Pipelines as Variables](pipelines-as-variables.md) - Future enhancements

---

## 🔗 Related Documentation

**v0.0.4 Overview:** [../README.md](../README.md) - Complete v0.0.4 features
**Syntax Refinement:** [../syntax-refinement/](../syntax-refinement/) - Other v0.0.4 features
**Version Roadmap:** [../../version-roadmap.md](../../version-roadmap.md) - Version timeline
**Current Implementation:** [/docs/user/](../../../user/) - v0.0.3 reference

---

## ⚠️ Important Notes

### Why Originally v0.0.3.1?

The loop system was initially planned as v0.0.3.1 (patch version), but analysis revealed it:
- Uses `$` variable prefix (v0.0.4 breaking change)
- Uses 3-space indentation (v0.0.4 breaking change)
- Uses lowercase `[v]` marker (v0.0.4 convention)

**Decision:** Absorbed into v0.0.4 as it logically depends on v0.0.4 syntax.

### Specification Status

✅ **Design Complete** - All features specified
✅ **Validation Complete** - Blind spots analysis cleared
✅ **Edge Cases Documented** - All scenarios covered
⏳ **Implementation Pending** - Target Q1-Q2 2026

---

## 📊 Implementation Priority

### Phase 1: Core Loop System (Q1 2026)
- Parser updates for `[~]`, `[*]`, `[v]` markers
- Scope isolation for iteration mini-pipelines
- Sequential execution mode `[r] ~*`
- Basic pack operators: `*Collect.Into.Array`, `*Join.All`
- Basic unpack operators: `~ForEach`, `~Range`
- Variable state checking: `$var;state`

### Phase 2: Advanced Features (Q2 2026)
- Parallel execution mode `[p] ~*`
- Fire-and-forget mode `[b] ~*`
- Advanced pack operators: `*Chain`, `*Reduce`, `*Partition`
- Advanced unpack operators: `~Zip`, `~Window`, `~Chunk`
- Chained loops: `~ForEach.Chained`
- Error handling: `*Join.All.Success`, `*Join.All.Failures`

---

**Last Updated:** 2025-12-12
**Specification Status:** ✅ Complete and Approved
**Implementation Target:** Q1-Q2 2026
