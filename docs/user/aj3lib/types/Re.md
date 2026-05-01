---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.#:Re"
metadata_instance: "%#:Re:N"
---

# #Re — Regex Pattern Type

<!-- @c:types -->
<!-- @c:jm3lib/types/string -->

`#Re` is a jm3lib type representing a compiled regular expression pattern. It stores the regex source text and is validated at compile time by the native `PgRegex` class — the compiler parses the pattern string using an actual regex engine, not regex-on-regex validation.

See [[scalars]] for scalar subtypes and [[syntax/constructors]] for the `$Re` constructor.

## Definition

```aljam3
{#} #Re
   [%] .description << "Compiled regular expression pattern"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "re"
   [.] .pattern#RawString
   [%] %Native.Class
      [.] .Rust << "PgRegex"
      [.] .Validate << #True
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.pattern` | `#RawString` | (none) | The regex source text |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#Re` as a scalar type
- `[#] ###ScalarValue` -- single-value scalar (no enum variants)
- `%##Alias << "re"` -- lets users write `#re` (lowercase) as shorthand

## Native Class

`#Re` is backed by a native Rust class (`PgRegex`) declared via `[%] %Native.Class`. Unlike `#String` subtypes that validate via regex pattern matching, `PgRegex` validates by **parsing the pattern string as regex** — it invokes the regex engine's parser at compile time to confirm syntactic validity.

| Field | Value | Meaning |
|-------|-------|---------|
| `.Rust` | `"PgRegex"` | Name of the Rust class backing `#Re` |
| `.Validate` | `#True` | The class exposes a validation function |

This means `$Re"^[a-z]+$"` succeeds because the compiler proves the string is valid regex syntax. `$Re"[unclosed"` fails at compile time — no runtime error possible.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Re` | Compile-time type template |
| Instance | `%#:Re:N` | Runtime instance (N = instance number) |
| Native class | `%definition.#:Re.%Native.Class` | Rust class backing (`PgRegex`) |

## Related

- [[jm3lib/constructors/Re\|$Re constructor]] -- compile-time regex construction
- [[jm3lib/pipelines/Re.Parse\|-Re.Parse]] -- runtime regex string parsing
- [[scalars]] -- scalar subtypes overview
- [[string]] -- `#String` foundation type
- [[syntax/types/INDEX\|types]] -- full type system specification
