---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.##:Flat"
---

# ##Flat Schema

<!-- @c:types -->

`##Flat` constrains a type to one level of flexible `:` children. Both `##Flat` and `##Scalar` set `Depth.Max=1`, but `##Flat` applies to types with flexible `:` children while `##Scalar` applies to types with fixed `.` children.

## Allows

```
#Job
├── :taskA  -> "running"    ← one level of flexible children
├── :taskB  -> "pending"
└── :taskC  -> "done"
```

## Disallows

```
#Job [##Flat]
├── :taskA  -> "running"
└── :taskB
    └── :sub1  -> "nested"  ✗ nesting creates depth 2
                              ##Flat allows only depth 1
```

## Property

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` | One level of flexible `:` children |

## Used By

<!-- @u:jm3lib/types/Job -->

- [[Job|#Job]]
- `##Enum` (composes ##Flat)
- `##Record` (composes ##Flat)

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Flat` | Schema definition template |

Schemas are compile-time metadata constraints — they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Scalar|##Scalar]] -- also depth 1 but for fixed `.` children
- [[schemas/Deep|##Deep]] -- unlimited nesting depth
- [[concepts/collections/INDEX|collections]] -- collection types using ##Flat
- [[syntax/types/INDEX|types]] -- full type system specification
