# Tag Registry - Polyglot v0.0.4

**Purpose:** Quick tag-based document lookup for BMAD agents
**Schema:** bmad-tags-v1
**Last Updated:** 2025-12-16

---

## Core Tags

### #getting-started
- [Core Principles](./getting-started/core-principles.md)
- Quick Reference (TBD)
- Hello World (TBD)
- Installation (TBD)

### #syntax
- [Markers](./language/syntax/markers.md)
- [Operators](./language/syntax/operators.md)
- [Prefix System](./language/syntax/prefix-system.md)
- [I/O Operators](./language/syntax/io-operators.md)

### #types
- [Type System](./language/types/type-system.md)
- [Enums & Serial](./language/types/enums-serial.md)
- [Variables Lifecycle](./language/types/variables-lifecycle.md)

### #control-flow
- [Pipeline Structure](./language/control-flow/pipeline-structure.md)

### #advanced
- [Loop System](./language/advanced/loop-system.md)
- [Metadata System](./language/advanced/metadata-system.md)
- [Reserved Indication](./language/advanced/reserved-indication.md)
- [Serial Load Block](./language/advanced/serial-load-block.md)

### #features
- [Loop System](./language/advanced/loop-system.md)
- [Metadata System](./language/advanced/metadata-system.md)
- [Reserved Indication](./language/advanced/reserved-indication.md)
- [Serial Load Block](./language/advanced/serial-load-block.md)

### #stdlib
- [Standard Library Index](./stdlib/index.md)
- All files under `./stdlib/`

### #api
- All stdlib files

### #loops
- [Loop System](./language/advanced/loop-system.md)
- [Unpack Operators](./stdlib/loops/unpack/)
- [Pack Operators](./stdlib/loops/pack/)

### #reference
- [Grammar (EBNF)](./reference/grammar.md)
- [AI Context](./reference/ai-context.md)

### #beginner
- [Core Principles](./getting-started/core-principles.md)
- Quick Reference (TBD)
- Hello World (TBD)

### #spec
- [Type System](./language/types/type-system.md)
- [Enums & Serial](./language/types/enums-serial.md)
- [Pipeline Structure](./language/control-flow/pipeline-structure.md)
- [Loop System](./language/advanced/loop-system.md)
- [Metadata System](./language/advanced/metadata-system.md)
- [Reserved Indication](./language/advanced/reserved-indication.md)
- [Serial Load Block](./language/advanced/serial-load-block.md)

---

## Usage

BMAD agents can query this registry to find documents by tag:

```python
# Example: Find all #beginner documents
docs = tag_registry.query("#beginner")
```

---

**Related:** [Navigation Graph](./_graph.yaml) | [Main Index](./index.md) | [Conventions](./_conventions.md)
