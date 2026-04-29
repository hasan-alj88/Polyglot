---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Chinese"
metadata_instance: "%-:DT.To.Chinese:N"
---

# -DT.To.Chinese

Projects a `#dt` value into a Chinese date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```aljam3
{N} -DT.To.Chinese
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToChinese"
   [%] .description << "DateTime to Chinese date"
   (-) <source#dt
   (-) >chinese#ChineseDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>chinese` | `#ChineseDate` | Chinese date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Chinese` | Compile-time pipeline template |
| Instance | `%-:DT.To.Chinese:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
