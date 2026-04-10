---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.###:None"
---

# ###None Field Type

<!-- @types -->

`###None` classifies a leaf where empty string `""` is the only valid value. This field type is used exclusively by the `#None` type. All other types reject `""` with PGE04021.

## Declaration

```polyglot
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

## Related

- [[boolean]] -- documents #None alongside #Boolean
- [[syntax/types/INDEX|types]] -- full type system specification
