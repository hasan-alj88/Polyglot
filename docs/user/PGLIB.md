---
audience: automation-builder
type: specification
updated: 2026-03-21
status: redirect
---

# Standard Library Reference

This document has been split into per-namespace files organized by identifier prefix. Each folder maps to a branch on the `%` metadata tree (see [[data-is-trees]]). See [pglib/INDEX.md](pglib/INDEX.md) for the full registry.

## Pipeline Namespaces (= → `%-`)
- [-File](pglib/pipelines/File.md) — file operations
- [-Path](pglib/pipelines/Path.md) — cross-platform path creation
- [-Sys](pglib/pipelines/Sys.md) — system information
- [-T](pglib/pipelines/T.md) — triggers
- [-Q](pglib/pipelines/Q.md) — queue configurations
- [-Math](pglib/pipelines/Math.md) — numeric operations (add, subtract, multiply, divide, etc.)
- [-RT](pglib/pipelines/RT.md) — runtime execution (Function, Script, CLI, Bind)
- [-W](pglib/pipelines/W.md) — wrappers

## Expanders (= → `%=`)
- [=ForEach](pglib/expanders/ForEach/) — expand operators

## Collectors (* → `%*`)
- [*Into](pglib/collectors/Into/) — collect into collection
- [*Agg](pglib/collectors/Agg.md) — reduce to single value
- [*All / *First / *Nth](pglib/collectors/Sync.md) — collect-all & race collectors

## Types (# → `%#`)
- [Built-in types](pglib/types/types.md) — type index
  - [#String](pglib/types/string.md), [scalars](pglib/types/scalars.md), [#Boolean](pglib/types/boolean.md)
  - [collections](pglib/types/collections.md), [enums](pglib/types/enums.md), [structs](pglib/types/structs.md)

## Errors (! → `%!`)
- [Error namespaces](pglib/errors/errors.md) — !File, !No, !Timeout, !Validation
