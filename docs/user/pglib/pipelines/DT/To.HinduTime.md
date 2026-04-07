---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.HinduTime

Converts a `#dt` into Hindu traditional time units (prahara/muhurta). These pipelines convert a `#dt` into culture-specific time representations.

## Definition

```polyglot
{N} =DT.To.HinduTime
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHinduTime"
   [%] .description << "DateTime to Hindu traditional time"
   [=] <source#dt
   [=] >time#HinduTime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>time` | `#HinduTime` | Hindu traditional time representation |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
