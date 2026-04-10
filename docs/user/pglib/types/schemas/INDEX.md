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
| [[schemas/Flat\|##Flat]] | `%##Depth.Max << 1` | ##Record, #Job |
| [[schemas/Inf\|##Inf]] | composable `.Inf` variant | #Bound |
| [[schemas/Sorted\|##Sorted]] | `%##Sorted << #True`, `%##Ordered << #True` | (user-defined) |
| [[schemas/Enum\|##Enum]] | `##Flat` + `%##Active << #One` + `%###Kind << #FieldKind.Enum` | #Boolean, #OS, all enums |

## Collection Schemas

These schemas describe collection types using `%##Fields` and other branch-level properties:

| Schema | Parameters | Purpose |
|--------|-----------|---------|
| [[schemas/Record\|##Record]] | `<#Fields(##Enum), <#ValueType` | Enum-keyed, all-active, value fields |
| [[schemas/Array\|##Array]] | `<#ValueType, <Dim` | Range-indexed, ordered, N-dimensional |
| [[schemas/Dataframe\|##Dataframe]] | `<#Columns, <#CellType` | Two-level: range rows + ##Record columns |

## Other Parameterized Schemas

| Schema | Parameters | Purpose |
|--------|-----------|---------|
| [[schemas/String\|##String]] | `<regex` | `.string` + `.regex` pattern validation |
| [[schemas/Nullable\|##Nullable]] | `<#ValueType` | `.Ok.Value` OR `.None` |
| [[schemas/Result\|##Result]] | `<#OkType, <#ErrType` | `.Ok.Value` OR `.Err.Value` |

## Retired Schemas

| Schema | Replaced By | Reason |
|--------|-------------|--------|
| ~~##Map~~ | `##Record` | Enum-keyed records replace sparse key-value maps |
| ~~##Set~~ | `##Array` + `%###Unique << #True` | Sets are arrays with uniqueness constraint |
| ~~##Contiguous~~ | `%##Gap << #False`, `%##Ordered << #True` | Properties stated directly |
| ~~##Rectangular~~ | `%##Propagate << #True`, `%##Count` | Properties stated directly |
| ~~##Sparse~~ | `%##Gap << #True` | Property stated directly |
| ~~##Deep~~ | `%##Depth.Max << #Inf` | Property stated directly |
| ~~##Fields~~ | `%##Fields` property | Replaced by branch-level property taking `#FieldsDescriptor` or enum ref |
| ~~##Homogeneous~~ | `%###Type << #SomeType` | Child type uniformity now expressed through leaf-level property |
| ~~##Heterogeneous~~ | `%###Type` absent | Per-field typing is the default when no `%###Type` is set |
| ~~##EnumLeafs~~ | `%###Kind << #FieldKind.Enum` | Leaf kind constraint now uses `%###Kind` property |

## Related

- [[syntax/types/schema-properties|Schema Properties]] -- full `%##` / `%###` property reference
- [[scalars]] -- scalar subtypes that compose ##Scalar
- [[concepts/collections/INDEX|collections]] -- collection types and their schema compositions
- [[syntax/types/INDEX|types]] -- full type system specification
