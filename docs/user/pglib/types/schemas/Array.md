---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Array"
---

# ##Array Schema (Parameterized)

<!-- @types -->

`##Array` is a parameterized schema that describes a contiguous, rectangular, N-dimensional structure. It takes value type and dimension parameters.

## Definition

```polyglot
{#} ##Array
   [#] <#ValueType
   [#] <Dim <~ "1D"
   [#] << ##Contiguous
   [#] << ##Rectangular
      [#] <Dim << <Dim
   [#] %##Key << #UnsignedInt
   [#] %###Type << <#ValueType
```

## Properties Set

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

## Used By

- `#Array` type composes this schema

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Array` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[Array]] -- `#Array` generic type composing ##Array
- [[schemas/Contiguous|##Contiguous]] -- no-gap base
- [[schemas/Rectangular|##Rectangular]] -- regular shape base
