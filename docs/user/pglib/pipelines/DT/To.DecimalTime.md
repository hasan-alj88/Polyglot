---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.DecimalTime

Converts a `#dt` to French Republican decimal time. These pipelines convert a `#dt` into culture-specific time representations.

## Definition

```polyglot
{N} =DT.To.DecimalTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToDecimalTime"
   [%] .description << "DateTime to decimal time"
   [=] <source#dt
   [=] >time#DecimalTime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>time` | `#DecimalTime` | French Republican decimal time representation |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
