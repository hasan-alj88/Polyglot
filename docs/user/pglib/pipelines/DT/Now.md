---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Now

Returns the current instant from the system clock.

## Definition

```polyglot
{N} =DT.Now
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtNow"
   [%] .description << "Current instant from system clock"
   [=] >dt#dt
   [ ] Requires System.Env capability (reads system clock)
```

## Inputs

None.

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>dt` | `#dt` | Current instant from system clock |

## Errors

None.

## Permissions

Requires `System.Env` capability -- reads system clock.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
