---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Dataframe"
metadata_instance: "%#:Dataframe:N"
---

# #Dataframe Collection

<!-- @c:types -->

Two-level tabular collection. Level 1 is range-indexed rows (like `##Array`). Level 2 is a `##Record` keyed by column enum. Access: `$df<row<column`.

---

## Definition

```polyglot
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

The `<#Columns` parameter must satisfy `##Enum` -- column names come from an enum type. The `<#CellType` parameter sets the cell value type (defaults to `#` -- any type). Level 1 uses `%##Fields << #Range` for integer row indices. Level 2 composes `##Record` with the column enum as fields.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `2` | Two levels: rows + columns |
| `%##Fields` | `#Range` | Integer row indices (L1) |
| `%##Ordered` | `#True` | Row order preserved |
| `%##Gap` | `#False` | No gaps in row indices |
| `%##Level.2` | `##Record` | Columns are enum-keyed record fields |

---

## Dataframe Access Pattern

Access uses double `<` -- first for row index, second for column name:

```polyglot
{#} #SalesColumns
   [#] ##Enum
   [#] ##Scalar
   [.] .product
   [.] .price
   [.] .quantity

[-] $sales#dataframe:SalesColumns:string <~ {}

[ ] Access: <row<column
[-] $name#string << $sales<0<product       [ ] row 0, column "product"
[-] $price#string << $sales<2<price        [ ] row 2, column "price"
[-] $row << $sales<0                       [ ] entire row as Record
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Dataframe` | Compile-time type template |
| Instance | `%#:Dataframe:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] -- collection type overview
- [[Map]] -- ##Record schema used for column structure
- [[Array]] -- range-indexed structure used for row indices
- [[schemas/Dataframe|##Dataframe]] -- parameterized schema
- [[schemas/Fields|%##Fields]] -- field descriptor property
- [[syntax/types/INDEX|types]] -- full type system specification

