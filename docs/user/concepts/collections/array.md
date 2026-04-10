---
audience: pg-coder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->

## #Array -- Range-Indexed Collection

`#Array` is a generic `{#}` type with `(#) <#ValueType` and `(#) <Dim##Dimension` parameters (default `"1D"`). It composes `##Array`, which sets `%##Fields << #Range` (integer-indexed children), `%##Gap << #False` (no gaps), `%##Ordered << #True` (order preserved), and `%##Propagate << #True` (properties apply to all levels). Use `:` positional binding: `#array:float:2D`.

See [[pglib/types/Array|#Array]] for the full definition and [[syntax/types/generic-types|Generic Types]] for the `(#) <param` syntax.

### Cartesian product keys

For a 1D array of length 3, the keys are `<0`, `<1`, `<2`.

For a 3x4 2D array, the key tree is a Cartesian product:

```text
<0 --> <0, <1, <2, <3
<1 --> <0, <1, <2, <3
<2 --> <0, <1, <2, <3
```

`%##Propagate << #True` ensures all levels share the same properties (ordered, no gaps, range-indexed).

### Access

```polyglot
[ ] 1D array access
[-] $first << $myArray<0

[ ] 2D matrix access -- branch 1, leaf 2
[-] $cell << $matrix<1<2

[ ] 3D cube access
[-] $voxel << $cube<2<3<0
```

## See Also

- [[syntax/types/arrays|Array Type Annotations]] -- element-typed and multidimensional array syntax
- [[concepts/collections/expand|Expand Operators]] -- `=ForEach.Array` iteration
- [[concepts/collections/collect|Collect Operators]] -- `*Into.Array` collection
- [[pglib/types/schemas/Fields|%##Fields]] -- field descriptor property

