---
audience: pg-coder
type: specification
updated: 2026-04-08
---

# Schema Properties

<!-- @syntax/types/INDEX -->

`{#}` definitions gain **schema properties** declared with `[#] %##Property`. These are compile-time metadata prefixed with `%##` to explicitly mark them as tree-structure properties. They describe structural constraints on the type's tree shape.

Schema properties apply universally via `[#]`, or branch-wise via `[.]`/`[:]`. Conflict between universal and branch-wise scope raises PGE11001. If a `%##` property is redundant with an inherited value, the compiler raises PGW11001; if it contradicts, the override takes effect with PGW11002.

Schema properties live in the metadata tree at `%definition.#:{TypeName}.{Property}`, making them introspectable at compile time. Schema references (`##`) are only valid inside `{#}` type definitions -- using them outside raises PGE05006.

## `%##` Branch-Level Properties

`##` implies "children at this level" -- no `Children.` prefix is needed. These properties describe how branches behave at the level where they are declared:

| Property | Type | Meaning |
|----------|------|---------|
| `%##Flexible` | `#FlexKind` | `.Fixed` (`.` fields), `.Flexible` (`:` user-controlled), `.Range` (`:` compiler-generated from `%##Range`) |
| `%##Key` | type ref | Type of flexible `:` child names. Only valid when `%##Flexible` is `.Flexible` or `.Range` |
| `%##Range` | range expr | Valid key interval (numeric keys only). Only valid when `%##Flexible` is `.Range` |
| `%##Schema` | list of `##` | Structural schemas children must satisfy (AND-composed) |
| `%##Active` | `#ActiveKind` | `.All` (every branch present) / `.One` (exactly one active) / `.Partial` (any non-zero subset) |
| `%##Ordered` | `#Boolean` | Insertion order preserved? |
| `%##Sorted` | `#Boolean` | Sorted by key? (order derived from key type: numeric, alphabetical, or declaration) |
| `%##Gap` | `#Boolean` | Gaps allowed in keys? |
| `%##Regular` | `#Boolean` | Same child count per sub-branch? |
| `%##Count` | `#Bound` | Max children (`.Inf` = unlimited) |
| `%##Count.Min` | `#uint` | Min children (0 if absent) |
| `%##Propagate` | `#Boolean` | Apply these properties recursively to all levels down to `%##Depth.Max` |
| `%##Level.N` | scope | Per-level override when `%##Propagate` is `#True` |

## `%##` Tree-Level Properties

These properties describe the whole type tree, not individual branches:

| Property | Type | Meaning |
|----------|------|---------|
| `%##Depth.Max` | `#Bound` | Max tree depth (0, 1, N, `.Inf`) |
| `%##Alias` | `#NestedKeyString` | Lowercase shorthand name |

### %##Depth.Max -- Inference Model

`%##Depth.Max` describes the maximum tree depth of a type's instances. Both fixed (`.`) and flexible (`:`) fields contribute to depth -- any field one level below the root is depth 1.

| Value | Meaning | Examples |
|-------|---------|---------|
| `0` | Atomic -- no fields at all (`##Leaf`) | `RawString` |
| `1` | Scalar/record -- one level of fields | `#String`, `#Int`, `#Boolean`, `#Person`, `#Map`, `#Array` (1D) |
| `N` | N levels of nesting | `#Array` with `:ND` dimension |
| `.Inf` | Unlimited nesting | `#Serial` |

**Compiler inference:** When a `{#}` definition does not explicitly set `%##Depth.Max`, the compiler infers it:
- **Has `.` fixed fields only** -- `%##Depth.Max = 1` (record/scalar)
- **Has `:` flexible fields** -- `%##Depth.Max` = count of nested `:` levels (minimum 1)
- **No fields at all** -- `%##Depth.Max = 0` (atomic, requires explicit `##Leaf`)
- **Explicit `[#] %##Depth.Max`** -- overrides inference

This means structs like `#Person` (with `.name#string`, `.age#int`) are automatically depth 1 and CAN be used as array/dict elements. A struct with `[:] :*#Handler` has depth 1 and CANNOT be nested inside collections without explicit `%##Depth.Max`.

Collections used as value types require explicit `%##Depth.Max` -- the compiler raises PGE11002 if depth is missing. Using `%##Depth.Max << .Inf` on a user-defined type triggers PGW11003 (only `#Serial` should use unlimited depth).

## `%###` Leaf-Level Properties

The `%###` properties describe leaf content constraints:

| Property | Type | Meaning |
|----------|------|---------|
| `%###Kind` | `#FieldKind` | `###Value` (data) or `###Enum` (identity) |
| `%###Type` | type ref | Type all leaves must be. `#` = any type. Absent = per-field annotation |
| `%###Unique` | `#Boolean` | Leaf values must be distinct? |

## `###` Field Types -- Leaf Content

The `###` prefix describes the nature of leaf content in a type's fields. There are five field types -- three general and two scalar-specific:

| Field Type | Declaration | Meaning |
|------------|-------------|---------|
| `###Value` | Leaf holds typed data | Field has a `#type` annotation -- inherits `#String` chain |
| `###Enum` | Leaf is variant selector | Field has no `#type` -- identity IS the value (active variant) |
| `###None` | Leaf is nullable | No fields -- empty string `""` is the only valid value |
| `###ScalarValue` | Scalar typed data | Regex-validated string data -- specifically `#String:*` family. Only valid with `##Scalar` |
| `###ScalarEnum` | Scalar variant selector | Variant selector in a scalar type -- `#Boolean`, `#NativeKind`. Only valid with `##Scalar` |

