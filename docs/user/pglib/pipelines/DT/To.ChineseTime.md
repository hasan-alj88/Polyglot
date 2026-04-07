---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.ChineseTime

Converts a `#dt` into Chinese traditional time units (shichen/ke/fen). These pipelines convert a `#dt` into culture-specific time representations.

## Definition

```polyglot
{N} =DT.To.ChineseTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToChineseTime"
   [%] .description << "DateTime to Chinese traditional time"
   [=] <source#dt
   [=] >time#ChineseTime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>time` | `#ChineseTime` | Chinese traditional time representation |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
