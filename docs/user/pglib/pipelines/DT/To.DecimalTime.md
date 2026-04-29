---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.DecimalTime"
metadata_instance: "%-:DT.To.DecimalTime:N"
---

# -DT.To.DecimalTime

Converts a `#dt` to French Republican decimal time. These pipelines convert a `#dt` into culture-specific time representations.

## Definition

```aljam3
{N} -DT.To.DecimalTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToDecimalTime"
   [%] .description << "DateTime to decimal time"
   (-) <source#dt
   (-) >time#DecimalTime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>time` | `#DecimalTime` | French Republican decimal time representation |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.DecimalTime` | Compile-time pipeline template |
| Instance | `%-:DT.To.DecimalTime:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
