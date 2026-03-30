---
audience: user
type: specification
updated: 2026-03-30
---

# Arrays

<!-- @syntax/types/INDEX -->

## Element-Typed Arrays

Arrays specify their element type using `:` (flexible field) notation:

```polyglot
[r] $files#array:path <~ {}
[r] $names#array:string <~ {}
[r] $scores#array:int <~ {}
```

This constrains the array to hold only elements of the specified type.

## Multidimensional Arrays

Arrays support a dimension specifier using an `<N>D` suffix. Omitting the dimension defaults to 1D:

```polyglot
[=] <items#array:string              [ ] 1D array (default)
[=] <matrix#array:float:2D           [ ] 2D matrix of floats
[=] <cube#array:int:3D               [ ] 3D cube of ints
[=] <hyper#array:float:4D            [ ] 4D hypercube of floats
```

Element access uses colon-separated integer indices. The number of indices must match the declared dimension count:

```polyglot
[r] $val << $items:0                 [ ] 1 index for 1D
[r] $val << $matrix:0:1              [ ] 2 indices for :2D
[r] $val << $cube:2:3:0              [ ] 3 indices for :3D
```

A `0D` array is a scalar container — it holds exactly one element with no indexing. Access is direct (no `:N` index):

```polyglot
[r] $scalar#array:int:0D <~ {42}
[r] $val#int << $scalar              [ ] direct access — no index
[r] $bad << $scalar:0                [ ] ✗ PGE-417 — no indices on 0D
```

The compiler enforces access depth — too many or too few indices triggers PGE-417. Nested array types (`#array:#array:X`) remain banned (PGE-412) — use `:ND` instead.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths and type annotation rules
- [[concepts/collections/array|#Array Collection]] — macro generation, cartesian keys, and collection access
- [[syntax/types/schema-properties|Schema Properties]] — `%##Depth.Max` and dimensional constraints
