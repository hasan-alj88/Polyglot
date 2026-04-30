---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Format.Calendar"
metadata_instance: "%-:DT.Format.Calendar:N"
---

# -DT.Format.Calendar

Formats a `#dt` as a string in a specific calendar system's conventional format.

## Definition

```aljam3
{N} -DT.Format.Calendar
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormatCalendar"
   [%] .description << "Format DateTime in calendar system format"
   (-) <source#dt
   (-) <system#CalendarSystem
   (-) >text#string
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to format |
| `<system` | `#CalendarSystem` | --- | Calendar system for formatting |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>text` | `#string` | Formatted string in the calendar system's convention |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Format.Calendar` | Compile-time pipeline template |
| Instance | `%-:DT.Format.Calendar:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[aj3lib/types/datetime|DateTime types]]
