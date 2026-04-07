---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =Q.Flush

Remove all pending jobs from a queue.

## Definition

```polyglot
{N} =Q.Flush
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QFlush"
   [%] .description << "Remove all pending jobs from a queue."
   [=] <queue#String
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<queue` | `#String` | Name of the queue to flush |

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Related

- [[pglib/pipelines/Q/Drain|=Q.Drain]]
- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
