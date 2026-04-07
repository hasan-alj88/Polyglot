---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Business.IsWorkDay

Returns `#bool` -- true when `source` falls on a work day. Requires an explicit `#BusinessWeek` configuration. No regional defaults are assumed -- all fields (`.firstDay`, `.workDays`, `.offDays`, `.hoursPerDay`) must be set by the user.

## Definition

```polyglot
{N} =DT.Business.IsWorkDay
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtBusinessIsWorkDay"
   [%] .description << "Check if DateTime is a work day"
   [=] <source#dt
   [=] <week#BusinessWeek
   [=] >result#bool
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
