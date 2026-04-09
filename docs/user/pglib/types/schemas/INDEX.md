---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
---

# ## Schema Types

<!-- @types -->

`##` schemas are compile-time metadata constraints on `{#}` types. Each schema lives at `%definition.##:{Name}` on the metadata tree. A `#` struct can compose multiple `##` schemas as long as they do not contradict -- the compiler validates all composed constraints at definition time.

## Static Schemas

These schemas set fixed property values with no parameters:

| Schema | Properties | Used By |
|--------|-----------|---------|
| [[schemas/Leaf\|##Leaf]] | `%##Depth.Max << 0` | (rare -- atomic types) |
| [[schemas/Scalar\|##Scalar]] | `%##Depth.Max << 1` | enums, #path, #Queue, scalars |
| [[schemas/Flat\|##Flat]] | `%##Depth.Max << 1` | #Map, #Job |
| [[schemas/Deep\|##Deep]] | `%##Depth.Max << .Inf` | #Serial |
| [[schemas/Inf\|##Inf]] | composable `.Inf` variant | #Bound |
| [[schemas/Contiguous\|##Contiguous]] | `%##Gap << #False`, `%##Ordered << #True` | #Array, #Dataframe |
| [[schemas/Sparse\|##Sparse]] | `%##Gap << #True` | #Map, #Set, #Serial |
| [[schemas/Sorted\|##Sorted]] | `%##Sorted << #True`, `%##Ordered << #True` | (user-defined) |
| [[schemas/Enum\|##Enum]] | `##Flat` + `%##Active << .One` + `%###Kind << .Enum` | #Boolean, #OS, all enums |

## Parameterized Schemas

These schemas use `[#] <param` inputs to generate structural constraints:

| Schema | Parameters | Purpose |
|--------|-----------|---------|
| [[schemas/Rectangular\|##Rectangular]] | `<Dim` | Regular shape, propagated to N levels |
| [[schemas/String\|##String]] | `<regex` | `.string` + `.regex` pattern validation |
| [[schemas/Fields\|##Fields]] | `<#Type(##Enum)` | Stamps enum variants as `[.]` fields |
| [[schemas/Nullable\|##Nullable]] | `<#ValueType` | `.OK.Value` OR `.None` |
| [[schemas/Result\|##Result]] | `<#OkType, <#ErrType` | `.OK.Value` OR `.Err.Value` |
| [[schemas/Map\|##Map]] | `<#KeyType, <#ValueType` | Flat sparse flexible key-value |
| [[schemas/Array\|##Array]] | `<#ValueType, <Dim` | Contiguous rectangular N-dimensional |
| [[schemas/Set\|##Set]] | `<#ValueType` | Sparse + `%###Unique` |
| [[schemas/Dataframe\|##Dataframe]] | `<#Columns, <#CellType` | Array of maps (row-oriented) |

## Retired Schemas

| Schema | Replaced By | Reason |
|--------|-------------|--------|
| ~~##Homogeneous~~ | `%###Type << #SomeType` | Child type uniformity now expressed through leaf-level property |
| ~~##Heterogeneous~~ | `%###Type` absent | Per-field typing is the default when no `%###Type` is set |
| ~~##EnumLeafs~~ | `%###Kind << #FieldKind.Enum` | Leaf kind constraint now uses `%###Kind` property |

## Related

- [[syntax/types/schema-properties|Schema Properties]] -- full `%##` / `%###` property reference
- [[scalars]] -- scalar subtypes that compose ##Scalar
- [[concepts/collections/INDEX|collections]] -- collection types and their schema compositions
- [[syntax/types/INDEX|types]] -- full type system specification
