---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: guide
topic: Polyglot v0.0.4 Language Specification
summary: Polyglot v0.0.4 Language Specification
keywords:
  - polyglot
  - documentation

# --- BMAD Agent Routing ---
agents:
  - developer
phase: any
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  []
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-18
version: 0.0.4
tags:
  - "#documentation"
last-redoc-date: 2025-12-18
---
# Polyglot v0.0.4 Language Specification

**Version:** v0.0.4
**Status:** ✅ Finalized - December 2025
**Implementation Target:** Q2 2026 (Epic 1 implements v0.0.3)
**Last Updated:** 2025-12-18

---

## 🆕 Parser Implementation Documentation

**⭐ START HERE:** [Parser Implementation Quick Start Guide](./PARSER-IMPLEMENTATION-GUIDE.md)
**📋 PRINT THIS:** [Parser Quick Reference Card](./PARSER-QUICK-REFERENCE.md) - One-page reference

**New comprehensive guides for v0.0.4 parser implementation:**

- **[Parser Implementation Guide](./PARSER-IMPLEMENTATION-GUIDE.md)** - ⭐ Quick start with checklist and priorities
- **[Parser Quick Reference](./PARSER-QUICK-REFERENCE.md)** - 📋 Printable one-page reference card
- **[Language Overview](./User/language/README.md)** - Complete language specification directory guide
  - [Syntax Guide](./User/language/syntax/README.md) - Core syntax patterns, reserved indication, inline pipelines, indentation rules
  - [Reference Documentation](./User/reference/README.md) - Operator precedence, parsing tables, token sequence patterns
  - [Token Patterns](./User/reference/token-patterns.md) - Complete token inventory (121 tokens)
  - [Syntax Patterns](./User/reference/syntax-patterns.md) - Pattern catalog with EBNF grammar

**Critical for parser developers**: These new guides provide exhaustive documentation on:
- Reserved vs custom hierarchy (`;` vs `.`)
- Definition vs invocation patterns (`{|}` vs `[|]`)
- Indentation-based nesting (3-space rule)
- Inline pipeline syntax (formatted string requirement)
- Complete pattern catalog (20 single-line + 12 multi-line patterns)
- Context-sensitive parsing rules
- Error recovery strategies
- Implementation priorities (Phase 1-4)

---

## Quick Navigation

### 🎯 What's New in v0.0.4
- [Changes from v0.0.3](./changes-from-v0.0.3/README.md) - Complete changelog
- [Serial Load Block](./User/language/advanced/serial-load-block.md) ⭐ NEW - Parallel file loading
- [Variable Lifecycle](./User/language/types/variables-lifecycle.md) - 5-state system
- [Pipeline Structure](./User/language/control-flow/pipeline-structure.md) - Execution order clarified

### 🚀 Getting Started
- [Quick Reference](./quick-reference/README.md) - One-page cheat sheets
- [Hello World](./User/examples/hello-world.md) - Your first Polyglot program
- [Core Principles](./User/getting-started/core-principles.md) - Language philosophy

### 📚 Core Concepts
- [Prefix System](./User/language/syntax/prefix-system.md) - `$`, `:`, `#`, `|`, `!`, `@`, `%`
- [Markers Reference](./User/language/syntax/markers.md) - All `[r]`, `[|]`, `{|}`, etc.
- [I/O Operators](./User/language/syntax/io-operators.md) - `<<`, `>>`, `<~`
- [Variables & Lifecycle](./User/language/types/variables-lifecycle.md) - Pending → Final → Released

### ⚡ Advanced Features
- [Loop System](./User/language/advanced/loop-system.md) - Unpack `[~]` / Pack `[*]`
- [Reserved Indication](./User/language/advanced/reserved-indication.md) - Semicolon `;` system
- [Metadata System](./User/language/advanced/metadata-system.md) - `%` prefix annotations
- [Error Handling](./features/error-handling/error-handling.md) - Try blocks and error types

### 📖 Reference
- [Standard Library](./User/stdlib/index.md) - Complete package tree
- [Operators](./User/language/syntax/operators.md) - All operators reference
- [Complete Feature List](./features/README.md) - All 21 features categorized

### 🤝 Contributing
- [Version Index](../VERSION-INDEX.md) - All Polyglot versions
- [Design History](./design-history/README.md) - Historical decisions
- [Master Index](../../MASTER-INDEX.md) - All documentation

---

## Complete Table of Contents

### Core Syntax (9 files)
1. [Core Principles](./User/getting-started/core-principles.md) - Language philosophy
2. [Prefix System](./User/language/syntax/prefix-system.md) - All prefixes
3. [Markers Reference](./User/language/syntax/markers.md) - All markers
4. [I/O Operators](./User/language/syntax/io-operators.md) - Input/output syntax
5. [Operators Reference](./User/language/syntax/operators.md) - All operators
6. [Variables & Lifecycle](./User/language/types/variables-lifecycle.md) - 5 states
7. [Pipeline Structure](./User/language/control-flow/pipeline-structure.md) - Execution order
8. [Enums & Structs](./core-syntax/enums-structs.md) - Type definitions
9. [Types System](./User/language/types/type-system.md) - Type notation

