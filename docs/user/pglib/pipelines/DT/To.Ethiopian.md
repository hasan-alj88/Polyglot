---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.Ethiopian

Projects a `#dt` value into an Ethiopian date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} =DT.To.Ethiopian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToEthiopian"
   [%] .description << "DateTime to Ethiopian date"
   [=] <source#dt
   [=] >ethiopian#EthiopianDate
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
