---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.###:Enum"
---

# ###Enum Field Type

<!-- @types -->

`###Enum` classifies a leaf node as a variant selector. The field is declared WITHOUT a type annotation in the `{#}` block: `[.] .VariantName`. Exactly one variant is active at a time.

This is distinct from the `##Enum` schema. `##Enum` is a schema that constrains an entire type; `###Enum` is the field type assigned to each individual variant leaf within that type.

## Declaration

```polyglot
{#} #Boolean
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [.] .True
   [.] .False
```

Each `[.]` line without a `#type` annotation declares a `###Enum` field. The compiler assigns `###Enum` as the field type on the metadata tree.

## Example Types

| Type | Variants | Purpose |
|------|----------|---------|
| `#Boolean` | `.True`, `.False` | Boolean logic |
| `#OS` | `.Unix`, `.Windows` | Operating system target |
| All permission enums | Various | Access control variants |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.###:Enum` | Field type definition template |

Field types are compile-time leaf content classifiers -- they have no runtime instances.

## Related

- [[schemas/Enum]] -- ##Enum schema that constrains enum types
- [[boolean]] -- #Boolean and #None types
- [[syntax/types/INDEX|types]] -- full type system specification
