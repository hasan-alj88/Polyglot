---
audience: pg-coder
type: specification
updated: 2026-04-04
status: complete
---

# Collection Types

<!-- @types -->

Polyglot provides four collection types: `#Map`, `#Array`, `#Dataframe`, and `#Serial`. All are available in every `.pg` file without `[@]` import.

`#Map`, `#Array`, and `#Dataframe` are defined as `{M}` type macros that generate `{#}` definitions at compile time. `#Serial` is a plain `{#}` definition (no macro needed). See [[syntax/types/INDEX|types]] for the full type hierarchy and schema property definitions.

---

## #Map

<!-- @macros -->

Sparse key-value pairs. `#Map` has two `{M}` macro overloads dispatched by signature. Child access uses the `<` operator (`$myMap<key`).

### Homogeneous Variant — signature `(<#, <#)`

All values share the same type. Two type inputs: key enum and value type.

```polyglot
{ } Homogeneous variant — dispatched by signature (<#, <#)
{M} #Map
   [#] <#KeyType
      [<] << ##EnumLeafs
   [#] <#ValueType
      [<] << ##Scalar

   [r] $UniformMapName##DataTypeString << "Map:{$KeyType%name}:{$ValueType%name}"
   {#} #{$UniformMapName}
      [#] %##Alias
         [:] << "map:{$KeyType%name}:{$ValueType%name}"
         [:] << "Map:{$KeyType%name}:{$ValueType%name}"
      [#] %##Children.Type << $KeyType
      [#] << ##Flat
      [#] << ##Homogeneous
      [#] << ##Sparse
      [:] :*#$ValueType
```

### Heterogeneous Variant — signature `(<#)`

Mixed value types. One type input: key enum only.

```polyglot
{ } Heterogeneous variant — dispatched by signature (<#)
{M} #Map
   [#] <#KeyType
      [<] << ##EnumLeafs

   [r] $MapName##DataTypeString << "Map:{$KeyType%name}"
   {#} #{$MapName}
      [#] %##Alias
         [:] << "map:{$KeyType%name}"
         [:] << "Map:{$KeyType%name}"
      [#] %##Children.Type << $KeyType
      [#] << ##Flat
      [#] << ##Heterogeneous
      [#] << ##Sparse
      [:] :*#*
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Type` | `$KeyType` | Key type from enum parameter |
| `##Flat` | `%##Depth.Max << 1` | One level of flexible children |
| `##Homogeneous` | `%##Children.Uniform << #True` | All values same type (homogeneous variant) |
| `##Heterogeneous` | `%##Children.Uniform << #False` | Mixed value types (heterogeneous variant) |
| `##Sparse` | `%##Children.Gap << #True` | Gaps allowed in keys |

---

## #Array

<!-- @macros -->

Contiguous, rectangular collection with typed elements and N-dimensional support. `#Array` inherits from `#Map` with `#UnsignedInt` keys.

```polyglot
{M} #Array
   [ ] First input: a DataType — <# means type input (all definitions are data trees)
   [#] <#ValueType
      [<] << ##Scalar
   [ ] Optional second input: a Dimension (defaults to 1D)
   [#] <Dim##Dimension <~ "1D"
      [<] << ##Scalar

   [ ] Direct substitution via {} inside double quotes — implicit inline pipeline
   [ ] ##DataTypeString: new stdlib schema for valid {x} definition names
   [r] $ArrayName##DataTypeString << "Array{$Dim}:{$ValueType%name}"
   [r] $dim#RawString << =String.Lower"{$Dim}"

   {#} #{$ArrayName}
      [#] <~ #Map:#UnsignedInt:$ValueType
      [#] %##Alias
         [:] << "array:{$ValueType%name}:{$dim}"
         [:] << "array{$dim}:{$ValueType%name}"
         [:] << "Array{$Dim}:{$ValueType%name}"
      [#] %##Children.Type << #UnsignedInt
      [#] %##Children.Ordered << #True
      [#] %##Children.Uniform << #True
      [#] << ##Contiguous
      [#] << ##Rectangular
      [#] %##Depth.Max << $Dim
      [:] :*#$ValueType
```

### Inheritance from #Map

The generated `{#}` inherits from `#Map` via `[#] <~ #Map:#UnsignedInt:$ValueType`, overriding the key type to `#UnsignedInt` and adding contiguity and rectangularity constraints.

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Type` | `#UnsignedInt` | Integer indices `:0`, `:1`, `:2` ... |
| `##Contiguous` | `%##Children.Gap << #False`, `%##Children.Ordered << #True` | No gaps, insertion order preserved |
| `##Rectangular` | `%##Children.Regular << #True`, `%##Children.Uniform << #True` | Regular shape, uniform types |
| `%##Depth.Max` | `$Dim` | Equals dimension parameter value |