`##Scalar` constrains leaf content to `###ScalarValue` or `###ScalarEnum` -- no other `###` type is valid with `##Scalar`.

`###None` marks a type as nullable. A variable annotated with a `###None` type holds empty string `""` -- it represents the absence of a value. Only types with `[#] << ###None` accept empty string; all other types reject `""` with **PGE04021**.

**Compiler inference:** The compiler infers `###Value` if fields have `#type` annotations, `###Enum` if fields have no `#type`, and `###None` if the type has zero fields and zero schema (explicit `[#] << ###None` is required -- it cannot be inferred). Explicit `[#] << ###Value` or `[#] << ###Enum` declaration is optional but allowed.

**Error codes:**
- **PGE04021** -- empty string `""` assigned to a non-`###None` type: only `###None` types accept empty string
- **PGE11003** -- explicit `###` contradicts fields: `###Value` on a type with untyped enum fields, `###Enum` on a type with typed value fields, or `###None` on a type with any fields
- **PGE05005** -- sibling fields mix typed (`#type`) and untyped (enum) declarations: all siblings must be the same `###` kind

Examples from the type hierarchy:

```polyglot
{#} #Boolean
   [#] << ##Enum
   [#] << ###ScalarEnum
   [ ] Matches -- .True/.False have no #type annotation; ##Enum + scalar enum type
   [.] .True
   [.] .False

{#} #String
   [#] << ###ScalarValue
   [ ] Matches -- .string#RawString has a #type annotation; #String:* family
   [.] .string#RawString
   [.] .regex#RawString <~ ".*"

{#} #None
   [#] << ##Scalar
   [#] << ###None
   [ ] No fields -- empty string "" is the only valid value
```

## Approved `##` Schema Types

Schema types are `{#}` definitions that set `%##` properties to describe common tree shapes. Types compose schemas with `[#] <<` lines (one line, one expression -- they accumulate):

### Depth Schemas

```polyglot
{#} ##Leaf
   [#] %##Depth.Max << 0

{#} ##Scalar
   [#] %##Depth.Max << 1

{#} ##Flat
   [#] %##Depth.Max << 1

{#} ##Deep
   [#] %##Depth.Max << .Inf
```

### Value Schema

```polyglot
{#} ##Inf
   [ ] Composable .Inf variant -- not a standalone type
```

`##Inf` adds an `.Inf` variant to a type. It is a schema, not a type -- compose it into `#` types that need an unbounded option (e.g., `#Bound`).

### Structure Schemas

```polyglot
{#} ##Contiguous
   [#] %##Gap << #False
   [#] %##Ordered << #True

{#} ##Sparse
   [#] %##Gap << #True

{#} ##Sorted
   [#] %##Sorted << #True
   [#] %##Ordered << #True

{#} ##Rectangular
   [#] <Dim <~ "1D"
   [#] %##Regular << #True
   [#] %##Depth.Max << <Dim
   [#] %##Flexible << #FlexKind.Range
   [#] %##Propagate << #True
```

`##Rectangular` is parameterized -- `<Dim` sets both depth and propagation scope. When composed, it generates per-level structure down to `Dim` levels.

### Classification Schemas

```polyglot
{#} ##Enum
   [#] << ##Flat
   [#] %##Active << #ActiveKind.One
   [#] %###Kind << #FieldKind.Enum
```

`##Enum` classifies struct types whose fields are all enum fields (no `#type` annotation) -- `#Boolean`, `#QueueStrategy`, `#KillPropagation`, `#OS`, etc. It composes `##Flat` (depth 1) with the enum leaf constraint and single-active-branch rule.

### Parameterized Schemas

Parameterized schemas use `[#] <param` inputs to generate structure. See individual schema files for details:

| Schema | Parameters | Purpose |
|--------|-----------|---------|
| `##Fields` | `<#Type(##Enum)` | Stamps enum variants as `[.]` fields |
| `##Nullable` | `<#ValueType` | `.OK.Value` OR `.None` |
| `##Result` | `<#OkType, <#ErrType` | `.OK.Value` OR `.Err.Value` |
| `##String` | `<regex` | `.string` + `.regex` pattern |
| `##Map` | `<#KeyType, <#ValueType` | Flat sparse flexible key-value |
| `##Array` | `<#ValueType, <Dim` | Contiguous rectangular N-dimensional |
| `##Set` | `<#ValueType` | Sparse + `%###Unique` |
| `##Dataframe` | `<#Columns, <#CellType` | Array of maps |

A type composes multiple schemas to describe its full shape. For example, `#Array` uses `##Contiguous` and `##Rectangular` together. User-defined schemas are possible but not generally recommended.

## See Also

- [[syntax/types/prefix-system|Prefix System]] -- three-tier `#`/`##`/`###` overview
- [[syntax/types/structs|Struct Types]] -- enum vs value fields and struct level rules
- [[FlexKind]] -- `#FlexKind` enum used by `%##Flexible`
- [[ActiveKind]] -- `#ActiveKind` enum used by `%##Active`
- [[Bound]] -- `#Bound` type used by `%##Count` and `%##Depth.Max`
