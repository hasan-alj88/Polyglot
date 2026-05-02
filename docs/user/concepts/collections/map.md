---
audience: automation-builder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->
<!-- @u:syntax/types/generic-types -->
<!-- @u:data-is-trees -->

## `##Map:K:V` -- Generic Key-Value Collection

`##Map` is the universal base collection schema for all key-value mappings in Aljam3. It parameterizes a key type `K` and a value type `V`. 

Unlike `##Array`, which enforces strict, incremental numeric keys, the keys in a `##Map` can be **anything** (Strings, Enums, UUIDs, or even complex objects depending on the definition).

### Schema composition

A `##Map` structurally acts as a parent node where its children (the values `V`) are bound to arbitrary unique keys `K`.

- `%##KeyType << K`
- `%##ValueType << V`

### N-Dimensional Labeled Maps

Because `##Map` requires uniform schema branches, deeply nested trees (like 3D or 4D data) would normally require repeating keys over and over.

Instead, Aljam3 separates the **labels (Keys)** from the **raw data**, conceptually similar to Python's `xarray` or R's `dimnames`. You define the keys for each depth level using schema properties (`<#Keys.N`), and then provide the raw data arrays. 

```aljam3
[ ] 3D Map: Country -> City -> Metric
[-] $stats##Map <<
   ( ) 1. The Labels (coords/dimnames)
   (#) <#Keys.1 << [.London, .Manchester]
   (#) <#Keys.2 << [.Population, .GDP]
   
   ( ) 2. The Raw Data
   ($) .UK << 
      ($) << 9.0 | 3.0
      ($) << 2.5 | 0.5
   ($) .US << 
      ($) << 8.5 | 2.0
      ($) << 4.0 | 1.2
```

In this structure:
- `(#)` is used to shape the schema's axes.
- `($) .UK` establishes the root level 0 keys dynamically.
- `($) <<` leverages the auto-incrementing push constructor to drop in the anonymous row data.
- `|` (the Tabular Separator) acts as the column separator for the deepest level.

### Access

Use `<` followed by the key to access elements.

```aljam3
[ ] Initialize a Map string -> float
[-] $prices##Map:String:Float <<
   ($) <"Apple"  << 150.5
   ($) <"Banana" << 1.2

[ ] Dynamic key access
[-] $item << "Apple"
[-] $cost << $prices<$item
```

## See Also

- [[concepts/collections/array|##Array]] -- A specialized Map constrained to incremental numeric keys
- [[concepts/collections/set|##Set]] -- A collection where all leaves are uniquely constrained
- [[concepts/collections/expand|Expand Operators]] -- Iterating over maps
- [[concepts/collections/collect|Collect Operators]] -- Collecting into maps

