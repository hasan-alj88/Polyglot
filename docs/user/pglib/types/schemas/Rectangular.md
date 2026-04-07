---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Rectangular"
---

# ##Rectangular Schema

<!-- @types -->

`##Rectangular` enforces a regular shape where all rows (or equivalent dimensions) have the same length, and all elements share a uniform type.

## Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Regular` | `#True` | Regular shape (all dimensions equal) |
| `%##Children.Uniform` | `#True` | All elements share the same type |

## Used By

- `#Array`
- `#Dataframe`

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Rectangular` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Homogeneous|##Homogeneous]] -- uniform types (subset of Rectangular)
- [[schemas/Contiguous|##Contiguous]] -- no gaps, ordered (often paired with Rectangular)
- [[concepts/collections/INDEX|collections]] -- collection types using ##Rectangular
- [[syntax/types/INDEX|types]] -- full type system specification
