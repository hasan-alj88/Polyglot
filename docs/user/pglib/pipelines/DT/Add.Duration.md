---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Add.Duration

Adds a `#Duration` (fixed time span) to a `#dt`.

## Definition

```polyglot
{N} =DT.Add.Duration
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtAddDuration"
   [%] .description << "Add duration to DateTime"
   [=] <source#dt
   [=] <duration#Duration
   [=] >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | Base DateTime |
| `<duration` | `#Duration` | --- | Fixed time span to add |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime after adding duration |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
