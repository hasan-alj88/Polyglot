---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Format.Calendar

Formats a `#dt` as a string in a specific calendar system's conventional format.

## Definition

```polyglot
{N} =DT.Format.Calendar
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormatCalendar"
   [%] .description << "Format DateTime in calendar system format"
   [=] <source#dt
   [=] <system#CalendarSystem
   [=] >text#string
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
