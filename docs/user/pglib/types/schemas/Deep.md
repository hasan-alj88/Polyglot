---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Deep"
---

# ##Deep Schema

<!-- @c:types -->

`##Deep` permits unlimited nesting depth. Types composing this schema can contain children that themselves contain children, recursively.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `.Inf` | Unlimited nesting depth |

## Used By

- `#Serial`

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Deep` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Flat|##Flat]] -- depth 1 (one level of flexible children)
- [[schemas/Scalar|##Scalar]] -- depth 1 (one level of fixed children)
