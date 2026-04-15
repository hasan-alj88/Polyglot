---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Snapshot"
metadata_instance: "%Q:Job.Snapshot:N"
---

# -Q.Job.Snapshot

Create a point-in-time state fork to disk. The original Job keeps running. The snapshot can spawn a second instance (fork).

## Definition

```polyglot
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

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Snapshot` | Compile-time pipeline template |
| Instance | `%Q:Job.Snapshot:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Pause.Free.All|-Q.Job.Pause.Free.All]]
- [[pglib/pipelines/Q/Job.Reassign|-Q.Job.Reassign]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
