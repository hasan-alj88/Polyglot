---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/README.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Syntax Reference

**Detailed syntax references for specific language features**

---

## Overview

This directory contains focused documentation on specific syntax elements of the Polyglot programming language. Each document provides in-depth coverage of a single topic with examples and best practices.

For a comprehensive language guide, see the [Language Directory](../language/).

---

## Syntax Topics

### 🔹 [Overview](./overview.md)
High-level overview of Polyglot syntax philosophy and design principles.

**Topics:**
- Syntax design goals
- Readability and clarity
- Marker-based structure
- Indentation significance

---

### 🔹 [Block Markers](./block-markers.md)
Complete reference for execution and control flow markers.

**Covered Markers:**
- **Execution:** `[r]` (sequential), `[p]` (parallel), `[b]` (background)
- **Control Flow:** `[y]` (fork), `[v]` (join), `[m]` (match), `[?]` (case)
- **Boolean:** `[&]` (AND), `[^]` (XOR)
- **Error Handling:** `[!]` (catch), `[z]` (try)
- **I/O:** `[|]` (parameter), `[~]` (unpack), `[*]` (pack)

**Includes:**
- Syntax rules
- Usage examples
- Common patterns
- Anti-patterns

---

### 🔹 [Operators](./operators.md)
Complete operator reference with precedence and semantics.

**Operator Categories:**
- **Prefix Operators:** `|`, `#`, `!`, `~`, `*`, `$`
- **I/O Operators:** `<`, `>`, `<<`, `>>`, `<~`
- **Comparison Operators:** `=?`, `>?`, `<?`, `>=?`, `<=?`, `!=?`
- **Arithmetic:** (future)
- **Logical:** (via markers)

**Includes:**
- Operator precedence table
- Associativity rules
- Type constraints
- Examples

---

### 🔹 [Type System](./type-system.md)
Comprehensive type system documentation.

**Types Covered:**
- **Primitives:** `:pg.string`, `:pg.int`, `:pg.float`, `:pg.bool`
- **Complex:** `:pg.serial`, `:pg.array.*`, `:pg.datetime`
- **Custom:** `:#EnumType`
- **Wildcard:** `:*`

**Topics:**
- Type annotations
- Type inference
- Type conversions
- Type constraints
- Type safety

---

### 🔹 [Enumerations](./enumerations.md)
Deep dive into enum definitions and usage.

**Topics:**
- Defining enums with `{#}` blocks
- Variant subfields (no datatype)
- Data subfields (with datatype)
- Enum construction with `[.]` marker
- Pattern matching and exhaustiveness
- Implicit conversion to/from `:pg.serial`

**Examples:**
- Simple value enums
- Container enums with data
- Nested enums
- Enum matching patterns

---

### 🔹 [Error Handling](./error-handling.md)
Structured error handling patterns and best practices.

**Topics:**
- Defining custom errors with `{!}` blocks
- Catching errors with `[!]` marker
- Error fields and data
- Error propagation with `[^]`
- Wildcard errors `[!] *!`
- Error composition

**Patterns:**
- Try-catch equivalents
- Error chaining
- Recovery strategies
- Logging patterns

---

### 🔹 [Comments](./comments.md)
Code documentation and commenting conventions.

**Topics:**
- Single-line comments: `//`
- Multi-line commenting strategies
- Documentation comments
- Inline vs block comments
- Best practices

**Includes:**
- Style guidelines
- Documentation templates
- Example annotated code

---

### 🔹 [Line Continuation](./line-continuation.md)
Breaking long statements across multiple lines.

**Topics:**
- `[+]` continuation marker
- Automatic continuation rules
- Indentation with continued lines
- When to use continuations

**Examples:**
- Long pipeline calls
- Complex conditionals
- Multi-parameter calls

---

### 🔹 [Safety Mechanisms](./safety-mechanisms.md)
Built-in language safety features and guarantees.

**Topics:**
- Type safety
- Memory safety
- Concurrency safety
- Error safety
- Compile-time checks
- Runtime protections

---

## AI-Generated Content

Several files in this directory include `.ai.yaml` companion files that contain structured metadata for AI models and documentation tools:

- `block-markers.ai.yaml`
- `comments.ai.yaml`
- `enumerations.ai.yaml`
- `error-handling.ai.yaml`
- `line-continuation.ai.yaml`
- `operators.ai.yaml`
- `overview.ai.yaml`
- `type-system.ai.yaml`

These files support:
- AI code completion
- Intelligent documentation search
- Context-aware help systems
- Automated testing

---

## Quick Navigation

### By Topic Category

**Core Syntax:**
- [Overview](./overview.md)
- [Block Markers](./block-markers.md)
- [Operators](./operators.md)
- [Comments](./comments.md)

**Type System:**
- [Type System](./type-system.md)
- [Enumerations](./enumerations.md)

**Control Flow:**
- [Block Markers](./block-markers.md) (conditional execution)
- [Error Handling](./error-handling.md)

**Advanced:**
- [Line Continuation](./line-continuation.md)
- [Safety Mechanisms](./safety-mechanisms.md)

### By Experience Level

**Beginners:**
1. Overview
2. Operators
3. Comments
4. Type System

**Intermediate:**
1. Block Markers
2. Enumerations
3. Error Handling

**Advanced:**
1. Safety Mechanisms
2. Line Continuation

---

## Related Documentation

**Broader Guides:**
- [Language Documentation](../language/) - Comprehensive language guides
- [Examples](../examples/) - Practical code examples
- [Quick Reference](../quick-reference/) - Syntax cheat sheets

**Implementation:**
- [Guides](../guides/) - Migration and usage guides
- [CLI](../cli/) - Command-line tools

**Technical:**
- [Architecture](../architecture/) - Language design and implementation
- [Planning](../planning/) - Future features and roadmap

---

## Contributing

When adding new syntax documentation:

1. **Create a markdown file** for the topic
2. **Add an `.ai.yaml` companion** for AI tooling support
3. **Update this README** with a link and description
4. **Cross-reference** related topics
5. **Include examples** for all syntax elements

---

## Conventions

**File Naming:**
- Use kebab-case: `block-markers.md`
- Descriptive names: not `markers.md` but `block-markers.md`

**Document Structure:**
- Start with clear topic statement
- Provide syntax reference
- Include practical examples
- Link to related topics

**Code Examples:**
- Use complete, runnable examples
- Show both correct and incorrect usage
- Include comments explaining key points

---

**Last Updated:** 2025-12-15
**Version:** Current
**Maintained by:** Polyglot Documentation Team
