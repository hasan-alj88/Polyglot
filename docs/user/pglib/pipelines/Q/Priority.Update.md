---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Priority.Update"
metadata_instance: "%Q:Priority.Update:N"
---

# -Q.Priority.Update

Change a job's priority score.

## Definition

```polyglot
{N} -Q.Priority.Update
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QPriorityUpdate"
   [%] .description << "Change a job's priority score."
   (-) <jobId#String
   (-) <score#Int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<jobId` | `#String` | Identifier of the job to update |
| `<score` | `#Int` | New priority score |

## Outputs

None.

## Errors

None.

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates priority update condition, sends command signal |
| 2. NATS command | `polyglot.command.priority.update.{jobId}` | `{jobId, score}` |
| 3. QH executes | Queue Handler | ZADD queue:dispatch:{queue} {score} {jobId} (Priority queues only) |

No control signal to Runner. No Unix mechanism. Redis-only queue position update.

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Priority.Update` | Compile-time pipeline template |
| Instance | `%Q:Priority.Update:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
