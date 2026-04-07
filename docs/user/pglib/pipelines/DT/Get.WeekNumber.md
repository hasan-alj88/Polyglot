---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.Get.WeekNumber"
metadata_instance: "%=:DT.Get.WeekNumber:N"
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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.Get.WeekNumber` | Compile-time pipeline template |
| Instance | `%=:DT.Get.WeekNumber:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
