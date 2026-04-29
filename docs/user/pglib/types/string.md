---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:String"
metadata_instance: "%#:String:N"
---

# #String — Foundation Type

<!-- @c:types -->

`#String` is the foundation type built on `RawString`. All scalar subtypes inherit from `#String` via `<~` (default schema inheritance). What users write as `#string` (lowercase) resolves here via alias.

See [[syntax/types/INDEX|types]] for the full type hierarchy and [[scalars]] for subtypes that specialize `.regex`.

## Definition

```aljam3
{#} #String
   [ ] #String and #string both resolve here
   [#] ##Scalar
   [#] %##Alias << "string"
   [ ] The actual string value
   [.] .string#RawString
   [ ] Regex constraint — default accepts all strings
   [ ] <~ allows subtypes to override once to specialize
   [.] .regex#RawString <~ ".*"
   [%] %alias
      [:] "re"
   [%] %Native.Class
      [.] .Rust << "PgString"
      [.] .Validate << #True
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.string` | `#RawString` | (none) | The raw string value |
| `.regex` | `#RawString` | `".*"` | Regex constraint; subtypes override with `<~`. Alias: `.re` |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#String` as a scalar
- `%##Alias << "string"` -- lets users write `#string` (lowercase) as shorthand

## Native Class

`#String` is backed by a native Rust class (`PgString`) declared via `[%] %Native.Class`. This tells the compiler that a host-language class exists for this type and provides validation — the Rust class contains regex validation logic that the compiler calls to verify `.string` values match the `.regex` pattern at runtime.

| Field | Value | Meaning |
|-------|-------|---------|
| `.Rust` | `"PgString"` | Name of the Rust class backing `#String` |
| `.Validate` | `#True` | The class exposes a validation function |

Scalar subtypes that inherit from `#String` (e.g. `#Int`, `#Float`) inherit this native class relationship — the same `PgString` validation is used with each subtype's specialized `.regex`.

## Scalar Subtypes via `##String`

All scalar subtypes compose the `##String` parameterized schema with a specific `<regex` parameter. The schema inherits `#String`'s structure (`.string` + `.regex` fields) and overrides `.regex` with the specified pattern.

See [[scalars]] for all subtypes and [[schemas/String|##String]] for the parameterized schema definition.

Users can define custom string subtypes with their own `.regex`:

```aljam3
{#} #phoneNumber
   [#] ##String
      (#) <regex << "^\+?[0-9]{7,15}$"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:String` | Compile-time type template |
| Instance | `%#:String:N` | Runtime instance (N = instance number) |
| Native class | `%definition.#:String.%Native.Class` | Rust class backing (`PgString`) |

String subtypes nest under the instance path: `%#:String:int`, `%#:String:float`, `%#:String:uint`, etc. See [[metadata-tree/string-subtypes]] for full resolution.

## Related

- [[scalars]] -- #Int, #Float, #Dimension, and other scalar subtypes
- [[boolean]] -- #Boolean is NOT a #String subtype
- [[syntax/types/INDEX|types]] -- full type system specification
