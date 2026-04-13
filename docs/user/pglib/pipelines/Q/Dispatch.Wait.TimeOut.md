---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Dispatch.Wait.TimeOut"
metadata_instance: "%Q:Dispatch.Wait.TimeOut:N"
---

# -Q.Dispatch.Wait.TimeOut

What happens when a job exceeds `.maxWaitTime` in the queue. Default behavior: escalate to max priority.

## Definition

```polyglot
{N} -Q.Dispatch.Wait.TimeOut
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QDispatchWaitTimeOut"
   [%] .description << "Dispatch wait timeout — escalate to max priority."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Variants

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `-Q.Dispatch.Wait.TimeOut` | (none) | Default — escalate to max priority |
| `-Q.Dispatch.Wait.TimeOut.Kill.Graceful` | (none) | Graceful kill the waiting job |
| `-Q.Dispatch.Wait.TimeOut.Kill.Hard` | (none) | Hard kill the waiting job |
| `-Q.Dispatch.Wait.TimeOut.Reassign` | `<queue#String` | Move job to a different queue |

If no `-Q.Dispatch.Wait.TimeOut.*` is specified, the default is priority escalation.

## Example

Used as nested `[Q]` line in a queue definition:

```polyglot
{Q} #Queue:BatchQueue
   [.] .strategy#QueueStrategy << #FIFO
   [.] .maxWaitTime#String << "30m"
   [ ] If wait exceeds 30m, move to faster queue
   [Q] -Q.Dispatch.Wait.TimeOut.Reassign
      (-) <queue << "ExpressQueue"
```

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Dispatch.Wait.TimeOut` | Compile-time pipeline template |
| Instance | `%Q:Dispatch.Wait.TimeOut:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
