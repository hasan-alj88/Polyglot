---
audience: user
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
- [=W](stdlib/pipelines/W.md) — wrappers

## Expanders (~ → `%~`)
- [~ForEach](stdlib/expanders/ForEach.md) — expand operators

## Collectors (* → `%*`)
- [*Into](stdlib/collectors/Into.md) — collect into collection
- [*Agg](stdlib/collectors/Agg.md) — reduce to single value
- [*All / *First / *Nth](stdlib/collectors/Sync.md) — sync & race collectors
- [*Continue](stdlib/collectors/Continue.md) — error recovery

## Types (# → `%#`)
- [Built-in types](stdlib/types/types.md) — #Boolean, #None, #OS, #path, #PipelineStatus, #VarState

## Errors (! → `%!`)
- [Error namespaces](stdlib/errors/errors.md) — !File, !No, !Timeout, !Validation
