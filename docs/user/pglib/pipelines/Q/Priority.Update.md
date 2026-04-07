---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
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

## Related

- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
