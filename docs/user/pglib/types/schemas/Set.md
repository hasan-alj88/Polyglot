---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Set"
---

# ##Set Schema (Parameterized)

<!-- @types -->

`##Set` is a parameterized schema that describes a collection of unique values. It combines sparse storage with a uniqueness constraint on leaf values.

## Definition

```polyglot
{#} ##Set
   [#] <#ValueType
   [#] << ##Sparse
   [#] %##Flexible << #FlexKind.Flexible
   [#] %###Type << <#ValueType
   [#] %###Unique << #True
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#True` (via ##Sparse) | Gaps allowed |
| `%##Flexible` | `#FlexKind.Flexible` | User adds/removes entries |
| `%###Type` | `<#ValueType` | Element type constraint |
| `%###Unique` | `#True` | No duplicate values |

## Used By

- `#Set` type composes this schema

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Set` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Sparse|##Sparse]] -- gap-allowing base
- [[syntax/types/schema-properties|Schema Properties]] -- `%###Unique` property
