---
audience: pg-coder
type: specification
updated: 2026-03-21
status: redirect
---

# Standard Library Reference

This document has been split into per-namespace files organized by identifier prefix. Each folder maps to a branch on the `%` metadata tree (see [[data-is-trees]]). See [stdlib/INDEX.md](stdlib/INDEX.md) for the full registry.

## Pipeline Namespaces (= → `%=`)
- [=File](stdlib/pipelines/File.md) — file operations
- [=Path](stdlib/pipelines/Path.md) — cross-platform path creation
- [=Sys](stdlib/pipelines/Sys.md) — system information
- [=T](stdlib/pipelines/T.md) — triggers
- [=Q](stdlib/pipelines/Q.md) — queue configurations
- [=Math](stdlib/pipelines/Math.md) — numeric operations (add, subtract, multiply, divide, etc.)
- [=RT](stdlib/pipelines/RT.md) — runtime execution (Function, Script, CLI, Bind)
- [=W](stdlib/pipelines/W.md) — wrappers

## Expanders (~ → `%~`)
- [~ForEach](stdlib/expanders/ForEach/) — expand operators

## Collectors (* → `%*`)
- [*Into](stdlib/collectors/Into/) — collect into collection
- [*Agg](stdlib/collectors/Agg.md) — reduce to single value
- [*All / *First / *Nth](stdlib/collectors/Sync.md) — collect-all & race collectors
- [*Continue](stdlib/collectors/Continue.md) — error recovery

## Types (# → `%#`)
- [Built-in types](stdlib/types/types.md) — type index
  - [#String](stdlib/types/string.md), [scalars](stdlib/types/scalars.md), [#Boolean](stdlib/types/boolean.md)
  - [collections](stdlib/types/collections.md), [enums](stdlib/types/enums.md), [structs](stdlib/types/structs.md)

## Errors (! → `%!`)
- [Error namespaces](stdlib/errors/errors.md) — !File, !No, !Timeout, !Validation
