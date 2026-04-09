---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Dataframe"
metadata_instance: "%#:Dataframe:N"
---

# #Dataframe Collection

<!-- @types -->

Row-oriented table. `#Dataframe` is a generic type -- an `#Array` of `#Map`. Each row is a map keyed by column names. Access: `$df<row<column`.

---

## Definition

```polyglot
{#} #Dataframe
   [#] <#Columns << ##Enum
   [#] <#CellType <~ #
   [#] << ##Dataframe
      [#] <#Columns << <#Columns
      [#] <#CellType << <#CellType
   [#] %##Alias << "dataframe"
```

The `<#Columns` parameter must satisfy `##Enum` -- column names come from an enum type. The `<#CellType` parameter sets the cell value type (defaults to `#` -- any type). The `##Dataframe` parameterized schema provides the structural constraints: contiguous rows, rectangular shape.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#False` (via ##Contiguous) | No gaps in row indices |
| `%##Ordered` | `#True` (via ##Contiguous) | Row order preserved |
| `%##Regular` | `#True` (via ##Rectangular) | Every row has same column count |
| `%##Key` | `#UnsignedInt` | Integer row indices |

---

## Dataframe Access Pattern

Access uses double `<` -- first for row index, second for column name:

```polyglot
{#} #SalesColumns
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [.] .product
   [.] .price
   [.] .quantity

[-] $sales#dataframe:SalesColumns:string <~ {}

[ ] Access: <row<column
[-] $name#string << $sales<0<product       [ ] row 0, column "product"
[-] $price#string << $sales<2<price        [ ] row 2, column "price"
[-] $row#map:SalesColumns:string << $sales<0   [ ] entire row as Map
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
- [[Map]] -- row type (each row is a #Map)
- [[Array]] -- parent structure (#Dataframe is an #Array of #Map)
- [[schemas/Dataframe|##Dataframe]] -- parameterized schema
- [[syntax/types/INDEX|types]] -- full type system specification
