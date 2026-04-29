---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Hebrew"
metadata_instance: "%-:DT.To.Hebrew:N"
---

# -DT.To.Hebrew

Projects a `#dt` value into a Hebrew date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```aljam3
{N} -DT.To.Hebrew
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHebrew"
   [%] .description << "DateTime to Hebrew date"
   (-) <source#dt
   (-) >hebrew#HebrewDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>hebrew` | `#HebrewDate` | Hebrew date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Hebrew` | Compile-time pipeline template |
| Instance | `%-:DT.To.Hebrew:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
