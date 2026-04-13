---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.###:ScalarEnum"
---

# ###ScalarEnum Field Type

<!-- @c:types -->

`###ScalarEnum` classifies a variant selector within a scalar type. It combines `##Enum` classification with `##Scalar` depth constraint -- the leaf is an enum variant (no type annotation) within a depth-1 type.

A type composes `###ScalarEnum` when it is both an enum (all fields are variants) and a scalar (one level of fixed children).

## Allows

```
#Boolean [###ScalarEnum]
├── .True                  ✓ active     ← variant selector in scalar context
└── .False                 ○ inactive     ##Scalar + ##Enum + no #type
```

## Disallows

```
#Boolean [###ScalarEnum — requires ##Scalar]
├── .True
├── .False
└── .Details                           ✗ nesting creates depth 2
    └── .Reason -> "override"            ##Scalar limits to depth 1

#NotScalar [###ScalarEnum invalid without ##Scalar]
├── :variant1                          ✗ ###ScalarEnum only valid
└── :variant2                            with ##Scalar types
```

## Declaration

```polyglot
{#} #Boolean
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [.] .True
   [.] .False
```

The `[#] ###ScalarEnum` line declares the field type for all variant fields in the type. Each `[.]` without a `#type` annotation is a scalar enum variant.

## Example Types

<!-- @u:pglib/types/boolean -->
<!-- @u:pglib/types/OS -->
<!-- @u:pglib/types/PipelineStatus -->
<!-- @u:pglib/types/VarState -->
<!-- @u:pglib/types/QueueStrategy -->

| Type | Variants | Purpose |
|------|----------|---------|
| [[boolean\|#Boolean]] | `.True`, `.False` | Boolean logic |
| [[OS\|#OS]] | `.Unix`, `.Windows` | Operating system target |
| [[PipelineStatus\|#PipelineStatus]] | Various | Pipeline lifecycle state |
| [[VarState\|#VarState]] | Various | Variable state tracking |
| [[QueueStrategy\|#QueueStrategy]] | Various | Queue dispatch strategy |

All enum types that compose both `##Enum` and `##Scalar` use `###ScalarEnum`.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.###:ScalarEnum` | Field type definition template |

Field types are compile-time leaf content classifiers -- they have no runtime instances.

## Related

- [[schemas/Enum]] -- ##Enum schema
- [[schemas/Scalar]] -- ##Scalar schema
- [[enums]] -- enum type documentation
- [[syntax/types/INDEX|types]] -- full type system specification
