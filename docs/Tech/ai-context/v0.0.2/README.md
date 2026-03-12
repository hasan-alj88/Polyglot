# Polyglot v0.0.2 - AI Context Package

**Purpose:** Maximum-density language specification for AI agent consumption

**Version:** 0.0.2
**Created:** 2025-11-25
**Target Audience:** AI Agents, LLMs, Code Analysis Tools

---

## 🚨 CRITICAL PARADIGM - READ THIS FIRST

### Polyglot is a PIPELINE ORCHESTRATION Language

**What Polyglot IS:**
- 🎯 **Pipeline orchestration DSL** for coordinating external runtimes
- 🔗 **Integration layer** that calls Python, Rust, Node.js, Go via runtime wrappers
- 📦 **Data flow coordinator** using `pg\serial` as universal interchange format
- ⚡ **Async-first** execution model with explicit state transitions

**What Polyglot is NOT:**
- ❌ **NOT a language where you write Python/Rust/JS code inline**
- ❌ **NOT `py\requests.get()` or `rs\serde::parse()` - these syntaxes don't exist**
- ❌ **NOT a "polyglot syntax" language mixing Python/Rust/JS code**
- ❌ **NOT direct foreign function interface (FFI)**

**How to Call External Code:**

```polyglot
// ❌ WRONG - This syntax doesn't exist
.data << py\requests.get(.url)
.clean << rs\cleaner::process(.data)

// ✅ CORRECT - Use runtime wrappers
[W] |W.RT.Python3.14                       // Declare Python runtime
[W] |W.RT.Rust1.8                          // Declare Rust runtime

[r] |RT.Python.Run.Function                // Call Python function
[<] <function: pg\string << "requests.get"
[<] <args: pg\serial << {.url}
[>] >output: pg\serial >> .response

[r] |RT.Rust.Run.File                      // Call Rust file
[<] <file: pg\path << \\FileDir\\process.rs
[<] <input: pg\serial << .response
[>] >output: pg\serial >> .cleaned
```

**Key Concepts:**
1. **Runtime Wrappers:** `[W] |W.RT.{Language}` - Declares execution environment
2. **Pipeline Calls:** `|RT.{Language}.Run.{File|Function}` - Executes external code
3. **Serial Interchange:** `pg\serial` - Universal data format across languages
4. **Input/Output Binding:** `[<]` pushes data IN, `[>]` pulls results OUT

**This is the #1 source of confusion for AI agents. Remember:**
> Polyglot ORCHESTRATES polyglot systems. It doesn't MIX polyglot syntax.

---

## Overview

This package provides the complete Polyglot language specification in machine-parseable formats optimized for AI consumption. Unlike human documentation (prose, examples, tutorials), this package prioritizes:

- **Density** - Maximum information per token
- **Structure** - Queryable, parseable formats
- **Completeness** - 100% of syntax rules and semantics
- **Unambiguity** - Zero interpretation required

**Total Size:** ~25KB across 8 files
**Information Density:** Complete language specification
**Parse Time:** Instant (structured formats)

---

## Package Contents

### 1. `grammar.ebnf` (Primary Syntax Reference)
**Format:** Extended Backus-Naur Form
**Size:** ~150 lines
**Purpose:** Complete syntactic grammar

