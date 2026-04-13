---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Get.Month"
metadata_instance: "%-:DT.Get.Month:N"
---

# -DT.Get.Month

Extracts the month component from a `#dt` value.

## Definition

```polyglot
{N} -DT.Get.Month
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetMonth"
   [%] .description << "Extract month from DateTime"
   (-) <source#dt
   (-) >month#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>month` | `#int` | Month component |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Get.Month` | Compile-time pipeline template |
| Instance | `%-:DT.Get.Month:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
