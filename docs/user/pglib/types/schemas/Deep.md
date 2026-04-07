---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Deep"
---

# ##Deep Schema

<!-- @types -->

`##Deep` permits unlimited nesting depth. Types composing this schema can contain children that themselves contain children, recursively.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `-1` | Unlimited nesting depth |

## Used By

- `#Serial`

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Deep` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Flat|##Flat]] -- depth 1 (one level of flexible children)
- [[schemas/Heterogeneous|##Heterogeneous]] -- mixed child types (also used by #Serial)
- [[syntax/types/INDEX|types]] -- full type system specification
