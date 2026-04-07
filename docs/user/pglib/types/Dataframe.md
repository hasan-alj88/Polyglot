---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:Dataframe"
metadata_instance: "%#:Dataframe:N"
---

# #Dataframe Collection

<!-- @types -->
<!-- @macros -->

Row-oriented table. Dataframe is an `#Array` of `#Map` — each row is a map, the array holds rows. Access: `$df<row<column`. `#Dataframe` has two `{M}` macro overloads dispatched by signature.

---

## Definition

### Compile-Time Safe Variant

Dispatched by signature `(<#, <#)` — column names and cell type known at compile time.

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

### Runtime Flexible Variant

Dispatched by signature `(<, <)` — column names provided as strings. Needs `=#.Validate` at runtime for schema enforcement.

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

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `##Contiguous` | `%##Children.Gap << #False` | No gaps in row indices |
| `##Rectangular` | `%##Children.Regular << #True` | Every row has the same columns |
| `##Ordered` | `%##Children.Ordered << #True` | Row order preserved |

---

## Dataframe Access Pattern

Access uses double `<` — first for row index, second for column name:

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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Dataframe` | Compile-time type template |
| Instance | `%#:Dataframe:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] — collection type overview
- [[Map]] — row type (each row is a #Map)
- [[Array]] — parent structure (#Dataframe is an #Array of #Map)
- [[scalars]] — scalar schema classifications
- [[syntax/types/INDEX|types]] — full type system specification
- [[macros]] — macro system
