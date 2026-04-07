---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =Q.Default

Standard FIFO queue, no constraints. The only queue that does not require a `{Q}` definition.

## Definition

```polyglot
{N} =Q.Default
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

## Related

- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