**Contents:**
- File structure (package → definitions)
- Enumeration syntax
- Pipeline syntax
- Block markers ([|], [i], [r], [?], [~], etc.)
- Operators (<<, >>, <~, =?, >?, etc.)
- Type syntax (namespace\\typename)
- Identifier rules (. # | ! prefixes)
- Literals (strings, numbers, datetime, paths)

**Why EBNF:**
- Unambiguous syntax specification
- Machine-parseable
- No interpretation required
- 100% of syntax in ~150 lines

**Usage:** Use this to validate Polyglot syntax, generate parsers, or understand syntactic structure.

---

### 2. `type-system.json` (Type Rules & Constraints)
**Format:** JSON
**Size:** ~300 lines
**Purpose:** Complete type system specification

**Contents:**
- Primitives (pg\\string, pg\\int, pg\\float, pg\\bool, pg\\dt, pg\\path)
- Composite types (pg\\serial, #Enumeration)
- Collections (pg\\array{T}, pg\\set{T})
- Foreign types (py\\*, rs\\*, go\\*, js\\*, node\\*)
- Type compatibility matrix
- Type inference rules
- Removed types (pg\\map - deprecated)

**Key Constraints:**
- Collections cannot nest directly (must wrap in enumeration)
- Enum/serial fields cannot be siblings (can be uncles)
- Type separator is `\\` (backslash only)

**Usage:** Validate types, check compatibility, infer output types.

---

### 3. `state-machine.yaml` (Variable Lifecycle)
**Format:** YAML
**Size:** ~250 lines
**Purpose:** Complete state transition specification

**Contents:**
- 9 variable states (5 MVP + 4 post-MVP)
- State transition rules
- Assignment operators → state mapping
- Push count semantics
- Auto-await triggers
- Reserved schema namespace (.*.pgvar.*)
- Immutability semantics

**State Machine:**
```
Declared → DefaultReady → Pending → Ready/Faulted
                    ↓         ↓
                 Ready    Retrying (post-MVP)
```

**Critical Rules:**
- Variables transition through states (not just values)
- Auto-await occurs when "pulling from" variables
- Ready is immutable (consequence of async completion)
- Push count limits enforcement

**Usage:** Understand variable states, validate transitions, implement runtime.

---

### 4. `operators.json` (Complete Operator Reference)
**Format:** JSON
**Size:** ~350 lines
**Purpose:** All operators + semantics

**Contents:**
- Assignment operators (<<, >>, <~)
- Comparison operators (=?, >?, <?, =>?, =<?, =!?)
- Pattern operators (*?, re?)
- Range operators (?[, ?(, ?], ?)
- Unpack operators (~ForEach, ~Enumerate, ~Zip)
- Join operators (~Y.IntoArray, ~Y.IntoSerial, ~Y.IntoSet)
- Operator prefixes (. # | !)

**Critical Rule:**
- NO KEYWORDS: All identifiers need operator prefix
- Examples: |Pipeline, .variable, #Enum, !Error

**Usage:** Lookup operator semantics, validate usage, understand auto-await.

---

### 5. `reserved-enums.json` (Reserved Enumerations)
**Format:** JSON
**Size:** ~150 lines
**Purpose:** Runtime-provided enumerations

**Contents:**
- #PgVar.States.* (9 states)
- #Boolean.True / #Boolean.False
- #None (unit type)
- Reserved namespace fields (.*.pgvar.*)
- Deprecated names (#Variables.States, #Bool)

**Critical:**
- Use `#PgVar.States` NOT `#Variables.States`
- Reserved namespace `.*.pgvar.*` is always Ready
- All reserved enums still need `#` prefix

**Usage:** Validate reserved enumeration usage, check state references.

---

### 6. `examples-annotated.pg` (Canonical Patterns)
**Format:** Polyglot Code with Inline Annotations
**Size:** ~400 lines
**Purpose:** Real patterns with state/type annotations

**Contents:**
- 10 canonical patterns
- Inline STATE/TYPE/PUSH_COUNT annotations
- Common use cases
- Edge cases and constraints

**Patterns Covered:**
1. Basic async flow (Declared → Pending → Ready)
2. DefaultReady with override
3. Exhaustive conditional triggers
4. Unpack operators (async alternative to loops)
5. Enum vs serial mixing
6. Collection nesting workaround
7. Reserved schema namespace
8. DateTime system
9. No keywords rule
10. Triggers and execution modes

**Usage:** Learn patterns, validate examples, understand idioms.

---

### 7. `constraints.yaml` (Edge Cases & Validation)
**Format:** YAML
**Size:** ~300 lines
**Purpose:** All constraints and edge cases

**Contents:**
- Collection nesting constraint
- Enum/serial mixing rule
- Operator prefix requirements
- Exhaustive condition requirement
- Invalid state transitions
- Push count limits
- Type compatibility constraints
- Removed features (maps, old enum names)
- Validation checklist

**Usage:** Validate code, catch errors, enforce constraints.

---

### 8. `datetime-system.yaml` (DateTime Type Specification)
**Format:** YAML
**Size:** ~500 lines
**Purpose:** Complete pg\dt type and DT.* namespace specification

**Contents:**
- pg\dt primitive type structure (DateTimePattern, DateTimeValue, DateTimeType)
- 7 DateTimeValue variants (TimeOnly, DateOnly, DayOfWeekOnly, etc.)
- DateTime literal syntax patterns (dates, times, durations)
- DT.* pipeline namespace hierarchy (100+ pipelines)
- Calendar systems (algorithmic and profile-based)
- Profile system with 3-tier priority (Manual → API → ICU4X)
- Timezone system (built-in and user-defined)
- Extension system (timezones, profiles, aliases, holidays)
- Relative date patterns (dot hierarchy)
- Validation rules (date, time, duration, wildcard, day-of-week)
- Equality semantics (Instant vs Duration)
- Comparison operators (>?, <?, =?, etc.)
- Trigger integration (T.DT.* semantics)
- MVP scope vs Post-MVP deferrals
- Best practices and critical rules

**Key Patterns:**
- `DT"formatted_string"` - Parse datetime
- `DT.{Calendar}"..."` - Calendar-specific (Gregorian, Hijri, Hebrew, Chinese)
- `DT.{Calendar}.{Profile}"..."` - Profile-based (SaudiArabia, Sephardic, etc.)
- `DT.Ago"duration"` - Past time
- `DT.Daily"time"`, `DT.Weekly"day time"` - Recurring patterns
- `DT.Gregorian.{Month}.{Occurrence}.{DayOfWeek}""` - Relative dates
- `DT.TimeZone.{Region}.{Location}"..."` - Timezone handling

**Critical Rules:**
- Time MUST have AM/PM or 24-hour format (no "3:00")
- Duration units MUST be descending order (no "30m 2h")
- No decimal durations (use "2h 30m" not "2.5h")
- DateAndDayOfWeek validated by ICU4X
- T.DT.* triggers when DT.* equals DT.Now""

**Usage:** Validate DateTime literals, understand calendar systems, implement pg\dt type.

---

## Quick Reference: Critical Rules

### 0. BLOCK MARKER HIERARCHY
**Rule:** Block markers have specific usage contexts
**[r]** - Variable declaration at pipeline scope OR sequential execution
  - Variable: `[r] .var: Type << value`
  - Pipeline: `[r] |Pipeline`
**[<]** - Input binding (ONLY within parent block - pipeline call, unpack, etc.)
  - Valid: `[r] |Pipeline [<] .input << value`
  - Invalid: `[<] .var << value` (no parent block)
**[>]** - Output binding (ONLY within parent block)
  - Valid: `[r] |Pipeline [>] .output >> .result`

### 1. NO KEYWORDS
**Rule:** ALL identifiers MUST have operator prefix (. # | !)
**Invalid:** `FetchUser`, `user`, `UserProfile`
**Valid:** `|FetchUser`, `.user`, `#UserProfile`

### 2. EXHAUSTIVE CONDITIONS
**Rule:** ALL [?] blocks MUST include [?] *? catch-all
**Reason:** Prevents indefinite waiting in async environment

### 3. NO NESTED COLLECTIONS
**Rule:** Collections cannot contain collections directly
**Invalid:** `pg\\array{pg\\array{T}}`
**Valid:** Wrap inner collection in enumeration

### 4. ENUM/SERIAL MIXING
**Rule:** Cannot be siblings, can be uncles
**Invalid:** Enum field + serial field at same level
**Valid:** Enum field nested, serial field at different level

### 5. AUTO-AWAIT
**Trigger:** When "pulling from" variables (<<, >>, =?, etc.)
**Behavior:** Runtime blocks until variable is Ready/Faulted

### 6. RESERVED NAMESPACE
**Namespace:** `.*.pgvar.*`
**Status:** Always Ready (database-tracked)
**Fields:** .state, .errors, .history.{StateName}.at

### 7. TYPE SEPARATOR
**Valid:** `\\` (backslash)
**Invalid:** `/`, `:`, `.`

### 8. RESERVED ENUMERATIONS
**Correct:** `#PgVar.States.Ready`
**Deprecated:** `#Variables.States.Ready`

### 9. PUSH COUNT
- Schema-only: 1 push allowed
- Default: 2 pushes allowed (default + 1 override)
- Constant: 0 pushes (immediately Ready)

### 10. REMOVED FEATURES
- `pg\\map{K,V}` → Use `pg\\serial` or `#Enumeration`
- `#Variables.States` → Use `#PgVar.States`
- `#Bool` → Use `#Boolean`

### 11. STRING LITERALS ARE INLINE PIPELINE CALLS
**CRITICAL:** String literals are NOT primitives - they are inline pipeline calls

**Syntax:** `Pipeline.Name"formatted_argument_string"`
**Implicit:** `"text"` → `U.String"text"`
**Interpolation:** `"{.var:fmt}"` calls `|U.String.{language}.{type}.{fmt}`

**Pipeline Signature:**
- Input: `.formatted_argument_string: pg\string` (MANDATORY name)
- Trigger: `|T.String.Call` (MANDATORY)
- Output: Single output of ANY type (not limited to strings!)

**CONSTRAINT - When inline syntax is valid:**
`Pipeline.Name"..."` can ONLY be used when:
1. Pipeline has NO input (except `.formatted_argument_string`), AND
2. EITHER:
   - Pipeline has exactly ONE output, OR
   - Pipeline outputs are packed into `pg\serial` type

**Examples:**
- `"hello"` → `U.String"hello"` (implicit call) ✅
- `DT.Now""` → returns `pg\dt` (no input, single output) ✅
- `DT.Minutes"5"` → returns `pg\dt` duration (single input, single output) ✅
- `"{.count:Hex}"` → calls `|U.String.Polyglot.Int.Hex` ✅

**Invalid:** `DT.Now`, `DT.Minutes(5)`, `DT.ToNow(.start)` ❌
**Valid:** `DT.Now""`, `DT.Minutes"5"`, `DT.ToNow"{.start}"` ✅

**Invalid (multiple outputs):** `Pipeline.TwoOutputs"..."` ❌
**Correct:** Use full pipeline call syntax with `[<]` and `[>]` bindings

**See:** `/docs/Tech/implementation/technical/string-literals-internals.md` for complete mechanics

### 12. COLLECTION SYNTAX
**Rule:** Collections use `{}` with comma separation, NOT `[]`
**Invalid:** `<< []`, `<< [1, 2, 3]`
**Valid:** `<< {}`, `<< {1, 2, 3}`, `<< {#Channel.Email, #Channel.SMS}`

### 13. ASYNC TERMINOLOGY
**Use:** PUSH/PULL, Variable States, Declared state, Ready state
**Avoid:** assign/assignment, mutable/immutable, initialize
**Why:** Polyglot is async-centric - variables transition through states

### 14. DECLARED STATE FOR COLLECTIONS
**Wrong:** `[r] .channels: pg\\array{T} << {}` (immediately Ready, no pushes allowed)
**Right:** `[r] .channels: pg\\array{T}` (Declared state, allows conditional pushes)

### 15. MANDATORY PIPELINE SECTIONS (CRITICAL!)
**Rule:** Every `[|]` pipeline MUST have ALL of these sections:
1. **Inputs** `[i]` - At least one (use `[i] !No.Input` if none)
2. **Triggers** `[t]` - At least one (pipeline will NEVER run without triggers!)
3. **Wrapper** - `[W]` wrapper (runtime wrapper OR scope placeholder)
4. **Outputs** `[o]` - At least one (use `[o] !No.Output` if no output)

### 16. HIERARCHY TREE DOCUMENTATION (REQUIRED!)
**Rule:** ALL hierarchical structures MUST include ASCII tree diagrams
**Why:** Polyglot data has serial hierarchy - variables, pipelines, enumerations, errors, and blocks all use dot notation

**Applies to:**
- Variable namespaces (`.variable.field.subfield`)
- Pipeline namespaces (`DT.Gregorian.November.Fourth.Thursday`)
- Enumeration fields (`#Enum.variant.nested`)
- Error hierarchies (`!Network.HTTP.4xx.NotFound`)
- Block element nesting
- Reserved namespaces (`.*.pgvar.*`, `#PgVar.States.*`)

**Tree Notation:**
```
.variable: pg\serial                // Variable (. prefix, with type)
│
├─ .variable.field: pg\string       // First branch with type
│   └─ .variable.field.nested: pg\int  // Nested item with type
└─ .variable.pgvar.*                // Reserved namespace (extendable)

|Pipeline.*                         // Pipeline (| prefix)
│
├─ |Pipeline.Operation              // → pg\dt (return type)
└─ |Pipeline.{UserDefined}*         // Extendable namespace

#Enum                               // Enumeration (# prefix)
│
├─ #Enum.Variant                    // Enum field (NO type)
└─ #Enum.field: pg\string           // Serial field (HAS type)

~ForEach                            // Unpack operator (~ prefix)
~Y.IntoArray                        // Join operator (~Y prefix)

!Error.*                            // Error (! prefix)
└─ !Error.Category.Specific
```

**Reference:** `/docs/Tech/implementation/technical/hierarchy-tree-notation.md`

**Minimal Valid Pipeline:**
```polyglot
[|] MinimalPipeline
[i] !No.Input
[t] |T.Call
[W] |W.Polyglot.Scope        // Placeholder: no setup/cleanup
[o] !No.Output
[X]
```

**Wrapper Options:**
- **Runtime Wrappers:** `[W] RT.Python"script.py"`, `[W] RT.Rust"module"`, `[W] RT.Node"module.js"`
- **Scope Placeholder:** `[W] |W.Polyglot.Scope` or `[W] W.Polyglot.Scope""`
  - Use when no explicit setup/cleanup needed
  - Makes it explicit (not accidental omission)
  - Indicates RAII-style scope cleanup
  - Replaces old `|W.Polyglot.Scope`
- **Explicit Setup/Cleanup:** `[\]` ... `[/]` (for custom logic)

**Why Mandatory:**
- Without `[t]` triggers, pipeline will never execute
- Without `[W]`, no execution environment
- Without `[i]`, ambiguous input requirements
- Without `[o]`, ambiguous output contract

**Common Triggers:**
- `[t] |T.Call` - Manual call (for pipelines called via `|PipelineName`)
- `[t] TG.Cron""` - Time-based
- `[t] TG.FileWatch""` - File system events
- `[t] TG.HTTP""` - HTTP endpoints

---

## Usage Recommendations

### For AI Code Generation
1. **Start with `grammar.ebnf`** - Validate syntax
2. **Check `constraints.yaml`** - Avoid invalid patterns
3. **Reference `examples-annotated.pg`** - Follow patterns
4. **Validate types with `type-system.json`**
5. **Check states with `state-machine.yaml`**

### For Code Analysis
1. **Parse with `grammar.ebnf`**
2. **Validate constraints with `constraints.yaml`**
3. **Type-check with `type-system.json`**
4. **State-check with `state-machine.yaml`**

### For Documentation Generation
1. **Extract patterns from `examples-annotated.pg`**
2. **Lookup operators in `operators.json`**
3. **Reference types in `type-system.json`**
4. **Explain states from `state-machine.yaml`**

---

## Information Density Comparison

| Format | Lines | Information | Density |
|--------|-------|-------------|---------|
| **Human Prose Docs** | ~10,000 | Language spec | 1x |
| **AI Context Package** | ~2,300 | Same spec | **4.3x** |

**Key Advantages:**
- **4.3x more compact** than prose documentation
- **100% structured** (queryable, parseable)
- **Zero ambiguity** (formal specifications)
- **Instant lookup** (JSON/YAML queries)
- **Complete coverage** (all syntax + semantics + DateTime)

---

## File Format Rationale

### Why EBNF for Grammar?
- Industry standard for syntax specification
- Unambiguous, machine-parseable
- Minimal tokens, maximum precision
- No interpretation required

### Why JSON for Type System & Operators?
- Queryable (can search for specific types/operators)
- Parseable by all languages
- Hierarchical structure matches data
- Easy to validate

### Why YAML for State Machine & Constraints?
- Human-readable structure (for debugging)
- Compact representation
- Hierarchical state definitions
- Comments for clarification

### Why Annotated .pg for Examples?
- Real code (not pseudocode)
- Inline annotations show intent
- Pattern recognition for AI
- Demonstrates idioms

---

## Version History

**v0.0.2** (2025-11-25 to 2025-11-30)
- Initial AI context package
- Complete language specification
- All 8 reference files
- Added datetime-system.yaml (2025-11-30)

**Future Enhancements:**
- AST schema (JSON Schema for IR structure)
- Error catalog (all compile/runtime errors)
- Optimization hints (performance patterns)
- Anti-patterns catalog (common mistakes)

---

## Integration Guide

### Reading Order (First Time)
1. README.md (this file) - Overview
2. grammar.ebnf - Syntax structure
3. examples-annotated.pg - Patterns
4. constraints.yaml - Rules
5. operators.json - Operator reference
6. type-system.json - Type rules
7. state-machine.yaml - Variable lifecycle
8. reserved-enums.json - Reserved enumerations
9. datetime-system.yaml - DateTime type (pg\dt)

### Reference Order (During Development)
1. Quick check: constraints.yaml (am I violating a rule?)
2. Syntax: grammar.ebnf (is this valid syntax?)
3. Types: type-system.json (is this type correct?)
4. Operators: operators.json (what does this operator do?)
5. States: state-machine.yaml (is this transition valid?)
6. DateTime: datetime-system.yaml (is this datetime literal valid?)
7. Examples: examples-annotated.pg (how do I use this pattern?)

---

## Maintenance

**Authority:** This package is canonical for v0.0.2 syntax
**Updates:** When language changes, update all 8 files
**Validation:** All examples must compile with v0.0.2 compiler

**Related Documentation:**
- Human docs: `/docs/user/` (prose, tutorials)
- Architecture: `/docs/Tech/implementation/technical/architecture.md` (implementation)
- PRD: `/docs/Agile/prd.md` (feature planning)

**This package is:** Language specification (what Polyglot IS)
**Not this package:** Implementation details (how to build it)

---

## Contact

**Questions about AI context package:**
- File: Issue in GitHub repository
- Tag: `ai-context-package`, `documentation`

**Questions about language syntax:**
- Reference: grammar.ebnf + constraints.yaml
- If unclear: File language design issue

---

**Last Updated:** 2025-11-25
**Package Version:** v0.0.2
**Maintainer:** Polyglot Language Team
