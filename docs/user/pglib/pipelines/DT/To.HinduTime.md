---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.HinduTime"
metadata_instance: "%-:DT.To.HinduTime:N"
---

# -DT.To.HinduTime

Converts a `#dt` into Hindu traditional time units (prahara/muhurta). These pipelines convert a `#dt` into culture-specific time representations.

## Definition

```aljam3
{N} -DT.To.HinduTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHinduTime"
   [%] .description << "DateTime to Hindu traditional time"
   (-) <source#dt
   (-) >time#HinduTime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>time` | `#HinduTime` | Hindu traditional time representation |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.HinduTime` | Compile-time pipeline template |
| Instance | `%-:DT.To.HinduTime:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
