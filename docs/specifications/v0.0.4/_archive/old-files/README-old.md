<!-- ARCHIVED: 2025-12-16 | Reason: Superseded by current README.md | Superseded by: ../README.md -->

# v0.0.4 - Major Syntax Refinement + Loop System

**Status:** 🔧 Design Phase - Breaking Changes + Major Features
**Target Release:** Q2 2026
**Rationale:** Combines loop system (originally v0.0.3.1) with syntax refinement into single major update

---

## 📋 Overview

Version 0.0.4 represents a major evolution of the Polyglot language, absorbing the loop system specification (originally planned as v0.0.3.1) and introducing 30 syntax refinement features. This consolidation creates a cleaner version progression and acknowledges that the loop system already uses v0.0.4 syntax conventions.

**Why v0.0.3.1 was absorbed:**
The loop system specification already uses v0.0.4 syntax features (`$` variable prefix, 3-space indentation, lowercase `[v]` marker), making it logically part of v0.0.4 rather than a standalone patch version.

---

## 📁 Specification Structure

### [Loop System](loop-system/)
Complete specification for the loop unpack/pack system with mini-pipeline iterations

**Key Files:**
- `v0.0.3.1-loop-system-specification.md` - Complete specification
- `v0.0.3.1-blind-spots-analysis.md` - Comprehensive review (20 issues resolved)
- `loop-unpack-pack-final-design.md` - Final design semantics
- Design evolution documents

**Key Features:**
- Mini-pipeline iterations with explicit I/O
- Three operators: `[~]` unpack, `[*]` pack, `[v]` join/sync
- Execution modes: `[r]` sequential, `[p]` parallel, `[b]` fire-and-forget
- Variable state checking: `$var;state`
- Standard pack/unpack operators

### [Syntax Refinement](syntax-refinement/)
30 syntax features and improvements for cleaner, more expressive code

**Key Files:**
- `v0.0.4-final-decisions.md` - Core v0.0.4 decisions
- `v0.0.4-complete-syntax.md` - Full syntax specification
- Feature-specific documents (reserved enums, metadata, etc.)

**Key Features:**
- Indentation-based nesting (3 spaces)
- Variable prefix change: `,` → `$`
- Reserved indication with `;` prefix
- Multi-line strings, inline pipelines, collection literals
- Range operators, operator negation, early return
- Metadata system, variadic input, pipeline composition

---

## 🎯 Major Breaking Changes

### 1. Variable Prefix: `,` → `$`
**Reason:** Comma creates ambiguity in range expressions
```polyglot
// v0.0.3
[r] ,age << 25
[y] ,age =? [18, 65]

// v0.0.4
[r] $age << 25
[y] $age ?[18, 65]  // Clear: variable vs range
```

### 2. Indentation-Based Nesting
**Reason:** Cleaner syntax, 44% character reduction
```polyglot
// v0.0.3
[r] |Pipeline
\~\[<] <input << ,value
\~\[>] >output >> ,result

// v0.0.4
[r] |Pipeline
   [<] <input << $value
   [>] >output >> $result
```

### 3. Reserved Enum Indication
**Reason:** Distinguish reserved from user-defined segments
```polyglot
// v0.0.3
#Boolean.True
#DT.Business.MyCompany.WorkingDays

// v0.0.4
#;Boolean;True
#;DT;Business.MyCompany.WorkingDays
```

---

## 🚀 New Features Highlights

### Loop System (from absorbed v0.0.3.1)

**Mini-Pipeline Iterations:**
```polyglot
[r] ~ForEach
[~] <array << $items
[~] >item >> $item

   // Each iteration is a mini-pipeline
   [r] $doubled << $item * 2

   [v] *Collect.Into.Array
   [*] <item << $doubled
   [*] >array >> $results
```

**Execution Modes:**
- `[r] ~*` - Sequential (blocking)
- `[p] ~*` - Parallel (concurrent)
- `[b] ~*` - Fire-and-forget (non-blocking)

**Variable State Checking:**
```polyglot
[y] $var;state =? #;Variables;States;Final
[y] $var;state =? #;Variables;States;Faulted
```

### Syntax Refinement Features

**Multi-line Strings:**
```polyglot
[r] $message << "First line"
[+] +"Second line"
[+] +|DT.Now""
[+] +"Final line"
```

**Inline Pipelines:**
```polyglot
[r] $now << |DT.Now""
[r] $sql << |SQL"{$input}"
```

**Range Operators (4 variants):**
```polyglot
?[min, max]   // Inclusive both
?(min, max]   // Exclusive min
?[min, max)   // Exclusive max
?(min, max)   // Exclusive both
```

**Operator Negation:**
```polyglot
=? / !=?      // Equal / not equal
>? / !>?      // Greater / not greater (equivalent to <=?)
in? / !in?    // In collection / not in
?[min,max] / !?[min,max]  // In range / not in range
```

**Boolean Markers:**
```polyglot
[y] $stock >? 0
[&] $price ?(0.0, 1000000.0]   // AND - all must be true
[&] $quantity >? 0
    // Executes if all conditions true

[y] $condition1
[|] $condition2                 // OR - any can be true
    // Executes if either true

[y] $flag1
[^] $flag2                      // XOR - exactly one true
    // Executes if one but not both
```

**Wildcard Condition (Exhaustive Matching):**
```polyglot
[y] $status =? #;Status;Active
    // Handle active case
[y] $status =? #;Status;Pending
    // Handle pending case
[y] *    // Wildcard - catches all remaining cases
    // Handle all other cases
```

