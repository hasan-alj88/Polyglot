---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Kill.WithCleanup"
metadata_instance: "%Q:Job.Kill.WithCleanup:N"
---

# -Q.Job.Kill.WithCleanup

Terminate a Job after running `[/]` wrapper cleanup. Terminal — Job state destroyed after cleanup.

## Definition

```aljam3
{N} -Q.Job.Kill.WithCleanup
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobKillWithCleanup"
   [%] .description << "Terminate a Job after running cleanup."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

Everything freed after cleanup runs. The Job enters the Dying state during `[/]` cleanup, then transitions to Dead.

| Resource | Effect |
|----------|--------|
| CPU | Freed after cleanup |
| RAM | Freed after cleanup |
| FDs | Freed after cleanup |
| TCP | Freed after cleanup |
| Locks | Freed after cleanup |

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates kill condition, sends command signal |
| 2. NATS command | `aljam3.command.job.kill.with-cleanup.{jobId}` | `{jobId}` |
| 3. QH executes | Queue Handler | Status-aware: remove from current set/queue, RPUSH queue:teardown, HSET job status "teardown.pending" |
| 4. DC dispatches | Dispatch Coordinator | Picks job from teardown queue, sends control signal |
| 5. Control signal | `aljam3.queue.control.{jobId}.job.kill.with-cleanup` | `{jobId}` → Runner |
| 6. Unix mechanism | Runner | `SIGTERM` → `[/]` cleanup runs → `SIGKILL` on timeout |
| 7. Runner ACK | `aljam3.runner.teardown_completed.{jobId}` | `{jobId, pipeline}` → QH + TM |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Kill.WithCleanup` | Compile-time pipeline template |
| Instance | `%Q:Job.Kill.WithCleanup:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Kill.Now|-Q.Job.Kill.Now]]
- [[pglib/pipelines/Q/Job.Pause.Free.CPU|-Q.Job.Pause.Free.CPU]] (non-terminal alternative)
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
