---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.Now"
metadata_instance: "%=:DT.Now:N"
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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.Now` | Compile-time pipeline template |
| Instance | `%=:DT.Now:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
