# Polyglot v0.0.2 - AI Context Package

**Purpose:** Maximum-density language specification for AI agent consumption

**Version:** 0.0.2
**Created:** 2025-11-25
**Target Audience:** AI Agents, LLMs, Code Analysis Tools

---

## Overview

This package provides the complete Polyglot language specification in machine-parseable formats optimized for AI consumption. Unlike human documentation (prose, examples, tutorials), this package prioritizes:

- **Density** - Maximum information per token
- **Structure** - Queryable, parseable formats
- **Completeness** - 100% of syntax rules and semantics
- **Unambiguity** - Zero interpretation required

**Total Size:** ~20KB across 7 files
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

**Examples:**
- `"hello"` → `U.String"hello"` (implicit call)
- `DT.Now""` → returns `pg\dt` (empty param required)
- `DT.Minutes"5"` → returns `pg\dt` duration
- `"{.count:Hex}"` → calls `|U.String.Polyglot.Int.Hex`

**Invalid:** `DT.Now`, `DT.Minutes(5)`, `DT.ToNow(.start)`
**Valid:** `DT.Now""`, `DT.Minutes"5"`, `DT.ToNow"{.start}"`

**See:** `/docs/technical/string-literals-internals.md` for complete mechanics

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
1. **Inputs** `[i]` - At least one (use `[i] #Pipeline.NoInput` if none)
2. **Triggers** `[t]` - At least one (pipeline will NEVER run without triggers!)
3. **Wrapper** - `[W]` wrapper (runtime wrapper OR scope placeholder)
4. **Outputs** `[o]` - At least one (use `[o] !NoError` if no output)

**Minimal Valid Pipeline:**
```polyglot
[|] MinimalPipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope        // Placeholder: no setup/cleanup
[o] !NoError
[X]
```

**Wrapper Options:**
- **Runtime Wrappers:** `[W] RT.Python"script.py"`, `[W] RT.Rust"module"`, `[W] RT.Node"module.js"`
- **Scope Placeholder:** `[W] |W.Polyglot.Scope` or `[W] W.Polyglot.Scope""`
  - Use when no explicit setup/cleanup needed
  - Makes it explicit (not accidental omission)
  - Indicates RAII-style scope cleanup
  - Replaces old `|W.NoSetup.NoCleanup`
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
| **AI Context Package** | ~1,800 | Same spec | **5.5x** |

**Key Advantages:**
- **5.5x more compact** than prose documentation
- **100% structured** (queryable, parseable)
- **Zero ambiguity** (formal specifications)
- **Instant lookup** (JSON/YAML queries)
- **Complete coverage** (all syntax + semantics)

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

**v0.0.2** (2025-11-25)
- Initial AI context package
- Complete language specification
- All 7 reference files

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

### Reference Order (During Development)
1. Quick check: constraints.yaml (am I violating a rule?)
2. Syntax: grammar.ebnf (is this valid syntax?)
3. Types: type-system.json (is this type correct?)
4. Operators: operators.json (what does this operator do?)
5. States: state-machine.yaml (is this transition valid?)
6. Examples: examples-annotated.pg (how do I use this pattern?)

---

## Maintenance

**Authority:** This package is canonical for v0.0.2 syntax
**Updates:** When language changes, update all 7 files
**Validation:** All examples must compile with v0.0.2 compiler

**Related Documentation:**
- Human docs: `/docs/user/` (prose, tutorials)
- Architecture: `/docs/technical/architecture.md` (implementation)
- PRD: `/docs/project/prd.md` (feature planning)

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
