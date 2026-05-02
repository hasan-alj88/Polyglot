---
audience: automation-builder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->
<!-- @u:syntax/types/generic-types -->
<!-- @u:collections/expand#=ForEach.Dataframe -->

## ##Dataframe -- Two-Level Matrix Schema

`##Dataframe` is a strictly 2D tabular schema. Unlike dynamic collections (`##Array`, `##Map`) that use flexible `<key` paths, `##Dataframe` enforces **Flat Enum** keys for both rows and columns. 

Because both dimensions are structural Enums (`.`), the Dataframe guarantees a fixed, rectangular shape at compile time.

```aljam3
[ ] Using the Tabular Syntax Sugar
[-] $salesData##Dataframe <<
   ($) .product#string | .price#float | .quantity#int
   ($) "Laptop"        | 1200.0       | 5
   ($) "Mouse"         | 45.0         | 12
```

This tabular layout is syntactic sugar that the compiler unrolls into an explicit matrix of flat row enums (`.0`, `.1`) and flat column enums (`.product`, `.price`):

```aljam3
[ ] The unrolled explicit matrix
[-] $salesData##Dataframe <<
   ($) .0
      ($) .product#string << "Laptop"
      ($) .price#float << 1200.0
      ($) .quantity#int << 5
   ($) .1
      ($) .product#string << "Mouse"
      ($) .price#float << 45.0
      ($) .quantity#int << 12
```

You can also explicitly name the row enums in the tabular layout by providing them in the first column:

```aljam3
[ ] Tabular Syntax with Named Row Enums
[-] $users##Dataframe <<
   ($)        | .name#string | .age#int | .active#bool
   ($) .user1 | "Hasan"      | 35       | #True
   ($) .user2 | "Paul"       | 28       | #False
```

Alternatively, the auto-incrementing push `($) <<` can be used to construct rows without explicitly naming the numeric enum:

```aljam3
[ ] Auto-incrementing builder
[-] $salesData##Dataframe <<
   ($) <<
      ($) .product#string << "Laptop"
      ($) .price#float << 1200.0
      ($) .quantity#int << 5
```

```aljam3
{#} #SalesColumns
   [#] ##Enum
   [#] ##Scalar
   [.] .product
   [.] .price
   [.] .quantity

[-] $sales#dataframe:SalesColumns:string <~ {}
```

### Access

Because both rows and columns are Enums, access uses the `.` prefix for both coordinates:

```aljam3
[ ] Row 0, column "product"
[-] $name#string << $sales.0.product

[ ] Row 1, column "price"
[-] $price#float << $sales.1.price

[ ] Entire row as a Record
[-] $row << $sales.0
```

To extract an entire column, use a `*Collect` pipeline.

## Nested Collection Safety

When a collection type is used as another collection's value type (e.g., an array of arrays), the compiler requires explicit depth bounds.

- **PGE11002** -- A collection used as a value type without explicit `%##Depth.Max` is a compile error. Unbounded nesting must be declared intentionally.
- **PGW11003** -- Setting `%##Depth.Max << -1` (unlimited) on a user-defined type raises a warning. Only `#Serial` should use unlimited depth.

```aljam3
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

- [[concepts/collections/map|##Record]] -- enum-keyed collection
- [[concepts/collections/array|##Array]] -- generic indexed structure
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Depth.Max` and nested collection safety

