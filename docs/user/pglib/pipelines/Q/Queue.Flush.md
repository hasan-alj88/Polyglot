---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Queue.Flush"
metadata_instance: "%Q:Queue.Flush:N"
---

# -Q.Queue.Flush

Kill.Now every Job on the queue. All Jobs immediately terminated with no cleanup.

## Definition

```polyglot
{N} -Q.Queue.Flush
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QQueueFlush"
   [%] .description << "Kill.Now every Job on the queue."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Queue.Flush` | Compile-time pipeline template |
| Instance | `%Q:Queue.Flush:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Queue.Drain|-Q.Queue.Drain]]
- [[pglib/pipelines/Q/Job.Kill.Now|-Q.Job.Kill.Now]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
