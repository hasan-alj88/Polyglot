---
audience: automation-builder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->
<!-- @u:syntax/types/generic-types -->
<!-- @u:data-is-trees -->

## `##Array` -- Range-Indexed Collection Schema

The **only difference** between an `##Array` and a `##Map` is that `##Array` strictly restricts its indices to **incremental numeric keys** (e.g., `<0`, `<1`, `<2`), whereas a `##Map` can have indices of any type.

`##Array` is a schema parameterizing the `#Array` generic `{#}` type with `(#) <#ValueType` and `(#) <Dim##Dimension` parameters (default `"1D"`). It composes `##Map` but overrides the key generation, setting `%##Fields << #Range` (integer-indexed children), `%##Gap << #False` (no gaps), `%##Ordered << #True` (order preserved), and `%##Propagate << #True` (properties apply to all levels). Use `:` positional binding: `##Array:Float:2D`.

See [[jm3lib/types/schemas/Array|##Array]] for the full definition and [[syntax/types/generic-types|Generic Types]] for the `(#) <param` syntax.

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

```aljam3
[ ] 1D array access
[-] $first << $myArray<0

[ ] 2D matrix access -- branch 1, leaf 2
[-] $cell << $matrix<1<2

[ ] 3D cube access
[-] $voxel << $cube<2<3<0
```

## See Also

- [[syntax/types/arrays|Array Type Annotations]] -- element-typed and multidimensional array syntax
- [[concepts/collections/expand|Expand Operators]] -- `=ForEach` iteration
- [[concepts/collections/collect|Collect Operators]] -- `*Collect` collection
- [[jm3lib/types/schemas/Fields|%##Fields]] -- field descriptor property

