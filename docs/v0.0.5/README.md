# Polyglot v0.0.5 Documentation

**Version:** 0.0.5
**Status:** Official Release
**Last Updated:** 2026-01-11

---

## 🚀 Quick Start

**New to Polyglot?** Start here:

1. **[What's New in v0.0.5](./whats-new-v0.0.5.md)** - Overview of major features and improvements
2. **[Hello World Example](./examples/hello-world-multi-runtime.pg)** - Complete working example
3. **[Loop System Guide](./language/loop-system.md)** - Learn iteration patterns
4. **[Runtime Orchestration](./quick-reference/runtime-orchestration.md)** - Execute Python, Rust, JS code

**Migrating from v0.0.4?**

- **[Migration Guide](./migration-guide-v0.0.4-to-v0.0.5.md)** - Step-by-step conversion guide

---

## 📚 Documentation Structure

### Language Guides

Core language features and concepts:

| Guide | Description |
|-------|-------------|
| **[Loop System](./language/loop-system.md)** | Unpack/pack operators, iteration patterns |
| **[Wrapper System](./language/wrapper-system.md)** | Resource management, DB, HTTP, runtime wrappers ⭐ NEW |
| **[Variable Lifecycle](./language/variable-lifecycle.md)** | Immutability, default vs final states |
| **[Error Handling](./language/error-handling.md)** | Error blocks, exhaustiveness, patterns |

### Quick References

Fast lookups for common tasks:

| Reference | Description |
|-----------|-------------|
| **[Runtime Orchestration](./quick-reference/runtime-orchestration.md)** | Python, Rust, JavaScript integration |

### Standard Library

Complete stdlib documentation:

| Document | Description |
|----------|-------------|
| **[Stdlib Overview](./stdlib/README.md)** | Index of all stdlib components |
| **[Reserved Enums](./stdlib/reserved-enums.yaml)** | Stdlib enum types and values |
| **[Standard Wrappers](./stdlib/standard-wrappers.yaml)** | Runtime, DB, HTTP, file wrappers |
| **[Standard Pipelines](./stdlib/standard-pipelines.yaml)** | Core pipelines and utilities |
| **[Standard Operators](./stdlib/standard-operators.yaml)** | Pack/unpack operators |
| **[Standard Triggers](./stdlib/standard-triggers.yaml)** | CLI, file, scheduled triggers |

#### Schemas

Reserved schema specifications:

| Schema | Description |
|--------|-------------|
| **[DB Settings](./stdlib/schemas/db-settings.yaml)** | Database connection configuration |

### Style Guide

Code conventions and best practices:

| Guide | Description |
|-------|-------------|
| **[Field Naming Conventions](./style-guide/field-naming-conventions.md)** | Underscores vs dashes, patterns |

### Examples

Complete working examples:

| Example | Description |
|---------|-------------|
| **[Hello World Multi-Runtime](./examples/hello-world-multi-runtime.pg)** | Python, Rust, JS orchestration |

### Training Materials

Learning through corrections and examples:

| Session | Topics Covered |
|---------|----------------|
| **[Training Sessions Index](./training-sessions/README.md)** | Overview of all training sessions |
| **[Session 001](./training-sessions/session-001-2026-01-02.md)** | 6 examples + runtime orchestration |

---

## 🎯 Documentation by Topic

### Getting Started

- [What's New in v0.0.5](./whats-new-v0.0.5.md)
- [Migration Guide (v0.0.4 → v0.0.5)](./migration-guide-v0.0.4-to-v0.0.5.md)
- [Hello World Example](./examples/hello-world-multi-runtime.pg)

### Core Language

- [Loop System Guide](./language/loop-system.md) - **NEW in v0.0.5**
- [Variable Lifecycle](./language/variable-lifecycle.md)
- [Error Handling](./language/error-handling.md)

### Advanced Features

- [Runtime Orchestration](./quick-reference/runtime-orchestration.md) - **NEW in v0.0.5**
- [Reserved Schemas](./stdlib/schemas/db-settings.yaml) - **NEW in v0.0.5**

### Reference

- [Stdlib Overview](./stdlib/README.md)
- [Reserved Enums](./stdlib/reserved-enums.yaml)
- [Standard Wrappers](./stdlib/standard-wrappers.yaml)
- [Standard Pipelines](./stdlib/standard-pipelines.yaml)
- [Standard Operators](./stdlib/standard-operators.yaml)
- [Standard Triggers](./stdlib/standard-triggers.yaml)

### Best Practices

- [Field Naming Conventions](./style-guide/field-naming-conventions.md)

---

## 🌟 New in v0.0.5

### Major Features

1. **Loop System** - Unpack (`~`) and pack (`*`) operators for iteration
   - Collection building: `*Into.Array`, `*Into.Set`
   - Aggregation: `*Aggregate.Sum`, `*Count`, `*Max`, etc.
   - String operations: `*String.Concat`, `*String.Lines`

2. **Runtime Orchestration** - Execute Python, Rust, JavaScript code
   - Runtime wrappers: `|W.RT.Python`, `|W.RT.Rust`, `|W.RT.JavaScript`
   - Code execution pipelines: `|RT.{Language}.Code`
   - Environment variable passing

3. **Collection Literals** - Inline collections
   - Arrays: `( 1, 2, 3 )`
   - Sets: `{ "a", "b", "c" }`
   - Serials: `{:}` or `{ .key: value }`

4. **Code Block Marker** - Cleaner multi-line code
   - New `[c]` marker for code blocks
   - `[+]` still supported for compatibility

5. **Reserved Schemas** - Type-safe configuration
   - `-DB-Settings` for database configuration
   - `-RT-Environment-{Language}` for runtimes
   - Compile-time field validation

### Syntax Improvements

- **Reserved enum prefix:** `-` instead of `#` (clearer distinction)
- **Field naming:** Underscores required (no dashes)
- **I/O markers:** ` | ` instead of `(|)` (cleaner)
- **Comments:** `%%` and `%{ }%` (Polyglot-specific)
- **DateTime type:** `:dt` instead of `:datetime` (shorter)
- **Boolean:** `-True`/`-False` (consistent with reserved enums)

### Developer Experience

- Comprehensive loop system documentation
- Runtime orchestration quick reference
- Migration guide with automation scripts
- Training sessions with 7 documented examples
- Better error messages and validation

---

## 📖 Learning Paths

### Path 1: New to Polyglot

1. Read [What's New in v0.0.5](./whats-new-v0.0.5.md)
2. Study [Hello World Example](./examples/hello-world-multi-runtime.pg)
3. Learn [Loop System](./language/loop-system.md)
4. Explore [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
5. Review [Training Sessions](./training-sessions/README.md)

### Path 2: Migrating from v0.0.4

1. Read [What's New in v0.0.5](./whats-new-v0.0.5.md)
2. Follow [Migration Guide](./migration-guide-v0.0.4-to-v0.0.5.md)
3. Review [Field Naming Conventions](./style-guide/field-naming-conventions.md)
4. Update your code using migration scripts
5. Explore new features (loops, runtime orchestration)

### Path 3: Advanced User

1. Deep dive into [Loop System Guide](./language/loop-system.md)
2. Master [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
3. Study [Reserved Schemas](./stdlib/schemas/db-settings.yaml)
4. Review [Training Sessions](./training-sessions/session-001-2026-01-02.md) for patterns
5. Explore [Stdlib Documentation](./stdlib/README.md)

---

## 🔍 Finding What You Need

### By Feature

- **Loops/Iteration:** [Loop System Guide](./language/loop-system.md)
- **Multi-language code:** [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
- **Error handling:** [Error Handling Guide](./language/error-handling.md)
- **Variables:** [Variable Lifecycle](./language/variable-lifecycle.md)
- **Database setup:** [DB Settings Schema](./stdlib/schemas/db-settings.yaml)
- **Code style:** [Field Naming Conventions](./style-guide/field-naming-conventions.md)

### By Task

- **Process collections:** [Loop System - Unpack Operators](./language/loop-system.md#unpack-operators)
- **Aggregate data:** [Loop System - Pack Operators](./language/loop-system.md#pack-operators)
- **Run Python code:** [Runtime Orchestration - Python](./quick-reference/runtime-orchestration.md#python)
- **Handle errors:** [Error Handling Guide](./language/error-handling.md)
- **Create enums:** [Reserved Enums Reference](./stdlib/reserved-enums.yaml)

### By Example

- **Complete pipeline:** [Hello World Multi-Runtime](./examples/hello-world-multi-runtime.pg)
- **Loop patterns:** [Loop System - Complete Examples](./language/loop-system.md#complete-examples)
- **Error handling:** [Training Session 001](./training-sessions/session-001-2026-01-02.md)

---

## 📊 Documentation Metrics

**Current Status:**

| Category | Count | Status |
|----------|-------|--------|
| Language Guides | 3 | ✅ Complete |
| Quick References | 1 | ✅ Complete |
| Stdlib Docs | 6 | ✅ Complete |
| Schemas | 1 | 🔄 Growing |
| Style Guides | 1 | ✅ Complete |
| Examples | 1 | 🔄 Growing |
| Training Sessions | 1 | ✅ Complete |

**Documentation Lines:** ~10,000+ lines total

---

## 🤝 Contributing to Documentation

Found an issue or want to improve the docs?

### Reporting Issues

- Unclear explanations
- Missing examples
- Outdated information
- Broken links

### Suggesting Improvements

- Additional examples
- More detailed explanations
- Code snippets
- Diagrams and visualizations

### Style Guidelines

When contributing documentation:

1. **Use clear, concise language**
2. **Include working examples**
3. **Follow existing formatting**
4. **Add cross-references**
5. **Update indexes**

---

## 🔄 Version History

### v0.0.5 (2026-01-04) - Current

**Major Features:**
- Loop system (unpack/pack operators)
- Runtime orchestration (Python, Rust, JavaScript)
- Collection literals
- Code block marker `[c]`
- Reserved schemas

**Documentation:**
- Loop System Guide
- Runtime Orchestration Quick Reference
- What's New in v0.0.5
- Migration Guide
- Field Naming Conventions
- Training Sessions

### v0.0.4 (Previous)

**Features:**
- Basic syntax and markers
- Variable lifecycle
- Error handling
- Wrapper system

**Known Issues:**
- Ambiguous field naming with dashes
- Mixed reserved enum prefixes
- No loop system

---

## 📞 Support & Resources

### Getting Help

- Review documentation (you're here!)
- Check [Training Sessions](./training-sessions/README.md) for examples
- Study [Hello World Example](./examples/hello-world-multi-runtime.pg)

### Reporting Bugs

Include:
- Polyglot version (v0.0.5)
- Code sample
- Expected vs actual behavior
- Error messages

### Feature Requests

Share ideas for:
- Language features
- Stdlib additions
- Documentation improvements
- Tooling enhancements

---

## 🎓 Learning Resources

### Beginner

1. [What's New in v0.0.5](./whats-new-v0.0.5.md)
2. [Hello World Example](./examples/hello-world-multi-runtime.pg)
3. [Variable Lifecycle](./language/variable-lifecycle.md)

### Intermediate

1. [Loop System Guide](./language/loop-system.md)
2. [Error Handling](./language/error-handling.md)
3. [Training Session 001](./training-sessions/session-001-2026-01-02.md)

### Advanced

1. [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
2. [Reserved Schemas](./stdlib/schemas/db-settings.yaml)
3. [Stdlib Complete Reference](./stdlib/README.md)

---

## ✅ Documentation Completeness

### Core Language: 95% Complete

- ✅ Loop system
- ✅ Variable lifecycle
- ✅ Error handling
- 🔄 Enum definitions (partial)
- 🔄 Trigger system (partial)
- 🔄 Wrapper system (runtime only)

### Standard Library: 90% Complete

- ✅ Reserved enums
- ✅ Standard wrappers (runtime focus)
- ✅ Standard pipelines (core + runtime)
- ✅ Standard operators (pack/unpack)
- ✅ Standard triggers (basic)
- ✅ Schemas (DB settings)

### Examples & Tutorials: 70% Complete

- ✅ Hello World multi-runtime
- ✅ Training session examples
- 🔄 More use cases needed
- 🔄 Tutorial series

### Reference: 85% Complete

- ✅ Quick references (runtime)
- ✅ Style guides (field naming)
- 🔄 Complete API reference
- 🔄 Language specification

---

## 🎯 Roadmap

### Short Term

- [ ] Additional examples (file processing, data pipelines)
- [ ] Complete trigger system guide
- [ ] Complete wrapper system guide
- [ ] Enum definitions guide

### Medium Term

- [ ] Tutorial series (beginner to advanced)
- [ ] Video walkthroughs
- [ ] Interactive examples
- [ ] IDE integration guide

### Long Term

- [ ] Complete language specification
- [ ] Architecture documentation
- [ ] Performance tuning guide
- [ ] Security best practices

---

## 📄 License & Attribution

**Documentation License:** MIT
**Version:** 0.0.5
**Maintained By:** Polyglot Documentation Team
**Last Updated:** 2026-01-04

---

**Welcome to Polyglot v0.0.5! Happy coding! 🚀**

---

## Quick Links

- [What's New](./whats-new-v0.0.5.md)
- [Migration Guide](./migration-guide-v0.0.4-to-v0.0.5.md)
- [Loop System](./language/loop-system.md)
- [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
- [Stdlib Index](./stdlib/README.md)
- [Hello World Example](./examples/hello-world-multi-runtime.pg)
