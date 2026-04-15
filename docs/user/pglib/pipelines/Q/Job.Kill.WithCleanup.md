---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Kill.WithCleanup"
metadata_instance: "%Q:Job.Kill.WithCleanup:N"
---

# -Q.Job.Kill.WithCleanup

Terminate a Job after running `[/]` wrapper cleanup. Terminal — Job state destroyed after cleanup.

## Definition

```polyglot
{N} -Q.Job.Kill.WithCleanup
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobKillWithCleanup"
   [%] .description << "Terminate a Job after running cleanup."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

Everything freed after cleanup runs. The Job enters the Dying state during `[/]` cleanup, then transitions to Dead.

| Resource | Effect |
|----------|--------|
| CPU | Freed after cleanup |
| RAM | Freed after cleanup |
| FDs | Freed after cleanup |
| TCP | Freed after cleanup |
| Locks | Freed after cleanup |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Kill.WithCleanup` | Compile-time pipeline template |
| Instance | `%Q:Job.Kill.WithCleanup:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Kill.Now|-Q.Job.Kill.Now]]
- [[pglib/pipelines/Q/Job.Pause.Free.CPU|-Q.Job.Pause.Free.CPU]] (non-terminal alternative)
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
