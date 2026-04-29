---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Format"
metadata_instance: "%-:DT.Format:N"
---

# -DT.Format

Formats a `#dt` using a pattern string (e.g. `"YYYY-MM-DD HH:mm:ss"`).

## Definition

```aljam3
{N} -DT.Format
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormat"
   [%] .description << "Format DateTime with pattern"
   (-) <source#dt
   (-) <pattern#string
   (-) >text#string
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to format |
| `<pattern` | `#string` | --- | Format pattern string |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>text` | `#string` | Formatted date-time string |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Format` | Compile-time pipeline template |
| Instance | `%-:DT.Format:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
