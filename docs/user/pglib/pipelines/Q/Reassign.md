---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Reassign"
metadata_instance: "%Q:Reassign:N"
---

# =Q.Reassign

Move a job to a different queue. Enables host offloading.

## Definition

```polyglot
{N} =Q.Reassign
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QReassign"
   [%] .description << "Move a job to a different queue."
   [=] <jobId#String
   [=] <queue#String
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<jobId` | `#String` | Identifier of the job to move |
| `<queue` | `#String` | Target queue name |

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Reassign` | Compile-time pipeline template |
| Instance | `%Q:Reassign:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
