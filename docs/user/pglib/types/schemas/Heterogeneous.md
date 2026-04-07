---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Heterogeneous"
---

# ##Heterogeneous Schema

<!-- @types -->

`##Heterogeneous` allows children to have different types. Each child can carry a distinct type annotation.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Uniform` | `#False` | Children can have different types |

## Used By

- `#Map` (heterogeneous variant)
- `#Serial`

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Heterogeneous` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Homogeneous|##Homogeneous]] -- opposite: uniform child types
- [[schemas/Deep|##Deep]] -- unlimited depth (also used by #Serial)
- [[concepts/collections/INDEX|collections]] -- collection types using ##Heterogeneous
- [[syntax/types/INDEX|types]] -- full type system specification
