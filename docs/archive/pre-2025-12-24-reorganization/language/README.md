# Language Documentation

## 📖 User-Friendly Language Guides

This folder contains user-friendly documentation about Polyglot language features. It complements the formal specification with practical examples and explanations.

---

## 📚 Language Documentation Locations

### Formal Specification (Authoritative)
👉 **[specifications/v0.0.4/language/](../User/specifications/v0.0.4/language/)**

The complete, authoritative language specification including:
- Syntax and operators
- Type system
- Control flow structures
- Advanced features

### User-Friendly Guides (This Folder)
👉 **[language/](./)**

Practical guides and tutorials with:
- Beginner-friendly explanations
- Additional examples
- Common patterns
- Best practices

---

## What's in This Folder?

### Advanced Topics
- [Inline Pipelines](advanced/inline-pipelines.md) - Nested pipeline expressions
- [Loop System](advanced/loop-system.md) - Pack/unpack iteration
- [Metadata System](advanced/metadata-system.md) - Inline annotations
- [Multi-line Strings](advanced/multi-line-strings.md) - String handling
- [Pipeline Inline Metadata](advanced/pipeline-inline-metadata/) - Advanced metadata patterns

### Syntax Documentation  
- [I/O Operators](syntax/io-operators.md)
- [Markers](syntax/markers.md)
- [Operators](syntax/operators.md)
- [Prefix System](syntax/prefix-system.md)

### Type System
- [Type System](types/type-system.md)
- [Enums & Serials](types/enums-serial.md)
- [Variables Lifecycle](types/variables-lifecycle.md)
- [Special Variables](types/special-variables/) - Built-in variables

### Control Flow
- [Pipeline Structure](control-flow/pipeline-structure.md)

---

## Documentation Organization

```
docs/
├── specifications/v0.0.4/
│   └── language/            ⭐ FORMAL SPEC - Authoritative reference
│       ├── syntax/
│       ├── types/
│       ├── control-flow/
│       └── advanced/
│
└── language/                💡 USER GUIDES - Practical examples
    ├── syntax/              (user-friendly explanations)
    ├── types/               (with more examples)
    ├── control-flow/        (practical patterns)
    └── advanced/            (tutorials & best practices)
```

---

## Which One Should I Use?

| Need | Use This |
|------|----------|
| Formal syntax rules | [specifications/v0.0.4/language/](../User/specifications/v0.0.4/language/) |
| Practical examples | This folder |
| Learning the language | This folder first, then specification |
| Implementing a parser | [specifications/v0.0.4/language/](../User/specifications/v0.0.4/language/) |
| Quick reference | Either (cross-referenced) |

---

**Maintained by:** Scribe Documentation System  
**Last Updated:** 2025-12-23

👉 **For formal specification:** [specifications/v0.0.4/language/](../User/specifications/v0.0.4/language/)  
👉 **For learning & examples:** Browse this folder
