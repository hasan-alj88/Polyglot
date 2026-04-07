---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Priority.Update"
metadata_instance: "%Q:Priority.Update:N"
---

# =Q.Priority.Update

Change a job's priority score.

## Definition

```polyglot
{N} =Q.Priority.Update
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QPriorityUpdate"
   [%] .description << "Change a job's priority score."
   [=] <jobId#String
   [=] <score#Int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<jobId` | `#String` | Identifier of the job to update |
| `<score` | `#Int` | New priority score |

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Priority.Update` | Compile-time pipeline template |
| Instance | `%Q:Priority.Update:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
