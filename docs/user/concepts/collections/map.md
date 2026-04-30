---
audience: automation-builder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->
<!-- @u:syntax/types/generic-types -->
<!-- @u:data-is-trees -->

## ##Record -- Enum-Keyed Collection

`##Record` is a parameterized schema for enum-keyed collections with typed value fields. It replaces the former `#Map` / `##Map`. The `<#Fields` parameter must satisfy `##Enum` -- field names come from an enum type. `<#ValueType` sets the value type for all fields.

See [[aj3lib/types/Map|##Record]] for the full definition and [[syntax/types/generic-types|Generic Types]] for the `(#) <param` syntax.

### Schema composition

`##Record` composes `##Flat` and sets:

- `%##Depth.Max << 1` -- flat (one level of children)
- `%##Fields << <#Fields` -- one child per enum variant
- `%##Active << #ActiveKind.All` -- all fields active simultaneously
- `%###Type << <#ValueType` -- uniform value type
- `%###Kind << #FieldKind.Value` -- all children are value fields

### Access

Use `<` to access fields by enum variant name:

```aljam3
{#} #Colors
   [#] ##Enum
   [#] ##Scalar
   [.] .Red
   [.] .Green
   [.] .Blue

{#} #RGBValues
   (#) <#Fields << #Colors
   (#) <#ValueType << #Int
   [#] ##Record
      (#) <#Fields << <#Fields
      (#) <#ValueType << <#ValueType

[-] $rgb#RGBValues <~ {}
[-] $red#int << $rgb<Red
[-] $green#int << $rgb<Green
```

## See Also

- [[concepts/collections/expand|Expand Operators]] -- iteration over record fields
- [[concepts/collections/collect|Collect Operators]] -- collection into records
- [[aj3lib/types/schemas/Fields|%##Fields]] -- field descriptor property

