---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.###:ScalarEnum"
---

# ###ScalarEnum Field Type

<!-- @c:types -->

`###ScalarEnum` classifies a variant selector within a scalar type. It combines `##Enum` classification with `##Scalar` depth constraint -- the leaf is an enum variant (no type annotation) within a depth-1 type.

A type composes `###ScalarEnum` when it is both an enum (all fields are variants) and a scalar (one level of fixed children).

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

| Type | Variants | Purpose |
|------|----------|---------|
| `#Boolean` | `.True`, `.False` | Boolean logic |
| `#OS` | `.Unix`, `.Windows` | Operating system target |
| `#PipelineStatus` | Various | Pipeline lifecycle state |
| `#VarState` | Various | Variable state tracking |
| `#QueueStrategy` | Various | Queue dispatch strategy |

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
