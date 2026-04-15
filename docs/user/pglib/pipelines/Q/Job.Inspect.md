---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Inspect"
metadata_instance: "%Q:Job.Inspect:N"
---

# -Q.Job.Inspect

Read a Job's runtime state without affecting it. Observation-only — no state changes.

## Definition

```polyglot
{N} -Q.Job.Inspect
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobInspect"
   [%] .description << "Read a Job's runtime state without affecting it."
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
| Definition | `%definition.Q:Job.Inspect` | Compile-time pipeline template |
| Instance | `%Q:Job.Inspect:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
