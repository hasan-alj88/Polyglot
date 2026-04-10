---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Leaf"
---

# ##Leaf Schema

<!-- @c:types -->

`##Leaf` constrains a type to zero depth — the type itself is its only node, with no children allowed.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `0` | No children permitted |

## Used By

Theoretical — most types have at least depth 1. `##Leaf` exists as the logical base case of the depth hierarchy.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Leaf` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Scalar|##Scalar]] -- depth 1 (one level of children)
- [[scalars]] -- scalar subtypes that compose ##Scalar
- [[syntax/types/INDEX|types]] -- full type system specification
