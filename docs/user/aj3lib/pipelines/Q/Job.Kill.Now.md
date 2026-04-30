---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Kill.Now"
metadata_instance: "%Q:Job.Kill.Now:N"
---

# -Q.Job.Kill.Now

Immediately terminate a Job with no cleanup. Terminal — Job state destroyed instantly.

## Definition

```aljam3
{N} -Q.Job.Kill.Now
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobKillNow"
   [%] .description << "Immediately terminate a Job with no cleanup."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

Everything freed immediately. No `[/]` cleanup runs. The Job goes directly to Dead state.

| Resource | Effect |
|----------|--------|
| CPU | Freed immediately |
| RAM | Freed immediately |
| FDs | Freed immediately |
| TCP | Freed immediately |
| Locks | Freed immediately |

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates kill condition, sends command signal |
| 2. NATS command | `aljam3.command.job.kill.now.{jobId}` | `{jobId}` |
| 3. QH executes | Queue Handler | Status-aware: remove from current set/queue, DEL job:{jobId} |
| 4. Control signal | `aljam3.queue.control.{jobId}.job.kill.now` | `{jobId}` → Runner (if was executing/teardown.executing) |
| 5. Unix mechanism | Runner | `SIGKILL` — immediate termination, no cleanup |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Kill.Now` | Compile-time pipeline template |
| Instance | `%Q:Job.Kill.Now:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Q/Job.Kill.WithCleanup|-Q.Job.Kill.WithCleanup]]
- [[aj3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
