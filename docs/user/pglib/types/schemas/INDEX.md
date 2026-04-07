---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# ## Schema Types

<!-- @types -->

`##` schemas are compile-time metadata constraints on `{#}` types. Each schema lives at `%definition.##:{Name}` on the metadata tree. A `#` struct can compose multiple `##` schemas as long as they do not contradict — the compiler validates all composed constraints at definition time.

## Summary

| Schema | Property | Value | Used By |
|--------|----------|-------|---------|
| [[schemas/Leaf\|##Leaf]] | `%##Depth.Max` | `0` | (rare) |
| [[schemas/Scalar\|##Scalar]] | `%##Depth.Max` | `1` | enums, #path, #Queue, scalars |
| [[schemas/Flat\|##Flat]] | `%##Depth.Max` | `1` | #Map, #Job |
| [[schemas/Deep\|##Deep]] | `%##Depth.Max` | `-1` | #Serial |
| [[schemas/Homogeneous\|##Homogeneous]] | `%##Children.Uniform` | `#True` | #Map (homo), #Array |
| [[schemas/Heterogeneous\|##Heterogeneous]] | `%##Children.Uniform` | `#False` | #Map (hetero), #Serial |
| [[schemas/Contiguous\|##Contiguous]] | `%##Children.Gap`, `.Ordered` | `#False`, `#True` | #Array, #Dataframe |
| [[schemas/Sparse\|##Sparse]] | `%##Children.Gap` | `#True` | #Map, #Serial |
| [[schemas/Rectangular\|##Rectangular]] | `%##Children.Regular`, `.Uniform` | `#True`, `#True` | #Array, #Dataframe |
| [[schemas/Enum\|##Enum]] | (classification) | — | #Boolean, #OS, all enums |

## Related

- [[scalars]] -- scalar subtypes that compose ##Scalar
- [[concepts/collections/INDEX|collections]] -- collection types and their schema compositions
- [[syntax/types/INDEX|types]] -- full type system specification
