---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Pause.Free.CPU"
metadata_instance: "%Q:Job.Pause.Free.CPU:N"
---

# -Q.Job.Pause.Free.CPU

Pause a Job, freeing CPU. Available in `.Now` (immediate) and `.Wait` (after current work unit) timing variants.

## Definition

```aljam3
{N} -Q.Job.Pause.Free.CPU
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobPauseFreeCPU"
   [%] .description << "Pause a Job, freeing CPU."
```

## Timing Variants

- `.Now` — immediate freeze. Job may be mid-operation.
- `.Wait` — waits for current work unit boundary before freezing.

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

| Resource | Effect |
|----------|--------|
| CPU | Freed |
| RAM | Kept |
| FDs | Kept |
| TCP | Kept |
| Locks | Kept |

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates `#JobRules` condition, sends command signal |
| 2. NATS command | `aljam3.command.job.pause.free.cpu.{jobId}` | `{jobId, timing: "now"\|"wait"}` |
| 3. QH executes | Queue Handler | SREM set:running, HSET set:suspended "cpu", decrement counters, HSET job status "suspended.cpu" |
| 4. Control signal | `aljam3.queue.control.{jobId}.job.pause.free.cpu` | `{jobId, timing}` → Runner |
| 5. Unix mechanism | Runner | .Now: `echo 1 > cgroup.freeze` (atomic freeze); .Wait: `SIGSTOP` after work unit boundary |
| 6. Runner ACK | `aljam3.runner.paused.{jobId}` | `{type: "cpu"}` → QH + TM |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Pause.Free.CPU` | Compile-time pipeline template |
| Instance | `%Q:Job.Pause.Free.CPU:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Q/Job.Pause.Free.RAM|-Q.Job.Pause.Free.RAM]]
- [[jm3lib/pipelines/Q/Job.Pause.Free.All|-Q.Job.Pause.Free.All]]
- [[jm3lib/pipelines/Q/Job.Resume|-Q.Job.Resume]]
- [[jm3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
