---
audience: automation-builder
type: specification
updated: 2026-05-04
status: draft
metadata_definition: "%definition.##:Uniform"
metadata_instance: "%##:Uniform:N"
---

# Uniform — Symmetric Data Tree Schema

<!-- @c:schemas -->

The `##Uniform` property is the foundational schema for **self-similar data trees** in Aljam3. It guarantees that every branch at a specific depth level has the exact same structural schema. 

## `##Uniform` vs `###Uniform`

While `##Uniform` enforces structural symmetry (e.g., every row in a table has the same columns), it does not restrict the data types of the leaves across the entire tree. For example, a `##Uniform` tree can have a `#String` in one column and an `#Int` in another.

To enforce strict, homogeneous data types across the entire tree, Aljam3 provides the `###Uniform` syntactic sugar. 

**`###Uniform`** is syntactic sugar for manually setting the `%##` schema metadata. It automatically applies `##Uniform` and further restricts the schema so that **every single leaf** in the branch must be of the exact same scalar Data Type (like a strict `#String` array or a float matrix).

## Structural Definition

Unlike `#Serial` trees which can have arbitrary nested paths, `##Uniform` trees define their structure at declaration time using the `#<` schema input operator.

```aljam3
[#] ##Uniform
```

### 1D Uniform (Records)
A `##Record` is simply a 1D `##Uniform` tree where the keys are strictly defined.
```aljam3
[$] $Person##Uniform
   ($) #<indices << 
      (#) #< "Name" | "Age" | "Active"
```

### 2D Uniform (DataFrames/Matrices)
A `##DataFrame` is a specialized 2D `##Uniform` tree.
```aljam3
[$] $Grid##Uniform
   ($) #<indices <<
      (#) << 0..2         [ ] L1 (Rows: int)
      (#) #< "X" | "Y"    [ ] L2 (Cols: string)
```

## Matrix Equivalency and Safety

The explicit symmetry of `##Uniform` unlocks specific compiler guarantees:
1. **Safe Transposition**: The `=*PermuteLevels` operator can safely swap Level 1 and Level 2 axes because every branch has identical depth and type definitions.
2. **Reconciliation**: When piping parallel operations into `*Collect`, the compiler can predict exactly how to merge the resulting branches into the final Uniform tree without path-matching errors.

## Interaction with Expanders

When parsing strictly tabular data from a file or database, the resulting Expanders inherently yield `##Uniform` structures:
* `=File.CSV.Rows` $\rightarrow$ Yields a 1D `##Record` per row.
* `=DB.Table.Rows` $\rightarrow$ Yields a 1D `##Record` per row.

## Related

- [[jm3lib/types/schemas/Record|##Record]]
- [[jm3lib/types/schemas/Dataframe|##Dataframe]]
- [[jm3lib/types/schemas/Serial|#Serial]]
