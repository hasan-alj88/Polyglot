---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Coptic"
metadata_instance: "%-:DT.To.Coptic:N"
---

# -DT.To.Coptic

Projects a `#dt` value into a Coptic date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```aljam3
{N} -DT.To.Coptic
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToCoptic"
   [%] .description << "DateTime to Coptic date"
   (-) <source#dt
   (-) >coptic#CopticDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>coptic` | `#CopticDate` | Coptic date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Coptic` | Compile-time pipeline template |
| Instance | `%-:DT.To.Coptic:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[jm3lib/types/datetime|DateTime types]]
