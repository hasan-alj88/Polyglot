---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Dataframe"
---

# ##Dataframe Schema (Parameterized)

<!-- @types -->

`##Dataframe` is a parameterized schema that describes a row-oriented tabular structure -- an array of maps. Each row is a map keyed by column names, and all rows share the same column set.

## Definition

```polyglot
{#} ##Dataframe
   [#] <#Columns << ##Enum
   [#] <#CellType <~ #
   [#] << ##Contiguous
   [#] << ##Rectangular
   [#] %##Key << #UnsignedInt
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#False` (via ##Contiguous) | No gaps in row indices |
| `%##Ordered` | `#True` (via ##Contiguous) | Row order preserved |
| `%##Regular` | `#True` (via ##Rectangular) | Every row has same column count |
| `%##Key` | `#UnsignedInt` | Integer row indices |

The `<#Columns` parameter must satisfy `##Enum` -- column names come from an enum type. `<#CellType` defaults to `#` (any type).

## Used By

- `#Dataframe` type composes this schema

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Dataframe` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[Dataframe]] -- `#Dataframe` generic type composing ##Dataframe
- [[schemas/Contiguous|##Contiguous]] -- no-gap base
- [[schemas/Rectangular|##Rectangular]] -- regular shape base
