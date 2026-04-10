---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.###:ScalarValue"
---

# ###ScalarValue Field Type

<!-- @c:types -->

`###ScalarValue` classifies a leaf node that holds regex-validated string data. The leaf content is a string that must match the type's `.regex` pattern at runtime.

This field type applies to `##Scalar` subtypes of `#String` where the leaf carries validated data rather than acting as a variant selector.

## Declaration

Scalar value types are defined through the `##Scalar` composition. The `.string` field inherits from `#String` and the `.regex` field constrains valid content:

```polyglot
{#} ##Int
   [#] ##Scalar
   [#] ###ScalarValue
   [.] .string#string
   [.] .regex << "^-?[0-9]+$"
```

## Example Types

| Type | Regex Pattern | Valid Content |
|------|--------------|---------------|
| `##Int` | `^-?[0-9]+$` | Integer strings |
| `##Float` | `^-?[0-9]+\.[0-9]+$` | Decimal strings |
| All `##` scalar subtypes | Various | Regex-constrained strings |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.###:ScalarValue` | Field type definition template |

Field types are compile-time leaf content classifiers -- they have no runtime instances.

## Related

- [[scalars]] -- scalar subtypes of #String
- [[schemas/Scalar]] -- ##Scalar schema
- [[syntax/types/INDEX|types]] -- full type system specification
