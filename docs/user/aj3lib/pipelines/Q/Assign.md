---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Assign"
metadata_instance: "%Q:Assign:N"
---

# -Q.Assign

Assign a pipeline to a named queue. The string argument is the name of a `{Q}` defined queue. Referencing an undefined queue is a compile error (PGE01014).

## Definition

```aljam3
{N} -Q.Assign
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QAssign"
   [%] .description << "Assign pipeline to a named queue."
```

## Usage

```aljam3
[Q] -Q.Assign"GPUQueue"
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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Assign` | Compile-time pipeline template |
| Instance | `%Q:Assign:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
