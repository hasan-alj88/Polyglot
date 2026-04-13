---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Default"
metadata_instance: "%Q:Default:N"
---

# -Q.Default

Standard FIFO queue, no constraints. The only queue that does not require a `{Q}` definition.

## Definition

```polyglot
{N} -Q.Default
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QDefault"
   [%] .description << "Standard FIFO queue, no constraints."
```

Metadata path: `%Queue.DispatchQueue:Default`.

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
| Definition | `%definition.Q:Default` | Compile-time pipeline template |
| Instance | `%Q:Default:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
