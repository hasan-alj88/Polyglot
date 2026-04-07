---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.Gregorian

Projects a `#dt` value into a Gregorian date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} =DT.To.Gregorian
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToGregorian"
   [%] .description << "DateTime to Gregorian date"
   [=] <source#dt
   [=] >gregorian#GregorianDate
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
