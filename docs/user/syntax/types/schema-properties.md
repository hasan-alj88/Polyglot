---
audience: pg-coder
type: specification
updated: 2026-04-09
---

# Schema Properties

<!-- @syntax/types/INDEX -->

`{#}` definitions gain **schema properties** declared with `[#] %##Property << value`. These are compile-time metadata prefixed with `%##` to explicitly mark them as tree-structure properties. They describe structural constraints on the type's tree shape.

<!-- @c:syntax/types/prefix-system#metadata -->

**`%##` = metadata address.** Each `%##Property` is a relative path into the metadata tree. Inside `{#} #Array`, writing `[#] %##Fields << #Range` resolves to the absolute metadata address `%definition.#:Array.%##Fields -> #Range`. The `%` prefix marks these as compile-time metadata entries — not runtime data. See [[prefix-system#metadata|c:Metadata and the Tree Address System]] for the full address model.

**`##` = named property bundle.** A `##` schema is syntactic sugar for a reusable group of `%##` assignments. Writing `[#] ##Flat` is equivalent to writing `[#] %##Depth.Max << 1` — the compiler expands the schema into its constituent `%##` properties. The `%##` properties are the ground truth; `##` schemas are a convenience layer. See [[prefix-system#named-property-bundles|c:Named Property Bundles]] for expansion examples.

Schema properties apply universally via `[#]`, or branch-wise via `[.]`/`[:]`. Conflict between universal and branch-wise scope raises PGE11001. If a `%##` property is redundant with an inherited value, the compiler raises PGW11001; if it contradicts, the override takes effect with PGW11002.

Schema properties live in the metadata tree at `%definition.#:{TypeName}.{Property}`, making them introspectable at compile time. Schema references (`##`) are only valid inside `{#}` type definitions -- using them outside raises PGE05006.

**Syntax distinction:** Schema composition uses `[#] ##Schema` (no `<<`). Property assignment uses `[#] %##Prop << value` (with `<<`). The `<<` signals "assign this value"; its absence signals "compose this schema's properties."

## `%##` Branch-Level Properties

`##` implies "children at this level" -- no `Children.` prefix is needed. These properties describe how branches behave at the level where they are declared:

| Property | Type | Meaning |
|----------|------|---------|
| [[properties/Fields\|%##Fields]] | `#FieldsDescriptor` or `##Enum` ref | `#Range` = integer-indexed; enum ref = stamp children from variants |
| [[properties/Schema\|%##Schema]] | list of `##` | Structural schemas children must satisfy (AND-composed) |
| [[properties/Active\|%##Active]] | `#ActiveKind` | `#All` (every branch present) / `#One` (exactly one active) / `#Partial` (any non-zero subset) |
| [[properties/Ordered\|%##Ordered]] | `#Boolean` | Insertion order preserved? |
| [[properties/Sorted\|%##Sorted]] | `#Boolean` | Sorted by key? (order derived from key type: numeric, alphabetical, or declaration) |
| [[properties/Gap\|%##Gap]] | `#Boolean` | Gaps allowed in keys? |
| [[properties/Count\|%##Count]] | `#Bound` | Max children (`#Inf` = unlimited) |
| `%##Count.Min` | `#uint` | Min children (0 if absent) |
| [[properties/Propagate\|%##Propagate]] | `#Boolean` | Apply these properties recursively to all levels down to `%##Depth.Max` |
| [[properties/Level\|%##Level.N]] | scope | Per-level override when `%##Propagate` is `#True` |

### `%##Fields` -- Child Field Descriptor

`%##Fields` unifies how a type's children are described. It accepts either `#FieldsDescriptor` variants or a `##Enum`-satisfying type reference:

| Value | Effect | Example |
|-------|--------|---------|
| `#Range` | Integer-indexed children (`:0`, `:1`, `:2`, ...) | `##Array` |
| `#SomeEnum` ref | Stamps one `:` child per enum variant | `##Record` with `%##Fields << #DayOfWeek` |

When `%##Fields` receives an enum reference, the compiler reads the enum's variants and generates corresponding flexible fields on the type. This replaces the old `##Fields` parameterized schema.

`#FieldsDescriptor` is the enum that governs `%##Fields`:

```polyglot
{#} #FieldsDescriptor
   [#] ##Scalar
   [#] %##Active << #ActiveKind.One
   [.] .Range
      [#] %##Alias << "range"
   [.] .Enum
      [#] %##Alias << "enum"
```

`#Range` (alias for `#FieldsDescriptor.Range`) and `#Enum` (alias for `#FieldsDescriptor.Enum`) are the two variants.

## `%##` Tree-Level Properties

These properties describe the whole type tree, not individual branches:

| Property | Type | Meaning |
|----------|------|---------|
| [[properties/Depth-Max\|%##Depth.Max]] | `#Bound` | Max tree depth (0, 1, N, `#Inf`) |
| [[properties/Alias\|%##Alias]] | `#NestedKeyString` | Lowercase shorthand name |

### %##Depth.Max -- Inference Model

`%##Depth.Max` describes the maximum tree depth of a type's instances. Both fixed (`.`) and flexible (`:`) fields contribute to depth -- any field one level below the root is depth 1.

| Value | Meaning | Examples |
|-------|---------|---------|
| `0` | Atomic -- no fields at all (`##Leaf`) | `RawString` |
| `1` | Scalar/record -- one level of fields | `#String`, `#Int`, `#Boolean`, `#Person`, `##Record` types, `#Array` (1D) |
| `N` | N levels of nesting | `#Array` with `:ND` dimension |
| `#Inf` | Unlimited nesting | `#Serial` |

**Compiler inference:** When a `{#}` definition does not explicitly set `%##Depth.Max`, the compiler infers it:
- **Has `.` fixed fields only** -- `%##Depth.Max = 1` (record/scalar)
- **Has `:` flexible fields** -- `%##Depth.Max` = count of nested `:` levels (minimum 1)
- **No fields at all** -- `%##Depth.Max = 0` (atomic, requires explicit `##Leaf`)
- **Explicit `[#] %##Depth.Max`** -- overrides inference

This means structs like `#Person` (with `.name#string`, `.age#int`) are automatically depth 1 and CAN be used as array/dict elements. A struct with `[:] :*#Handler` has depth 1 and CANNOT be nested inside collections without explicit `%##Depth.Max`.

Collections used as value types require explicit `%##Depth.Max` -- the compiler raises PGE11002 if depth is missing. Using `%##Depth.Max << #Inf` on a user-defined type triggers PGW11003 (only `#Serial` should use unlimited depth).

### #Bound and #Inf

`#Bound` is the type used by `%##Count` and `%##Depth.Max`. It has an `.Inf` variant aliased as `#Inf`, following the `#Boolean.True` → `#True` pattern:

```polyglot
{#} #Bound
   [#] ##Int
   [#] ##Inf
   [.] .Inf
      [#] %##Alias << "inf"
   [#] %##Active << #ActiveKind.One
```

`#Inf` (alias for `#Bound.Inf`) represents an unlimited bound. Use `%##Depth.Max << #Inf` for unlimited nesting, `%##Count << #Inf` for unlimited children.

## `%###` Leaf-Level Properties

The `%###` properties describe leaf content constraints:

| Property | Type | Meaning |
|----------|------|---------|
| [[properties/Kind\|%###Kind]] | `#FieldKind` | `###Value` (data) or `###Enum` (identity) |
| [[properties/Type\|%###Type]] | type ref | Type all leaves must be. `#` = any type. Absent = per-field annotation |
| [[properties/Unique\|%###Unique]] | `#Boolean` | Leaf values must be distinct? |

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

`###None` marks a type as nullable. A variable annotated with a `###None` type holds empty string `""` -- it represents the absence of a value. Only types with `[#] ###None` accept empty string; all other types reject `""` with **PGE04021**.

**Compiler inference:** The compiler infers `###Value` if fields have `#type` annotations, `###Enum` if fields have no `#type`, and `###None` if the type has zero fields and zero schema (explicit `[#] ###None` is required -- it cannot be inferred). Explicit `[#] ###Value` or `[#] ###Enum` declaration is optional but allowed.

**Error codes:**
- **PGE04021** -- empty string `""` assigned to a non-`###None` type: only `###None` types accept empty string
- **PGE11003** -- explicit `###` contradicts fields: `###Value` on a type with untyped enum fields, `###Enum` on a type with typed value fields, or `###None` on a type with any fields
- **PGE05005** -- sibling fields mix typed (`#type`) and untyped (enum) declarations: all siblings must be the same `###` kind

Examples from the type hierarchy:

```polyglot
{#} #Boolean
   [#] ##Enum
   [#] ###ScalarEnum
   [ ] Matches -- .True/.False have no #type annotation; ##Enum + scalar enum type
   [.] .True
   [.] .False

{#} #String
   [#] ###ScalarValue
   [ ] Matches -- .string#RawString has a #type annotation; #String:* family
   [.] .string#RawString
   [.] .regex#RawString <~ ".*"

{#} #None
   [#] ##Scalar
   [#] ###None
   [ ] No fields -- empty string "" is the only valid value
```

## Approved `##` Schema Types

Schema types are `{#}` definitions that set `%##` properties to describe common tree shapes. Types compose schemas with `[#] ##Schema` lines (one line, one expression -- they accumulate). Note: schema composition uses `[#] ##Schema` (no `<<`), while property assignment uses `[#] %##Prop << value` (with `<<`).

### Depth Schemas

```polyglot
{#} ##Leaf
   [#] %##Depth.Max << 0

{#} ##Scalar
   [#] %##Depth.Max << 1

{#} ##Flat
   [#] %##Depth.Max << 1
```

### Value Schema

```polyglot
{#} ##Inf
   [ ] Composable .Inf variant -- not a standalone type
```

`##Inf` adds an `.Inf` variant to a type. It is a schema, not a type -- compose it into `#` types that need an unbounded option (e.g., `#Bound`).

### Structure Schemas

```polyglot
{#} ##Sorted
   [#] %##Sorted << #True
   [#] %##Ordered << #True
```

### Classification Schemas

```polyglot
{#} ##Enum
   [#] ##Flat
   [#] %##Active << #ActiveKind.One
   [#] %###Kind << #FieldKind.Enum
```

`##Enum` classifies struct types whose fields are all enum fields (no `#type` annotation) -- `#Boolean`, `#QueueStrategy`, `#KillPropagation`, `#OS`, etc. It composes `##Flat` (depth 1) with the enum leaf constraint and single-active-branch rule.

### Collection Schemas

```polyglot
{#} ##Record
   (#) <#Fields << ##Enum
   (#) <#ValueType <~ #
   [#] ##Flat
   [#] %##Fields << <#Fields
   [#] %##Active << #ActiveKind.All
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value

{#} ##Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] %##Depth.Max << <Dim
   [#] %##Fields << #Range
   [#] %##Ordered << #True
   [#] %##Gap << #False
   [#] %##Propagate << #True
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value

{#} ##Dataframe
   (#) <#Columns << ##Enum
   (#) <#CellType <~ #
   [#] %##Depth.Max << 2
   [#] %##Fields << #Range
   [#] %##Ordered << #True
   [#] %##Gap << #False
   [#] %##Level.2 ##Record
      (#) <#Fields << <#Columns
      (#) <#ValueType << <#CellType
```

`##Record` is enum-keyed: the compiler reads `%##Fields << #SomeEnum` and stamps one `:` child per variant. `##Array` is range-indexed: `%##Fields << #Range` means integer-indexed children. `##Dataframe` is two-level: L1 range-indexed rows, L2 `##Record` columns.

### Other Parameterized Schemas

| Schema | Parameters | Purpose |
|--------|-----------|---------|
| `##Nullable` | `<#ValueType` | `.Ok.Value` OR `.None` |
| `##Result` | `<#OkType, <#ErrType` | `.Ok.Value` OR `.Err.Value` |
| `##String` | `<regex` | `.string` + `.regex` pattern |

A type composes multiple schemas to describe its full shape. User-defined schemas are possible but not generally recommended.

### Retired Schemas

| Schema | Replacement | Reason |
|--------|-------------|--------|
| `##Map` | `##Record` | Enum-keyed records replace sparse key-value maps |
| `##Set` | `##Array` + `%###Unique << #True` | Sets are arrays with uniqueness constraint |
| `##Contiguous` | `%##Gap << #False`, `%##Ordered << #True` | Properties stated directly |
| `##Rectangular` | `%##Propagate << #True`, `%##Count` | Properties stated directly |
| `##Sparse` | `%##Gap << #True` | Property stated directly |
| `##Deep` | `%##Depth.Max << #Inf` | Property stated directly |

## See Also

- [[properties/INDEX|%## and %### Properties]] -- individual property files with allow/disallow ASCII trees
- [[schemas/INDEX|## Schema Types]] -- named bundles of these properties
- [[field-types/INDEX|### Field Types]] -- leaf content classification
- [[syntax/types/prefix-system|Prefix System]] -- three-tier `#`/`##`/`###` overview and `%` metadata addressing
- [[syntax/types/structs|Struct Types]] -- enum vs value fields and struct level rules
- [[FieldsDescriptor]] -- `#FieldsDescriptor` enum used by `%##Fields`
- [[ActiveKind]] -- `#ActiveKind` enum used by `%##Active`
- [[Bound]] -- `#Bound` type with `#Inf` alias, used by `%##Count` and `%##Depth.Max`
