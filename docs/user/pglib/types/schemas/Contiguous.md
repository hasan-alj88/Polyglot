---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Contiguous"
---

# ##Contiguous Schema

<!-- @c:types -->

`##Contiguous` enforces that no gaps exist in child keys and that insertion order is preserved. Elements form a dense, ordered sequence.

## Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#False` | No gaps allowed in child keys |
| `%##Ordered` | `#True` | Insertion order preserved |

## Used By

- `#Array` (via `##Array`)
- `#Dataframe` (via `##Dataframe`)

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Contiguous` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Sparse|##Sparse]] -- opposite: gaps allowed
- [[schemas/Rectangular|##Rectangular]] -- regular shape (often paired with Contiguous)
