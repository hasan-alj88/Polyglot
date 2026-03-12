---
last-redoc-date: 2025-12-18
---

# Polyglot v0.0.4 Language Specification

This directory contains the complete language specification for Polyglot v0.0.4, covering syntax, semantics, type system, and control flow.

## Overview

Polyglot is an asynchronous automation language designed for cross-language FFI and event-driven workflows. Version 0.0.4 introduces major enhancements including indentation-based nesting, improved prefix system, and refined I/O operators.

## Directory Structure

### [syntax/](./syntax/)
**Core syntax specification**

Complete reference for Polyglot's syntax including:
- **Prefix System**: All identifier prefixes (`.`, `#`, `|`, `!`, `~`, `%`, etc.)
- **Markers**: Block markers for execution control (`[r]`, `[p]`, `[?]`, etc.)
- **Operators**: All operators with usage patterns
- **I/O System**: Input/output binding operators

**Critical for**: Parser implementation, syntax highlighting, code editors

### [types/](./types/)
**Type system specification**

Covers:
- Primitive types (`:pg.int`, `:pg.string`, etc.)
- Collection types (`:pg.array`, `:pg.set`)
- Type annotations and inference
- Cross-language type mapping

**Critical for**: Type checker implementation, FFI bindings

### [control-flow/](./control-flow/)
**Control flow mechanisms**

Includes:
- Conditional execution (`[?]` marker)
- Match/select patterns (`[m]` marker)
- Error handling (`[!]` marker)
- Sequential vs parallel execution
- Loop constructs (via unpacks/packs)

**Critical for**: Execution engine, runtime implementation

### [advanced/](./advanced/)
**Advanced language features**

Covers:
- Metadata system (`%` prefix)
- Pipeline composition (`|>` operator)
- Variadic operations (`<<<`, `>>>`)
- Package management (`@Registry::Package:Version`)
- Reserved vs custom hierarchies (`;` vs `.`)

**Critical for**: Advanced tooling, package managers, IDE features

## Key Language Concepts

### 1. Predictable Line Structure

**Every valid line matches**: `[Optional Indentation] + [Marker(s)] + [One Expression]`

```polyglot
[r] $x << 5                    # Basic form
   [?] $x >? 10                # Indented (sub-marker)
      [r] $log << "Greater"    # Double-indented (nested)
```

### 2. Prefix-Based Identity

All identifiers use prefix characters to indicate their type:

```polyglot
$variable      # Variable identifier
#Status.Active # Enum identifier
|ProcessData   # Pipeline identifier
!Network.Error # Error identifier
~ForEach.Array # Unpack operator
%Doc           # Metadata annotation
```

### 3. Context-Dependent Markers

The same marker serves different purposes based on context:

```polyglot
{|} |Pipeline         # Definition context
[|] <input :pg.string # [|] declares parameter

[r] |Pipeline         # Invocation context
[|] <input << $value  # [|] binds argument
```

### 4. Indentation as Structure

3-space indentation creates sub-marker relationships:

```polyglot
[m] $status
   [?] 1 ? #Active    # 1 level deep
   [?] 2 ? #Inactive  # Same level
      [r] $log << "Inactive detected"  # 2 levels deep
```

## Breaking Changes from v0.0.3

### Syntax Changes

| Feature | v0.0.3 | v0.0.4 | Impact |
|---------|--------|--------|--------|
| Variable prefix | `,varName` | `$varName` | **Breaking** - All code needs update |
| Reserved indication | Not supported | `#Boolean.True` | **New feature** |
| Indentation | `~\` marker | 3 spaces | **Breaking** - Structure change |
| Metadata | Not supported | `%Doc` prefix | **New feature** |
| Pipe composition | Not supported | `\|>` operator | **New feature** |
| Variadic push | Not supported | `<<<`, `>>>` | **New feature** |

### Migration Path

1. **Variable prefix**: Replace all `,var` with `$var`
2. **Indentation**: Replace `~\` markers with 3-space indentation
3. **Reserved enums**: Update to use `;` separator where applicable
4. **Operators**: Update to new operator set (see `syntax/operators.md`)

## Version History

- **v0.0.4** (Dec 2025) - Current specification
  - Indentation-based nesting
  - Improved prefix system (`$` for variables, `%` for metadata)
  - Reserved indication with `;`
  - Enhanced operator set
  - 33 major features

- **v0.0.3** (Nov 2025) - Previous stable
  - Basic lexer and parser
  - Core syntax with `,` variable prefix
  - Explicit `~\` for nesting
  - Foundation for v0.0.4 enhancements

- **v0.0.2** and earlier - Legacy versions (archived)

## Implementation Status

**As of December 18, 2025:**

- ✅ **Lexer**: ~85% v0.0.4 complete
  - All operators implemented
  - All prefixes implemented
  - Metadata and semicolon tokens added
  - Tests passing (45/45)

- ⏳ **Parser**: 100% v0.0.3 (v0.0.4 not started)
  - Need: Indentation parsing
  - Need: Definition block parsing
  - Need: Reserved indication parsing
  - Tests passing for v0.0.3 (106/106)

- 📋 **Documentation**: In progress
  - Syntax reference complete
  - Parser implementation guidance complete
  - Examples and tutorials pending

## Quick Start for Implementers

### For Parser Developers

**Start here:**
1. [`syntax/README.md`](./syntax/) - Core syntax patterns
2. [`../User/reference/README.md`](../User/reference/) - Operator precedence, parsing tables
3. [`../User/reference/grammar.md`](../User/reference/grammar.md) - Formal EBNF grammar

**Critical patterns to implement:**
- Reserved indication (`;` hierarchies)
- Inline pipelines (formatted string input)
- Indentation-based nesting
- Definition vs invocation contexts

### For Type System Developers

**Start here:**
1. [`types/README.md`](./types/) - Type system overview
2. Type inference rules (see `../User/reference/README.md`)
3. Cross-language type mapping

### For Runtime Developers

**Start here:**
1. [`control-flow/README.md`](./control-flow/) - Execution semantics
2. Pipeline lifecycle and state machines
3. Error propagation and handling

## Resources

**Complete Specification**: `docs/User/specifications/v0.0.4/`
**Examples**: `docs/User/specifications/v0.0.4/examples/`
**Getting Started**: `docs/User/specifications/v0.0.4/getting-started/`
**Reference**: `docs/User/specifications/v0.0.4/reference/`

**Project Files**:
- PRD: `docs/Agile/prd.md`
- Epics: `docs/Agile/epics.md`
- Architecture: `docs/Tech/implementation/technical/architecture/`

---

**Note**: This is living documentation. As the parser and runtime are implemented, this spec will be refined based on practical implementation findings.
