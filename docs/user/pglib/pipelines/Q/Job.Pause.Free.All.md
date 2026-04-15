---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Pause.Free.All"
metadata_instance: "%Q:Job.Pause.Free.All:N"
---

# -Q.Job.Pause.Free.All

Pause a Job, serializing all state to disk. Everything freed from memory. Available in `.Now`/`.Wait` timing.

## Definition

```polyglot
{N} -Q.Job.Pause.Free.All
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobPauseFreeAll"
   [%] .description << "Pause a Job, serializing all state to disk."
```

## Timing Variants

- `.Now` — immediate freeze. Job may be mid-operation.
- `.Wait` — waits for current work unit boundary before freezing.

## Inputs

None.

## Outputs

None.

## Errors

None.

## Resource Effects

| Resource | Effect |
|----------|--------|
| CPU | Freed |
| RAM | Freed |
| FDs | Freed |
| TCP | Freed |
| Locks | Freed |

Everything is serialized to disk. Nothing remains in memory. State is persisted in image files.

## Requirements

Requires runtime checkpoint/restore capability. If not available, using this action is a compile error.

## Compiler Rules

- Compile error if held locks at pause point.
- Compile error if non-repairable TCP connections are open (unless the queue has `.tcpRepairable << true`).

Resume with `-Q.Job.Resume.From.Disk`.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Pause.Free.All` | Compile-time pipeline template |
| Instance | `%Q:Job.Pause.Free.All:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Pause.Free.CPU|-Q.Job.Pause.Free.CPU]]
- [[pglib/pipelines/Q/Job.Pause.Free.RAM|-Q.Job.Pause.Free.RAM]]
- [[pglib/pipelines/Q/Job.Resume|-Q.Job.Resume]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
