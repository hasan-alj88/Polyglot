---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Reassign"
metadata_instance: "%Q:Job.Reassign:N"
---

# -Q.Job.Reassign

Move a Job to a different queue. Same-host reassignment is bookkeeping only. Cross-host reassignment serializes state, transfers, and restores automatically. Job ID stays the same.

## Definition

```polyglot
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

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Reassign` | Compile-time pipeline template |
| Instance | `%Q:Job.Reassign:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Snapshot|-Q.Job.Snapshot]]
- [[pglib/pipelines/Q/Queue.Drain|-Q.Queue.Drain]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
