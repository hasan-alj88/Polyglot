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
updated: 2025-12-16
version: 0.0.4
tags:
  - "#documentation"
---
# Polyglot v0.0.4 Language Specification

**Version:** v0.0.4
**Status:** ✅ Finalized - December 2025
**Implementation Target:** Q2 2026 (Epic 1 implements v0.0.3)
**Last Updated:** 2025-12-15

---

## Quick Navigation

### 🎯 What's New in v0.0.4
- [Changes from v0.0.3](./changes-from-v0.0.3/README.md) - Complete changelog
- [Serial Load Block](./language/advanced/serial-load-block.md) ⭐ NEW - Parallel file loading
- [Variable Lifecycle](./language/types/variables-lifecycle.md) - 5-state system
- [Pipeline Structure](./language/control-flow/pipeline-structure.md) - Execution order clarified

### 🚀 Getting Started
- [Quick Reference](./quick-reference/README.md) - One-page cheat sheets
- [Hello World](./examples/hello-world.md) - Your first Polyglot program
- [Core Principles](./getting-started/core-principles.md) - Language philosophy

### 📚 Core Concepts
- [Prefix System](./language/syntax/prefix-system.md) - `$`, `:`, `#`, `|`, `!`, `@`, `%`
- [Markers Reference](./language/syntax/markers.md) - All `[r]`, `[|]`, `{|}`, etc.
- [I/O Operators](./language/syntax/io-operators.md) - `<<`, `>>`, `<~`
- [Variables & Lifecycle](./language/types/variables-lifecycle.md) - Pending → Final → Released

### ⚡ Advanced Features
- [Loop System](./language/advanced/loop-system.md) - Unpack `[~]` / Pack `[*]`
- [Reserved Indication](./language/advanced/reserved-indication.md) - Semicolon `;` system
- [Metadata System](./language/advanced/metadata-system.md) - `%` prefix annotations
- [Error Handling](./features/error-handling/error-handling.md) - Try blocks and error types

### 📖 Reference
- [Standard Library](./stdlib/index.md) - Complete package tree
- [Operators](./language/syntax/operators.md) - All operators reference
- [Complete Feature List](./features/README.md) - All 21 features categorized

### 🤝 Contributing
- [Version Index](../VERSION-INDEX.md) - All Polyglot versions
- [Design History](./design-history/README.md) - Historical decisions
- [Master Index](../../MASTER-INDEX.md) - All documentation

---

## Complete Table of Contents

### Core Syntax (9 files)
1. [Core Principles](./getting-started/core-principles.md) - Language philosophy
2. [Prefix System](./language/syntax/prefix-system.md) - All prefixes
3. [Markers Reference](./language/syntax/markers.md) - All markers
4. [I/O Operators](./language/syntax/io-operators.md) - Input/output syntax
5. [Operators Reference](./language/syntax/operators.md) - All operators
6. [Variables & Lifecycle](./language/types/variables-lifecycle.md) - 5 states
7. [Pipeline Structure](./language/control-flow/pipeline-structure.md) - Execution order
8. [Enums & Structs](./core-syntax/enums-structs.md) - Type definitions
9. [Types System](./language/types/type-system.md) - Type notation

### Features (21 files, 7 categories)

#### Core Features (4 files)
- [Loop System](./language/advanced/loop-system.md) - `[~]` and `[*]`
- [Reserved Indication](./language/advanced/reserved-indication.md) - `;` system
- [Metadata System](./language/advanced/metadata-system.md) - `%` prefix
- [Serial Load Block](./language/advanced/serial-load-block.md) ⭐ NEW - `[s]` marker

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
- [Complete Package Tree](./stdlib/index.md)
- [Utilities (`|U.*`)](./stdlib/utilities/README.md) - Math, String, DateTime, Data
- [Wrappers (`|W.*`)](./stdlib/wrappers/README.md) - Runtime integration
- [Unpack Operators (`~*`)](./stdlib/loops/unpack/README.md) - ForEach, Enumerate, etc.
- [Pack Operators (`**`)](./stdlib/loops/pack/README.md) - Into.Array, Join.All, etc.

