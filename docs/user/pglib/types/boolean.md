---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Boolean"
metadata_instance: "%#:Boolean:N"
---

# #Boolean and #None

<!-- @c:types -->

## #Boolean

`#Boolean` is a `##Enum` type — a struct whose fields are all enum fields (no `#type` annotation). It is intentionally NOT a `#String` subtype. `.True` and `.False` are enum fields -- exactly one is active at a time. This is a separate type tree from `#String`.

```polyglot
{#} #Boolean
   [%] .description << "boolean type Enum"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
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

- `[#] ##Enum` -- classifies as enum: `%##Active << #ActiveKind.One`, `%###Kind << #FieldKind.Enum`
- `[#] ##Scalar` -- sets `%##Depth.Max << 1`
- `[#] ###ScalarEnum` -- leaf content is variant selection, not typed data
- `%##Alias << "bool"` -- shorthand `#bool`

---

## #None

`#None` represents the absence of a value. It is the only type with `###None` — empty string `""` is its only valid value. All other types reject `""` with PGE04021.

```polyglot
{#} #None
   [ ] Represents the absence of a value
   [ ] Empty string "" is the only valid value
   [#] ##Scalar
   [#] ###None
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Boolean` | Compile-time type template |
| Instance | `%#:Boolean:0` | Runtime instance (enum — one active field) |

`#None` also lives here: definition at `%definition.#:None`, instance at `%#:None:0`.

## Related

- [[string]] -- #String foundation type (separate type tree)
- [[scalars]] -- scalar subtypes of #String
- [[syntax/types/INDEX|types]] -- full type system specification
