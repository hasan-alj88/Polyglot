---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Add.Duration"
metadata_instance: "%-:DT.Add.Duration:N"
---

# -DT.Add.Duration

Adds a `#Duration` (fixed time span) to a `#dt`.

## Definition

```aljam3
{N} -DT.Add.Duration
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtAddDuration"
   [%] .description << "Add duration to DateTime"
   (-) <source#dt
   (-) <duration#Duration
   (-) >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | Base DateTime |
| `<duration` | `#Duration` | --- | Fixed time span to add |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime after adding duration |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Add.Duration` | Compile-time pipeline template |
| Instance | `%-:DT.Add.Duration:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
