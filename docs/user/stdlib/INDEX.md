---
audience: user
type: specification
updated: 2026-03-30
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
| `=File` | [pipelines/File.md](pipelines/File.md) | File operations (text read/write/append, serial read/write/read.field, copy, move, delete, access, list) | Stable | `_File.*` |
| `=#` | [pipelines/#.md](pipelines/%23.md) | Schema validation, field extraction, format parsing, dataframe column extraction | Stable | None |
| `=Path` | [pipelines/Path.md](pipelines/Path.md) | Cross-platform path creation from string literals | Stable | None |
| `=Math` | [pipelines/Math.md](pipelines/Math.md) | Numeric operations (add, subtract, multiply, divide, modulo, power, abs, negate) | Stable | None |
| `=Sys` | [pipelines/Sys.md](pipelines/Sys.md) | System information (OS detection) | Stable | `_System.env` |
| `=T` | [pipelines/T.md](pipelines/T.md) | Triggers (call, manual, daily, folder, webhook) | Stable | Mixed |
| `=Q` | [pipelines/Q.md](pipelines/Q.md) | Queue configurations (default, FIFO, LIFO, priority, pause, resume, kill) | Stable | None |
| `=W` | [pipelines/W.md](pipelines/W.md) | Wrappers (Polyglot, DB, File, HTTP, SSH, Auth, Log, Queue, Cache, Python) | Stable | Mixed |

## Expander Operators (~)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `~ForEach` | [expanders/ForEach/](expanders/ForEach/) | Expand operators (iterate arrays, maps, serials, levels) | Stable |

## Collector Operators (*)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `*Into` | [collectors/Into/](collectors/Into/) | Collect into collection (array, map, serial, level) | Stable |
| `*Agg` | [collectors/Agg.md](collectors/Agg.md) | Reduce to single value (sum, count, average, max, min, concatenate) | Stable |
| `*All` / `*First` / `*Nth` | [collectors/Sync.md](collectors/Sync.md) | Sync barriers and race collectors | Stable |
| `*Continue` | [collectors/Continue.md](collectors/Continue.md) | Error recovery with fallback value | Stable |

## Built-in Types (#)

| Name | File | Description | Status |
|------|------|-------------|--------|
| `#String` | [types/string.md](types/string.md) | Foundation type | Stable |
| `#Int`, `#UnsignedInt`, `#Float`, `#Sci`, `#Eng`, `#Dimension`, `#KeyString`, `#NestedKeyString` | [types/scalars.md](types/scalars.md) | Scalar subtypes | Stable |
| `#Boolean`, `#None` | [types/boolean.md](types/boolean.md) | Boolean enum + absence type | Stable |
| `#Map`, `#Array`, `#Serial`, `#Dataframe` | [types/collections.md](types/collections.md) | Collection types | Stable |
| `#OS`, `#PipelineStatus`, `#QueueStrategy`, `#RetriggerStrategy`, `#QueueState`, `#FileAccess`, `#VarState`, `#FieldKind` | [types/enums.md](types/enums.md) | Runtime enums | Stable |
| `#path`, `#Queue` | [types/structs.md](types/structs.md) | Struct types | Stable |

## Error Namespaces (!)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission` | [errors/errors.md](errors/errors.md) | Standard error trees | Stable |
