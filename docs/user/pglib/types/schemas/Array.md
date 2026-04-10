---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Array"
---

# ##Array Schema (Parameterized)

<!-- @c:types -->

`##Array` is a parameterized schema that describes a contiguous, rectangular, N-dimensional structure. It takes value type and dimension parameters.

## Definition

```polyglot
{#} ##Array
   (#) <#ValueType
   (#) <Dim <~ "1D"
   [#] %##Gap << #False
   [#] %##Ordered << #True
   [#] %##Regular << #True
   [#] %##Depth.Max << <Dim
   [#] %##Propagate << #True
   [#] %##Fields << #Range
   [#] %###Type << <#ValueType
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#False` | No gaps in indices |
| `%##Ordered` | `#True` | Insertion order preserved |
| `%##Regular` | `#True` | Same child count per dimension |
| `%##Depth.Max` | `<Dim` | Dimension count |
| `%##Propagate` | `#True` | Properties apply to all levels |
| `%##Fields` | `#Range` | Compiler-generated integer indices |
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
- [[syntax/types/schema-properties|Schema Properties]] -- property definitions
