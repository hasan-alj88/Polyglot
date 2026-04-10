---
audience: pg-coder
type: specification
updated: 2026-04-09
status: retired
redirect: schema-properties
---

# ##Rectangular Schema (Retired)

<!-- @types -->

`##Rectangular` has been retired. The properties it bundled are now stated directly on each type definition using individual `%##` property assignments.

## Migration

The properties that `##Rectangular` set are now composed individually:

| Old (via ##Rectangular) | New (direct property) |
|---|---|
| `%##Regular << #True` | `[#] %##Regular << #True` |
| `%##Depth.Max << <Dim` | `[#] %##Depth.Max << <Dim` |
| `%##Propagate << #True` | `[#] %##Propagate << #True` |

The retired `%##Flexible` property (formerly `#FlexKind.Range`) has been replaced by `%##Fields` with `#Range` value. See [[schemas/Fields|%##Fields]].

## See Also

- [[syntax/types/schema-properties|Schema Properties]] -- current property definitions
- [[schemas/INDEX|## Schema Types]] -- all schema definitions
