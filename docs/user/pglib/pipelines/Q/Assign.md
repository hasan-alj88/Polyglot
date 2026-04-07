---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =Q.Assign

Assign a pipeline to a named queue. The string argument is the name of a `{Q}` defined queue. Referencing an undefined queue is a compile error (PGE01014).

## Definition

```polyglot
{N} =Q.Assign
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QAssign"
   [%] .description << "Assign pipeline to a named queue."
```

## Usage

```polyglot
[Q] =Q.Assign"GPUQueue"
```

Queue strategy (FIFO, LIFO, Priority), constraints, and host are configured on the `{Q}` definition — not on the `[Q]` assignment line. See [[structs#Queue]] for the `#Queue` schema fields.

## Inputs

None.

## Outputs

None.

## Errors

| Code | Condition |
|------|-----------|
| PGE01014 | Referenced queue name is not defined in any `{Q}` block |

## Permissions

None — pure computation (queue scheduling and resource management).

## Related

- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
