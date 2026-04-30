---
audience: automation-builder
type: specification
updated: 2026-03-21
status: redirect
---

# Standard Library Reference

This document has been split into per-namespace files organized by identifier prefix. Each folder maps to a branch on the `%` metadata tree (see [[data-is-trees]]). See [aj3lib/INDEX.md](aj3lib/INDEX.md) for the full registry.

## Pipeline Namespaces (= → `%-`)
- [-File](aj3lib/pipelines/File.md) — file operations
- [-Path](aj3lib/pipelines/Path.md) — cross-platform path creation
- [-Sys](aj3lib/pipelines/Sys.md) — system information
- [-T](aj3lib/pipelines/T.md) — triggers
- [-Q](aj3lib/pipelines/Q.md) — queue configurations
- [-Math](aj3lib/pipelines/Math.md) — numeric operations (add, subtract, multiply, divide, etc.)
- [-RT](aj3lib/pipelines/RT.md) — runtime execution (Function, Script, CLI, Bind)
- [-W](aj3lib/pipelines/W.md) — wrappers

## Expanders (= → `%=`)
- [=ForEach](aj3lib/expanders/ForEach/) — expand operators

## Collectors (* → `%*`)
- [*Into](aj3lib/collectors/Into/) — collect into collection
- [*Agg](aj3lib/collectors/Agg.md) — reduce to single value
- [*All / *First / *Nth](aj3lib/collectors/Sync.md) — collect-all & race collectors

## Types (# → `%#`)
- [Built-in types](aj3lib/types/types.md) — type index
  - [#String](aj3lib/types/string.md), [scalars](aj3lib/types/scalars.md), [#Boolean](aj3lib/types/boolean.md)
  - [collections](aj3lib/types/collections.md), [enums](aj3lib/types/enums.md), [structs](aj3lib/types/structs.md)

## Errors (! → `%!`)
- [Error namespaces](aj3lib/errors/errors.md) — !File, !No, !Timeout, !Validation
