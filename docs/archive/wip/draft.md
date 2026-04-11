---
status: deprecated
archived: 2026-04-11
source_branch: main
note: Working draft for Plan 94-01 macro-for-generics redesign — content now merged into spec
replaced_by: docs/user/syntax/types.md
---
<!-- @d:docs/user/syntax/types.md -->
> **Deprecated:** This document is superseded. See the current spec for up-to-date content.

# Plan 94-01: Ground Truths + Macro-for-Generics Redesign

## Context

Issues #94, #91, #79 converge on a deeper redesign of Polyglot's type system foundations. Before tackling individual issues, we need to:

1. Codify the **ground truths** of Polyglot's data model as the authoritative preamble in `types.md`
2. **Replace generic `{#}` type parameters** (`<param`) with `{M}` macro-generated definitions
3. Split `{M}` into `{M}` (type macros) and `{W}` (wrappers) with `[M]`/`[W]` invocation markers
4. Rewrite stdlib collection types (#Array, #Map, #Dataframe) and scalar subtypes (##Int, ##Float, etc.) as macro-generated

This plan is **Plan 94-01** (ground truths + redesign). Follow-up plans will address:
- 94-02: `=#.*` runtime validation pipelines (issue #94)
- 94-03: Expand/collect audit with tree schema system (issue #91)
- 94-04: Serial file loading pipelines (issue #79)

## Ground Truths (to add as types.md preamble)

These axioms define Polyglot's data model:

1. **ALL Polyglot objects are data trees with `#RawString` leafs** — types, definitions, pipelines, expanders, collectors, errors — everything is a tree
2. **ALL data has a `%` metadata address** — both compile-time and runtime data lives on the unified metadata tree (future: backed by NoSQL DB)
3. **`##` schemas are data trees that describe tree structure and constraints** — shape, depth, key patterns, ordering, uniformity, regex patterns. A `#` struct may have multiple `##` schema specifications. `##` schemas are NOT structs — they describe properties of structs
4. **`#RawString` is the only primitive** — its limitations match the compiler host language; currently assumed theoretically unlimited
5. **`#String` extends `#RawString` with regex enforcement** — from it derive all `##Scalar` types (##Int, ##Float, ##Dimension, ##KeyString, etc.)
6. **Not all tree properties can be expressed via regex** — hence `%` metadata properties (`##` schemas) describe tree shapes the compiler can identify and enforce
7. **`#`, `##`, `###` are user-friendly aliases for `%` metadata** — users should never need to use `%` directly; the prefix tiers are the user-facing alternative
8. **`{x}` = compile-time definitions; `=` pipelines = runtime definitions** — clean separation
9. **`{M}` macros replace generic type parameters** — parameterized types use macros to generate `{#}` definitions at compile time

## Design: {M} Type Macros and {W} Wrappers

### Current State (to be removed)

Generic type parameters via `<param` in `{#}` headers:
```polyglot
{#} #Array<ValueType<Dim
   (#) <ValueType << #*
      [<] ##Scalar
   (#) <Dim << #Dimension
      [<] ##Scalar
```

### New State: {M} and {W} as Separate Entities

The old `{M}` is split into two distinct definition blocks:

| Block | Role | Invoked by | Input marker | Must contain | Must NOT contain |
|-------|------|-----------|-------------|-------------|-----------------|
| `{M} #Name` | Type macro | `[M]` | `(#) <Param` | `{#}` definition | `[\]`/`[/]` |
| `{W} =W.Name` | Wrapper | `[W]` | `[{] $Param` / `[}] $Param` | `[\]`/`[/]` | `{#}` definition |

- `{M}` **defines** type macros, `[M]` **invokes** them — mirrors `{#}`/`[#]`, `{=}`/`[=]` pattern
- `{W}` **defines** wrappers, `[W]` **invokes** them — wrappers are their own entity, not a macro variant
- `<~` returns to meaning **only** inheritance/default — never macro invocation

### Macro Invocation with `[M]`

`[M]` inside a `{#}` block invokes a `{M}` macro. Arguments use `[#]` — the same marker as the macro's parameter declarations:

```polyglot
{#} ##Int
   [M] #String.Subtype
      (#) <Name << "Int"
      (#) <Alias << "int"
         [<] !Alias.Clash << "integer"
         [<] !Alias.Clash << "Integer"
      (#) <Regex << "^-?[0-9]+$"
```

- `[M] #String.Subtype` — invoke the `{M} #String.Subtype` macro
- `(#) <Name << "Int"` — pass argument matching macro's `(#) <Name#RawString` parameter
- `[<] !Alias.Clash` — error-driven fallback: if alias "int" clashes, try "integer", then "Integer"
- **`[M]` merge rule (identity):** The outer `{#} ##Int` **names** the result; the macro's internal `{#} ##{$Name}` resolves to the same name, confirming identity. The macro fills the body. Any `[#]` lines after `[M]` in the outer `{#}` extend/override the macro's output

### `!Alias.Clash` Error and Namespace Aliases

Aliases serve two purposes:
1. **Shorthand** — shorter name for convenience (`#int` → `##Int`)
2. **Namespace placement** — places a definition at multiple locations in the `%` metadata tree

**PGW-9XX** — Namespace alias warning: "Alias `{alias}` creates namespace placement at `{path}` — compiler verifies no name clash in target namespace." Fires when an alias places a definition into a different namespace branch than its canonical location.

**!Alias.Clash** — Compile error when an alias collides with an existing name in the target namespace. The `[<]` fallback chain under `(#) <Alias` provides alternative alias values:

```polyglot
(#) <Alias << "int"           [ ] Try "int" first
   [<] !Alias.Clash << "integer"  [ ] If "int" clashes, try "integer"
   [<] !Alias.Clash << "Integer"  [ ] If "integer" also clashes, try "Integer"
```

### Macro Dispatch Rule

Macros overload by **signature** — the ordered list of parameter count and kind:

| Kind | Syntax | Meaning |
|------|--------|---------|
| Type input | `(#) <#ParamName` | A `#` type definition as input (datatypes are data trees) |
| Value input | `(#) <ParamName##Type` | A typed value as input |

Dispatch matches by **parameter count AND parameter kind** (`<#` type vs `<` value). Examples:
- `{M} #Map` → signature `(<#, <#)` = homogeneous (2 type inputs)
- `{M} #Map` → signature `(<#)` = heterogeneous (1 type input)
- `{M} #Dataframe` → signature `(<#, <#)` = compile-time safe (2 type inputs)
- `{M} #Dataframe` → signature `(<, <)` = runtime flexible (2 value inputs)

### `##` Schema Composition Rules

`##` schemas are **not inheritance** — they are shorthand for setting `%` metadata properties. When a `{#}` definition composes multiple `##` schemas via `[#] <<`:

- Two schemas setting the **same** `%` property to the **same** value → consistent (no error, they agree)
- Two schemas setting the **same** `%` property to **different** values → **PGE11001** (schema property conflict)
- Schemas accumulate — each `[#] ##Schema` adds its `%` properties to the definition's metadata

Example: `[#] ##Contiguous` sets `%##Children.Gap << #False` + `%##Children.Ordered << #True`. If another schema also sets `%##Children.Ordered << #True`, they agree. If it sets `%##Children.Ordered << #False`, PGE11001.

### `%name` Metadata Accessor

`%name` returns the definition name string from the `{x}` block header:

| Context | `%name` returns |
|---------|----------------|
| `{#} #ThisName` | `"ThisName"` |
| `{M} #String.Subtype` | `"String.Subtype"` |
| `{=} =Pipeline.Name` | `"Pipeline.Name"` |
| `{W} =W.Polyglot` | `"W.Polyglot"` |

**`%name.Last`** — splits by `.` and returns the last segment:
- `{M} #String.Subtype` → `%name.Last` = `"Subtype"`
- `{#} ##Int` → `%name.Last` = `"Int"`

### Compiler Bootstrap Layers

Type macros are compiled in a staged sequence:

| Layer | What | Capabilities | Cannot Use |
|-------|------|-------------|------------|
| 0 — Hardcoded | `#RawString`, `#String`, `{M}` engine | Compiler intrinsics — not defined in Polyglot code | N/A |
| 1 — Self-hosted | `#String.Subtype` macro, all `##` scalar types, `##CommaSeparatedList` | `{$var}` interpolation, `{%This}` metadata access | `[r] =Pipeline` calls |
| 2 — Full macros | `#Array`, `#Map`, `#Dataframe` | `=String.Lower`, `=UID`, `=#list.into.Enum` — full pipeline execution | N/A |

Layer 1 macros bootstrap without a pipeline engine (string substitution only). Layer 2 macros run after scalar types exist. `##CommaSeparatedList` (Layer 1) breaks the circular dependency that `#Array1D:String` (Layer 2) would create.

### `%This` Scoping Rule

`%This` refers to the **innermost enclosing `{x}` definition block**:

| Context | `%This` refers to |
|---------|-------------------|
| Inside `{M} #String.Subtype` body (outside nested `{#}`) | The macro |
| Inside `{#} ##{$Name}` nested within the macro | The `{#}` definition being generated |
| Outside any `{x}` block | Compile error (PGE-XXX) |

To reference the enclosing macro from inside a nested `{#}`, use `%Parent` (one level up).

### `[#]` Roles Inside `{#}` and `{M}` Blocks

`[#]` is overloaded but distinguished by what follows:

| Pattern | Role | Context |
|---------|------|---------|
| `[#] %Property` | Schema property declaration | `{#}` |
| `[#] ##Schema` | Schema composition (property substitution) | `{#}` |
| `(#) <~ #Parent` | Inheritance (copy parent's `%` properties as defaults) | `{#}` |
| `(#) <Param` | Macro input parameter declaration | `{M}` only |
| `(#) <Param << "value"` | Macro argument (pass value to `(#) <Param`) | `[M]` invocation |

Note: `<~` in `{#}` means **only** inheritance — copying the parent type's `%` metadata properties as defaults. It never means macro invocation. Macros are invoked exclusively via `[M]`.

### Collection Type Macros

```polyglot
{M} #Array
   [ ] First input: a DataType — `<#` means type input (all definitions are data trees)
   (#) <#ValueType
      [<] ##Scalar
   [ ] Optional second input: a Dimension (defaults to 1D)
   (#) <Dim##Dimension <~ "1D"
      [<] ##Scalar

   [ ] Direct substitution via {} inside double quotes — implicit inline pipeline
   [ ] ##DataTypeString: new stdlib schema for valid {x} definition names
   [r] $ArrayName##DataTypeString << "Array{$Dim}:{$ValueType%name}"
   [r] $dim#RawString << =String.Lower"{$Dim}"

   {#} #{$ArrayName}
      (#) <~ #Map:#UnsignedInt:$ValueType
      [#] %##Alias
         [:] << "array:{$ValueType%name}:{$dim}"
         [:] << "array{$dim}:{$ValueType%name}"
         [:] << "Array{$Dim}:{$ValueType%name}"
      [#] %##Children.Type << #UnsignedInt
      [#] %##Children.Ordered << #True
      [#] %##Children.Uniform << #True
      [#] ##Contiguous
      [#] ##Rectangular
      [#] %##Depth.Max << $Dim
      [:] :*#$ValueType

{ } Homogeneous variant — dispatched by signature (<#, <#)
{M} #Map
   (#) <#KeyType
      [<] ##EnumLeafs
   (#) <#ValueType
      [<] ##Scalar

   [r] $UniformMapName##DataTypeString << "Map:{$KeyType%name}:{$ValueType%name}"
   {#} #{$UniformMapName}
      [#] %##Alias
         [:] << "map:{$KeyType%name}:{$ValueType%name}"
         [:] << "Map:{$KeyType%name}:{$ValueType%name}"
      [#] %##Children.Type << $KeyType
      [#] ##Flat
      [#] ##Homogeneous
      [#] ##Sparse
      [:] :*#$ValueType

{ } Heterogeneous variant — dispatched by signature (<#)
{M} #Map
   (#) <#KeyType
      [<] ##EnumLeafs

   [r] $MapName##DataTypeString << "Map:{$KeyType%name}"
   {#} #{$MapName}
      [#] %##Alias
         [:] << "map:{$KeyType%name}"
         [:] << "Map:{$KeyType%name}"
      [#] %##Children.Type << $KeyType
      [#] ##Flat
      [#] ##Heterogeneous
      [#] ##Sparse
      [:] :*#*

{ } Compile-time safe Dataframe — dispatched by signature (<#, <#)
{ } Dataframe = Array of Map — each row is a Map, array holds rows
{M} #Dataframe
   (#) <#ColumnEnum
      [<] ##EnumLeafs
   (#) <#CellType
      [<] ##Scalar

   [r] $DfName##DataTypeString << "Dataframe:{$ColumnEnum%name}:{$CellType%name}"
   {#} #{$DfName}
      [#] %##Alias
         [:] << "dataframe:{$ColumnEnum%name}:{$CellType%name}"
         [:] << "Dataframe:{$ColumnEnum%name}:{$CellType%name}"
      [#] ##Contiguous
      [#] ##Rectangular
      [#] ##Ordered
      [:] :*#Map:$ColumnEnum:$CellType

{ } Runtime flexible Dataframe — dispatched by signature (<, <)
{ } Needs =#.Validate at runtime for schema enforcement
{ } Dataframe = Array of Map — each row is a string-keyed map
{M} #Dataframe
   (#) <Columns##CommaSeparatedList
   (#) <DataFrameName##DataTypeString

   [ ] Dynamically generate an Enum from the column names
   [r] $uid##DataTypeString << =UID""
   {#} #DataFrameColumns{$uid}
      [ ] Generates an Enum from the column name list
      [#] << =#list.into.Enum"{$Columns}"

   {#} #{$DataFrameName}
      [#] %##Alias
         [:] << "dataframe:{$Columns%name}"
         [:] << "DataFrame:{$Columns%name}"
      [#] ##Contiguous
      [#] ##Rectangular
      [#] ##Ordered
      [:] :*#Map:#String:#String
```

### Dataframe Access Pattern

Dataframe is an `#Array` of `#Map` — row-oriented. Access: `$df<row<column`.

```polyglot
{#} #SalesColumns
   [#] ##Scalar
   [#] ###Enum
   [.] .product
   [.] .price
   [.] .quantity

[r] $sales#dataframe:SalesColumns:string <~ {}

[ ] Access: <row<column
[r] $name#string << $sales<0<product       [ ] row 0, column "product"
[r] $price#string << $sales<2<price        [ ] row 2, column "price"
[r] $row#map:SalesColumns:string << $sales<0   [ ] entire row as Map
```

### Non-Parameterized Types

`#Serial` has no type parameters — it stays as a plain `{#}` definition (no macro needed):

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
   [#] ##Deep
   [#] ##Sparse
   [#] ##Heterogeneous
   [:] :*#*
```

### Scalar Subtype Macro

Scalars are `##` schemas (not `#` structs) — they are regex constraints on `#String`, living at `%##` on the metadata tree. This avoids name clashes with `#` structs. A `#` struct can have multiple `##` schema specifications — `##` describes properties, `#` is the concrete type. `{#}` can define any tier of the `#`/`##`/`###` prefix system.

```polyglot
{M} #String.Subtype
   (#) <Name#RawString
   (#) <Alias#RawString
   (#) <Regex#RawString

   {#} ##{$Name}
      (#) <~ #String
      [#] %##Alias
         [:] << $Alias
         [ ] Auto-alias with definition name if no conflict
         [:] <~ {%This.%name.Last}
      [#] ##Scalar
      [.] .re << $Regex
```

### Stdlib Scalar Invocations

Scalars are invoked via `[M]` inside `{#}` definitions. Arguments use `(#) <Param` matching the macro's parameters:

```polyglot
{#} ##Int
   [M] #String.Subtype
      (#) <Name << "Int"
      (#) <Alias << "int"
         [<] !Alias.Clash << "integer"
         [<] !Alias.Clash << "Integer"
      (#) <Regex << "^-?[0-9]+$"

{#} ##UnsignedInt
   [M] #String.Subtype
      (#) <Name << "UnsignedInt"
      (#) <Alias << "uint"
      (#) <Regex << "^[0-9]+$"

{#} ##Float
   [M] #String.Subtype
      (#) <Name << "Float"
      (#) <Alias << "float"
      (#) <Regex << "^-?[0-9]+\.[0-9]+$"

{#} ##Sci
   [M] #String.Subtype
      (#) <Name << "Sci"
      (#) <Alias << "sci"
      (#) <Regex << "^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$"

{#} ##Eng
   [M] #String.Subtype
      (#) <Name << "Eng"
      (#) <Alias << "eng"
      (#) <Regex << "^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$"

{#} ##Dimension
   [M] #String.Subtype
      (#) <Name << "Dimension"
      (#) <Alias << "dim"
      (#) <Regex << "^[1-9][0-9]*D$"

{#} ##KeyString
   [M] #String.Subtype
      (#) <Name << "KeyString"
      (#) <Alias << "key"
      (#) <Regex << "^[a-zA-Z_][a-zA-Z0-9_]*$"

{#} ##NestedKeyString
   [M] #String.Subtype
      (#) <Name << "NestedKeyString"
      (#) <Alias << "nestedkey"
      (#) <Regex << "^[a-zA-Z_][a-zA-Z0-9_.]*$"

{#} ##CommaSeparatedList
   [M] #String.Subtype
      (#) <Name << "CommaSeparatedList"
      (#) <Alias << "csvlist"
      (#) <Regex << "^[a-zA-Z_][a-zA-Z0-9_]*(,[a-zA-Z_][a-zA-Z0-9_]*)*$"

{#} ##DataTypeString
   [M] #String.Subtype
      (#) <Name << "DataTypeString"
      (#) <Alias << "dtstring"
      (#) <Regex << "^[A-Z][a-zA-Z0-9]*(:[A-Z][a-zA-Z0-9]*)*$"
```

### Usage (unchanged for users)

```polyglot
[r] $scores#array:int <~ {...}
[r] $lookup#map:string:int <~ {...}
[r] $matrix#array:float:2D <~ {...}
```

The `:` separator still binds positionally to macro inputs. Users don't see the macro — they just use `#array:int`.

### Key Changes

| What | Before | After |
|------|--------|-------|
| Generic syntax | `{#} #Array<ValueType<Dim` | `{M} #Array` with `(#) <Param` inputs |
| `(#) <param` in `{#}` | Type param declaration | Removed from `{#}` — now in `{M}` only |
| Macro definition | `{M}` (both macros + wrappers) | `{M}` = type macros only; `{W}` = wrappers |
| Macro invocation | `(#) <~ #MacroName` (ambiguous with inheritance) | `[M] #MacroName` (explicit, unambiguous) |
| Wrapper invocation | `[W]` (called `{M}`) | `[W]` (now calls `{W}` — wrappers are own entity) |
| `<~` meaning | Inheritance OR macro invocation | **Only** inheritance/default — never macro invocation |
| `[<]` constraints | Nested under `(#) <param` | Nested under `(#) <Param` in `{M}`; also `[<] !Error` fallback in `[M]` |
| `[.] .*Param` expansion | References `<param` in `{#}` | References `$Param` from `(#) <Param` in `{M}` |
| Scalar subtypes | `{#} #Int` (concrete `#` types) | `{#} ##Int` via `[M] #String.Subtype` (`##` schemas) |
| `<` in `{#}` header | Type parameter | **Removed** from `{#}` headers |
| Macro dispatch | N/A | By signature: parameter count + kind (`<#` type vs `<` value) |
| Dataframe structure | Column-oriented (columns are arrays) | Row-oriented: array of maps (each row is a map); no `<~` inheritance — uses `##` composition |
| Dataframe access | `$df.column<row` | `$df<row<column`; column extraction via `=#.Column` pipeline |
| Map keys | `#KeyString` (any string) | `##EnumLeafs` (predefined enum set); arbitrary keys → use `#Serial` |
| Alias errors | N/A | `[<] !Alias.Clash << "fallback"` — error-driven alias fallback chain |

### New Concepts Introduced

| Concept | Description |
|---------|-------------|
| `{W}` definition block | Wrappers as own entity — separated from `{M}` type macros |
| `[M]` block element | Macro invocation — expand `{M}` macro inside `{#}` definition |
| `(#) <#Param` (type macro input) | Types as inputs — datatypes are data trees |
| `(#) <Param##Type` (value macro input) | Typed values as macro inputs |
| `(#) <Param << "value"` (macro argument) | Pass arguments in `[M]` invocation — mirrors macro parameter syntax |
| `[<] !Alias.Clash` | Error-driven alias fallback chain under `(#) <Alias` |
| `!Alias.Clash` error | Compile error when alias collides with existing name in target namespace |
| PGW-9XX namespace alias | Warning when alias creates namespace placement — compiler verifies no clash |
| Dynamic type naming | `{#} #{$ArrayName}` — computed names via `{$var}` interpolation |
| Multiple aliases | `%##Alias` with `[:]` children — one type, many aliases |
| `%name` metadata accessor | Returns definition name from `{x}` header: `{#} #Foo` → `%name` = `"Foo"` |
| `%name.Last` | Splits `%name` by `.` and returns last segment |
| `%This` self-reference | Innermost `{x}` block — like Python `self` / C++ `this` |
| `%Parent` | One level up from `%This` — access enclosing `{x}` from nested blocks |
| Macro overloading | Multiple `{M}` with same name dispatch by signature |
| Scalars are `##` schemas | `{#} ##Int` — regex constraints on `#String`, not structs |
| `##` = property substitution | `[#] ##Schema` sets `%` properties — NOT inheritance. Two schemas agree or PGE11001 |
| `{#}` defines all tiers | `{#}` can define `#`, `##`, or `###` prefixed names |
| Bootstrap layers | Layer 0 (hardcoded), Layer 1 (no pipelines), Layer 2 (full pipelines) |
| Compile-time `=` pipelines | Layer 2 macros can invoke `=` pipelines during compilation |
| `##DataTypeString` | New stdlib schema: regex for valid `{x}` definition names |
| `##CommaSeparatedList` | New stdlib schema: breaks Layer 2 circular dependency |
| `=UID` | New stdlib pipeline: generates unique identifiers |
| `=String.Lower` | New stdlib pipeline: lowercase conversion |
| Dataframe = Array of Map | Row-oriented: `$df<row<column` access pattern |

## Files to Modify

### Primary (spec changes)

| File | Action | What |
|------|--------|------|
| types.md | **Edit** | Add ground truths preamble; replace "Generic Type Parameters" with "Macro-Generated Types"; update `[#]` roles table; document `{#}` defining all tiers; update Dataframe to row-oriented |
| stdlib/types/collections.md | **Edit** | Rewrite #Map, #Array, #Dataframe as macro-generated; Dataframe = Array of Map; keep #Serial as plain {#} |
| blocks.md | **Edit** | Add `{W}` definition block; add `[M]` block element; update `{M}` to type-macros-only; document `%This`/`%Parent` scoping; document `%name`/`%name.Last` |
| concepts/pipelines.md | **Edit** | Update wrapper references from `{M}` to `{W}`; note `[W]` invokes `{W}` |

### Secondary (references to update)

| File | Action | What |
|------|--------|------|
| concepts/collections.md | **Edit** | Update #Map and #Array definitions; document Map key semantics change; Dataframe row-oriented |
| EBNF.md | **Edit** | Remove `type_param` production; add `{M}`/`{W}` split; add `[M]` production; add dispatch grammar |
| EDGE-CASES.md | **Edit** | Update EC-24.12 and other generic-dependent cases; update Dataframe access pattern |
| schema-properties.md | **Edit** | Update decision record to reflect macro-based generics; document `##` as property substitution |
| stdlib/types/scalars.md | **Edit** | Show scalar subtypes as `##` schema macro-generated via `[M]`; add ##DataTypeString, ##CommaSeparatedList |
| stdlib/pipelines/ | **Create** | Add `=UID`, `=String.Lower` stdlib pipeline specs |
| stdlib/errors/ | **Edit** | Add `!Alias.Clash` error type |

### Compile Rules Impact

| Rule | Impact |
|------|--------|
| PGE11001 | Clarify: applies to `##` schema composition conflicts (two schemas setting same `%` property to different values) |
| PGE04022 | Field expansion constraint — moves from `[<]` under `[#]` to `[<]` under `(#) <Param` in `{M}` |
| PGE01004 | Macro structural constraints — update for `{M}`/`{W}` split |
| New PGW-9XX | Namespace alias warning — alias creates placement in different namespace branch; compiler checks for clashes |
| New rule | `{M} #Name` must contain `{#}` and must NOT contain `[\]`/`[/]` |
| New rule | `{W} =W.Name` must contain `[\]`/`[/]` and must NOT contain `{#}` |
| New rule | `[M]` can only appear inside `{#}` blocks; `[W]` can only appear inside `{=}` blocks |
| New rule | `[M]` arguments must use `(#) <Param` syntax matching macro parameter names |
| New rule | `!Alias.Clash` — alias collides with existing name in target namespace |
| New rule | Layer 1 type macros must NOT contain `[r] =Pipeline` calls |
| New rule | `%This` outside any `{x}` block — compile error |
| New rule | `%name` outside any `{x}` block — compile error |
| New rule | Macro dispatch ambiguity (two overloads with same signature) — compile error |

## Verification

1. **Consistency:** All `{#} #Type<Param` patterns replaced with `{M}` equivalents across all docs
2. **No orphans:** Grep for `<param` in `{#}` headers — should find zero matches
3. **Ground truths:** 9 axioms documented in types.md preamble
4. **Usage unchanged:** `#array:int`, `#map:string:int`, `#dataframe:SalesColumns:string` syntax still works
5. **{M}/{W} split:** `{M}` only contains type macros; `{W}` only contains wrappers; no cross-contamination
6. **[M] invocation:** All macro invocations use `[M]` block element, not `(#) <~`
7. **`<~` clean:** `<~` only means inheritance/default in all docs — never macro invocation
8. **Macro examples:** At least 3 complete macro-generated type examples (collection + scalar + user-defined)
9. **Bootstrap:** Layer 0 → 1 → 2 sequence documented; Layer 1 macros verified pipeline-free; ##CommaSeparatedList breaks circular dependency
10. **Dataframe:** Row-oriented (Array of Map); both compile-time and runtime overloads; access via `$df<row<column`
11. **`%This`/`%Parent`:** Scoping rules documented with examples
12. **`%name`/`%name.Last`:** Formally specified with examples per `{x}` block type
13. **Map semantics:** Key change (enum keys) documented; #Serial as alternative for arbitrary keys
14. **#Boolean values:** All schema properties use `#True`/`#False`, not `true`/`false`
15. **Flexible children:** All collection `{#}` include `[:] :*#Type` declarations
16. **`##` composition:** Documented as property substitution (not inheritance); PGE11001 for conflicts
17. **Alias system:** `!Alias.Clash` error + `[<]` fallback chain + PGW-9XX namespace warning
18. **No contradictions:** Cross-check with EDGE-CASES.md, COMPILE-RULES.md

---

# Plan 94-02/04: Serial File Loading + Schema Validation Pipelines

## Context

Issue #94 (`=#.*` validation) and #79 (serial file loading). Builds on Plan 94-01 above.

**Assumption:** `=#.JSON.Parse`, `=#.YAML.Parse`, `=#.TOML.Parse` exist as compiler-internal implementations that take `#RawString` → `#serial`. We build higher-level pipelines on top of these.

## 1. `=File.Serial.*` — File Loading Pipelines

Built on `=#.JSON/YAML/TOML` base parsers. Auto-detect format by extension.

### `=File.Serial.Read`

Reads a file, detects format from extension (.json/.yaml/.toml), delegates to the appropriate `=#.*.Parse` base parser, returns parsed data as `#serial` tree.

| IO | Type | Description |
|----|------|-------------|
| `<path` | `#path` | File path to read |
| `>data` | `#serial` | Parsed data tree |

| Error | When |
|-------|------|
| `!File.NotFound` | File doesn't exist at path |
| `!File.ReadError` | File exists but can't be read (permissions, locked) |
| `!File.ParseError` | File content isn't valid JSON/YAML/TOML |

Permission: `_File.read`

### `=File.Serial.Write`

Serializes `#serial` data to file. Detects target format from extension, delegates to `=#.*.Serialize`.

| IO | Type | Description |
|----|------|-------------|
| `<path` | `#path` | File path to write |
| `<data` | `#serial` | Data tree to serialize |

| Error | When |
|-------|------|
| `!File.NotFound` | Parent directory doesn't exist |
| `!File.WriteError` | Can't write to path (permissions, disk full) |

Permission: `_File.write`

### `=File.Serial.Read.Field`

One-step field extraction: reads file, parses, and extracts a single field by tree path. Combines `=File.Serial.Read` + `=#.Field`.

| IO | Type | Description |
|----|------|-------------|
| `<path` | `#path` | File path to read |
| `<field` | `#RawString` | Tree path using `<` separator (e.g. `"database<host"`) |
| `>value` | `#serial` | Extracted field value |

| Error | When |
|-------|------|
| `!File.NotFound` | File doesn't exist |
| `!File.ReadError` | Can't read file |
| `!File.ParseError` | Invalid format |
| `!Field.NotFound` | Field path doesn't exist in parsed data |

Permission: `_File.read`

---

## 2. `=#.*` — Schema Validation Pipelines

Types are data trees (GT-1). `<#type` passes the type's `%` metadata tree as pipeline input — same `<#` syntax as `{M}` macro type inputs.

**New concept: `<#type` in pipeline IO** — extends `<#` from `{M}` to `{=}`. The pipeline receives the type definition's data tree (schema properties, field definitions, constraints). Works with any tier: `#Config`, `##Scalar`, `###Enum`.

### `=#.Match`

Boolean schema check — does this data match this type? Returns `#True`/`#False`, no side effects.

| IO | Type | Description |
|----|------|-------------|
| `<data` | `#serial` | Data to check |
| `<#type` | type tree | Type definition as data tree input |
| `>match` | `#bool` | Whether data matches the type's schema |

No errors — always succeeds.

### `=#.Validate`

Detailed validation — checks data against type and reports all mismatches as a list of human-readable error strings.

| IO | Type | Description |
|----|------|-------------|
| `<data` | `#serial` | Data to validate |
| `<#type` | type tree | Type definition as data tree input |
| `>valid` | `#bool` | Overall pass/fail |
| `>errors` | `#array:string` | List of validation error descriptions |

No errors — always succeeds (validation failures reported via `>errors`, not `[!]`).

### `=#.Describe`

Schema introspection — returns a type's full schema (properties, fields, constraints) as a `#serial` data tree. Useful for runtime reflection.

| IO | Type | Description |
|----|------|-------------|
| `<#type` | type tree | Type definition to describe |
| `>schema` | `#serial` | Full schema as data tree |

No errors — always succeeds.

### `=#.Coerce`

Best-effort type conversion — takes data and a target type, keeps fields that match, reports fields that don't. Always succeeds (never throws).

| IO | Type | Description |
|----|------|-------------|
| `<data` | `#serial` | Data to coerce |
| `<#type` | type tree | Target type |
| `>result` | `#serial` | Coerced data (matching fields only) |
| `>dropped` | `#array:string` | List of field paths that couldn't be coerced |

No errors — always succeeds.

---

## 3. `=#.Field` — Safe Field Extraction

Extracts a single field from a `#serial` data tree by path. Like tree access (`$data<key<subkey`) but with error handling for missing paths.

| IO | Type | Description |
|----|------|-------------|
| `<data` | `#serial` | Data tree to extract from |
| `<path` | `#RawString` | Tree path using `<` separator (e.g. `"database<host"`) |
| `>value` | `#serial` | Extracted value |

| Error | When |
|-------|------|
| `!Field.NotFound` | Path doesn't exist in data tree |
| `!Field.PathError` | Invalid path syntax |

**Three approaches to field access — user picks based on needs:**

| Approach | When to use |
|----------|-------------|
| `$data<database<host` | Direct tree access — fast, no error handling on missing field |
| `=#.Field` | Safe extraction — error handling with `[!]` fallback chains |
| `=File.Serial.Read.Field` | Single step from file to field — combines read + parse + extract |

---

## 4. `=#.Column` — Dataframe Column Extraction

Extracts all values for a given column across all rows of a row-oriented Dataframe, returning them as an array. Solves the column access pattern that row-oriented storage doesn't natively provide.

| IO | Type | Description |
|----|------|-------------|
| `<data` | `#dataframe` | Source dataframe |
| `<column` | `#enum` | Column identifier from the dataframe's column enum |
| `>values` | `#array` | All values in that column, one per row |

| Error | When |
|-------|------|
| `!Field.NotFound` | Column doesn't exist in dataframe's column enum |

## Error Types (94-02/04)

```polyglot
{ } New errors to add
{!} !Validation
   [.] .Schema              [ ] Data doesn't match type's ## schema properties
   [.] .Type                [ ] Data doesn't match type's # structure
   [.] .Regex               [ ] Leaf value doesn't match ## scalar's .re pattern

{!} !Field
   [.] .NotFound            [ ] Requested field path doesn't exist
   [.] .PathError           [ ] Invalid field path syntax

{ } Add to existing !File
{!} !File
   [.] .ParseError          [ ] File content isn't valid serial format
```

## Pipeline Index (94-02/04)

| Pipeline | IO | Purpose |
|----------|-----|---------|
| **Base (exist)** | | |
| `=#.JSON.Parse` | `<#RawString >data#serial` | JSON → #serial |
| `=#.YAML.Parse` | `<#RawString >data#serial` | YAML → #serial |
| `=#.TOML.Parse` | `<#RawString >data#serial` | TOML → #serial |
| **File IO** | | |
| `=File.Serial.Read` | `<path >data#serial` | Load file → #serial (auto-detect) |
| `=File.Serial.Write` | `<path <data#serial` | #serial → write file |
| `=File.Serial.Read.Field` | `<path <field >value#serial` | Load one field from file |
| **Validation** | | |
| `=#.Match` | `<data#serial <#type >match#bool` | Boolean schema check |
| `=#.Validate` | `<data#serial <#type >valid#bool >errors` | Detailed validation |
| `=#.Describe` | `<#type >schema#serial` | Schema introspection |
| `=#.Coerce` | `<data#serial <#type >result >dropped` | Best-effort conversion |
| **Field / Column** | | |
| `=#.Field` | `<data#serial <path#RawString >value#serial` | Safe field extraction |
| `=#.Column` | `<data#dataframe <column#enum >values#array` | Dataframe column extraction |

## Files to Modify (94-02/04)

| File | Action | What |
|------|--------|------|
| `docs/user/stdlib/pipelines/File.md` | **Edit** | Add `=File.Serial.Read/Write/Read.Field` |
| `docs/user/stdlib/pipelines/` | **Create** | `=#.md` — format parsers, validation, field extraction |
| `docs/user/stdlib/errors/errors.md` | **Edit** | Add `!Validation.*`, `!Field.*`, `!File.ParseError` |
| `docs/user/stdlib/INDEX.md` | **Edit** | Add `=#.*` and `=File.Serial.*` entries |
| `docs/user/syntax/types.md` | **Edit** | Document `<#type` as pipeline IO pattern |
| `docs/user/concepts/pipelines.md` | **Edit** | Note `<#type` extends `<#` from macros to pipelines |
| `docs/technical/EBNF.md` | **Edit** | Add `type_input` production for `[=] <#type` |

## Verification (94-02/04)

1. All pipelines follow stdlib pattern: `[t] [Q] [W] [=] [!]`
2. `<#type` works with `#Config`, `##Scalar`, `###Enum` — any tier
3. File pipelines require `[_]` permissions
4. `=File.Serial.Read` delegates to `=#.JSON/YAML/TOML.Parse` internally
5. `=#.Match` returns `#bool` — no conversion, no side effects
6. `=#.Coerce` always succeeds — reports dropped fields, never throws
7. `=#.Field` uses `<` path syntax consistent with tree access
8. Error fallback chains use `[<] !Error << fallback` pattern
