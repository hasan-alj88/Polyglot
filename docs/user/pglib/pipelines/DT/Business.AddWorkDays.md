---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.Business.AddWorkDays"
metadata_instance: "%=:DT.Business.AddWorkDays:N"
---

# =DT.Business.AddWorkDays

Adds `N` work days to a `#dt`, skipping non-work days. Requires an explicit `#BusinessWeek` configuration. No regional defaults are assumed -- all fields (`.firstDay`, `.workDays`, `.offDays`, `.hoursPerDay`) must be set by the user.

## Definition

```polyglot
{N} =DT.Business.AddWorkDays
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessAddWorkDays"
   [%] .description << "Add work days to DateTime"
   [=] <source#dt
   [=] <days#int
   [=] <week#BusinessWeek
   [=] >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | Starting DateTime |
| `<days` | `#int` | --- | Number of work days to add |
| `<week` | `#BusinessWeek` | --- | Business week configuration |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime after adding N work days |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.Business.AddWorkDays` | Compile-time pipeline template |
| Instance | `%=:DT.Business.AddWorkDays:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
