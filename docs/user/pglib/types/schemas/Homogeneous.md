---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Homogeneous"
---

# ##Homogeneous Schema

<!-- @types -->

`##Homogeneous` requires all children to have the same type. The compiler enforces uniform typing across every child element.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Uniform` | `#True` | All children must have same type |

## Used By

- `#Map` (homogeneous variant)
- `#Array`

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Homogeneous` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Heterogeneous|##Heterogeneous]] -- opposite: mixed child types
- [[schemas/Rectangular|##Rectangular]] -- combines uniform types with regular shape
- [[concepts/collections/INDEX|collections]] -- collection types using ##Homogeneous
- [[syntax/types/INDEX|types]] -- full type system specification