---

## #Dataframe

<!-- @macros -->

Row-oriented table. Dataframe is an `#Array` of `#Map` — each row is a map, the array holds rows. Access: `$df<row<column`. `#Dataframe` has two `{M}` macro overloads dispatched by signature.

### Compile-Time Safe Variant — signature `(<#, <#)`

Two type inputs: column enum and cell type. Full compile-time validation.

```polyglot
{ } Compile-time safe Dataframe — dispatched by signature (<#, <#)
{ } Dataframe = Array of Map — each row is a Map, array holds rows
{M} #Dataframe
   [#] <#ColumnEnum
      [<] << ##EnumLeafs
   [#] <#CellType
      [<] << ##Scalar

   [r] $DfName##DataTypeString << "Dataframe:{$ColumnEnum%name}:{$CellType%name}"
   {#} #{$DfName}
      [#] %##Alias
         [:] << "dataframe:{$ColumnEnum%name}:{$CellType%name}"
         [:] << "Dataframe:{$ColumnEnum%name}:{$CellType%name}"
      [#] << ##Contiguous
      [#] << ##Rectangular
      [#] << ##Ordered
      [:] :*#Map:$ColumnEnum:$CellType
```

### Runtime Flexible Variant — signature `(<, <)`

Two value inputs: column names and dataframe name. Schema enforcement via `=#.Validate` at runtime.

```polyglot
{ } Runtime flexible Dataframe — dispatched by signature (<, <)
{ } Needs =#.Validate at runtime for schema enforcement
{ } Dataframe = Array of Map — each row is a string-keyed map
{M} #Dataframe
   [#] <Columns##CommaSeparatedList
   [#] <DataFrameName##DataTypeString

   [ ] Dynamically generate an Enum from the column names
   [r] $uid##DataTypeString << =UID""
   {#} #DataFrameColumns{$uid}
      [ ] Generates an Enum from the column name list
      [#] << =#list.into.Enum"{$Columns}"

   {#} #{$DataFrameName}
      [#] %##Alias
         [:] << "dataframe:{$Columns%name}"
         [:] << "DataFrame:{$Columns%name}"
      [#] << ##Contiguous
      [#] << ##Rectangular
      [#] << ##Ordered
      [:] :*#Map:#String:#String
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `##Contiguous` | `%##Children.Gap << #False`, `%##Children.Ordered << #True` | No gaps, rows in insertion order |
| `##Rectangular` | `%##Children.Regular << #True`, `%##Children.Uniform << #True` | All rows same shape |
| `##Ordered` | `%##Children.Ordered << #True` | Row order preserved |

### Dataframe Access Pattern

Dataframe rows are accessed by index, columns by key: `$df<row<column`.

```polyglot
{#} #SalesColumns
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [.] .product
   [.] .price
   [.] .quantity

[r] $sales#dataframe:SalesColumns:string <~ {}

[ ] Access: <row<column
[r] $name#string << $sales<0<product       [ ] row 0, column "product"
[r] $price#string << $sales<2<price        [ ] row 2, column "price"
[r] $row#map:SalesColumns:string << $sales<0   [ ] entire row as Map
```

---

## #Serial

Schema-free collection with unlimited depth. Any keys, any types, any nesting. No compile-time validation of shape. Child access uses the `<` operator (`$data<key`). `#Serial` is a plain `{#}` definition — no macro needed.

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
   [#] << ##Deep
   [#] << ##Sparse
   [#] << ##Heterogeneous
   [:] :*#*
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Alias` | `"serial"` | Shorthand `#serial` |
| `##Deep` | `%##Depth.Max << -1` | Unlimited flexible nesting |
| `##Sparse` | `%##Children.Gap << #True` | Gaps allowed |
| `##Heterogeneous` | `%##Children.Uniform << #False` | Mixed value types |

---

## Usage

The `:` separator binds positionally to macro inputs. Users use the generated type names directly:

```polyglot
[r] $scores#array:int <~ {...}
[r] $lookup#map:string:int <~ {...}
[r] $matrix#array:float:2D <~ {...}
[r] $sales#dataframe:SalesColumns:string <~ {}
```

## Related

- [[string]] -- #String foundation type
- [[scalars]] -- scalar subtypes (## schemas)
- [[structs]] -- #path and #Queue struct types
- [[enums]] -- #FieldKind enum (used by `%##Leafs.Kind`)
- [[syntax/types/INDEX|types]] -- full type system specification
- [[macros]] -- {M} type macro definitions
