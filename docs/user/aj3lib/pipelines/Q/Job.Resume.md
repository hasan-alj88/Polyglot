---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:Job.Resume"
metadata_instance: "%Q:Job.Resume:N"
---

# -Q.Job.Resume

Resume a paused Job. "Resume" means "this rule no longer wants the Job paused." The Job actually resumes only when **no rule** wants it paused (pause reason set is empty).

## Definition

```aljam3
{N} -Q.Job.Resume
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QJobResume"
   [%] .description << "Resume a paused Job."
```

## Pause Reason Set

Multiple rules can pause a Job simultaneously. Each rule adds its own reason to the pause reason set. Calling `-Q.Job.Resume` removes the calling rule's reason from the set. The Job actually resumes only when the set is empty — meaning no rule wants it paused.

This prevents conflicts where one rule resumes a Job that another rule still needs paused.

## Variants

- `-Q.Job.Resume` — thaw from CPU or RAM pause. All state is already in memory.
- `-Q.Job.Resume.From.Disk` — restore from `Free.All` image files. State is deserialized from disk back into memory before execution continues.

## Inputs

None.

## Outputs

None.

## Errors

None.

## Runtime Behavior

| Step | Component | Action |
|------|-----------|--------|
| 1. TM decides | Trigger Monitor | Pause reason set is empty for this rule, sends resume command |
| 2. NATS command | `aljam3.command.job.resume.{jobId}` | `{jobId}` |
| 3. QH executes | Queue Handler | HDEL set:suspended, RPUSH queue:resume, HSET job status "resuming" |
| 4. DC dispatches | Dispatch Coordinator | Picks job from resume queue, sends control signal based on prior suspended type |
| 5. Control signal | `aljam3.queue.control.{jobId}.job.resume` | cpu/ram.soft/ram.hard: `{jobId}`; all: `{jobId, images_dir}` |
| 6. Unix mechanism | Runner | cpu/ram: `echo 0 > cgroup.freeze` (+ remove memory limits); all: `criu restore --images-dir {path}` |

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Job.Resume` | Compile-time pipeline template |
| Instance | `%Q:Job.Resume:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Q/Job.Pause.Free.CPU|-Q.Job.Pause.Free.CPU]]
- [[aj3lib/pipelines/Q/Job.Pause.Free.RAM|-Q.Job.Pause.Free.RAM]]
- [[aj3lib/pipelines/Q/Job.Pause.Free.All|-Q.Job.Pause.Free.All]]
- [[aj3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
