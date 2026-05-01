---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Inspect"
metadata_instance: "%Q:Job.Inspect:N"
---

# -Q.Job.Inspect

Read a Job's runtime state without affecting it. Observation-only — no state changes.

## Definition

```aljam3
{N} -Q.Job.Inspect
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobInspect"
   [%] .description << "Read a Job's runtime state without affecting it."
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
| 1. TM decides | Trigger Monitor | Evaluates inspect condition, sends command signal |
| 2. NATS command | `aljam3.command.job.inspect.{jobId}` | `{jobId}` |
| 3. QH executes | Queue Handler | Read-only: HGETALL job:{jobId} — no state mutation |
| 4. State output | `aljam3.state.job.{jobId}.inspected` | Full job hash contents returned to TM |

No control signal to Runner. No Unix mechanism. Observation-only operation.

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Inspect` | Compile-time pipeline template |
| Instance | `%Q:Job.Inspect:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
