---
audience: pg-coder
type: specification
updated: 2026-04-09
status: retired
redirect: pglib/types/schemas/Array
metadata_definition: "%definition.##:Set"
---

# ##Set Schema (Retired)

<!-- @c:types -->

`##Set` has been retired. Use `##Array` with `%###Unique << #True` instead.

The uniqueness guarantee formerly provided by `##Set` is now a leaf-level property on any collection. Compose `##Array` for ordered, range-indexed storage and add `%###Unique << #True` to reject duplicates.

## Migration

| Former (##Set) | Now |
|----------------|-----|
| `##Set` schema | `##Array` + `%###Unique << #True` |
| `%##Flexible << #FlexKind.Flexible` | Retired |
| `%###Unique << #True` | `%###Unique << #True` (unchanged) |

## Related

- [[schemas/Array|##Array]] -- replacement schema
- [[schemas/INDEX|## Schema Types]] -- retired schemas list

