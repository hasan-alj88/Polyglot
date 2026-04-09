---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Map"
---

# ##Map Schema (Parameterized)

<!-- @types -->

`##Map` is a parameterized schema that describes a flat, sparse, flexible key-value structure. It takes key and value type parameters.

## Definition

```polyglot
{#} ##Map
   [#] <#KeyType
   [#] <#ValueType <~ #
   [#] << ##Flat
   [#] << ##Sparse
   [#] %##Flexible << #FlexKind.Flexible
   [#] %##Key << <#KeyType
   [#] %###Type << <#ValueType
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` (via ##Flat) | One level of children |
| `%##Gap` | `#True` (via ##Sparse) | Gaps allowed in keys |
| `%##Flexible` | `#FlexKind.Flexible` | User adds/removes entries |
| `%##Key` | `<#KeyType` | Key type from parameter |
| `%###Type` | `<#ValueType` | Value type constraint |

## Used By

- `#Map` type composes this schema

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Map` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[Map]] -- `#Map` generic type composing ##Map
- [[schemas/Sparse|##Sparse]] -- gap-allowing base
- [[schemas/Flat|##Flat]] -- depth 1 base
