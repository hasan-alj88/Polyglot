---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# Error System

<!-- @c:errors -->
<!-- @c:blocks -->

Errors use the `!` prefix and hierarchical dot names. Every error terminal is typed `#Error` — enforced by `%##TerminalType << #Error` (see [[pglib/types/properties/TerminalType|%##TerminalType]]). `{!}` blocks are effectively `{#}` data trees with this terminal constraint. Custom errors are defined with `{!}` blocks; pglib errors are built-in and require no `[@]` import.

## Core

| Doc | Content |
|-----|---------|
| [[pglib/errors/error-struct\|Error Struct]] | `#NullableRecord`, `#Error` struct, field documentation |
| [[pglib/errors/custom-errors\|Custom Errors]] | Defining `{!}` blocks, `!Error` user-extensible namespace |
| [[pglib/errors/alias-clash\|!Alias.Clash]] | Compile error behavior, `[<] !Alias.Clash` fallback chain |
| [[pglib/errors/pipeline-associations\|Pipeline Associations]] | Which pglib pipelines raise which errors |

## Built-in Error Namespaces

No `[@]` import needed. pglib errors are defined as `{!}` blocks by the runtime:

| Namespace | Doc | Leaves |
|-----------|-----|--------|
| `!File` | [[pglib/errors/builtin/file\|!File]] | `.NotFound`, `.ReadError`, `.WriteError`, `.ParseError` |
| `!No` | [[pglib/errors/builtin/no\|!No]] | `.Input`, `.Output` |
| `!Timeout` | [[pglib/errors/builtin/timeout\|!Timeout]] | `.Connection`, `.Read` |
| `!Math` | [[pglib/errors/builtin/math\|!Math]] | `.DivideByZero` |
| `!Validation` | [[pglib/errors/builtin/validation\|!Validation]] | `.Schema`, `.Type`, `.Regex` |
| `!Field` | [[pglib/errors/builtin/field\|!Field]] | `.NotFound`, `.PathError` |
| `!Alias` | [[pglib/errors/builtin/alias\|!Alias]] | `.Clash` |
| `!Permission` | [[pglib/errors/builtin/permission\|!Permission]] | `.File.Denied`, `.Web.Denied`, `.Database.Denied`, `.System.Denied`, `.Crypto.Denied`, `.IPC.Denied`, `.Device.Denied`, `.Memory.Denied` |
| `!RT` | [[pglib/errors/builtin/rt\|!RT]] | `.CompileError`, `.RuntimeError`, `.Timeout`, `.EnvironmentError` |
| `!Env` | [[pglib/errors/builtin/env\|!Env]] | `.NotFound`, `.VersionMismatch`, `.SetupFailed`, `.TeardownFailed`, `:Dependency.*` |
| `!Storage` | [[pglib/errors/builtin/storage\|!Storage]] | `.Space` |
| `!Text` | [[pglib/errors/builtin/text\|!Text]] | `:Diff.EmptyInput`, `:Lines.Empty`, `:Append.EmptyResult`, `:Merge.*` |
| `!CSV` | [[pglib/errors/builtin/csv\|!CSV]] | `:Parse.*`, `:Collect.*`, `:Merge.HeaderConflict` |

### `!Error` — User-Extensible Namespace

`!Error` is the only namespace with user-extensible children. All other namespaces have Aljam3-defined fixed leaves. See [[pglib/errors/custom-errors|Custom Errors]] for the full `{!}` definition syntax.

### `!Env` vs `!RT` — Phase Distinction

| Namespace | Phase | Scope |
|-----------|-------|-------|
| `!Env.*` | `[W]` wrapper setup | Environment availability + dependency resolution |
| `!RT.*` | `[-]` body execution | Foreign code compile/runtime errors |

See [[pglib/errors/builtin/rt|!RT]] for the full phase distinction explanation.
