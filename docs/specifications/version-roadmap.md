# Polyglot Version Roadmap

**Date:** 2025-12-12 (Updated)
**Status:** 📋 VERSION PLANNING - v0.0.3.1 Absorbed into v0.0.4

---

## Version Overview

### v0.0.3 (Current Stable)
**Status:** Completed

**Features:**
- Basic syntax with backslash markers (`\~\`)
- Pipeline definitions with `{|}` blocks
- Enum/struct definitions with `{#}` blocks
- Error definitions with `{!}` blocks
- Registry system with `{@}` blocks
- Variable prefix: `,` (comma)
- Basic I/O operators
- Conditional markers `[y]`
- Pattern markers (ForEach, etc.)

---

### v0.0.4 (Major Syntax Refinement + Loop System) - PLANNED
**Status:** 🔧 Design Phase - Absorbs v0.0.3.1 Loop System

**Rationale for Absorption:**
The loop system specification (originally v0.0.3.1) already uses v0.0.4 syntax features (`$` prefix, 3-space indentation, lowercase `[v]`), making it logically part of v0.0.4 rather than a standalone patch version.

**Major Features:**

### Part 1: Loop Unpack/Pack System (from v0.0.3.1)

1. **New Loop Markers:**
   - `[~]` - Unpack marker (collection → iterations)
   - `[*]` - Pack marker (iterations → collection)
   - `[v]` - Join/sync marker (lowercase!)

2. **Execution Modes:**
   - `[r] ~*` - Sequential loops
   - `[p] ~*` - Parallel loops
   - `[b] ~*` - Fire-and-forget loops

3. **Mini-Pipeline Iteration:**
   - Each iteration is a mini-pipeline with I/O
   - Explicit scope boundaries
   - Unpack: Main scope → Iteration scope
   - Pack: Iteration scope → Main scope

4. **Standard Unpack Operators:**
   - `~ForEach`, `~Enumerate`, `~Range`, `~Zip`
   - `~Window`, `~Chunk`, `~ForEach.Chained`
   - `~Reduce`, `~While`, `~Until`

5. **Standard Pack Operators:**
   - `*Collect.Into.Array`, `*Collect.Into.Set`, `*Collect.Last`
   - `*Join.All.Success`, `*Join.All.Failures`
   - `*Partition.Status` - State-based partitioning
   - `*Chain`, `*Reduce`

6. **Variable State Integration:**
   - `$var;state =? #;Variables;States;Final`
   - `$var;state =? #;Variables;States;Faulted`
   - Error handling in iterations using state checking

### Part 2: Syntax Refinement

**Breaking Changes:**

1. **Indentation-Based Nesting (BREAKING)**
   - Replace `\~\` markers with 3-space indentation
   - Cleaner syntax, 44% character reduction
   - All markers become `[]` except top-level `{}`

2. **Variable Prefix Change (BREAKING)**
   - Change from `,` (comma) to `$` (dollar)
   - Reason: Comma creates ambiguity in ranges
   - Example: `[y] $age ?[$min, $max]`

3. **Reserved Indication with `;`**
   - Use `;` prefix for reserved enum/error segments
   - Example: `#;Boolean;True`
   - Can mix reserved/user: `#;DT;Business.MyCompany;WorkingDays`
   - Only applies to `#` and `!`, not `:` types

4. **Pipeline I/O Distinction**
   - `i<` / `o>` for pipeline definition (signature)
   - `<` / `>` for pipeline call (usage)
   - Universal `[|]` marker for all pipeline I/O

5. **Multi-line Strings with `[+]`**
   - Explicit marker for string continuation
   - Prevents Python-style silent concatenation bugs
   - Can mix literals and inline pipelines

6. **Inline Pipelines**
   - Single-output pipelines that evaluate in place
   - Example: `|DT.Now""`, `|SQL"{$input}"`
   - Trigger: `|T.Call.Inline`

7. **Collection Literals**
   - Arrays/sets use `{}` syntax
   - Example: `$items:array.int << {1, 2, 3}`

8. **Range Operators (4 variants)**
   - `?[min, max]` - Inclusive both
   - `?(min, max]` - Exclusive min
   - `?[min, max)` - Exclusive max
   - `?(min, max)` - Exclusive both

9. **Operator Negation: `!?`**
   - Universal negation for all conditional operators
   - `=?` / `!=?`, `>?` / `!>?`, etc.

10. **Collection Membership**
    - `in?` - In collection
    - `!in?` - NOT in collection

11. **Early Return Pattern**
    - `[>] o>error << !Error"message"`
    - Inside conditionals for early exit

12. **Match Exhaustiveness**
    - Compiler enforces wildcard `*` case
    - No unhandled cases

13. **Enum Value Fields**
    - Enums can contain value fields
    - More flexible than Rust enums

14. **Struct Shorthand**
    - Field name inference when matching variable name
    - Example: `[.] .name` assumes `<< $name`

15. **Struct Auto-fill**
    - Pipeline outputs can auto-fill matching struct fields

16. **Metadata System: `%`**
    - `%Doc`, `%Author`, `%Deprecated`, etc.
    - Enables features without syntax bloat

17. **Variadic Input via Metadata**
    - `<<<` shorthand enabled by `%InStream` metadata
    - Example: `<<< $set1`, `<<< $set2`, `>>> $result`

18. **Pipeline Composition: `|>`**
    - Chain pipelines: `|Pipeline1 |> |Pipeline2 |> |Pipeline3`

19. **Block Comments**
    - `/* ... */` syntax

20. **ForEach.Range**
    - String literal ranges: `~ForEach.Range"1..100"`

**Breaking Changes:**
- Variable prefix: `,` → `$`
- Indentation replaces `\~\` markers
- Reserved enums: add `;` prefix

**Migration Required:** Yes (automated tools recommended)

---

### v0.0.5 (Type System) - FUTURE
**Status:** 💡 Concept Phase

**Planned Features:**

1. **Type Definition Blocks: `{:}`**
   - Define constrained types
   - Example: `:data.age` with min/max constraints
   - Violation handlers (clip, raise, transform, default)

2. **Cross-Language Types**
   - `:py.str`, `:rust.i32`, `:js.number`
   - Backend-specific type mappings

3. **Type Composition**
   - Constrained collections
   - Optional types
   - Type conversions

4. **Metadata-Driven Type Features**
   - `%Constraint` - Validation rules
   - `%Backend` - Target language
   - `%Native` - Native type mapping

---

### v0.1.0 (First Stable) - FUTURE
**Status:** 🎯 Target Milestone

**Goals:**
- All core syntax finalized
- Complete standard library
- Full compiler implementation
- Comprehensive test suite
- Documentation complete
- Real-world usage examples

---

## Version Strategy

### Semantic Versioning

**Format:** `MAJOR.MINOR.PATCH`

- **MAJOR (0):** Pre-1.0, breaking changes allowed
- **MINOR (0-5):** Feature additions, may include breaking changes
- **PATCH (0-1):** Bug fixes, refinements, non-breaking additions

### Current Strategy (Pre-1.0)

- **v0.0.x:** Rapid iteration, breaking changes acceptable
- **v0.x.y:** Stabilizing syntax
- **v1.0.0:** Stable release, backward compatibility guarantees

---

## Implementation Priorities

### Phase 1: v0.0.3.1 (Loop System)
**Target:** Q1 2026

1. Parser updates for `[~]`, `[*]`, `[V]` markers
2. Scope isolation for iteration mini-pipelines
3. Sequential execution mode `[r] ~*`
4. Basic pack operators: `*Join.All`
5. Basic unpack operators: `~ForEach`, `~Range`
6. Variable state checking: `$var;state`

### Phase 2: v0.0.4 (Syntax Refinement)
**Target:** Q2 2026

1. Indentation-based nesting (parser rewrite)
2. Variable prefix change: `,` → `$` (find-replace + parser)
3. Reserved indication: `;` prefix (parser + stdlib)
4. Pipeline I/O distinction: `i<`/`o>` vs `<`/`>`
5. Multi-line strings: `[+]` marker
6. All v0.0.4 features (see above)

### Phase 3: v0.0.4.1 (Advanced Loop Features)
**Target:** Q3 2026

1. Parallel execution mode `[p] ~*`
2. Fire-and-forget mode `[b] ~*`
3. Advanced pack operators: `*Chain`, `*Reduce`, `*Join.First`
4. Advanced unpack operators: `~Zip`, `~Window`, `~Chunk`
5. Chained loops: `~ForEach.Chained`
6. Error handling: `*Join.All.Success`, `*Join.All.Failures`

### Phase 4: v0.0.5 (Type System)
**Target:** Q4 2026

1. Type definition blocks: `{:}`
2. Constraint system
3. Violation handlers
4. Cross-language types
5. Type composition

### Phase 5: v0.1.0 (Stabilization)
**Target:** Q1 2027

1. Full compiler implementation
2. Complete standard library
3. Comprehensive test coverage
4. Performance optimization
5. Documentation completion
6. Real-world usage validation

---

## Feature Matrix

| Feature | v0.0.3 | v0.0.4 | v0.0.5 | v0.1.0 |
|---------|--------|--------|--------|--------|
| Basic syntax | ✅ | ✅ | ✅ | ✅ |
| Backslash markers | ✅ | ❌ | ❌ | ❌ |
| Indentation nesting | ❌ | ✅ | ✅ | ✅ |
| Variable prefix `,` | ✅ | ❌ | ❌ | ❌ |
| Variable prefix `$` | ❌ | ✅ | ✅ | ✅ |
| Loop unpack/pack | ❌ | ✅ | ✅ | ✅ |
| Sequential loops | ❌ | ✅ | ✅ | ✅ |
| Parallel loops | ❌ | ✅ | ✅ | ✅ |
| Reserved `;` prefix | ❌ | ✅ | ✅ | ✅ |
| Multi-line `[+]` | ❌ | ✅ | ✅ | ✅ |
| Inline pipelines | ❌ | ✅ | ✅ | ✅ |
| Range operators | ❌ | ✅ | ✅ | ✅ |
| Metadata system | ❌ | ✅ | ✅ | ✅ |
| Type definitions | ❌ | ❌ | ✅ | ✅ |
| Cross-language types | ❌ | ❌ | ✅ | ✅ |

**Legend:**
- ✅ Implemented
- 🔧 Partial/Planned
- ❌ Not Available

---

## Migration Guides

### v0.0.3 → v0.0.4

**Impact:** High (breaking changes + major new features)

**Breaking Changes:**
1. Variable prefix: `,` → `$` (automated migration)
2. Nesting: `\~\` markers → 3-space indentation (automated)
3. Reserved enums: Add `;` prefix to reserved segments (automated)

**New Features:**
4. Loop unpack/pack system with mini-pipeline iterations
5. Variable state checking
6. All v0.0.4 syntax refinements (33 features including boolean markers)

**Migration Example:**
```polyglot
// v0.0.3 syntax
[r] ,items << {1, 2, 3}
\~\[r] ,doubled << ,items * 2

// v0.0.4 syntax
[r] $items << {1, 2, 3}

[r] ~ForEach
[~] <array << $items
[~] >item >> $item

   [r] $doubled << $item * 2

   [v] *Collect.Into.Array
   [*] <item << $doubled
   [*] >array >> $results
```

**Migration Tools:** Provided by compiler team

---

**Status:** 📋 Roadmap Updated - v0.0.3.1 Absorbed into v0.0.4

**Next Steps:**
1. Implement v0.0.4 (loop system + syntax refinement)
2. Validate with real-world examples
3. Begin v0.0.5 design finalization
