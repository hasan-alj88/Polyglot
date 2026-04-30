---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Format.ISO"
metadata_instance: "%-:DT.Format.ISO:N"
---

# -DT.Format.ISO

Formats a `#dt` as an ISO-8601 string.

## Definition

```aljam3
{N} -DT.Format.ISO
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormatIso"
   [%] .description << "Format DateTime as ISO-8601"
   (-) <source#dt
   (-) >text#string
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to format |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>text` | `#string` | ISO-8601 formatted string |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Format.ISO` | Compile-time pipeline template |
| Instance | `%-:DT.Format.ISO:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[aj3lib/types/datetime|DateTime types]]
