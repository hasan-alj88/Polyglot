---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Persian"
metadata_instance: "%-:DT.To.Persian:N"
---

# -DT.To.Persian

Projects a `#dt` value into a Persian date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} -DT.To.Persian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToPersian"
   [%] .description << "DateTime to Persian date"
   (-) <source#dt
   (-) >persian#PersianDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>persian` | `#PersianDate` | Persian date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Persian` | Compile-time pipeline template |
| Instance | `%-:DT.To.Persian:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
