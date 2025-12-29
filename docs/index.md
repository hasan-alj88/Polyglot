# Polyglot v0.0.4 Documentation Index

**Version:** v0.0.4
**Status:** ✅ Production Ready
**Last Updated:** 2025-12-29
**Schema:** BMAD-optimized for agent consumption

---

## 📂 Documentation Structure

### 🚀 Getting Started
> **Phase:** any | **Complexity:** low | **Agents:** developer, tech-writer

Start here if you're new to Polyglot.

- [Core Principles](./User/getting-started/core-principles.md) - Language philosophy and design
- **[Hello World Tutorial](./User/getting-started/hello-world.md)** - 🎯 **START HERE** - Multi-language pipeline orchestration
- **[Inline Pipelines](./User/language/advanced/inline-pipelines.md)** - 🔥 **Most common feature** - formatted string templates
- Quick Reference (TBD) - One-page cheatsheet
- Installation (TBD) - Setup guide

**Quick Example - Inline Pipeline Calls:**
```polyglot
[r] $sum :pg.int << |U.Math.Add"{$x}, {$y}"          // Math utility
[r] $upper :pg.string << |U.String.Upper"{$text}"    // String utility
[r] $now :pg.string << |DT.Now""                     // DateTime utility
```

---

### 📖 Language Reference
> **Phase:** planning, solutioning | **Complexity:** low-high | **Agents:** architect, developer

#### Syntax (Complexity: medium)
- **[Operators Reference](./User/language/syntax/operators.md)** - 📕 Complete guide: `<<`, `<~`, `>>`, `~>`, variable lifecycle
- [Markers](./User/language/syntax/markers.md) - `[r]`, `[|]`, `{|}`, etc.
- [Prefix System](./User/language/syntax/prefix-system.md) - `$`, `:`, `#`, `|`, `!`, `@`, `%`
- [I/O Operators](./User/language/syntax/io-operators.md) - Input/output wiring

#### Types (Complexity: medium)
- [Type System](./User/language/types/type-system.md) - Complete type reference
- **[Enum Syntax Guide](./User/language/types/enums.md)** - 📗 v0.0.4 syntax, aliases, custom extensions, migration
- **[Enum Definitions](./User/language/types/enum-definitions.md)** - 📗 `{#}` blocks, serial load, field accessors, configuration-driven enums (NEW!)
- [Enums & Serial](./User/language/types/enums-serial.md) - Enumerations and serial data
- [Variables Lifecycle](./User/language/types/variables-lifecycle.md) - 5-state variable system

#### Control Flow (Complexity: medium)
- [Pipeline Structure](./User/language/control-flow/pipeline-structure.md) - Execution model
- **[Fork Patterns](./User/language/control-flow/fork-patterns.md)** - 📙 Conditional execution, exhaustiveness, wildcard `[f] *?`
- **[Parallel Execution](./User/language/control-flow/parallel-execution.md)** - 📙 `[p]` marker, race conditions, performance
- **[Loops](./User/language/control-flow/loops.md)** - 📘 Unpack/pack operators, iteration, collection (NEW!)

#### Error Handling (Complexity: medium)
- **[Error Handling Basics](./User/language/error-handling/basics.md)** - 📕 Error types, faulted states, fork-based handling
- **[Error Blocks](./User/language/error-handling/error-blocks.md)** - 📕 Pattern matching, inline conversion (NEW!)

#### Triggers (Complexity: medium)
- **[Trigger I/O Wiring](./User/language/triggers/io-wiring.md)** - 📘 Trigger outputs, pipeline input wiring, reactive patterns (NEW!)

#### Advanced Features
- **[Pipeline Composition](./User/language/advanced/pipeline-composition.md)** - 📗 Chain pipelines with `|>` operator (NEW!)
- **[Inline Pipelines](./User/language/advanced/inline-pipelines.md)** - 🔥 **Most common** - Formatted string templates (Complexity: medium)
- [Loop System](./User/language/advanced/loop-system.md) - Unpack `[~]` / Pack `[*]` (Complexity: high)
- [Metadata System](./User/language/advanced/metadata-system.md) - `%` annotations (Complexity: medium)
- [Reserved Indication](./User/language/advanced/reserved-indication.md) - Semicolon `;` system (Complexity: medium)
- [Serial Load Block](./User/language/advanced/serial-load-block.md) - Parallel file loading (Complexity: high)

---

### 🔧 Standard Library
> **Phase:** implementation | **Complexity:** low-medium | **Agents:** developer

- [Overview](./User/stdlib/index.md) - Complete stdlib reference

#### Loops
- [Unpack Operators](./User/stdlib/loops/unpack/) - ForEach, Iter, Zip
- [Pack Operators](./User/stdlib/loops/pack/) - Collection, Math aggregation

#### Utilities
- [Data](./User/stdlib/utilities/data/) - JSON, YAML, TOML, XML
- [DateTime](./User/stdlib/utilities/datetime/) - Date/time manipulation
- [Math](./User/stdlib/utilities/math/) - Mathematical operations
- [String](./User/stdlib/utilities/string/) - String utilities

#### Wrappers
- [Runtime Wrappers](./User/stdlib/wrappers/) - Execution context control

---

### 📚 Guides
> **Phase:** any | **Complexity:** low-medium | **Agents:** all

Coming soon:
- Best Practices
- Common Patterns
- Migration Guide (v0.0.3 → v0.0.4)
- Troubleshooting

---

### 📑 Reference
> **Phase:** any | **Complexity:** medium | **Agents:** architect, developer

- [Grammar (EBNF)](./User/reference/grammar.md) - Complete language grammar
- [AI Context](./User/reference/ai-context.md) - AI-specific guidance
- Complete Syntax Reference (TBD)
- Changelog (TBD)

---

## 🤖 BMAD Agent Quick Links

### For Developers
- **Getting Started:** [Core Principles](./User/getting-started/core-principles.md)
- **Quick Reference:** [Syntax](./User/language/syntax/), [Stdlib](./User/stdlib/index.md)
- **Implementation:** [Standard Library](./User/stdlib/index.md), [Types](./User/language/types/)

### For Architects
- **Planning:** [Language Reference](./User/language/), [Advanced Features](./User/language/advanced/)
- **Design:** [Type System](./User/language/types/type-system.md), [Pipeline Structure](./User/language/control-flow/pipeline-structure.md)
- **Reference:** [Grammar](./User/reference/grammar.md)

### For Product Managers
- **Overview:** [Core Principles](./User/getting-started/core-principles.md)
- **Features:** [Language Overview](./User/language/), Changelog (TBD)

---

## 📊 Navigation by Workflow

- **Greenfield Projects:** getting-started → language → stdlib
- **Bug Fixes:** stdlib → language/syntax
- **New Features:** language → stdlib
- **Refactoring:** language/advanced → guides/best-practices

---

## 📈 Navigation by Phase

- **Analysis:** getting-started, reference
- **Planning:** language, guides
- **Solutioning:** language/advanced, stdlib
- **Implementation:** stdlib, language/syntax

---

## 📦 Archived Content

Historical design decisions and superseded documentation:
- [Archive Index](./archive/) - Archived documentation

---

**BMAD Optimized** | [Full Navigation Graph](./_graph.yaml) | [Tag Registry](./_tags.md) | [Conventions](./_conventions.md)
