---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:Queue.Consumer"
metadata_instance: "%W:Queue.Consumer:N"
---

# -W.Queue.Consumer

Connects to message queue on setup, disconnects on cleanup.

## Definition

```polyglot
{N} -W.Queue.Consumer
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WQueueConsumer"
   [%] .description << "Connects to message queue on setup, disconnects on cleanup."
   [{] $queueUrl#string   [ ] Message queue connection URL
   [}] $consumer          [ ] Active message queue consumer handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$queueUrl` | `#string` | Message queue connection URL |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$consumer` | consumer handle | Active message queue consumer handle |

## Errors

None.

## Permissions

IPC.Receive

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:Queue.Consumer` | Compile-time pipeline template |
| Instance | `%W:Queue.Consumer:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
