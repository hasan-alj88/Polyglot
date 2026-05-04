---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.###:None"
---

# ###None Field Type

<!-- @c:types -->

`###None` classifies a leaf where empty string `""` is the only valid value. This field type is used exclusively by the `#None` type. All other types reject `""` with PGE04021.

## Allows

```
#None [###None]
  ""                        ← empty string is the only valid value
                              no fields, no children
```

## Disallows

```
#None [###None]
  "hello"                   ✗ non-empty string — ###None only accepts ""

$var#string <~ ""           ✗ PGE04021 — empty string on a non-###None type
                              only ###None types accept ""
```

## Declaration

```aljam3
{#} #None
   [#] ##Scalar
   [#] ###None
```

The `[#] ###None` line declares the field type. No `[.]` fields are needed -- the type itself holds only `""`.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.###:None` | Field type definition template |

Field types are compile-time leaf content classifiers -- they have no runtime instances.

## Used By

<!-- @u:jm3lib/types/boolean -->

- [[boolean|#None]] -- the only type using `###None`

## Related

- [[boolean]] -- documents #None alongside #Boolean
- [[syntax/types/INDEX|types]] -- full type system specification
