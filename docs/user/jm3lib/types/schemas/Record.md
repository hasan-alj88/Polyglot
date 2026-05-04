---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
metadata_definition: "%definition.##:Record"
---

# ##Record Schema (Parameterized)

<!-- @c:types -->

`##Record` is a parameterized schema that describes an enum-keyed, all-active, value-typed structure. The compiler reads the enum's variants and stamps one `:` child per variant.

## Allows

```
#WeeklyHours;Record:#DayOfWeek,#int
├── :Monday     -> 8       ← every #DayOfWeek variant present
├── :Tuesday    -> 8         all active (%##Active << #All)
├── :Wednesday  -> 6         all values #int (%###Type)
├── :Thursday   -> 8
├── :Friday     -> 7
├── :Saturday   -> 0
└── :Sunday     -> 0
```

## Disallows

```
#WeeklyHours;Record:#DayOfWeek,#int
├── :Monday     -> 8
├── :Tuesday    -> 8
└── :Friday     -> 7
                            ✗ missing Wed/Thu/Sat/Sun
                              %##Active << #All requires every variant

#WeeklyHours;Record:#DayOfWeek,#int
├── :Monday     -> 8
├── :Tuesday    -> "eight"#string  ✗ #string — %###Type requires #int
└── ...

#WeeklyHours;Record:#DayOfWeek,#int
├── :Monday     -> 8
├── :Funday     -> 10       ✗ not a #DayOfWeek variant
└── ...                       %##Fields << #DayOfWeek
```

## Definition

```aljam3
{#} ##Record
   (#) <#Fields << ##Enum
   (#) <#ValueType <~ #
   [#] ##Flat
   [#] %##Fields << <#Fields
   [#] %##Active << #ActiveKind.All
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` (via ##Flat) | One level of children |
| `%##Fields` | `<#Fields` | Enum-keyed children |
| `%##Active` | `#ActiveKind.All` | Every variant must be present |
| `%###Type` | `<#ValueType` | Uniform value type (default `#` = any) |
| `%###Kind` | `#FieldKind.Value` | All leaves are typed data |

## Used By

User-defined record types compose this schema. `##Dataframe` uses `##Record` for its L2 column structure.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Record` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Array|##Array]] -- range-indexed alternative
- [[schemas/Dataframe|##Dataframe]] -- uses ##Record for L2 columns
- [[properties/Fields|%##Fields]] -- field descriptor property
- [[properties/Active|%##Active]] -- branch presence property
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
