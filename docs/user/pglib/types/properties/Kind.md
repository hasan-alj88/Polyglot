---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%###Kind"
---

# %###Kind

<!-- @c:syntax/types/schema-properties -->

`%###Kind` declares whether leaf nodes hold typed data (`###Value`) or act as variant selectors (`###Enum`). This is the fundamental classification of what a field contains.

## Allows (`%###Kind << ###Value`)

```
#Person
├── .name   -> "Alice"#string      ← typed data: field has #type annotation
├── .age    -> 30#int
└── .email  -> "a@b.com"#string
                                    leaves hold values
```

## Allows (`%###Kind << ###Enum`)

```
#Boolean
├── .True                           ← no #type annotation
└── .False                            identity IS the value
                                    leaves are variant selectors
```

## Disallows

```
#Mixed [%###Kind << ###Enum]
├── .Active                         ✓ enum field (no type)
├── .Inactive                       ✓ enum field (no type)
└── .count  -> 5#int                ✗ typed field — PGE05005
                                      siblings must all be same ### kind

#Mixed [%###Kind << ###Value]
├── .name  -> "Alice"#string        ✓ typed field
└── .Status                         ✗ enum field (no type) — PGE05005
                                      siblings must all be same ### kind
```

## Values

| Value | Meaning |
|-------|---------|
| `#FieldKind.Value` / `###Value` | Leaves hold typed data (`#type` annotation present) |
| `#FieldKind.Enum` / `###Enum` | Leaves are variant selectors (no `#type` annotation) |

The compiler infers `###Value` if fields have `#type` annotations, `###Enum` if they do not. Explicit declaration is optional. Contradiction between explicit declaration and fields raises PGE11003.

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[field-types/Value|###Value]] -- typed data field type
- [[field-types/Enum|###Enum]] -- variant selector field type
- [[schemas/Enum|##Enum]] -- bundles `%###Kind << #FieldKind.Enum`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
