---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Ethiopian"
metadata_instance: "%-:DT.To.Ethiopian:N"
---

# -DT.To.Ethiopian

Projects a `#dt` value into an Ethiopian date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```aljam3
{N} -DT.To.Ethiopian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToEthiopian"
   [%] .description << "DateTime to Ethiopian date"
   (-) <source#dt
   (-) >ethiopian#EthiopianDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>ethiopian` | `#EthiopianDate` | Ethiopian date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Ethiopian` | Compile-time pipeline template |
| Instance | `%-:DT.To.Ethiopian:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[jm3lib/types/datetime|DateTime types]]
