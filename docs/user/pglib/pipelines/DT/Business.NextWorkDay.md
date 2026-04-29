---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Business.NextWorkDay"
metadata_instance: "%-:DT.Business.NextWorkDay:N"
---

# -DT.Business.NextWorkDay

Returns the next work day as a `#dt`. Requires an explicit `#BusinessWeek` configuration. No regional defaults are assumed -- all fields (`.firstDay`, `.workDays`, `.offDays`, `.hoursPerDay`) must be set by the user.

## Definition

```aljam3
{N} -DT.Business.NextWorkDay
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessNextWorkDay"
   [%] .description << "Find next work day"
   (-) <source#dt
   (-) <week#BusinessWeek
   (-) >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | Starting DateTime |
| `<week` | `#BusinessWeek` | --- | Business week configuration |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | Next work day DateTime |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Business.NextWorkDay` | Compile-time pipeline template |
| Instance | `%-:DT.Business.NextWorkDay:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
