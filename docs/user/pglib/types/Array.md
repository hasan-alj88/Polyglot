---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Array"
metadata_instance: "%#:Array:N"
---

# #Array Collection

<!-- @types -->

Contiguous, rectangular collection with typed elements and N-dimensional support. `#Array` is a generic type with `[#] <param` inputs.

---

## Definition

```polyglot
{#} #Array
   [#] <#ValueType
   [#] <Dim##Dimension <~ "1D"
   [#] << ##Array
      [#] <#ValueType << <#ValueType
      [#] <Dim << <Dim
   [#] %##Alias << "array"
```

The `<#ValueType` parameter sets the element type. The `<Dim` parameter sets the dimension (defaults to `"1D"`). The `##Array` parameterized schema provides the structural constraints: contiguous, rectangular, propagated to all levels.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#False` (via ##Contiguous) | No gaps in indices |
| `%##Ordered` | `#True` (via ##Contiguous) | Insertion order preserved |
| `%##Regular` | `#True` (via ##Rectangular) | Same child count per dimension |
| `%##Depth.Max` | `<Dim` (via ##Rectangular) | Dimension count |
| `%##Propagate` | `#True` (via ##Rectangular) | Properties apply to all levels |
| `%##Flexible` | `#FlexKind.Range` (via ##Rectangular) | Compiler-generated indices |
| `%##Key` | `#UnsignedInt` | Integer indices |
| `%###Type` | `<#ValueType` | Element type constraint |

---

## Usage

```polyglot
[ ] #array:int → ValueType=Int, Dim=1D (default)
[-] $scores#array:int <~ {}

[ ] #array:float:2D → ValueType=Float, Dim=2D
[-] $matrix#array:float:2D <~ {}
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
- [[Map]] -- base key-value collection
- [[schemas/Array|##Array]] -- parameterized schema
- [[syntax/types/INDEX|types]] -- full type system specification
