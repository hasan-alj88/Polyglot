---
audience: automation-builder
type: specification
updated: 2026-03-21
status: redirect
---

# Standard Library Reference

This document has been split into per-namespace files organized by identifier prefix. Each folder maps to a branch on the `%` metadata tree (see [[data-is-trees]]). See [jm3lib/INDEX.md](jm3lib/INDEX.md) for the full registry.

## Pipeline Namespaces (= → `%-`)
- [-File](jm3lib/pipelines/File.md) — file operations
- [-Path](jm3lib/pipelines/Path.md) — cross-platform path creation
- [-Sys](jm3lib/pipelines/Sys.md) — system information
- [-T](jm3lib/pipelines/T.md) — triggers
- [-Q](jm3lib/pipelines/Q.md) — queue configurations
- [-Math](jm3lib/pipelines/Math.md) — numeric operations (add, subtract, multiply, divide, etc.)
- [-RT](jm3lib/pipelines/RT.md) — runtime execution (Function, Script, CLI, Bind)
- [-W](jm3lib/pipelines/W.md) — wrappers

## Expanders (= → `%=`)
- [=ForEach](jm3lib/expanders/ForEach/) — expand operators

## Collectors (* → `%*`)
- [*Into](jm3lib/collectors/Into/) — collect into collection
- [*Agg](jm3lib/collectors/Agg.md) — reduce to single value
- [*All / *First / *Nth](jm3lib/collectors/Sync.md) — collect-all & race collectors

## Types (# → `%#`)
- [Built-in types](jm3lib/types/types.md) — type index
  - [#String](jm3lib/types/string.md), [scalars](jm3lib/types/scalars.md), [#Boolean](jm3lib/types/boolean.md)
  - [collections](jm3lib/types/collections.md), [enums](jm3lib/types/enums.md), [structs](jm3lib/types/structs.md)

## Errors (! → `%!`)
- [Error namespaces](jm3lib/errors/errors.md) — !File, !No, !Timeout, !Validation
