---
audience: pg-coder
type: specification
updated: 2026-03-28
status: complete
---

# #Boolean and #None

<!-- @types -->

## #Boolean

`#Boolean` is intentionally NOT a `#String` subtype. Booleans are enum fields (no `#type`), not string values with regex. `.True` and `.False` are enum fields -- exactly one is active at a time. This is a separate type tree from `#String`.

```polyglot
{#} #Boolean
   [%] .description << "boolean type Enum"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "bool"
   [.] .True
   [.] .False
```

### Fields

| Field | Kind | Purpose |
|-------|------|---------|
| `.True` | Enum | Boolean true variant |
| `.False` | Enum | Boolean false variant |

### Schema Properties

- `[#] << ##Scalar` -- sets `%##Depth.Max << 0`
- `[#] << ###Enum` -- leaf content is variant selection, not typed data
- `%##Alias << "bool"` -- shorthand `#bool`

---

## #None

`#None` represents the absence of a value. It is the only type with `###None` — empty string `""` is its only valid value. All other types reject `""` with PGE04021.

```polyglot
{#} #None
   [ ] Represents the absence of a value
   [ ] Empty string "" is the only valid value
   [#] << ##Scalar
   [#] << ###None
```

## Related

- [[string]] -- #String foundation type (separate type tree)
- [[scalars]] -- scalar subtypes of #String
- [[syntax/types/INDEX|types]] -- full type system specification
