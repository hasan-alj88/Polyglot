---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Business.IsWorkDay"
metadata_instance: "%-:DT.Business.IsWorkDay:N"
---

# -DT.Business.IsWorkDay

Returns `#bool` -- true when `source` falls on a work day. Requires an explicit `#BusinessWeek` configuration. No regional defaults are assumed -- all fields (`.firstDay`, `.workDays`, `.offDays`, `.hoursPerDay`) must be set by the user.

## Definition

```aljam3
{N} -DT.Business.IsWorkDay
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessIsWorkDay"
   [%] .description << "Check if DateTime is a work day"
   (-) <source#dt
   (-) <week#BusinessWeek
   (-) >result#bool
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to check |
| `<week` | `#BusinessWeek` | --- | Business week configuration |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#bool` | True if `source` falls on a work day |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Business.IsWorkDay` | Compile-time pipeline template |
| Instance | `%-:DT.Business.IsWorkDay:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[aj3lib/types/datetime|DateTime types]]