**Trigger OR:**
```polyglot
[t] |T.Cron"0 2 * * *"    // Scheduled trigger
[|] |T.Call                // OR manual call
// Pipeline runs on schedule OR when called
```

**Early Return Pattern:**
```polyglot
[y] $age <? 18
   [>] o>error << !;Error;AgeRestriction
```

**Metadata System:**
```polyglot
%Doc "Pipeline documentation"
%Author "Developer name"
%Deprecated "Use NewPipeline instead"
```

**Pipeline Composition:**
```polyglot
[r] |Pipeline1 |> |Pipeline2 |> |Pipeline3
[|] <input:datatype << $source
[|] >out1:datatype >> <in2
[|] >out2:datatype >> <in3
[|] |>
[|] >final:datatype >> $result
```

---

## 📊 Feature Comparison

| Feature Category | v0.0.3 | v0.0.4 | Notes |
|-----------------|--------|--------|-------|
| **Core Syntax** | ||||
| Variable prefix | `,` | `$` | Breaking change |
| Nesting | `\~\` markers | 3-space indent | Breaking change |
| Reserved indication | No | `;` prefix | New syntax |
| **Loop System** | ||||
| Unpack/pack operators | ❌ | `[~]` `[*]` `[v]` | New feature |
| Mini-pipeline iterations | ❌ | ✅ | New paradigm |
| Execution modes | ❌ | `[r]` `[p]` `[b]` | New modes |
| Variable state checking | ❌ | `$var;state` | New syntax |
| **Refinement Features** | ||||
| Multi-line strings | ❌ | `[+]` marker | New syntax |
| Inline pipelines | ❌ | `\|Pipe""` | New syntax |
| Range operators | Basic | 4 variants | Enhanced |
| Operator negation | Limited | `!?` prefix | Universal |
| Boolean markers | ❌ | `[&]` `[|]` `[^]` | New feature |
| Wildcard condition | ❌ | `[y] *` | New syntax |
| Trigger OR | ❌ | `[|]` under `[t]` | New feature |
| Early return | ❌ | `[>] o>` | New pattern |
| Metadata system | ❌ | `%` prefix | New system |
| Pipeline composition | ❌ | `\|>` operator | New syntax |

---

## 🔄 Migration Path

### v0.0.3 → v0.0.4

**Automated Migration:**
1. Variable prefix: Replace all `,` → `$` (context-aware)
2. Nesting: Convert `\~\` markers → 3-space indentation
3. Reserved enums: Add `;` prefix to reserved segments

**Manual Review Required:**
1. Loop patterns: Consider using new unpack/pack system
2. Conditionals: Evaluate early return patterns
3. Strings: Identify multi-line string candidates
4. Pipelines: Consider composition with `|>` operator

**Migration Example:**
```polyglot
// v0.0.3
[r] ,items << {1, 2, 3}
\~\[r] ,doubled << ,items * 2

// v0.0.4 (basic migration)
[r] $items << {1, 2, 3}
   [r] $doubled << $items * 2

// v0.0.4 (idiomatic with loop system)
[r] $items << {1, 2, 3}

[r] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] $doubled << $item * 2

   [v] *Collect.Into.Array
   [*] <item << $doubled
   [*] >array >> $results
```

---

## 📝 Key Design Decisions

### 1. Absorb v0.0.3.1 into v0.0.4
**Rationale:** Loop system already uses v0.0.4 syntax, creating logical dependency

### 2. Breaking Changes Acceptable
**Rationale:** Pre-1.0 allows major syntax evolution for long-term improvements

### 3. Indentation Over Markers
**Rationale:** 44% character reduction, matches industry conventions (Python, YAML)

### 4. Dollar Sign Variable Prefix
**Rationale:** Familiar from shell/Perl, eliminates comma ambiguity

### 5. Semicolon for Reserved Indication
**Rationale:** Visually distinct, doesn't conflict with existing syntax

### 6. Mini-Pipeline Iteration Model
**Rationale:** Explicit scope, consistent with pipeline-first paradigm, natural async handling

---

## 🔗 Related Documentation

**Version Roadmap:** [../version-roadmap.md](../version-roadmap.md) - Complete version timeline
**Loop System Details:** [loop-system/](loop-system/) - Complete loop specification
**Syntax Refinement Details:** [syntax-refinement/](syntax-refinement/) - All 30 features
**v0.0.5 Concepts:** [../v0.0.5/](../v0.0.5/) - Future type system
**Current Implementation:** [/docs/user/](../../user/) - v0.0.3 syntax reference

---

## ⏱️ Implementation Timeline

### Phase 1: Parser Foundation (Q1 2026)
- Indentation-based nesting parser rewrite
- Variable prefix migration tooling
- Reserved indication parser support

### Phase 2: Loop System (Q2 2026)
- Unpack/pack operators: `[~]` `[*]` `[v]`
- Mini-pipeline scope isolation
- Sequential execution mode `[r] ~*`
- Basic pack/unpack operators

### Phase 3: Syntax Refinement (Q2 2026)
- Multi-line strings `[+]`
- Inline pipelines
- Range operators (4 variants)
- Operator negation `!?`
- Metadata system `%`
- All remaining features

### Phase 4: Advanced Features (Q3 2026)
- Parallel execution `[p] ~*`
- Fire-and-forget `[b] ~*`
- Advanced pack operators
- Pipeline composition `|>`
- Complete standard library integration

---

**Last Updated:** 2025-12-12
**Maintained By:** Polyglot Language Design Team
**Spec Completion:** 95%
