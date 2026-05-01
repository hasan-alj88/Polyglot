---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Job.Branch"
metadata_instance: "%Q:Job.Branch:N"
---

# -Q.Job.Branch

Names a marker subtree as a branch group for external reference. Reserved for future branch-level operations beyond nested `[Q]` scoping.

## Definition

```aljam3
{N} -Q.Job.Branch
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobBranch"
   [%] .description << "Names a marker subtree as a branch group for external reference."
```

## Inputs

None (reserved).

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Branch` | Compile-time pipeline template |
| Instance | `%Q:Job.Branch:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[concepts/pipelines/queue#Job-Level Queue Conditions]]
- [[jm3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
