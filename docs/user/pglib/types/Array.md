---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Array"
metadata_instance: "%#:Array:N"
---

# #Array Collection

<!-- @c:types -->

Ordered, range-indexed collection with typed elements and N-dimensional support. `#Array` is a generic type with `(#) <param` inputs.

---

## Definition

```polyglot
{#} ##Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] %##Depth.Max << <Dim
   [#] %##Fields << #Range
   [#] %##Ordered << #True
   [#] %##Gap << #False
   [#] %##Propagate << #True
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value
```

The `<#ValueType` parameter sets the element type. The `<Dim` parameter sets the dimension (defaults to `"1D"`). `%##Fields << #Range` provides integer-indexed children. Properties propagate to all levels via `%##Propagate << #True`.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `<Dim` | Dimension count from parameter |
| `%##Fields` | `#Range` | Integer-indexed children |
| `%##Ordered` | `#True` | Insertion order preserved |
| `%##Gap` | `#False` | No gaps in indices |
| `%##Propagate` | `#True` | Properties apply to all levels |
| `%###Type` | `<#ValueType` | Element type constraint |
| `%###Kind` | `#FieldKind.Value` | All children are value fields |

---

## Usage

The `:` separator binds positionally to `(#) <param` declarations:

```polyglot
[ ] #array:int -- ValueType=Int, Dim=1D (default)
[-] $scores#array:int <~ {}

[ ] #array:float:2D -- ValueType=Float, Dim=2D
[-] $matrix#array:float:2D <~ {}
```

---

## Access

```polyglot
[ ] 1D array access
[-] $first << $myArray<0

[ ] 2D matrix access -- branch 1, leaf 2
[-] $cell << $matrix<1<2

[ ] 3D cube access
[-] $voxel << $cube<2<3<0
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Array` | Compile-time type template |
| Instance | `%#:Array:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] -- collection type overview
- [[Map]] -- enum-keyed collection (composes ##Record)
- [[schemas/Array|##Array]] -- parameterized schema
- [[schemas/Fields|%##Fields]] -- field descriptor property
- [[syntax/types/INDEX|types]] -- full type system specification

