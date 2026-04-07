---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.Coptic

Projects a `#dt` value into a Coptic date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} =DT.To.Coptic
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToCoptic"
   [%] .description << "DateTime to Coptic date"
   [=] <source#dt
   [=] >coptic#CopticDate
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
