---
audience: user
type: specification
updated: 2026-03-25
status: complete
---

# Standard Library — Namespace Registry

<!-- @packages -->
This is the authoritative list of all stdlib namespaces. Any top-level namespace listed here is **reserved** — user `[@]` import aliases must not shadow these names (PGE-913). See [[packages#Import Rules]].

Standard library items do NOT require an `[@]` import — they are available in every `.pg` file by default.

All stdlib items live on the `%` metadata tree (see [[data-is-trees#How Concepts Connect]]). Pipeline namespaces are at `%=`, operators at `%~` and `%*`, types at `%#`, errors at `%!`.

**Legend**

    .   fixed field navigation (subpackage, subtype, subfield)
    :   flexible field navigation
    <   input parameter
    >   output parameter
    #   type annotation on a parameter

## Pipeline Namespaces (=)

| Prefix | File | Description | Status | Permission |
|--------|------|-------------|--------|------------|
| `=File` | [pipelines/File.md](pipelines/File.md) | File operations (read, write, append, copy, move, delete, access, list) | Stable | `_File.*` |
| `=Path` | [pipelines/Path.md](pipelines/Path.md) | Cross-platform path creation from string literals | Stable | None |
| `=Math` | [pipelines/Math.md](pipelines/Math.md) | Numeric operations (add, subtract, multiply, divide, modulo, power, abs, negate) | Stable | None |
| `=Sys` | [pipelines/Sys.md](pipelines/Sys.md) | System information (OS detection) | Stable | `_System.env` |
| `=T` | [pipelines/T.md](pipelines/T.md) | Triggers (call, manual, daily, folder, webhook) | Stable | Mixed |
| `=Q` | [pipelines/Q.md](pipelines/Q.md) | Queue configurations (default, FIFO, LIFO, priority, pause, resume, kill) | Stable | None |
| `=W` | [pipelines/W.md](pipelines/W.md) | Wrappers (Polyglot, DB, File, HTTP, SSH, Auth, Log, Queue, Cache, Python) | Stable | Mixed |

## Expander Operators (~)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `~ForEach` | [expanders/ForEach.md](expanders/ForEach.md) | Expand operators (iterate arrays, serials, levels) | Stable |

## Collector Operators (*)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `*Into` | [collectors/Into.md](collectors/Into.md) | Collect into collection (array, serial, level) | Stable |
| `*Agg` | [collectors/Agg.md](collectors/Agg.md) | Reduce to single value (sum, count, average, max, min, concatenate) | Stable |
| `*All` / `*First` / `*Nth` | [collectors/Sync.md](collectors/Sync.md) | Sync barriers and race collectors | Stable |
| `*Continue` | [collectors/Continue.md](collectors/Continue.md) | Error recovery with fallback value | Stable |

## Built-in Types (#)

| Name | File | Description | Status |
|------|------|-------------|--------|
| `#Boolean`, `#None`, `#OS`, `#path`, `#PipelineStatus`, `#VarState` | [types/types.md](types/types.md) | Stdlib structs and enums | Stable |

## Error Namespaces (!)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Permission` | [errors/errors.md](errors/errors.md) | Standard error trees | Stable |
