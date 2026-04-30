---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Snapshot"
metadata_instance: "%Q:Job.Snapshot:N"
---

# -Q.Job.Snapshot

Create a point-in-time state fork to disk. The original Job keeps running. The snapshot can spawn a second instance (fork).

## Definition

```aljam3
{N} -Q.Job.Snapshot
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobSnapshot"
   [%] .description << "Create a point-in-time state fork to disk."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Compiler Safety Rules

- Compile error if held locks at snapshot point (inconsistent restore point).
- Compile error if non-repairable TCP connections are open.
- Compile error if non-idempotent side effects are present unless wrapped in an idempotency guard.

## Fork Semantics

- The forked instance gets a new Job ID. The original keeps its ID.
- The forked instance starts with zero locks and must re-acquire any it needs.
- Process identity is distinct. There is no shared mutable state between original and fork.

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates snapshot condition, sends command signal |
| 2. NATS command | `aljam3.command.job.snapshot.{jobId}` | `{jobId, targetQueue?}` |
| 3. QH executes | Queue Handler | Generate forkId, enqueue fork in targetQueue, HSET fork job hash with forked_from |
| 4. Control signal | `aljam3.queue.control.{jobId}.job.snapshot` | `{jobId, forkId}` → Runner |
| 5. Unix mechanism | Runner | `criu dump --leave-running` (original continues, fork images saved to disk) |
| 6. Runner ACK | `aljam3.runner.snapshot_completed.{jobId}` | `{jobId, forkId, images_dir}` → QH stores images_dir on fork job hash |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Snapshot` | Compile-time pipeline template |
| Instance | `%Q:Job.Snapshot:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Q/Job.Pause.Free.All|-Q.Job.Pause.Free.All]]
- [[aj3lib/pipelines/Q/Job.Reassign|-Q.Job.Reassign]]
- [[aj3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
