---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Dataframe"
---

# ##Dataframe Schema (Parameterized)

<!-- @c:types -->

`##Dataframe` is a parameterized schema that describes a two-level tabular structure. Level 1 uses `%##Fields << #Range` for integer row indices. Level 2 composes `##Record` with the column enum as fields.

## Definition

```polyglot
{#} ##Dataframe
   (#) <#Columns << ##Enum
   (#) <#CellType <~ #
   [#] %##Depth.Max << 2
   [#] %##Fields << #Range
   [#] %##Ordered << #True
   [#] %##Gap << #False
   [#] %##Level.2 ##Record
      (#) <#Fields << <#Columns
      (#) <#ValueType << <#CellType
```

## Allows

```
#SalesData;Dataframe:#Product
├── :0                              ← L1: range-indexed rows
│   ├── :Widget  -> 9.99            ← L2: ##Record (enum-keyed columns)
│   ├── :Gadget  -> 19.99
│   ��── :Doohickey -> 4.99
└── :1
    ├── :Widget  -> 10.99
    ├── :Gadget  -> 18.99
    └── :Doohickey -> 5.49
```

## Disallows

```
#SalesData;Dataframe:#Product
├── :0
│   ├── :Widget  -> 9.99
│   └── :Gadget  -> 19.99
│                               ✗ :Doohickey missing — L2 is ##Record
│                                 with %##Active << #All
└── :2                          ✗ gap at :1 — L1 has %##Gap << #False
    ├── :Widget  -> 10.99
    ├── :Gadget  -> 18.99
    └─�� :Doohickey -> 5.49
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `2` | Two levels: rows + columns |
| `%##Fields` | `#Range` | Integer row indices (L1) |
| `%##Ordered` | `#True` | Row order preserved |
| `%##Gap` | `#False` | No gaps in row indices |
| `%##Level.2` | `##Record` | Columns are enum-keyed record fields |

The `<#Columns` parameter must satisfy `##Enum` -- column names come from an enum type. `<#CellType` defaults to `#` (any type). The level 2 `##Record` composition passes `<#Columns` as `<#Fields` and `<#CellType` as `<#ValueType`.

## Used By

<!-- @u:pglib/types/Dataframe -->

- [[Dataframe|#Dataframe]] type composes this schema

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Dataframe` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[Dataframe]] -- `#Dataframe` generic type composing ##Dataframe
- [[schemas/Record|##Record]] -- enum-keyed record used for columns (L2)
- [[schemas/Fields|%##Fields]] -- field descriptor property

