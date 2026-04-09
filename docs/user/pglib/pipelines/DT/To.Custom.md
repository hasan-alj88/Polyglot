---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Custom"
metadata_instance: "%-:DT.To.Custom:N"
---

# -DT.To.Custom

Projects a `#dt` into a user-supplied custom calendar. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} -DT.To.Custom
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToCustom"
   [%] .description << "DateTime to custom calendar date"
   (-) <source#dt
   (-) <calendar#CustomCalendar
   (-) >date#Date
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |
| `<calendar` | `#CustomCalendar` | --- | Custom calendar definition |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>date` | `#Date` | Date in the custom calendar |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Custom` | Compile-time pipeline template |
| Instance | `%-:DT.To.Custom:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