### Quick Reference (5 files)
- [Syntax at a Glance](./quick-reference/syntax-at-a-glance.md)
- [Markers Cheat Sheet](./quick-reference/markers-cheatsheet.md)
- [Operators Cheat Sheet](./quick-reference/operators-cheatsheet.md)
- [Common Patterns](./quick-reference/common-patterns.md)
- [Migration Quick Guide](./quick-reference/migration-quick-guide.md)

### Examples (7 files)
- [Hello World](./examples/hello-world.md)
- [Variables & Assignment](./examples/variables-assignment.md)
- [Pipeline Basics](./examples/pipeline-basics.md)
- [Loops & Iteration](./examples/loops-iteration.md)
- [Error Handling](./examples/error-handling-example.md)
- [Metadata Usage](./examples/metadata-usage.md)
- [Complete Application](./examples/complete-application.md)

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
- **Variables:** [Variables & Lifecycle](./language/types/variables-lifecycle.md), [Quick Ref](./quick-reference/syntax-at-a-glance.md)
- **Pipelines:** [Pipeline Structure](./language/control-flow/pipeline-structure.md), [Examples](./examples/pipeline-basics.md)
- **Loops:** [Loop System](./language/advanced/loop-system.md), [Examples](./examples/loops-iteration.md)
- **Types:** [Types System](./language/types/type-system.md), [Enums & Structs](./core-syntax/enums-structs.md)
- **Errors:** [Error Handling](./features/error-handling/error-handling.md), [Examples](./examples/error-handling-example.md)
- **Metadata:** [Metadata System](./language/advanced/metadata-system.md), [Examples](./examples/metadata-usage.md)

### By Use Case
- **Reading Files:** [Serial Load Block](./language/advanced/serial-load-block.md)
- **Iterating Collections:** [Loop System](./language/advanced/loop-system.md)
- **Making Decisions:** [Match Expressions](./features/control-flow/match-expressions.md)
- **Handling Failures:** [Error Handling](./features/error-handling/error-handling.md)
- **String Processing:** [Multi-line Strings](./features/string-handling/multi-line-strings.md)
- **Calling External Code:** [Wrappers](./stdlib/wrappers/README.md)

### By Marker
- `[r]` - [Markers](./language/syntax/markers.md#execution-markers), [Variables](./language/types/variables-lifecycle.md)
- `[|]` - [I/O Operators](./language/syntax/io-operators.md), [Pipeline Structure](./language/control-flow/pipeline-structure.md)
- `[~]` / `[*]` - [Loop System](./language/advanced/loop-system.md)
- `[s]` - [Serial Load Block](./language/advanced/serial-load-block.md) ⭐ NEW
- `[y]` - [Markers](./language/syntax/markers.md#conditional-markers), [Control Flow](./features/control-flow/)
- `[v]` - [Loop System](./language/advanced/loop-system.md#join-operations)

### By Operator/Prefix
- `$` - [Variables & Lifecycle](./language/types/variables-lifecycle.md)
- `:` - [Types System](./language/types/type-system.md)
- `#` - [Enums & Structs](./core-syntax/enums-structs.md)
- `|` - [Standard Library](./stdlib/index.md)
- `!` - [Error Handling](./features/error-handling/error-handling.md)
- `@` - [Core Syntax](./language/syntax/prefix-system.md#registry-prefix)
- `%` - [Metadata System](./language/advanced/metadata-system.md)

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
- `[y]` - Fork/conditional

---

## Related Documentation

### User Documentation
- [User Guide](../../user/) - Practical tutorials and guides
- [Async-Centric Paradigm](../../user/async-centric-paradigm.md) - Core execution model

### Project Documentation
- [PRD](../../project/prd.md) - Product requirements
- [Epics](../../project/epics/) - Development epics
- [Stories](../../project/stories/) - User stories

### Version Information
- [Version Index](../VERSION-INDEX.md) - All versions comparison
- [Master Index](../../MASTER-INDEX.md) - Complete documentation map

### AI Context
- [AI Context Package](../../ai-context/v0.0.4/) - Machine-readable specification
- [EBNF Grammar](../../ai-context/v0.0.4/grammar.ebnf) - Formal grammar

---

## Implementation Status

**Current Implementation:** Epic 1 (v0.0.3)
**Target Implementation:** Q2 2026 (v0.0.4)
**Specification Status:** ✅ Finalized

See [Project Documentation](../../project/) for current implementation progress.

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
