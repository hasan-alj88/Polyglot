---
audience: automation-builder
type: specification
updated: 2026-04-16
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

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates drain condition, sends command signal |
| 2. NATS command | `polyglot.command.drain.{queue}` | `{queue}` |
| 3. QH executes | Queue Handler | SADD queues:draining {queue} |

No control signal to Runner. No Unix mechanism. Subsequent `command.enqueue` to this queue is rejected. Existing jobs continue normally.

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

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
