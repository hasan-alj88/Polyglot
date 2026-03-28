---
audience: user
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

`#None` represents the absence of a value.

```polyglot
{#} #None
   [ ] Represents the absence of a value
   [#] << ##Scalar
```

## Related

- [[string]] -- #String foundation type (separate type tree)
- [[scalars]] -- scalar subtypes of #String
- [[types]] -- full type system specification
