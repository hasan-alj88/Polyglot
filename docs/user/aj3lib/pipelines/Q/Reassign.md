---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Reassign"
metadata_instance: "%Q:Reassign:N"
---

# -Q.Reassign

<!-- @d:jm3lib/pipelines/Q/Job.Reassign -->
**Deprecated.** Renamed to [[jm3lib/pipelines/Q/Job.Reassign|d:-Q.Job.Reassign]] under the scoped naming convention (`-Q.<Scope>.<Action>`).

---

*Original content preserved below for historical reference.*

Move a job to a different queue. Enables host offloading.

## Definition

```aljam3
{N} -Q.Reassign
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QReassign"
   [%] .description << "Move a job to a different queue."
   (-) <jobId#String
   (-) <queue#String
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

- [[jm3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
