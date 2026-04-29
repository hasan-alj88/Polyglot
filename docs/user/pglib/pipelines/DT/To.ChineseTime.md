---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.ChineseTime"
metadata_instance: "%-:DT.To.ChineseTime:N"
---

# -DT.To.ChineseTime

Converts a `#dt` into Chinese traditional time units (shichen/ke/fen). These pipelines convert a `#dt` into culture-specific time representations.

## Definition

```aljam3
{N} -DT.To.ChineseTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToChineseTime"
   [%] .description << "DateTime to Chinese traditional time"
   (-) <source#dt
   (-) >time#ChineseTime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>time` | `#ChineseTime` | Chinese traditional time representation |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.ChineseTime` | Compile-time pipeline template |
| Instance | `%-:DT.To.ChineseTime:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
