---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Queue.Flush"
metadata_instance: "%Q:Queue.Flush:N"
---

# -Q.Queue.Flush

Kill.Now every Job on the queue. All Jobs immediately terminated with no cleanup.

## Definition

```aljam3
{N} -Q.Queue.Flush
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QQueueFlush"
   [%] .description << "Kill.Now every Job on the queue."
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
| 1. TM decides | Trigger Monitor | Evaluates flush condition, sends command signal |
| 2. NATS command | `aljam3.command.flush.{queue}` | `{queue}` |
| 3. QH executes | Queue Handler | FOR each job: remove from all sets/queues, DEL job hash, DEL queue, SREM queues:registered |
| 4. Control signal | `aljam3.queue.control.{jobId}.job.kill.now` | Per executing/teardown.executing job → Runner |
| 5. Unix mechanism | Runner | `SIGKILL` per executing job — immediate termination |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Queue.Flush` | Compile-time pipeline template |
| Instance | `%Q:Queue.Flush:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Q/Queue.Drain|-Q.Queue.Drain]]
- [[aj3lib/pipelines/Q/Job.Kill.Now|-Q.Job.Kill.Now]]
- [[aj3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
