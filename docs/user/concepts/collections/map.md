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

