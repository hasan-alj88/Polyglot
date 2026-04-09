---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Fields"
---

# ##Fields Schema (Parameterized)

<!-- @types -->

`##Fields` is a parameterized schema that stamps enum variants as `[.]` fixed fields on a type. It takes an enum type as input and generates one `.` field per variant.

## Definition

```polyglot
{#} ##Fields
   [#] <#Type << ##Enum
```

The `<#Type` parameter must satisfy `##Enum` -- only enum types can be used to generate fields.

## Usage

```polyglot
{#} #DaySchedule
   [#] << ##Fields
      [#] <#Type << #DayOfWeek
   [ ] Generates .Monday, .Tuesday, ... .Sunday as fixed fields
```

Each variant of the input enum becomes a `.` field on the consuming type. This replaces manual field declaration when the field set mirrors an enum.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Fields` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Enum|##Enum]] -- input constraint for `<#Type`
- [[syntax/types/schema-properties|Schema Properties]] -- parameterized schema overview
