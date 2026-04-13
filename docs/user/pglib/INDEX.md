---
audience: automation-builder
type: specification
updated: 2026-04-11
status: complete
---

# Standard Library — Namespace Registry

<!-- @c:packages -->
This is the authoritative list of all pglib namespaces. Any top-level namespace listed here is **reserved** — user `[@]` import aliases must not shadow these names (PGE09012). See [[packages#Import Rules]].

Standard library items do NOT require an `[@]` import — they are available in every `.pg` file by default.

All pglib items live on the `%` metadata tree (see [[data-is-trees#How Concepts Connect]]). Pipeline namespaces are at `%-`, operators at `%=` and `%*`, types at `%#`, errors at `%!`.

**Legend**

    .   fixed field navigation (subpackage, subtype, subfield)
    :   flexible field navigation
    <   input parameter
    >   output parameter
    #   type annotation on a parameter

## Pipeline Namespaces (=)

| Prefix | File | Description | Status | Permission |
|--------|------|-------------|--------|------------|
| `-File` | [pipelines/File.md](pipelines/File.md) | File operations (text read/write/append, serial read/write/read.field, copy, move, delete, access, list) | Stable | `_File.*` |
| `-#` | [pipelines/#.md](pipelines/%23.md) | Schema validation, field extraction, format parsing, dataframe column extraction | Stable | None |
| `-Path` | [pipelines/Path.md](pipelines/Path.md) | Cross-platform path creation from string literals | Stable | None |
| `-Math` | [pipelines/Math.md](pipelines/Math.md) | Numeric operations (add, subtract, multiply, divide, modulo, power, abs, negate) | Stable | None |
| `-Sys` | [pipelines/Sys.md](pipelines/Sys.md) | System information (OS detection) | Stable | `_System.env` |
| `-T` | [pipelines/T.md](pipelines/T.md) | Triggers (call, manual, daily, folder, webhook) | Stable | Mixed |
| `-Q` | [pipelines/Q.md](pipelines/Q.md) | Queue assignment, conditional controls (pause, resume, kill), dispatch timeout, admin operations | Stable | None |
| `-RT` | [pipelines/RT.md](pipelines/RT.md) | Runtime execution (Function, Script, CLI, Bind — inline and file modes) | Stable | `_System.process` |
| `-DT` | [pipelines/DT/INDEX.md](pipelines/DT/INDEX.md) | DateTime construction, conversion, arithmetic, comparison, extraction, formatting | Stable | `_IO.Read` (Now) / None |
| `-W` | [pipelines/W.md](pipelines/W.md) | Wrappers (Polyglot, DB, File, HTTP, SSH, Auth, Log, Queue, Cache, RT) | Stable | Mixed |
| `-Text` | [pipelines/Text/Diff.md](pipelines/Text/Diff.md) | Text comparison (line-level diff) | Draft | None |

## Expander Operators (=)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `=ForEach` | [expanders/ForEach/](expanders/ForEach/) | Expand operators (iterate arrays, maps, serials, levels, text, CSV) | Stable |

## Collector Operators (*)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `*Into` | [collectors/Into/](collectors/Into/) | Collect into collection (array, map, serial, level, text, CSV) | Stable |
| `*Agg` | [collectors/Agg.md](collectors/Agg.md) | Reduce to single value (sum, count, average, max, min, concatenate) | Stable |
| `*All` / `*First` / `*Nth` | [collectors/Sync.md](collectors/Sync.md) | Collect-all and race collectors | Stable |

## Built-in Types (#)

| Name | File | Description | Status |
|------|------|-------------|--------|
| `#String` | [types/string.md](types/string.md) | Foundation type | Stable |
| `#Int`, `#UnsignedInt`, `#Float`, `#Sci`, `#Eng`, `#Dimension`, `#KeyString`, `#NestedKeyString` | [types/scalars.md](types/scalars.md) | Scalar subtypes | Stable |
| `#Boolean`, `#None` | [types/boolean.md](types/boolean.md) | Boolean enum + absence type | Stable |
| `#Map`, `#Array`, `#Serial`, `#Dataframe` | [types/collections.md](types/collections.md) | Collection types | Stable |
| `#OS`, `#PipelineStatus`, `#QueueStrategy`, `#RetriggerStrategy`, `#QueueState`, `#KillPropagation`, `#ResourceTag`, `#FileAccess`, `#VarState`, `#FieldKind` | [types/enums.md](types/enums.md) | Runtime enums | Stable |
| `#path`, `#Queue` | [types/structs.md](types/structs.md) | Struct types | Stable |
| `#DateTime` (`#dt`) | [types/datetime.md](types/datetime.md) | Date, time, calendar, timezone, duration types | Stable |
| `#Code`, `#PyEnv`, `#RsEnv` | [types/rt.md](types/rt.md) | Runtime types | Stable |
| `#TextDiff`, `#DiffOp`, `#DiffStats`, `#MergeConflict`, `#MergeResult`, `#MergeStrategy`, `#CollectOrder` | [types/](types/) | Text diff and merge types | Draft |

## Generic Permissions (__)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `__File` | [permissions/File.md](permissions/File.md) | File system permissions (Read, Write, Execute, Delete, Create) | Stable |
| `__Web` | [permissions/Web.md](permissions/Web.md) | Network/web permissions (Request, Socket, Listen) | Stable |
| `__Database` | [permissions/Database.md](permissions/Database.md) | Database permissions (Connect, Read, Write) | Stable |
| `__System` | [permissions/System.md](permissions/System.md) | System permissions (Env, Process, Signal) | Stable |
| `__Crypto` | [permissions/Crypto.md](permissions/Crypto.md) | Cryptographic permissions (Key, Sign, Encrypt) | Stable |
| `__IPC` | [permissions/IPC.md](permissions/IPC.md) | IPC permissions (Send, Receive, Subscribe) | Stable |
| `__Device` | [permissions/Device.md](permissions/Device.md) | Device permissions (Camera, Microphone, Location, Bluetooth) | Stable |
| `__Memory` | [permissions/Memory.md](permissions/Memory.md) | Memory permissions (Allocate, Shared) | Stable |

See [[pglib/permissions/INDEX|Generic Permissions INDEX]] for full capability-level breakdown.

## Error Namespaces (!)

| Prefix | File | Description | Status |
|--------|------|-------------|--------|
| `!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission`, `!RT` | [errors/errors.md](errors/errors.md) | Standard error trees | Stable |
| `!Storage`, `!Text`, `!CSV` | [errors/errors.md](errors/errors.md) | Text/CSV operation errors | Draft |
