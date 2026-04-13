---
audience: automation-builder
type: specification
updated: 2026-04-09
---

# Arrays

<!-- @syntax/types/INDEX -->

## Alias

`#array` is a lowercase alias for `#Array`, following the same convention as `#int`/`#Int` and `#string`/`#String`. In type annotations, use lowercase `#array`; in prose or definition references, use PascalCase `#Array`. The alias is registered via `%##Alias` in the `#Array` generic type definition (see [[pglib/types/collections|Collection Types]]).

## Element-Typed Arrays

Arrays specify their element type using `:` (flexible field) notation:

```polyglot
[-] $files#array:path <~ {}
[-] $names#array:string <~ {}
[-] $scores#array:int <~ {}
```

This constrains the array to hold only elements of the specified type.

## Multidimensional Arrays

Arrays support a dimension specifier using an `<N>D` suffix. Omitting the dimension defaults to 1D:

```polyglot
(-) <items#array:string              [ ] 1D array (default)
(-) <matrix#array:float:2D           [ ] 2D matrix of floats
(-) <cube#array:int:3D               [ ] 3D cube of ints
(-) <hyper#array:float:4D            [ ] 4D hypercube of floats
```

Element access uses `<` (the tree child accessor) with integer indices. The number of indices must match the declared dimension count:

```polyglot
[-] $val << $items<0                 [ ] 1 index for 1D
[-] $val << $matrix<0<1              [ ] 2 indices for :2D
[-] $val << $cube<2<3<0              [ ] 3 indices for :3D
```

`:ND` in the type annotation is a declaration-time dimension specifier (using `:` as a flexible schema field). `<` is the runtime tree child accessor used for element access. Declaration and access use different separators because `:` marks flexible schema fields while `<` navigates tree children — this follows the same pattern as all Polyglot data access (see [[concepts/data-is-trees]]).

A `0D` array is a scalar container — it holds exactly one element with no indexing. Access is direct (no `<N` index):

```polyglot
[-] $scalar#array:int:0D <~ {42}
[-] $val#int << $scalar              [ ] direct access — no index
[-] $bad << $scalar<0                [ ] ✗ PGE04017 — no indices on 0D
```

The compiler enforces access depth — too many or too few indices triggers PGE04017. Nested array types (`#array:#array:X`) remain banned (PGE04013) — use `:ND` instead.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths and type annotation rules
- [[concepts/collections/array|#Array Collection]] -- generic type, cartesian keys, and collection access
- [[syntax/types/schema-properties|Schema Properties]] — `%##Depth.Max` and dimensional constraints
