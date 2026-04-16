---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Throttle"
metadata_instance: "%Q:Job.Throttle:N"
---

# -Q.Job.Throttle

Reduce a running Job's resource allocation. Job keeps running with reduced CPU/RAM/IO limits. Use `-Q.Job.Unthrottle` to restore full allocation.

## Definition

```polyglot
{N} -Q.Job.Throttle
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobThrottle"
   [%] .description << "Reduce a running Job's resource allocation."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

The Job keeps running. CPU, RAM, and IO allocation are reduced. No state is lost.

| Resource | Effect |
|----------|--------|
| CPU | Reduced allocation |
| RAM | Reduced allocation |
| IO | Reduced allocation |
| FDs | Kept |
| TCP | Kept |
| Locks | Kept |

## Unthrottle

`-Q.Job.Unthrottle` removes all throttling limits and restores full resource allocation. The Job continues running without interruption.

## Runtime Behavior

### Throttle

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates `#JobRules` condition, sends throttle command |
| 2. NATS command | `polyglot.command.job.throttle.{jobId}` | `{jobId, cpu?, memory?, io?}` |
| 3. QH executes | Queue Handler | HSET job status "executing.throttled", throttled true, throttle_config {cpu, memory, io} |
| 4. Control signal | `polyglot.queue.control.{jobId}.job.throttle` | `{jobId, cpu, memory, io}` → Runner |
| 5. Unix mechanism | Runner | `echo {quota} {period} > cpu.max`, `echo {bytes} > memory.high`, `echo {limits} > io.max` |

### Unthrottle

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates condition cleared, sends unthrottle command |
| 2. NATS command | `polyglot.command.job.unthrottle.{jobId}` | `{jobId}` |
| 3. QH executes | Queue Handler | HSET job status "executing", throttled false, HDEL throttle_config |
| 4. Control signal | `polyglot.queue.control.{jobId}.job.unthrottle` | `{jobId}` → Runner |
| 5. Unix mechanism | Runner | Remove cgroup limits (restore `cpu.max`, `memory.high`, `io.max` to defaults) |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Throttle` | Compile-time pipeline template |
| Instance | `%Q:Job.Throttle:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Pause.Free.CPU|-Q.Job.Pause.Free.CPU]] (full CPU stop)
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
