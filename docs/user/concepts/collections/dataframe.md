---
audience: automation-builder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->

## #Dataframe -- Two-Level Table

`#Dataframe` is a two-level tabular collection -- a generic `{#}` type with `(#) <#Columns` and `(#) <#CellType` parameters. Level 1 uses `%##Fields << #Range` for integer row indices. Level 2 composes `##Record` with the column enum as fields. Composes `##Dataframe`. Use `:` positional binding: `#dataframe:SalesColumns:string`. See [[pglib/types/Dataframe|#Dataframe]] for the full definition.

```polyglot
{#} #SalesColumns
   [#] ##Enum
   [#] ##Scalar
   [.] .product
   [.] .price
   [.] .quantity

[-] $sales#dataframe:SalesColumns:string <~ {}
```

### Access

Row access uses `<` (range index), column access uses a second `<` (enum variant from `##Record`):

```polyglot
[ ] Row 0, column "product"
[-] $name#string << $sales<0<product

[ ] Row 2, column "price"
[-] $price#string << $sales<2<price

[ ] Entire row as Record
[-] $row << $sales<0
```

For entire-column extraction, use `=#.Column` pipeline (see [[pglib/pipelines/Schema/INDEX]]).

Build dataframes using `*Into.Dataframe` collectors, not incremental assignment.

## Nested Collection Safety

When a collection type is used as another collection's value type (e.g., an array of arrays), the compiler requires explicit depth bounds.

- **PGE11002** -- A collection used as a value type without explicit `%##Depth.Max` is a compile error. Unbounded nesting must be declared intentionally.
- **PGW11003** -- Setting `%##Depth.Max << -1` (unlimited) on a user-defined type raises a warning. Only `#Serial` should use unlimited depth.

```polyglot
[ ] PGE11002: compile error -- no depth bound on nested array
{#} #BadGrid
   (#) <~ #array:array:int

[ ] Correct: explicit depth bound
{#} #Grid
   (#) <~ #array:array:int
   [#] %##Depth.Max << 2
```

This prevents accidentally creating unbounded recursive structures while still allowing intentional nesting with explicit bounds.

## See Also

- [[concepts/collections/map|##Record]] -- enum-keyed collection used for column structure
- [[concepts/collections/array|#Array]] -- range-indexed structure used for row indices
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Depth.Max` and nested collection safety

