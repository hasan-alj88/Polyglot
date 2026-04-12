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

## Allows

```
#Int [###ScalarValue]
├── .string  -> "42"               ← regex-validated string data
└── .regex   -> "^-?[0-9]+$"        .string matches .regex ✓

#Float [###ScalarValue]
├── .string  -> "3.14"
└── .regex   -> "^-?[0-9]+\.[0-9]+$"
```

## Disallows

```
#Int [###ScalarValue]
├── .string  -> "hello"            ✗ does not match "^-?[0-9]+$"
└── .regex   -> "^-?[0-9]+$"

#Int [###ScalarValue — only valid with ##Scalar]
├── .string  -> "42"
├── .regex   -> "^-?[0-9]+$"
└── .extra   -> "metadata"         ✗ ##Scalar constrains to depth 1
                                     with only .string and .regex
```

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

<!-- @u:pglib/types/scalars -->
<!-- @u:pglib/types/string -->

| Type | Regex Pattern | Valid Content |
|------|--------------|---------------|
| [[scalars\|#Int]] | `^-?[0-9]+$` | Integer strings |
| [[scalars\|#Float]] | `^-?[0-9]+\.[0-9]+$` | Decimal strings |
| All [[scalars\|scalar subtypes]] | Various | Regex-constrained strings |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.###:ScalarValue` | Field type definition template |

Field types are compile-time leaf content classifiers -- they have no runtime instances.

## Related

- [[scalars]] -- scalar subtypes of #String
- [[schemas/Scalar]] -- ##Scalar schema
- [[syntax/types/INDEX|types]] -- full type system specification
