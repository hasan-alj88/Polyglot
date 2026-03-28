---
audience: user
type: specification
updated: 2026-03-28
status: complete
---

# #String — Foundation Type

<!-- @types -->

`#String` is the foundation type built on `RawString`. All scalar subtypes inherit from `#String` via `<~` (default schema inheritance). What users write as `#string` (lowercase) resolves here via alias.

See [[types]] for the full type hierarchy and [[scalars]] for subtypes that specialize `.re`.

## Definition

```polyglot
{#} #String
   [ ] #String and #string both resolve here
   [#] << ##Scalar
   [#] %##Alias << "string"
   [ ] The actual string value
   [.] .string#RawString
   [ ] Regex constraint — default accepts all strings
   [ ] <~ allows subtypes to override once to specialize
   [.] .re#RawString <~ ".*"
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.string` | `#RawString` | (none) | The raw string value |
| `.re` | `#RawString` | `".*"` | Regex constraint; subtypes override with `<~` |

## Schema Properties

- `[#] << ##Scalar` -- sets `%##Depth.Max << 0`, marking `#String` as a scalar with no flexible children
- `%##Alias << "string"` -- lets users write `#string` (lowercase) as shorthand

## Subtype Inheritance via `<~`

All scalar subtypes inherit `#String`'s schema using `[#] <~ #String` and override `.re` with a specific regex. The `<~` operator means "default schema, can be specialized further" -- consistent with assignment semantics where `<~` is an overridable default.

Each subtype then sets `.re` with `<<` (final -- cannot be overridden further). See [[scalars]] for all subtypes.

Users can define custom string subtypes with their own `.re`:

```polyglot
{#} #emailAddress
   [#] <~ #String
   [.] .re#RawString << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

## Related

- [[scalars]] -- #Int, #Float, #Dimension, and other scalar subtypes
- [[boolean]] -- #Boolean is NOT a #String subtype
- [[types]] -- full type system specification
