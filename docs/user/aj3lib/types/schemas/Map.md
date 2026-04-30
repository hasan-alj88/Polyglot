---
audience: automation-builder
type: specification
updated: 2026-04-09
status: retired
redirect: aj3lib/types/schemas/Record
metadata_definition: "%definition.##:Map"
---

# ##Map Schema (Retired)

<!-- @c:types -->

`##Map` has been retired. Use `##Record` instead.

`##Record` provides enum-keyed fields with `%##Fields << <#Fields` instead of arbitrary sparse keys. See [[schemas/Record|##Record]] for the full definition.

## Migration

| Former (##Map) | Now (##Record) |
|----------------|----------------|
| `%##Key << <#KeyType` | `%##Fields << <#Fields` (enum ref) |
| `%##Flexible << #FlexKind.Flexible` | Retired -- fields determined by enum |
| `%##Gap << #True` (via ##Sparse) | Retired -- all enum fields present |
| `%###Type << <#ValueType` | `%###Type << <#ValueType` (unchanged) |

## Related

- [[schemas/Record|##Record]] -- replacement schema
- [[schemas/INDEX|## Schema Types]] -- retired schemas list
- [[schemas/Fields|%##Fields]] -- field descriptor property

