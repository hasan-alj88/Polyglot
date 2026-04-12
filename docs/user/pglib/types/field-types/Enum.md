---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.###:Enum"
---

# ###Enum Field Type

<!-- @c:types -->

`###Enum` classifies a leaf node as a variant selector. The field is declared WITHOUT a type annotation in the `{#}` block: `[.] .VariantName`. Exactly one variant is active at a time.

This is distinct from the `##Enum` schema. `##Enum` is a schema that constrains an entire type; `###Enum` is the field type assigned to each individual variant leaf within that type.

## Allows

```
#Boolean [###Enum]
├── .True                  ✓ active     ← no #type — identity IS the value
└── .False                 ○ inactive     the active variant is the data
```

## Disallows

```
#Boolean [###Enum]
├── .True
├── .False
└── .count -> 0#int        ✗ typed field among enum fields — PGE05005
                             siblings must all be same ### kind
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

Each `[.]` line without a `#type` annotation declares a `###Enum` field. The compiler assigns `###Enum` as the field type on the metadata tree.

## Example Types

<!-- @u:pglib/types/boolean -->
<!-- @u:pglib/types/OS -->

| Type | Variants | Purpose |
|------|----------|---------|
| [[boolean\|#Boolean]] | `.True`, `.False` | Boolean logic |
| [[OS\|#OS]] | `.Unix`, `.Windows` | Operating system target |
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
