# Tag Registry - Polyglot v0.0.4

**Purpose:** Quick tag-based document lookup for BMAD agents
**Schema:** bmad-tags-v1
**Last Updated:** 2025-12-16

---

## Core Tags

### #getting-started
- [Core Principles](./User/getting-started/core-principles.md)
- Quick Reference (TBD)
- Hello World (TBD)
- Installation (TBD)

### #syntax
- [Markers](./User/language/syntax/markers.md)
- [Operators](./User/language/syntax/operators.md)
- [Prefix System](./User/language/syntax/prefix-system.md)
- [I/O Operators](./User/language/syntax/io-operators.md)

### #types
- [Type System](./User/language/types/type-system.md)
- [Enums & Serial](./User/language/types/enums-serial.md)
- [Variables Lifecycle](./User/language/types/variables-lifecycle.md)

### #control-flow
- [Pipeline Structure](./User/language/control-flow/pipeline-structure.md)

### #advanced
- [Loop System](./User/language/advanced/loop-system.md)
- [Metadata System](./User/language/advanced/metadata-system.md)
- [Reserved Indication](./User/language/advanced/reserved-indication.md)
- [Serial Load Block](./User/language/advanced/serial-load-block.md)

### #features
- [Loop System](./User/language/advanced/loop-system.md)
- [Metadata System](./User/language/advanced/metadata-system.md)
- [Reserved Indication](./User/language/advanced/reserved-indication.md)
- [Serial Load Block](./User/language/advanced/serial-load-block.md)

### #stdlib
- [Standard Library Index](./User/stdlib/index.md)
- All files under `./User/stdlib/`

### #api
- All stdlib files

### #loops
- [Loop System](./User/language/advanced/loop-system.md)
- [Unpack Operators](./User/stdlib/loops/unpack/)
- [Pack Operators](./User/stdlib/loops/pack/)

### #reference
- [Grammar (EBNF)](./User/reference/grammar.md)
- [AI Context](./User/reference/ai-context.md)

### #beginner
- [Core Principles](./User/getting-started/core-principles.md)
- Quick Reference (TBD)
- Hello World (TBD)

### #spec
- [Type System](./User/language/types/type-system.md)
- [Enums & Serial](./User/language/types/enums-serial.md)
- [Pipeline Structure](./User/language/control-flow/pipeline-structure.md)
- [Loop System](./User/language/advanced/loop-system.md)
- [Metadata System](./User/language/advanced/metadata-system.md)
- [Reserved Indication](./User/language/advanced/reserved-indication.md)
- [Serial Load Block](./User/language/advanced/serial-load-block.md)

---

## Usage

BMAD agents can query this registry to find documents by tag:

```python
# Example: Find all #beginner documents
docs = tag_registry.query("#beginner")
```

---

**Related:** [Navigation Graph](./_graph.yaml) | [Main Index](./index.md) | [Conventions](./_conventions.md)
