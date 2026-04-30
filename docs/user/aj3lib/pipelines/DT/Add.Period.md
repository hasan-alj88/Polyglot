---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Add.Period"
metadata_instance: "%-:DT.Add.Period:N"
---

# -DT.Add.Period

Adds a `#Period` (calendar-aware span such as "1 month") to a `#dt`.

## Definition

```aljam3
{N} -DT.Add.Period
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtAddPeriod"
   [%] .description << "Add calendar period to DateTime"
   (-) <source#dt
   (-) <period#Period
   (-) >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | Base DateTime |
| `<period` | `#Period` | --- | Calendar-aware span to add |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime after adding period |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Add.Period` | Compile-time pipeline template |
| Instance | `%-:DT.Add.Period:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[aj3lib/types/datetime|DateTime types]]
