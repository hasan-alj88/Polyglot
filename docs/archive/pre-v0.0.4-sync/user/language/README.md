---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/language/README.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Language Documentation

**Comprehensive guides to Polyglot programming language features and syntax**

---

## Getting Started

### 📘 [00 - Quick Start](./00-quick-start.md)
Jump right in with a quick introduction to Polyglot basics. Perfect for first-time users.

**Topics:**
- Installation and setup
- Your first Polyglot program
- Basic syntax overview
- Running Polyglot code

---

## Core Language Features

### 📗 [01 - Syntax Complete](./01-syntax-complete.md)
Complete reference for Polyglot syntax elements, markers, and code structure.

**Topics:**
- Markers and their purposes
- Block structures
- Indentation rules
- Statement types

### 📙 [02 - Type System](./02-type-system.md)
Understanding Polyglot's type system and type annotations.

**Topics:**
- Primitive types (`:pg.string`, `:pg.int`, `:pg.float`, `:pg.bool`)
- Complex types (`:pg.serial`, `:pg.array.*`, `:pg.datetime`)
- Type inference
- Type annotations syntax

### 📕 [03 - Enumerations](./03-enumerations.md)
Working with enums (`#Enum`) and structured data.

**Topics:**
- Defining enums with `{#}` blocks
- Variant subfields vs data subfields
- Enum construction and access
- Exhaustive matching requirements
- Serial data interoperability

### 📔 [04 - Error Handling](./04-error-handling.md)
Structured error handling with typed errors.

**Topics:**
- Defining custom errors with `{!}` blocks
- Catching errors with `[!]` marker
- Error propagation
- Wildcard error handling `[!] *!`
- Best practices

### 📓 [05 - Operators](./05-operators.md)
Complete guide to Polyglot's operators.

**Topics:**
- Prefix operators (`|`, `#`, `!`, `~`, `*`, `$`)
- I/O operators (`<`, `>`, `<<`, `>>`, `<~`)
- Comparison operators (`=?`, `>?`, `<?`, `>=?`, `<=?`, `!=?`)
- Operator precedence

### 📘 [06 - Block Markers](./06-block-markers.md)
Understanding execution flow markers.

**Topics:**
- Execution markers (`[r]`, `[p]`, `[b]`, `[y]`, `[v]`)
- Boolean markers (`[&]`, `[^]`)
- Control flow markers (`[m]`, `[?]`, `[!]`, `[z]`)
- I/O markers (`[|]`, `[~]`, `[*]`)

---

## Advanced Topics

### 📗 [07 - DateTime System](./07-datetime-system.md)
Working with dates, times, and durations.

**Topics:**
- `:pg.datetime` type
- DateTime literals and parsing
- Formatting and localization
- Timezone handling
- DateTime arithmetic

### 📙 [07 - Macros](./07-macros.md)
(If applicable) Code generation and compile-time transformations.

### 📕 [08 - Line Continuation](./08-line-continuation.md)
Breaking long statements across multiple lines.

**Topics:**
- `[+]` continuation marker
- Indentation with continuations
- Best practices

### 📔 [08 - Parallel Execution](./08-parallel-execution.md)
Concurrent and parallel programming in Polyglot.

**Topics:**
- `[p]` parallel marker
- `[b]` background marker
- Race conditions and safety
- Synchronization patterns

### 📓 [09 - Expansion Operator](./09-expansion-operator.md)
(If applicable) Expanding collections and iterables.

### 📘 [10 - Pipeline Lifecycle](./10-pipeline-lifecycle.md)
Understanding pipeline execution and state management.

**Topics:**
- Pipeline triggers (`[t]`)
- Queue configurations (`[Q]`)
- Wrappers (`[W]`)
- Pipeline lifecycle phases
- State management

### 📗 [11 - Comments](./11-comments.md)
Documentation and code comments.

**Topics:**
- Single-line comments (`//`)
- Documentation comments
- Best practices

### 📙 [12 - BNF Grammar](./12-bnf-grammar.md)
Formal grammar specification in Backus-Naur Form.

**Topics:**
- Complete BNF grammar
- Parser implementation reference
- Syntax validation rules

---

## Reference Materials

### 📕 [Variables User Guide](./variables-user-guide.md)
Comprehensive guide to variable usage and scope.

**Topics:**
- Variable declaration with `$`
- Scope rules
- Variable lifecycle
- Best practices

### 📂 [BNF Grammar Directory](./bnf/)
Additional BNF and EBNF grammar resources.

---

## Reading Order

### For Beginners:
1. **Quick Start** (00) - Get started quickly
2. **Syntax Complete** (01) - Learn the basics
3. **Type System** (02) - Understand types
4. **Operators** (05) - Learn operators
5. **Block Markers** (06) - Master execution flow

### For Intermediate Users:
1. **Enumerations** (03) - Structured data
2. **Error Handling** (04) - Robust programs
3. **DateTime System** (07) - Time handling
4. **Parallel Execution** (08) - Concurrency

### For Advanced Users:
1. **Pipeline Lifecycle** (10) - Deep dive into pipelines
2. **BNF Grammar** (12) - Formal specification
3. **Macros** (07) - Advanced metaprogramming

---

## Quick Reference

### Essential Markers

**Execution:**
- `[r]` - Sequential execution
- `[p]` - Parallel execution
- `[b]` - Background execution
- `[y]` - Fork (conditional)
- `[v]` - Join (merge)

**Boolean:**
- `[&]` - AND continuation
- `[^]` - XOR continuation

**Control Flow:**
- `[m]` - Match expression
- `[?]` - Match case
- `[!]` - Error handler
- `[z]` - Try block

**I/O:**
- `[|]` - Pipeline I/O parameter
- `[~]` - Unpack (loop input)
- `[*]` - Pack (loop output)

### Essential Operators

**Prefix:**
- `|Pipeline` - Pipeline identifier
- `#Enum` - Enum identifier
- `!Error` - Error identifier
- `~Unpack` - Unpack operator (loops)
- `*Pack` - Pack operator (collect)
- `$variable` - Variable identifier

**I/O:**
- `<input` - Input parameter
- `>output` - Output parameter
- `<<` - Assign input
- `>>` - Capture output
- `<~` - Default value

**Comparison:**
- `=?` - Equal
- `>?` - Greater than
- `<?` - Less than
- `>=?` - Greater or equal
- `<=?` - Less or equal
- `!=?` - Not equal

---

## Related Documentation

- [Syntax Directory](../syntax/) - Detailed syntax references
- [Examples](../examples/) - Code examples
- [Quick Reference](../quick-reference/) - Cheat sheets
- [Standard Library](../standard-library/) - Library documentation
- [CLI Tools](../cli/) - Command-line interface

---

**Last Updated:** 2025-12-15
**Version:** Current
**Maintained by:** Polyglot Documentation Team
