---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =Q — Queue Pipelines

<!-- @pipelines -->
Queue pipelines manage the multi-queue execution model: **Dispatch Queues** (pipelines awaiting dispatch, one per `{Q}` definition) and **Executing Set** (pipelines currently running). The **Dispatch Coordinator** reads from all Dispatch Queues simultaneously, faithfully honoring each queue's ordering and concurrency rules. No `[@]` import needed. See [[concepts/pipelines/queue#Queue]] for queue usage rules.

All `=Q.*` pipelines are used via `[Q]` — either in a `{Q}` queue definition (queue-level defaults) or in a pipeline's `[Q]` section (pipeline-specific controls). Controls in `{Q}` apply to all pipelines on that queue. Controls in `[Q]` are pipeline-specific. Contradictions raise PGE01013.

**PRIMITIVE** — Queue pipelines are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

## Permissions

No permissions required. All operations are pure computation (queue scheduling and resource management). See [[permissions]].

## Pipeline Listing

### Queue Assignment

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Default\|=Q.Default]] | Standard FIFO queue, no constraints |
| [[pglib/pipelines/Q/Assign\|=Q.Assign]] | Assign pipeline to a named queue |

### Direct Commands

| Pipeline | Signal | Description |
|----------|--------|-------------|
| [[pglib/pipelines/Q/Pause.Soft\|=Q.Pause.Soft]] | command.pause.soft | Finish current work, then suspend. Frees CPU |
| [[pglib/pipelines/Q/Pause.Hard\|=Q.Pause.Hard]] | command.pause.hard | Immediate suspend. Frees CPU+RAM |
| [[pglib/pipelines/Q/Resume\|=Q.Resume]] | command.resume | Move from Suspended Set to Resume Queue |
| [[pglib/pipelines/Q/Kill.Graceful\|=Q.Kill.Graceful]] | command.kill.graceful | Finish work + `[/]` cleanup, terminate |
| [[pglib/pipelines/Q/Kill.Hard\|=Q.Kill.Hard]] | command.kill.hard | Immediate OS kill, no cleanup |

### Conditional Pause / Resume / Kill

Conditional variants are documented inside their parent command files: [[pglib/pipelines/Q/Pause.Soft|Pause.Soft]], [[pglib/pipelines/Q/Pause.Hard|Pause.Hard]], [[pglib/pipelines/Q/Resume|Resume]], [[pglib/pipelines/Q/Kill.Graceful|Kill.Graceful]], and [[pglib/pipelines/Q/Kill.Hard|Kill.Hard]].

### Dispatch Timeout

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Dispatch.Wait.TimeOut\|=Q.Dispatch.Wait.TimeOut]] | What happens when a job exceeds `.maxWaitTime` |

### Queue Admin

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Drain\|=Q.Drain]] | Stop accepting new jobs, finish existing |
| [[pglib/pipelines/Q/Flush\|=Q.Flush]] | Remove all pending jobs from a queue |
| [[pglib/pipelines/Q/Priority.Update\|=Q.Priority.Update]] | Change a job's priority score |
| [[pglib/pipelines/Q/Reassign\|=Q.Reassign]] | Move a job to a different queue |

### Job Addressing

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Q/Job.Branch\|=Q.Job.Branch]] | Names a marker subtree as a branch group for external reference |

## Example: Full Queue Definition + Pipeline Assignment

```polyglot
{Q} #Queue:GPUQueue
   [.] .strategy#QueueStrategy << #LIFO
   [.] .host#String << "gpu-server-01"
   [.] .maxInstances#UnsignedInt << 1
   [.] .killPropagation#KillPropagation << #Downgrade
   [.] .resourceTags#Array:ResourceTag << [#GPU]
   [.] .maxWaitTime#String << "30m"
   [.] .description#String << "GPU-intensive work"
   [ ] Pause when RAM drops below 3GB
   [Q] =Q.Pause.Hard.RAM.LessThan
      [=] <mb << 3072.0
   [ ] Resume when RAM recovers above 5GB
   [Q] =Q.Resume.RAM.MoreThan
      [=] <mb << 5120.0
   [ ] Kill after 4 hours
   [Q] =Q.Kill.Graceful.Time.MoreThan
      [=] <duration << "4h"

{=} =GPU.RenderFrames
   [=] <frames#array:serial
   [=] >rendered#array:serial ~> {}
   [T] =T.Call
   [Q] =Q.Assign"GPUQueue"
   [W] =W.Polyglot
   [p] ~ForEach.Array
      [~] <Array << $frames
      [~] >item >> $frame
      [r] =GPU.Render
         [=] <frame << $frame
         [=] >result >> $out
      [r] *Into.Array
         [*] <item << $out
         [*] >Array >> $rendered
```

## Related

- [[concepts/pipelines/queue]]
- [[pglib/INDEX]]
