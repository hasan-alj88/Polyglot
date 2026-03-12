# Polyglot v0.0.4 Documentation Index

**Version:** v0.0.4
**Status:** ✅ Production Ready
**Last Updated:** 2025-12-16
**Schema:** BMAD-optimized for agent consumption

---

## 📂 Documentation Structure

### 🚀 Getting Started
> **Phase:** any | **Complexity:** low | **Agents:** developer, tech-writer

Start here if you're new to Polyglot.

- [Core Principles](./User/getting-started/core-principles.md) - Language philosophy and design
- **[Inline Pipelines](./User/language/advanced/inline-pipelines.md)** - 🔥 **Most common feature** - formatted string templates
- Quick Reference (TBD) - One-page cheatsheet
- Hello World (TBD) - Your first pipeline
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
- [Markers](./User/language/syntax/markers.md) - `[r]`, `[|]`, `{|}`, etc.
- [Operators](./User/language/syntax/operators.md) - `<<`, `>>`, `|>`, etc.
- [Prefix System](./User/language/syntax/prefix-system.md) - `$`, `:`, `#`, `|`, `!`, `@`, `%`
- [I/O Operators](./User/language/syntax/io-operators.md) - Input/output wiring

#### Types (Complexity: medium)
- [Type System](./User/language/types/type-system.md) - Complete type reference
- [Enums & Serial](./User/language/types/enums-serial.md) - Enumerations and serial data
- [Variables Lifecycle](./User/language/types/variables-lifecycle.md) - 5-state variable system

#### Control Flow (Complexity: medium)
- [Pipeline Structure](./User/language/control-flow/pipeline-structure.md) - Execution model

#### Advanced Features
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
- [Design History](./_archive/design-history/) - Evolution and decisions
- [Old Files](./_archive/old-files/) - Superseded specifications
- [Meta](./_archive/meta/) - Organizational documents

---

**BMAD Optimized** | [Full Navigation Graph](./_graph.yaml) | [Tag Registry](./_tags.md) | [Conventions](./_conventions.md)
