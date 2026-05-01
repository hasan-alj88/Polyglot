---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Get.Day"
metadata_instance: "%-:DT.Get.Day:N"
---

# -DT.Get.Day

Extracts the day component from a `#dt` value.

## Definition

```aljam3
{N} -DT.Get.Day
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetDay"
   [%] .description << "Extract day from DateTime"
   (-) <source#dt
   (-) >day#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>day` | `#int` | Day component |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Get.Day` | Compile-time pipeline template |
| Instance | `%-:DT.Get.Day:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[jm3lib/types/datetime|DateTime types]]
