---
audience: pg-coder
type: specification
updated: 2026-04-04
status: complete
---

# #NativeKind Enum

<!-- @types -->
<!-- @concepts/pipelines/INDEX -->

`#NativeKind` classifies which subsystem role a native `{N}` definition fulfills. Every `{N}` block must declare `[%] .Kind << #NativeKind.<variant>` — omitting it is a compile error (PGE01028c).

See [[concepts/pipelines/INDEX#Native vs Derived|Native vs Derived pipelines]] for when `{N}` blocks are used and how they differ from derived `{=}` pipelines.

---

## Definition

```polyglot
{#} #NativeKind
   [%] .description << "Subsystem role classifier for native {N} definitions"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [.] .Trigger
   [.] .Queue
   [.] .Wrapper
   [.] .Execution
   [.] .Intrinsic
```

| Variant | Role | Example |
|---------|------|---------|
| `.Trigger` | Event source — fires pipeline execution | `=T.Call`, `=T.Daily`, `=T.Webhook` |
| `.Queue` | Queue strategy — controls dispatch ordering | `=Q.Default`, `=Q.Pause.Hard` |
| `.Wrapper` | Resource lifecycle — setup/teardown around body | `=W.Polyglot`, `=W.RT` |
| `.Execution` | Data operation — reads, writes, transforms | `=File.Text.Read`, `=DB.Query`, `=Math.Add` |
| `.Intrinsic` | Compiler built-in — no backing host function | `=DoNothing`, `=#.JSON.Parse` |

---

## Usage

Native definitions declare `.Kind` in their `[%]` metadata along with a `.<Language>` field naming the host function:

```polyglot
{N} =File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   [=] <path#path
   [=] >content#string
   [=] !File.NotFound
   [=] !File.PermissionDenied
```

Only pglib `{N}` definitions use `#NativeKind`. User-defined pipelines are always **derived** `{=}` — they have full Polyglot execution bodies and no `%Native.*` metadata.

---

## Configuration

The Polyglot config file selects the active base language:

```yaml
base: Rust
```

The compiler validates that every `{N}` definition has a `.<Language>` field matching the configured base language. If a definition has `.Go` but the config says `base: Rust`, that is a compile error (PGE01028e).

Future base languages (e.g., `.Go`, `.Cpp`) can be added by extending the `.<Language>` fields on existing `{N}` definitions — no pipeline names or #NativeKind variants need to change.

---

## Related

- [[concepts/pipelines/INDEX#Native vs Derived|Native vs Derived pipelines]] — native vs derived distinction
- [[concepts/metadata|Metadata]] — `%Native.*` metadata scope
- [[enums]] — other pglib enum types
- [[syntax/types/INDEX|types]] — full type system specification
