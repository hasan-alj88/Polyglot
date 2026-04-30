---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Get.Weekday"
metadata_instance: "%-:DT.Get.Weekday:N"
---

# -DT.Get.Weekday

Returns a `#Weekday` enum value from a `#dt`.

## Definition

```aljam3
{N} -DT.Get.Weekday
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetWeekday"
   [%] .description << "Extract weekday from DateTime"
   (-) <source#dt
   (-) >weekday#Weekday
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>weekday` | `#Weekday` | Weekday enum value |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Get.Weekday` | Compile-time pipeline template |
| Instance | `%-:DT.Get.Weekday:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[aj3lib/types/datetime|DateTime types]]
