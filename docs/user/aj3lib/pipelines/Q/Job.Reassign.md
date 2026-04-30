---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Reassign"
metadata_instance: "%Q:Job.Reassign:N"
---

# -Q.Job.Reassign

Move a Job to a different queue. Same-host reassignment is bookkeeping only. Cross-host reassignment serializes state, transfers, and restores automatically. Job ID stays the same.

## Definition

```aljam3
{N} -Q.Job.Reassign
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobReassign"
   [%] .description << "Move a Job to a different queue."
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Queue` | `#Queue` | Target queue to reassign the Job to |

## Outputs

None.

## Errors

None.

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Evaluates reassign condition, sends command signal |
| 2. NATS command | `aljam3.command.reassign.{jobId}` | `{jobId, fromQueue, toQueue, priority?}` |
| 3. QH executes | Queue Handler | Lua script: LREM/ZREM from source queue, RPUSH/ZADD to target queue, HSET job queue |
| 4. Same-host | — | Bookkeeping only — no Runner involvement |
| 5. Cross-host | Runner | `criu dump` → image transfer (TCP/shared storage) → `criu restore` on target host |

No control signal for same-host reassignment. Cross-host reassignment uses CRIU image transfer handled by the Runner.

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Reassign` | Compile-time pipeline template |
| Instance | `%Q:Job.Reassign:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Q/Job.Snapshot|-Q.Job.Snapshot]]
- [[aj3lib/pipelines/Q/Queue.Drain|-Q.Queue.Drain]]
- [[aj3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
