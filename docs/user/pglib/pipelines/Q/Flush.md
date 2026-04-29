---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Flush"
metadata_instance: "%Q:Flush:N"
---

# -Q.Flush

<!-- @d:pglib/pipelines/Q/Queue.Flush -->
**Deprecated.** Renamed to [[pglib/pipelines/Q/Queue.Flush|d:-Q.Queue.Flush]] under the scoped naming convention (`-Q.<Scope>.<Action>`).

---

*Original content preserved below for historical reference.*

Remove all pending jobs from a queue.

## Definition

```aljam3
{N} -Q.Flush
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QFlush"
   [%] .description << "Remove all pending jobs from a queue."
   (-) <queue#String
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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Flush` | Compile-time pipeline template |
| Instance | `%Q:Flush:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Drain|-Q.Drain]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
