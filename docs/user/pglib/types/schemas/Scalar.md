---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Scalar"
---

# ##Scalar Schema

<!-- @c:types -->

`##Scalar` constrains a type to one level of fixed `.` children. This is the most common schema for simple types with named fields.

## Allows

```
#String
├── .string  -> "hello"    ← one level of fixed children
└── .regex   -> ".*"
```

## Disallows

```
#String [##Scalar]
├── .string  -> "hello"
├── .regex   -> ".*"
└── .metadata                ✗ nesting creates depth 2
    └── .encoding -> "utf8"    ##Scalar allows only depth 1
```

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` | One level of fixed `.` children |

## Used By

<!-- @u:pglib/types/boolean -->
<!-- @u:pglib/types/OS -->
<!-- @u:pglib/types/path -->
<!-- @u:pglib/types/Queue -->
<!-- @u:pglib/types/scalars -->

- All enum types ([[boolean|#Boolean]], [[OS|#OS]], etc.)
- [[path|#path]]
- [[Queue|#Queue]]
- [[string|#String]] subtypes ([[scalars|#Int, #Float, etc.]])
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
