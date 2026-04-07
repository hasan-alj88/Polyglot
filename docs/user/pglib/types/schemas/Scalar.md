---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Scalar"
---

# ##Scalar Schema

<!-- @types -->

`##Scalar` constrains a type to one level of fixed `.` children. This is the most common schema for simple types with named fields.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` | One level of fixed `.` children |

## Used By

- All enum types (#Boolean, #OS, etc.)
- `#path`
- `#Queue`
- `#String` subtypes (`##Int`, `##Float`, etc.)
- Collection types

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Scalar` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Leaf|##Leaf]] -- depth 0 (no children)
- [[schemas/Flat|##Flat]] -- also depth 1 but for flexible `:` children
- [[scalars]] -- scalar subtypes that compose ##Scalar
- [[syntax/types/INDEX|types]] -- full type system specification
