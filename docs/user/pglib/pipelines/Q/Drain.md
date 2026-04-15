---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Drain"
metadata_instance: "%Q:Drain:N"
---

# -Q.Drain

<!-- @d:pglib/pipelines/Q/Queue.Drain -->
**Deprecated.** Renamed to [[pglib/pipelines/Q/Queue.Drain|d:-Q.Queue.Drain]] under the scoped naming convention (`-Q.<Scope>.<Action>`).

---

*Original content preserved below for historical reference.*

Stop accepting new jobs, finish existing.

## Definition

```polyglot
{N} -Q.Drain
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QDrain"
   [%] .description << "Stop accepting new jobs, finish existing."
   (-) <queue#String
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<queue` | `#String` | Name of the queue to drain |

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Drain` | Compile-time pipeline template |
| Instance | `%Q:Drain:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Flush|-Q.Flush]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
