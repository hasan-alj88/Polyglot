---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Enum"
---

# ##Enum Schema

<!-- @types -->

`##Enum` classifies a type as an enumeration. All fields are enum fields (no type annotation) — they represent distinct named values rather than typed data.

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| (classification) | Enum | All fields are enum fields without `;type` |

## Used By

- `#Boolean`
- `#OS`
- `#PipelineStatus`
- `#VarState`
- All permission enums

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Enum` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Scalar|##Scalar]] -- enum types also compose ##Scalar (depth 1)
- [[boolean]] -- #Boolean enum type
- [[syntax/types/INDEX|types]] -- full type system specification
