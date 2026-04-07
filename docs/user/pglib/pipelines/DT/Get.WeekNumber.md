---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Get.WeekNumber

Returns the week-of-year as `#int`. The `<system` input selects the week numbering system; defaults to `#WeekSystem.ISO`.

## Definition

```polyglot
{N} =DT.Get.WeekNumber
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetWeekNumber"
   [%] .description << "Extract week number from DateTime"
   [=] <source#dt
   [=] <system#WeekSystem <~ #WeekSystem.ISO
   [=] >week#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |
| `<system` | `#WeekSystem` | `#WeekSystem.ISO` | Week numbering system |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>week` | `#int` | Week-of-year number |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
