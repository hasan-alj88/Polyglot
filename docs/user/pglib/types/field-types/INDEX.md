---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# ### Field Types

<!-- @c:types -->

`###` field types classify what leaf nodes contain in `{#}` type definitions. They are compile-time metadata that tells the compiler how to validate and interpret the data stored at each leaf. Field types live at `%definition.###:{Name}` on the metadata tree.

Every field in a `{#}` type definition receives exactly one `###` field type classification.

## Summary

| Field Type | Leaf Content | Declaration | Example Types |
|------------|-------------|-------------|---------------|
| [[field-types/Value\|###Value]] | Typed data | `[.] .name#type` | #path, #Queue, #Job |
| [[field-types/Enum\|###Enum]] | Variant selector | `[.] .Name` (no type) | #Boolean, #OS, all enums |
| [[field-types/ScalarValue\|###ScalarValue]] | Regex-validated string | (via ##Scalar schema) | ##Int, ##Float |
| [[field-types/ScalarEnum\|###ScalarEnum]] | Variant in scalar | `[.] .Name` in ##Scalar | #Boolean, #OS |
| [[field-types/None\|###None]] | Empty string only | (special) | #None |

## Related

- [[properties/INDEX|%## and %### Properties]] -- individual property reference files with allow/disallow trees
- [[schemas/INDEX|schemas]] -- ## schema types that constrain type structure
- [[syntax/types/INDEX|types]] -- full type system specification
