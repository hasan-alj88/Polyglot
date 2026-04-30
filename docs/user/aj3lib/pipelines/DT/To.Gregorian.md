---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Gregorian"
metadata_instance: "%-:DT.To.Gregorian:N"
---

# -DT.To.Gregorian

Projects a `#dt` value into a Gregorian date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```aljam3
{N} -DT.To.Gregorian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToGregorian"
   [%] .description << "DateTime to Gregorian date"
   (-) <source#dt
   (-) >gregorian#GregorianDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>gregorian` | `#GregorianDate` | Gregorian date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Gregorian` | Compile-time pipeline template |
| Instance | `%-:DT.To.Gregorian:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[aj3lib/types/datetime|DateTime types]]