### Features (21 files, 7 categories)

#### Core Features (4 files)
- [Loop System](./User/language/advanced/loop-system.md) - `[~]` and `[*]`
- [Reserved Indication](./User/language/advanced/reserved-indication.md) - `;` system
- [Metadata System](./User/language/advanced/metadata-system.md) - `%` prefix
- [Serial Load Block](./User/language/advanced/serial-load-block.md) ⭐ NEW - `[s]` marker

#### Data Structures (3 files)
- [Enums with Fields](./features/data-structures/enums-with-fields.md)
- [Struct Shorthand](./features/data-structures/struct-shorthand.md)
- [Collection Literals](./features/data-structures/collection-literals.md)

#### Control Flow (3 files)
- [Match Expressions](./features/control-flow/match-expressions.md)
- [Early Return](./features/control-flow/early-return.md)
- [Boolean Markers](./features/control-flow/boolean-markers.md)

#### String Handling (2 files)
- [Multi-line Strings](./features/string-handling/multi-line-strings.md)
- [Inline Pipelines](./features/string-handling/inline-pipelines.md)

#### Operators (3 files)
- [Range Operators](./features/operators/range-operators.md)
- [Operator Negation](./features/operators/operator-negation.md)
- [Collection Membership](./features/operators/collection-membership.md)

#### Pipeline Features (3 files)
- [Pipeline Composition](./features/pipeline-features/pipeline-composition.md)
- [Variadic Input](./features/pipeline-features/variadic-input.md)
- [Trigger OR](./features/pipeline-features/trigger-or.md)

#### Error Handling (1 file)
- [Error Handling](./features/error-handling/error-handling.md)

### Standard Library (80-100 files)
- [Complete Package Tree](./User/stdlib/index.md)
- [Utilities (`|U.*`)](./User/stdlib/utilities/README.md) - Math, String, DateTime, Data
- [Wrappers (`|W.*`)](./User/stdlib/wrappers/README.md) - Runtime integration
- [Unpack Operators (`~*`)](./User/stdlib/loops/unpack/README.md) - ForEach, Enumerate, etc.
- [Pack Operators (`**`)](./User/stdlib/loops/pack/README.md) - Into.Array, Join.All, etc.

### Quick Reference (5 files)
- [Syntax at a Glance](./quick-reference/syntax-at-a-glance.md)
- [Markers Cheat Sheet](./quick-reference/markers-cheatsheet.md)
- [Operators Cheat Sheet](./quick-reference/operators-cheatsheet.md)
- [Common Patterns](./quick-reference/common-patterns.md)
- [Migration Quick Guide](./quick-reference/migration-quick-guide.md)

### Examples (7 files)
- [Hello World](./User/examples/hello-world.md)
- [Variables & Assignment](./User/examples/variables-assignment.md)
- [Pipeline Basics](./User/examples/pipeline-basics.md)
- [Loops & Iteration](./User/examples/loops-iteration.md)
- [Error Handling](./User/examples/error-handling-example.md)
- [Metadata Usage](./User/examples/metadata-usage.md)
- [Complete Application](./User/examples/complete-application.md)

### Changes from v0.0.3 (2 files)
- [Overview](./changes-from-v0.0.3/README.md)
- [Syntax Changes](./changes-from-v0.0.3/syntax-changes.md)

### Design History (3 READMEs)
- [Overview](./design-history/README.md)
- [Loop System Evolution](./design-history/loop-system/README.md)
- [Syntax Refinement](./design-history/syntax-refinement/README.md)

---

## Finding Information by Topic

### By Syntax Element
- **Variables:** [Variables & Lifecycle](./User/language/types/variables-lifecycle.md), [Quick Ref](./quick-reference/syntax-at-a-glance.md)
- **Pipelines:** [Pipeline Structure](./User/language/control-flow/pipeline-structure.md), [Examples](./User/examples/pipeline-basics.md)
- **Loops:** [Loop System](./User/language/advanced/loop-system.md), [Examples](./User/examples/loops-iteration.md)
- **Types:** [Types System](./User/language/types/type-system.md), [Enums & Structs](./core-syntax/enums-structs.md)
- **Errors:** [Error Handling](./features/error-handling/error-handling.md), [Examples](./User/examples/error-handling-example.md)
- **Metadata:** [Metadata System](./User/language/advanced/metadata-system.md), [Examples](./User/examples/metadata-usage.md)

