---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Drain"
metadata_instance: "%Q:Drain:N"
---

# -Q.Drain

Stop accepting new jobs, finish existing.

## Definition

```polyglot
{N} -Q.Drain
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QDrain"
   [%] .description << "Stop accepting new jobs, finish existing."
   (-) <queue#String
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<queue` | `#String` | Name of the queue to drain |

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Drain` | Compile-time pipeline template |
| Instance | `%Q:Drain:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Flush|-Q.Flush]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
