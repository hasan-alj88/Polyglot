---
audience: user
type: specification
updated: 2026-03-30
---

# Schema Properties

<!-- @syntax/types/INDEX -->

## Schema Properties

`{#}` definitions gain **schema properties** declared with `[#] %##Property`. These are compile-time metadata prefixed with `%##` to explicitly mark them as tree-structure properties. They describe structural constraints on the type's tree shape:

| Property | Type | Applies to | Meaning |
|----------|------|-----------|---------|
| `%##Depth.Max` | `#int` | Universal | Max tree depth (0=scalar, 1=flat, -1=unlimited) |
| `%##Children.Type` | type ref | Branch nodes (depth > 0) | Data type of child keys |
| `%##Children.Gap` | `#Boolean` | Branch nodes (depth > 0) | Gaps allowed in child keys? |
| `%##Children.Uniform` | `#Boolean` | Branch nodes (depth > 0) | All children same schema? |
| `%##Children.Regular` | `#Boolean` | Branch nodes (depth > 0) | All branches at same depth have same child count? |
| `%##Children.Min` | `#uint` | Branch nodes (depth > 0) | Minimum child count |
| `%##Children.Max` | `#int` | Branch nodes (depth > 0) | Max child count (-1=unlimited) |
| `%##Children.Ordered` | `#Boolean` | Branch nodes (depth > 0) | Are children ordered? |
| `%##Leafs.Kind` | `#FieldKind` | Universal | Constrains what `###` field type all leafs must be |
| `%##Alias` | `#NestedKeyString` | Universal | Lowercase shorthand name |

Schema properties apply universally via `[#]`, or branch-wise via `[.]`/`[:]`. Conflict between universal and branch-wise scope raises PGE11001. If a `%##` property is redundant with an inherited value, the compiler raises PGW11001; if it contradicts, the override takes effect with PGW11002.

Schema properties live in the metadata tree at `%definition.#:{TypeName}.{Property}`, making them introspectable at compile time. Schema references (`##`) are only valid inside `{#}` type definitions — using them outside raises PGE05006.

### %##Depth.Max — Inference Model

`%##Depth.Max` describes how many levels of **flexible** (`:`) nesting a type supports. Fixed (`.`) fields define static schema structure and do NOT count as depth.

| Value | Meaning | Examples |
|-------|---------|---------|
| `0` | Scalar/record — no flexible children | #String, #Int, #Boolean, #Person (all `.` fields) |
| `1` | One level of flexible children | #Array (1D), #Map |
| `N` | N levels of flexible nesting | #Array with `:ND` dimension |
| `-1` | Unlimited flexible nesting | #Serial |

**Compiler inference:** When a `{#}` definition does not explicitly set `%##Depth.Max`, the compiler infers it:
- **All `.` fixed fields** → `%##Depth.Max = 0` (record/scalar)
- **Has `:` flexible fields** → `%##Depth.Max` = count of nested `:` levels
- **Explicit `[#] %##Depth.Max`** → overrides inference

This means structs like `#Person` (with `.name#string`, `.age#int`) are automatically depth 0 and CAN be used as array/dict elements. A struct with `[:] :*#Handler` has depth 1 and CANNOT.

Collections used as value types require explicit `%##Depth.Max` — the compiler raises PGE11002 if depth is missing. Using `%##Depth.Max << -1` on a user-defined type triggers PGW11003 (only `#Serial` should use unlimited depth).

### `###` Field Types — Leaf Content

The `###` prefix describes the nature of leaf content in a type's fields. There are three field types:

| Field Type | Declaration | Meaning |
|------------|-------------|---------|
| `###Value` | Leaf holds typed data | Field has a `#type` annotation — inherits `#String` chain |
| `###Enum` | Leaf is variant selector | Field has no `#type` — identity IS the value (active variant) |
| `###None` | Leaf is nullable | No fields — empty string `""` is the only valid value |

`###None` marks a type as nullable. A variable annotated with a `###None` type holds empty string `""` — it represents the absence of a value. Only types with `[#] << ###None` accept empty string; all other types reject `""` with **PGE04021**.

**Compiler inference:** The compiler infers `###Value` if fields have `#type` annotations, `###Enum` if fields have no `#type`, and `###None` if the type has zero fields and zero schema (explicit `[#] << ###None` is required — it cannot be inferred). Explicit `[#] << ###Value` or `[#] << ###Enum` declaration is optional but allowed.

**Error codes:**
- **PGE04021** — empty string `""` assigned to a non-`###None` type: only `###None` types accept empty string
- **PGE11003** — explicit `###` contradicts fields: `###Value` on a type with untyped enum fields, `###Enum` on a type with typed value fields, or `###None` on a type with any fields
- **PGE05005** — sibling fields mix typed (`#type`) and untyped (enum) declarations: all siblings must be the same `###` kind

Examples from the type hierarchy:

```polyglot
{#} #Boolean
   [#] << ###Enum
   [ ] Matches — .True/.False have no #type annotation
   [.] .True
   [.] .False

{#} #String
   [#] << ###Value
   [ ] Matches — .string#RawString has a #type annotation
   [.] .string#RawString
   [.] .regex#RawString <~ ".*"

{#} #None
   [#] << ##Scalar
   [#] << ###None
   [ ] No fields — empty string "" is the only valid value
```

### Approved `##` Schema Types

Schema types are `{#}` definitions that set `%##` properties to describe common tree shapes. Types compose schemas with `[#] <<` lines (one line, one expression — they accumulate):

```polyglot
{#} ##Scalar
   [#] %##Depth.Max << 0

{#} ##Flat
   [#] %##Depth.Max << 1

{#} ##Deep
   [#] %##Depth.Max << -1

{#} ##Homogeneous
   [#] %##Children.Uniform << #True

{#} ##Heterogeneous
   [#] %##Children.Uniform << #False

{#} ##Contiguous
   [#] %##Children.Gap << #False
   [#] %##Children.Ordered << #True

{#} ##Sparse
   [#] %##Children.Gap << #True

{#} ##Rectangular
   [#] %##Children.Regular << #True
   [#] %##Children.Uniform << #True

{#} ##EnumLeafs
   [ ] All leaf fields must be ###Enum (no type annotation)
   [#] %##Leafs.Kind << #FieldKind.Enum
```

A type composes multiple schemas to describe its full shape. For example, `#Array` uses `##Contiguous` and `##Rectangular` together. User-defined schemas are possible but not generally recommended.

## See Also

- [[syntax/types/prefix-system|Prefix System]] — three-tier `#`/`##`/`###` overview
- [[syntax/types/structs|Struct Types]] — enum vs value fields and struct level rules
- [[syntax/types/macro-types|Macro-Generated Types]] — how macros set `%##` properties on generated types
