---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Enum"
---

# ##Enum Schema

<!-- @c:types -->

`##Enum` classifies a type as an enumeration -- all fields are enum fields (no type annotation), and exactly one branch is active at any time. It composes `##Flat` (depth 1) with active-one and enum leaf constraints.

## Definition

```polyglot
{#} ##Enum
   [#] ##Flat
   [#] %##Active << #ActiveKind.One
   [#] %###Kind << #FieldKind.Enum
```

## Allows

```
#Boolean [##Enum]
├── .True                  ✓ active      ← exactly one branch active
└── .False                 ○ inactive      all fields are enum (no #type)
```

## Disallows

```
#BadEnum [##Enum]
├── .Active                ✓ active
└── .Inactive              ✓ active
                           ✗ two branches active — #One requires exactly one

#BadEnum [##Enum]
├── .Name  -> "Alice"#string             ✗ typed field — ##Enum requires
└── .Status                                 all fields to be enum (no #type)

#BadEnum [##Enum]
├── .Status
└── .Detail
    └── .Code  -> 404      ✗ nesting creates depth 2
                             ##Enum composes ##Flat (depth 1)
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` (via ##Flat) | One level of children |
| `%##Active` | `#ActiveKind.One` | Exactly one branch active |
| `%###Kind` | `#FieldKind.Enum` | All leaves are variant selectors |

## Used By

<!-- @u:pglib/types/boolean -->
<!-- @u:pglib/types/OS -->
<!-- @u:pglib/types/PipelineStatus -->
<!-- @u:pglib/types/VarState -->
<!-- @u:pglib/types/ActiveKind -->
<!-- @u:pglib/types/QueueStrategy -->
<!-- @u:pglib/types/KillPropagation -->

- [[boolean|#Boolean]]
- [[OS|#OS]]
- [[PipelineStatus|#PipelineStatus]]
- [[VarState|#VarState]]
- [[ActiveKind|#ActiveKind]]
- [[QueueStrategy|#QueueStrategy]]
- [[KillPropagation|#KillPropagation]]
- All permission enums

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Enum` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Scalar|##Scalar]] -- enum types also compose ##Scalar (depth 1)
- [[schemas/Flat|##Flat]] -- depth 1 base
- [[ActiveKind]] -- `#ActiveKind` enum used by `%##Active`
- [[FieldKind]] -- `#FieldKind` enum used by `%###Kind`
- [[boolean]] -- #Boolean enum type
