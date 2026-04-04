---
audience: pg-coder
type: specification
updated: 2026-04-04
---

# Schema Properties

<!-- @syntax/types/INDEX -->

## Schema Properties

`{#}` definitions gain **schema properties** declared with `[#] %##Property`. These are compile-time metadata prefixed with `%##` to explicitly mark them as tree-structure properties. They describe structural constraints on the type's tree shape:

| Property | Type | Applies to | Meaning |
|----------|------|-----------|---------|
| `%##Depth.Max` | `#int` | Universal | Max tree depth (0=atomic, 1=scalar/record, -1=unlimited) |
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

`%##Depth.Max` describes the maximum tree depth of a type's instances. Both fixed (`.`) and flexible (`:`) fields contribute to depth — any field one level below the root is depth 1.

| Value | Meaning | Examples |
|-------|---------|---------|
| `0` | Atomic — no fields at all (`##Leaf`) | `RawString` |
| `1` | Scalar/record — one level of fields | `#String`, `#Int`, `#Boolean`, `#Person`, `#Map`, `#Array` (1D) |
| `N` | N levels of nesting | `#Array` with `:ND` dimension |
| `-1` | Unlimited nesting | `#Serial` |

**Compiler inference:** When a `{#}` definition does not explicitly set `%##Depth.Max`, the compiler infers it:
- **Has `.` fixed fields only** → `%##Depth.Max = 1` (record/scalar)
- **Has `:` flexible fields** → `%##Depth.Max` = count of nested `:` levels (minimum 1)
- **No fields at all** → `%##Depth.Max = 0` (atomic, requires explicit `##Leaf`)
- **Explicit `[#] %##Depth.Max`** → overrides inference

This means structs like `#Person` (with `.name#string`, `.age#int`) are automatically depth 1 and CAN be used as array/dict elements. A struct with `[:] :*#Handler` has depth 1 and CANNOT be nested inside collections without explicit `%##Depth.Max`.

Collections used as value types require explicit `%##Depth.Max` — the compiler raises PGE11002 if depth is missing. Using `%##Depth.Max << -1` on a user-defined type triggers PGW11003 (only `#Serial` should use unlimited depth).

### `###` Field Types — Leaf Content

The `###` prefix describes the nature of leaf content in a type's fields. There are five field types — three general and two scalar-specific:

| Field Type | Declaration | Meaning |
|------------|-------------|---------|
| `###Value` | Leaf holds typed data | Field has a `#type` annotation — inherits `#String` chain |
| `###Enum` | Leaf is variant selector | Field has no `#type` — identity IS the value (active variant) |
| `###None` | Leaf is nullable | No fields — empty string `""` is the only valid value |
| `###ScalarValue` | Scalar typed data | Regex-validated string data — specifically `#String:*` family. Only valid with `##Scalar` |
| `###ScalarEnum` | Scalar variant selector | Variant selector in a scalar type — `#Boolean`, `#BaseCode`. Only valid with `##Scalar` |

`##Scalar` constrains leaf content to `###ScalarValue` or `###ScalarEnum` — no other `###` type is valid with `##Scalar`.

`###None` marks a type as nullable. A variable annotated with a `###None` type holds empty string `""` — it represents the absence of a value. Only types with `[#] << ###None` accept empty string; all other types reject `""` with **PGE04021**.

**Compiler inference:** The compiler infers `###Value` if fields have `#type` annotations, `###Enum` if fields have no `#type`, and `###None` if the type has zero fields and zero schema (explicit `[#] << ###None` is required — it cannot be inferred). Explicit `[#] << ###Value` or `[#] << ###Enum` declaration is optional but allowed.

**Error codes:**
- **PGE04021** — empty string `""` assigned to a non-`###None` type: only `###None` types accept empty string
- **PGE11003** — explicit `###` contradicts fields: `###Value` on a type with untyped enum fields, `###Enum` on a type with typed value fields, or `###None` on a type with any fields
- **PGE05005** — sibling fields mix typed (`#type`) and untyped (enum) declarations: all siblings must be the same `###` kind

Examples from the type hierarchy:

```polyglot
{#} #Boolean
   [#] << ###ScalarEnum
   [ ] Matches — .True/.False have no #type annotation; scalar enum type
   [.] .True
   [.] .False

{#} #String
   [#] << ###ScalarValue
   [ ] Matches — .string#RawString has a #type annotation; #String:* family
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
{#} ##Leaf
   [#] %##Depth.Max << 0

{#} ##Scalar
   [#] %##Depth.Max << 1

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

`##Scalar` is specifically for the `#String:*` family and scalar enums. It constrains leaf content to `###ScalarValue` or `###ScalarEnum`. `##Leaf` is reserved for truly atomic types with no fields at all (`RawString`).

A type composes multiple schemas to describe its full shape. For example, `#Array` uses `##Contiguous` and `##Rectangular` together. User-defined schemas are possible but not generally recommended.

## See Also

- [[syntax/types/prefix-system|Prefix System]] — three-tier `#`/`##`/`###` overview
- [[syntax/types/structs|Struct Types]] — enum vs value fields and struct level rules
- [[syntax/types/macro-types|Macro-Generated Types]] — how macros set `%##` properties on generated types
