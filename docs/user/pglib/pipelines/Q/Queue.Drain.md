---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Queue.Drain"
metadata_instance: "%Q:Queue.Drain:N"
---

# -Q.Queue.Drain

Stop accepting new Jobs into the queue. Existing Jobs continue until completion.

## Definition

```polyglot
{N} -Q.Queue.Drain
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QQueueDrain"
   [%] .description << "Stop accepting new Jobs into the queue."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Queue.Drain` | Compile-time pipeline template |
| Instance | `%Q:Queue.Drain:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Queue.Flush|-Q.Queue.Flush]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