### By Use Case
- **Reading Files:** [Serial Load Block](./User/language/advanced/serial-load-block.md)
- **Iterating Collections:** [Loop System](./User/language/advanced/loop-system.md)
- **Making Decisions:** [Match Expressions](./features/control-flow/match-expressions.md)
- **Handling Failures:** [Error Handling](./features/error-handling/error-handling.md)
- **String Processing:** [Multi-line Strings](./features/string-handling/multi-line-strings.md)
- **Calling External Code:** [Wrappers](./User/stdlib/wrappers/README.md)

### By Marker
- `[r]` - [Markers](./User/language/syntax/markers.md#execution-markers), [Variables](./User/language/types/variables-lifecycle.md)
- `[|]` - [I/O Operators](./User/language/syntax/io-operators.md), [Pipeline Structure](./User/language/control-flow/pipeline-structure.md)
- `[~]` / `[*]` - [Loop System](./User/language/advanced/loop-system.md)
- `[s]` - [Serial Load Block](./User/language/advanced/serial-load-block.md) ⭐ NEW
- `[f]` - [Markers](./User/language/syntax/markers.md#conditional-markers), [Control Flow](./features/control-flow/)
- `[v]` - [Loop System](./User/language/advanced/loop-system.md#join-operations)

### By Operator/Prefix
- `$` - [Variables & Lifecycle](./User/language/types/variables-lifecycle.md)
- `:` - [Types System](./User/language/types/type-system.md)
- `#` - [Enums & Structs](./core-syntax/enums-structs.md)
- `|` - [Standard Library](./User/stdlib/index.md)
- `!` - [Error Handling](./features/error-handling/error-handling.md)
- `@` - [Core Syntax](./User/language/syntax/prefix-system.md#registry-prefix)
- `%` - [Metadata System](./User/language/advanced/metadata-system.md)

---

## Key Concepts at a Glance

### Core Principles
1. **No Keywords** - Only markers and operators
2. **One Line = One Marker + One Expression** - No semicolon separators
3. **Indentation for Nesting** - 3-space indentation
4. **Universal Hierarchy** - `PREFIX.identifier.path` everywhere
5. **Explicit Over Implicit** - Metadata makes intent clear
6. **Variable Prefix: `$`** - Clear, unambiguous, greppable

### Variable Lifecycle (5 States)
```
Pending → Default → Final → Released
    ↓                 ↓
  Faulted → Released
```

### Pipeline Execution Order
```
1. Inputs (implicit triggers)
2. Trigger [t]
3. Queue [Q]
4. Wrapper [W]
5. Logic
6. Outputs
```

### Critical Operators/Prefixes
- `$` - Variable (e.g., `$user`)
- `:` - Type (e.g., `:pg.string`)
- `#` - Enum/Struct (e.g., `#OrderStatus.Processing`)
- `|` - Pipeline (e.g., `|Database.Users.Find`)
- `!` - Error (e.g., `!Network.HTTP.Timeout`)
- `@` - Registry (e.g., `@Local::MyApp:1.0.0.0`)
- `%` - Metadata (e.g., `%Doc`)

### Critical Markers
- `[r]` - Sequential execution
- `[p]` - Parallel execution
- `[|]` - Pipeline I/O
- `[~]` - Unpack (main → iteration)
- `[*]` - Pack (iteration → main)
- `[s]` - Serial load block ⭐ NEW
- `[v]` - Join operation
- `[f]` - Fork/conditional

---

## Related Documentation

### User Documentation
- [User Guide](../../user/) - Practical tutorials and guides
- [Async-Centric Paradigm](../../user/async-centric-paradigm.md) - Core execution model

### Project Documentation
- [PRD](../../Agile/prd.md) - Product requirements
- [Epics](../../Agile/epics/) - Development epics
- [Stories](../../Agile/stories/) - User stories

### Version Information
- [Version Index](../VERSION-INDEX.md) - All versions comparison
- [Master Index](../../MASTER-INDEX.md) - Complete documentation map

### AI Context
- [AI Context Package](../../Tech/ai-context/v0.0.4/) - Machine-readable specification
- [EBNF Grammar](../../Tech/ai-context/v0.0.4/grammar.ebnf) - Formal grammar

---

## Implementation Status

**Current Implementation:** Epic 1 (v0.0.3)
**Target Implementation:** Q2 2026 (v0.0.4)
**Specification Status:** ✅ Finalized

See [Project Documentation](../../Agile/) for current implementation progress.

---

## Contributing & Feedback

For questions, suggestions, or bug reports:
- **Issues:** [GitHub Issues](https://github.com/polyglot/issues)
- **Documentation:** This specification is the source of truth
- **Design Decisions:** See [Design History](./design-history/README.md)

---

**Last Updated:** 2025-12-15
**Maintained By:** Polyglot Documentation Team
**License:** See project LICENSE file
