---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Pause.Free.RAM"
metadata_instance: "%Q:Job.Pause.Free.RAM:N"
---

# -Q.Job.Pause.Free.RAM

Pause a Job, freeing CPU and RAM. Available in `.Soft` (best-effort) and `.Hard` (guaranteed) precision, each with `.Now`/`.Wait` timing.

## Definition

```polyglot
{N} -Q.Job.Pause.Free.RAM
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobPauseFreeRAM"
   [%] .description << "Pause a Job, freeing CPU and RAM."
```

## Precision Variants

- `.Soft` — best-effort RAM reclaim. The runtime hints that RAM should be freed, but reclamation is not guaranteed.
- `.Hard` — guaranteed RAM freed. Risk of forced termination if RAM cannot be freed.

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
| RAM | Freed (Soft: best-effort; Hard: guaranteed) |
| FDs | Kept |
| TCP | Kept |
| Locks | Kept |

## Compiler Warnings

`Free.RAM.Hard` emits a compiler warning about forced termination risk. If no `[/]` cleanup wrapper exists, the warning escalates to a compile error.

## Runtime Behavior

### `.Soft` variant

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates `#JobRules` condition, sends command signal |
| 2. NATS command | `polyglot.command.job.pause.free.ram.soft.{jobId}` | `{jobId, timing: "now"\|"wait"}` |
| 3. QH executes | Queue Handler | SREM set:executing, HSET set:suspended "ram.soft", decrement counters, HSET job status "suspended.ram.soft" |
| 4. Control signal | `polyglot.queue.control.{jobId}.job.pause.free.ram.soft` | `{jobId, timing}` → Runner |
| 5. Unix mechanism | Runner | `cgroup.freeze` + `echo {limit} > memory.high` (kernel hint, best-effort swap) |
| 6. Runner ACK | `polyglot.runner.paused.{jobId}` | `{type: "ram.soft"}` → QH + TM |

### `.Hard` variant

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates `#JobRules` condition, sends command signal |
| 2. NATS command | `polyglot.command.job.pause.free.ram.hard.{jobId}` | `{jobId, timing: "now"\|"wait"}` |
| 3. QH executes | Queue Handler | SREM set:executing, HSET set:suspended "ram.hard", decrement counters, HSET job status "suspended.ram.hard" |
| 4. Control signal | `polyglot.queue.control.{jobId}.job.pause.free.ram.hard` | `{jobId, timing}` → Runner |
| 5. Unix mechanism | Runner | `cgroup.freeze` + `echo {limit} > memory.max` (hard cap, OOM-kill risk) |
| 6. Runner ACK | `polyglot.runner.paused.{jobId}` | `{type: "ram.hard"}` → QH + TM |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Pause.Free.RAM` | Compile-time pipeline template |
| Instance | `%Q:Job.Pause.Free.RAM:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Pause.Free.CPU|-Q.Job.Pause.Free.CPU]]
- [[pglib/pipelines/Q/Job.Pause.Free.All|-Q.Job.Pause.Free.All]]
- [[pglib/pipelines/Q/Job.Resume|-Q.Job.Resume]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
