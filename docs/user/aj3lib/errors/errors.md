---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# Error System

<!-- @c:errors -->
<!-- @c:blocks -->

Errors use the `!` prefix and hierarchical dot names. Every error terminal is typed `#Error` — enforced by `%##TerminalType << #Error` (see [[aj3lib/types/properties/TerminalType|%##TerminalType]]). `{!}` blocks are effectively `{#}` data trees with this terminal constraint. Custom errors are defined with `{!}` blocks; aj3lib errors are built-in and require no `[@]` import.

## Core

| Doc | Content |
|-----|---------|
| [[aj3lib/errors/error-struct\|Error Struct]] | `#NullableRecord`, `#Error` struct, field documentation |
| [[aj3lib/errors/custom-errors\|Custom Errors]] | Defining `{!}` blocks, `!Error` user-extensible namespace |
| [[aj3lib/errors/alias-clash\|!Alias.Clash]] | Compile error behavior, `[<] !Alias.Clash` fallback chain |
| [[aj3lib/errors/pipeline-associations\|Pipeline Associations]] | Which aj3lib pipelines raise which errors |

## Built-in Error Namespaces

No `[@]` import needed. aj3lib errors are defined as `{!}` blocks by the runtime:

| Namespace | Doc | Leaves |
|-----------|-----|--------|
| `!File` | [[aj3lib/errors/builtin/file\|!File]] | `.NotFound`, `.ReadError`, `.WriteError`, `.ParseError` |
| `!No` | [[aj3lib/errors/builtin/no\|!No]] | `.Input`, `.Output` |
| `!Timeout` | [[aj3lib/errors/builtin/timeout\|!Timeout]] | `.Connection`, `.Read` |
| `!Math` | [[aj3lib/errors/builtin/math\|!Math]] | `.DivideByZero` |
| `!Validation` | [[aj3lib/errors/builtin/validation\|!Validation]] | `.Schema`, `.Type`, `.Regex` |
| `!Field` | [[aj3lib/errors/builtin/field\|!Field]] | `.NotFound`, `.PathError` |
| `!Alias` | [[aj3lib/errors/builtin/alias\|!Alias]] | `.Clash` |
| `!Permission` | [[aj3lib/errors/builtin/permission\|!Permission]] | `.File.Denied`, `.Web.Denied`, `.Database.Denied`, `.System.Denied`, `.Crypto.Denied`, `.IPC.Denied`, `.Device.Denied`, `.Memory.Denied` |
| `!RT` | [[aj3lib/errors/builtin/rt\|!RT]] | `.CompileError`, `.RuntimeError`, `.Timeout`, `.EnvironmentError` |
| `!Env` | [[aj3lib/errors/builtin/env\|!Env]] | `.NotFound`, `.VersionMismatch`, `.SetupFailed`, `.TeardownFailed`, `:Dependency.*` |
| `!Storage` | [[aj3lib/errors/builtin/storage\|!Storage]] | `.Space` |
| `!Text` | [[aj3lib/errors/builtin/text\|!Text]] | `:Diff.EmptyInput`, `:Lines.Empty`, `:Append.EmptyResult`, `:Merge.*` |
| `!CSV` | [[aj3lib/errors/builtin/csv\|!CSV]] | `:Parse.*`, `:Collect.*`, `:Merge.HeaderConflict` |

### `!Error` — User-Extensible Namespace

`!Error` is the only namespace with user-extensible children. All other namespaces have Aljam3-defined fixed leaves. See [[aj3lib/errors/custom-errors|Custom Errors]] for the full `{!}` definition syntax.

### `!Env` vs `!RT` — Phase Distinction

| Namespace | Phase | Scope |
|-----------|-------|-------|
| `!Env.*` | `[W]` wrapper setup | Environment availability + dependency resolution |
| `!RT.*` | `[-]` body execution | Foreign code compile/runtime errors |

See [[aj3lib/errors/builtin/rt|!RT]] for the full phase distinction explanation.
