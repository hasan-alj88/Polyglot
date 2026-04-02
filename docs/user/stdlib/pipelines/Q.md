---
audience: user
type: specification
updated: 2026-04-02
status: complete
---

# =Q — Queue Pipelines

<!-- @pipelines -->
Queue pipelines manage the multi-queue execution model: **Dispatch Queues** (pipelines awaiting dispatch, one per `{Q}` definition) and **Executing Set** (pipelines currently running). The **Dispatch Coordinator** reads from all Dispatch Queues simultaneously, faithfully honoring each queue's ordering and concurrency rules. No `[@]` import needed. See [[concepts/pipelines/queue#Queue]] for queue usage rules.

All `=Q.*` pipelines are used via `[Q]` — either in a `{Q}` queue definition (queue-level defaults) or in a pipeline's `[Q]` section (pipeline-specific controls). Controls in `{Q}` apply to all pipelines on that queue. Controls in `[Q]` are pipeline-specific. Contradictions raise PGE01013.

**PRIMITIVE** — Queue pipelines are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

## Permissions

No permissions required. All operations are pure computation (queue scheduling and resource management). See [[permissions]].

## Queue Assignment

The `[Q]` line in a pipeline declares which queue it uses.

### =Q.Default

Standard queue — FIFO ordering, no constraints. The only queue that does not require a `{Q}` definition. Metadata path: `%Queue.DispatchQueue:Default`.

```polyglot
[Q] =Q.Default
```

### =Q.Assign

Assign a pipeline to a named queue. The string argument is the name of a `{Q}` defined queue. Referencing an undefined queue is a compile error (PGE01014).

```polyglot
[Q] =Q.Assign"GPUQueue"
```

Queue strategy (FIFO, LIFO, Priority), constraints, and host are configured on the `{Q}` definition — not on the `[Q]` assignment line. See [[structs#Queue]] for the `#Queue` schema fields.

---

## Direct Commands (unconditional)

Sent by the Trigger Monitor as explicit commands. No condition — immediate execution.

| Pipeline | Signal | Purpose |
|----------|--------|---------|
| `=Q.Pause.Soft` | command.pause.soft | Finish current work, then suspend. Frees CPU |
| `=Q.Pause.Hard` | command.pause.hard | Immediate suspend. Frees CPU+RAM |
| `=Q.Resume` | command.resume | Move from Suspended Set to Resume Queue |
| `=Q.Kill.Graceful` | command.kill.graceful | Finish work + `[/]` cleanup, terminate |
| `=Q.Kill.Hard` | command.kill.hard | Immediate OS kill, no cleanup |

---

## Conditional Pause — `=Q.Pause.{Soft\|Hard}.{Condition}`

Pause when a resource condition is met. Used as nested `[Q]` lines in `{Q}` definitions or pipeline `[Q]` sections.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Pause.Soft.RAM.LessThan` | `<mb;#Float` | Soft pause when RAM drops below threshold |
| `=Q.Pause.Soft.CPU.MoreThan` | `<percent;#Float` | Soft pause when CPU exceeds threshold |
| `=Q.Pause.Soft.Disk.LessThan` | `<mb;#Float` | Soft pause when disk space drops below threshold |
| `=Q.Pause.Soft.GPU.InUse` | (none) | Soft pause when GPU is occupied |
| `=Q.Pause.Hard.RAM.LessThan` | `<mb;#Float` | Hard pause when RAM drops below threshold |
| `=Q.Pause.Hard.CPU.MoreThan` | `<percent;#Float` | Hard pause when CPU exceeds threshold |
| `=Q.Pause.Hard.Disk.LessThan` | `<mb;#Float` | Hard pause when disk space drops below threshold |
| `=Q.Pause.Hard.GPU.InUse` | (none) | Hard pause when GPU is occupied |

---

## Conditional Resume — `=Q.Resume.{Condition}`

Resume when a resource condition recovers.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Resume.RAM.MoreThan` | `<mb;#Float` | Resume when RAM recovers above threshold |
| `=Q.Resume.CPU.LessThan` | `<percent;#Float` | Resume when CPU drops below threshold |
| `=Q.Resume.Disk.MoreThan` | `<mb;#Float` | Resume when disk space recovers |
| `=Q.Resume.GPU.Free` | (none) | Resume when GPU becomes available |

---

## Conditional Kill — `=Q.Kill.{Graceful\|Hard}.{Condition}`

Terminate jobs based on time, state, or resource conditions.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Kill.Graceful.Time.MoreThan` | `<duration;#String` | Graceful kill after execution time exceeds limit |
| `=Q.Kill.Graceful.RAM.LessThan` | `<mb;#Float` | Graceful kill when RAM critically low |
| `=Q.Kill.Graceful.Pipeline.Completed` | `<name;#String` | Graceful kill when named pipeline completes |
| `=Q.Kill.Graceful.Pipeline.Failed` | `<name;#String` | Graceful kill when named pipeline fails |
| `=Q.Kill.Hard.Time.MoreThan` | `<duration;#String` | Hard kill after execution time exceeds limit |
| `=Q.Kill.Hard.RAM.LessThan` | `<mb;#Float` | Hard kill when RAM critically low |

---

## Dispatch Wait Timeout — `=Q.Dispatch.Wait.TimeOut.*`

What happens when a job exceeds `.maxWaitTime` in the queue. Default behavior: escalate to max priority.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Dispatch.Wait.TimeOut` | (none) | Default — escalate to max priority |
| `=Q.Dispatch.Wait.TimeOut.Kill.Graceful` | (none) | Graceful kill the waiting job |
| `=Q.Dispatch.Wait.TimeOut.Kill.Hard` | (none) | Hard kill the waiting job |
| `=Q.Dispatch.Wait.TimeOut.Reassign` | `<queue;#String` | Move job to a different queue |

Used as nested `[Q]` line in queue definition:

```polyglot
{Q} #Queue:BatchQueue
   [.] .strategy;#QueueStrategy << #FIFO
   [.] .maxWaitTime;#String << "30m"
   [ ] If wait exceeds 30m, move to faster queue
   [Q] =Q.Dispatch.Wait.TimeOut.Reassign
      [=] <queue << "ExpressQueue"
```

If no `=Q.Dispatch.Wait.TimeOut.*` is specified, the default is priority escalation.

---

## Queue Admin — `=Q.{Operation}`

Queue-level operations. Target the queue itself, not individual jobs.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Drain` | `<queue;#String` | Stop accepting new jobs, finish existing |
| `=Q.Flush` | `<queue;#String` | Remove all pending jobs from a queue |
| `=Q.Priority.Update` | `<jobId;#String`, `<score;#Int` | Change a job's priority score |
| `=Q.Reassign` | `<jobId;#String`, `<queue;#String` | Move a job to a different queue (enables host offloading) |

---

## Example: Full Queue Definition + Pipeline Assignment

```polyglot
{Q} #Queue:GPUQueue
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .host;#String << "gpu-server-01"
   [.] .maxInstances;#UnsignedInt << 1
   [.] .killPropagation;#KillPropagation << #Downgrade
   [.] .resourceTags;#Array:ResourceTag << [#GPU]
   [.] .maxWaitTime;#String << "30m"
   [.] .description;#String << "GPU-intensive work"
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
