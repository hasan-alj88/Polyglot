---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Add.Period

Adds a `#Period` (calendar-aware span such as "1 month") to a `#dt`.

## Definition

```polyglot
{N} =DT.Add.Period
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtAddPeriod"
   [%] .description << "Add calendar period to DateTime"
   [=] <source#dt
   [=] <period#Period
   [=] >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | Base DateTime |
| `<period` | `#Period` | --- | Calendar-aware span to add |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime after adding period |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
