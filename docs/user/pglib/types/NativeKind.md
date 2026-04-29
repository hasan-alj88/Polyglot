---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:NativeKind"
metadata_instance: "%#:NativeKind:N"
---

# #NativeKind Enum

<!-- @c:types -->
<!-- @concepts/pipelines/INDEX -->

`#NativeKind` classifies which subsystem role a native `{N}` definition fulfills. Every `{N}` block must declare `[%] .Kind << #NativeKind.<variant>` — omitting it is a compile error (PGE01028c).

See [[concepts/pipelines/INDEX#Native vs Derived|Native vs Derived pipelines]] for when `{N}` blocks are used and how they differ from derived `{-}` pipelines.

---

## Definition

```aljam3
{#} #NativeKind
   [%] .description << "Subsystem role classifier for native {N} definitions"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [.] .Trigger
   [.] .Queue
   [.] .Wrapper
   [.] .Execution
   [.] .Intrinsic
```

| Variant | Role | Example |
|---------|------|---------|
| `.Trigger` | Signal source — fires pipeline execution | `-T.Call`, `-T.Daily`, `-T.Webhook` |
| `.Queue` | Queue strategy — controls dispatch ordering | `-Q.Default`, `-Q.Pause.Hard` |
| `.Wrapper` | Resource lifecycle — setup/teardown around body | `-W.Aljam3`, `-W.RT` |
| `.Execution` | Data operation — reads, writes, transforms | `-File.Text.Read`, `=DB.Query`, `-Math.Add` |
| `.Intrinsic` | Compiler built-in — no backing host function | `-DoNothing`, `-#.JSON.Parse` |

---

## Usage

Native definitions declare `.Kind` in their `[%]` metadata along with a `.<Language>` field naming the host function:

```aljam3
{N} -File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   (-) <path#path
   (-) >content#string
   (-) !File.NotFound
   (-) !File.PermissionDenied
```

Only pglib `{N}` definitions use `#NativeKind`. User-defined pipelines are always **derived** `{-}` — they have full Aljam3 execution bodies and no `%Native.*` metadata.

---

## Configuration

The Aljam3 service configuration file selects which host language implements each native operation using **subsystem defaults with per-operation overrides**:

```yaml
native:
  defaults:
    tm: Rust           # default for all Trigger operations
    qh: Rust           # default for all Queue operations
    runner: Rust       # default for all Execution + Wrapper operations
    pgcompiler: Rust   # compiler implementation language

  overrides:
    "Math.Add": Go     # override specific operations by pipeline name
    "DB.Query": Go
```

The compiler resolves each `{N}` definition's language by checking `overrides` first, then falling back to the `defaults` entry for the operation's subsystem (determined by `#NativeKind`). Every `{N}` definition must have a `.<Language>` field matching its resolved language — missing it is a compile error (PGE01028e).

Future host languages (e.g., `.Go`, `.Cpp`) can be added by extending the `.<Language>` fields on existing `{N}` definitions — no pipeline names or #NativeKind variants need to change. See [[technical/spec/native-dispatch|native-dispatch]] for the full configuration spec.

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:NativeKind` | Compile-time type template |
| Instance | `%#:NativeKind:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/pipelines/INDEX#Native vs Derived|Native vs Derived pipelines]] — native vs derived distinction
- [[concepts/metadata|Metadata]] — `%Native.*` metadata scope
- [[enums]] — other pglib enum types
- [[syntax/types/INDEX|types]] — full type system specification
