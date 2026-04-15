---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Kill.Now"
metadata_instance: "%Q:Job.Kill.Now:N"
---

# -Q.Job.Kill.Now

Immediately terminate a Job with no cleanup. Terminal — Job state destroyed instantly.

## Definition

```polyglot
{N} -Q.Job.Kill.Now
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobKillNow"
   [%] .description << "Immediately terminate a Job with no cleanup."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

Everything freed immediately. No `[/]` cleanup runs. The Job goes directly to Dead state.

| Resource | Effect |
|----------|--------|
| CPU | Freed immediately |
| RAM | Freed immediately |
| FDs | Freed immediately |
| TCP | Freed immediately |
| Locks | Freed immediately |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Kill.Now` | Compile-time pipeline template |
| Instance | `%Q:Job.Kill.Now:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Kill.WithCleanup|-Q.Job.Kill.WithCleanup]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
