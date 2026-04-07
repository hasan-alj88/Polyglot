---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Sparse"
---

# ##Sparse Schema

<!-- @types -->

`##Sparse` permits gaps in child keys. Not all key positions need to be occupied.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Gap` | `#True` | Gaps allowed in child keys |

## Used By

- `#Map`
- `#Serial`

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Sparse` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Contiguous|##Contiguous]] -- opposite: no gaps, ordered
- [[concepts/collections/INDEX|collections]] -- collection types using ##Sparse
- [[syntax/types/INDEX|types]] -- full type system specification
