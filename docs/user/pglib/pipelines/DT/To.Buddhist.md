---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.To.Buddhist"
metadata_instance: "%=:DT.To.Buddhist:N"
---

# =DT.To.Buddhist

Projects a `#dt` value into a Buddhist date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} =DT.To.Buddhist
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToBuddhist"
   [%] .description << "DateTime to Buddhist date"
   [=] <source#dt
   [=] >buddhist#BuddhistDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>buddhist` | `#BuddhistDate` | Buddhist date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.To.Buddhist` | Compile-time pipeline template |
| Instance | `%=:DT.To.Buddhist:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
