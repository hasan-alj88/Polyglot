---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Japanese"
metadata_instance: "%-:DT.To.Japanese:N"
---

# -DT.To.Japanese

Projects a `#dt` value into a Japanese date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```aljam3
{N} -DT.To.Japanese
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToJapanese"
   [%] .description << "DateTime to Japanese date"
   (-) <source#dt
   (-) >japanese#JapaneseDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>japanese` | `#JapaneseDate` | Japanese date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Japanese` | Compile-time pipeline template |
| Instance | `%-:DT.To.Japanese:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[jm3lib/types/datetime|DateTime types]]
