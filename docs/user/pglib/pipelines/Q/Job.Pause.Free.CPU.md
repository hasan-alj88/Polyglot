---
audience: automation-builder
type: specification
updated: 2026-04-15
status: complete
metadata_definition: "%definition.Q:Job.Pause.Free.CPU"
metadata_instance: "%Q:Job.Pause.Free.CPU:N"
---

# -Q.Job.Pause.Free.CPU

Pause a Job, freeing CPU. Available in `.Now` (immediate) and `.Wait` (after current work unit) timing variants.

## Definition

```polyglot
{N} -Q.Job.Pause.Free.CPU
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobPauseFreeCPU"
   [%] .description << "Pause a Job, freeing CPU."
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
| RAM | Kept |
| FDs | Kept |
| TCP | Kept |
| Locks | Kept |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Pause.Free.CPU` | Compile-time pipeline template |
| Instance | `%Q:Job.Pause.Free.CPU:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Job.Pause.Free.RAM|-Q.Job.Pause.Free.RAM]]
- [[pglib/pipelines/Q/Job.Pause.Free.All|-Q.Job.Pause.Free.All]]
- [[pglib/pipelines/Q/Job.Resume|-Q.Job.Resume]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
