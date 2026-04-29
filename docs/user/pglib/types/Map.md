---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Record"
metadata_instance: "%#:Record:N"
---

# Record (##Record)

<!-- @c:types -->

Enum-keyed collection with typed value fields. `##Record` is a parameterized schema that stamps one field per enum variant. Previously `#Map` / `##Map`.

---

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

The `<#Fields` parameter must satisfy `##Enum` -- field names come from an enum type. The `<#ValueType` parameter sets the value type for all fields (defaults to `#` -- any type). `##Flat` limits depth to one level. `%##Fields << <#Fields` stamps one child per enum variant.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` (via ##Flat) | One level of children |
| `%##Fields` | `<#Fields` | One child per enum variant |
| `%##Active` | `#ActiveKind.All` | All fields active simultaneously |
| `%###Type` | `<#ValueType` | Value type constraint |
| `%###Kind` | `#FieldKind.Value` | All children are value fields |

---

## Usage

Types that compose `##Record` use `:` positional binding for their `(#) <param` declarations:

```aljam3
{#} #DayOfWeek
   [#] ##Enum
   [#] ##Scalar
   [.] .Monday
   [.] .Tuesday
   [.] .Wednesday
   [.] .Thursday
   [.] .Friday
   [.] .Saturday
   [.] .Sunday

{#} #WeeklySchedule
   (#) <#Fields << #DayOfWeek
   (#) <#ValueType << #String
   [#] ##Record
      (#) <#Fields << <#Fields
      (#) <#ValueType << <#ValueType

[-] $schedule#WeeklySchedule <~ {}
```

Access uses `<` with enum variant names:

```aljam3
[-] $monday#string << $schedule<Monday
[-] $friday#string << $schedule<Friday
```

---

## Migration

`##Record` replaces the former `#Map` / `##Map` types. The key difference: `##Record` uses enum-keyed fields (`%##Fields << <#Fields`) instead of arbitrary sparse keys (`%##Key`). All fields are declared at compile time via the enum, making the structure fully type-safe.

| Former | Now |
|--------|-----|
| `#Map` / `##Map` | `##Record` |
| `%##Key` | `%##Fields` with enum ref |
| `%##Flexible` | Retired -- fields determined by enum |
| `%##Gap` | Retired -- all enum fields present |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Record` | Compile-time schema template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

---

## Related

- [[collections]] -- collection type overview
- [[Array]] -- range-indexed ordered collection (composes ##Array)
- [[schemas/Record|##Record]] -- schema definition
- [[schemas/Fields|%##Fields]] -- field descriptor property
- [[syntax/types/INDEX|types]] -- full type system specification

