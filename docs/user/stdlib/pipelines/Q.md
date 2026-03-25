---
audience: user
type: specification
updated: 2026-03-25
status: complete
---

# =Q — Queue Pipelines

<!-- @pipelines -->
Queue pipelines manage the two-queue execution model: **Pending Queue** (pipelines awaiting dispatch) and **Active Queue** (pipelines currently executing). No `[@]` import needed. See [[pipelines#Queues]] for queue usage rules.

All `=Q.*` pipelines are used via `[Q]` — either in a `{Q}` queue definition (queue-level defaults) or in a pipeline's `[Q]` section (pipeline-specific controls). Controls in `{Q}` apply to all pipelines on that queue. Controls in `[Q]` are pipeline-specific. Contradictions raise PGE-113.

**PRIMITIVE** — Queue pipelines are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

## Permissions

No permissions required. All operations are pure computation (queue scheduling and resource management). See [[permissions]].

## Pending Queue Strategies

Referenced by the top-level `[Q]` line in a pipeline definition. `=Q.Default` is the only stdlib-provided queue — custom queues require a `{Q}` definition first.

All strategies accept optional IO parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| `<maxInstances;int` | int | Max parallel instances of this pipeline |
| `<maxConcurrent;int` | int | Max other pipelines running alongside |
| `<retrigger;#RetriggerStrategy` | enum | Behavior on re-trigger while active |

### =Q.Default

Standard queue — FIFO ordering, no concurrency limits, allows retrigger. The only queue that does not require a `{Q}` definition.

```polyglot
[Q] =Q.Default
```

### =Q.FIFO

Explicit FIFO (first-in-first-out) ordering. Same behavior as Default but explicit. Requires `{Q}` with `.strategy;#QueueStrategy << #FIFO`.

### =Q.LIFO

Last-in-first-out ordering. Most recently queued pipeline runs first. Requires `{Q}` with `.strategy;#QueueStrategy << #LIFO`.

### =Q.Priority

Priority-based ordering. Higher priority pipelines dispatch first. Requires `{Q}` with `.strategy;#QueueStrategy << #Priority`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `<priority;int` | int | Priority level (higher = dispatched first) |

---

## Active Queue Controls

Declared as nested `[Q]` lines inside a pipeline's `[Q]` section or inside a `{Q}` definition. These control running pipeline instances.

### =Q.Pause.Soft

Pause a running pipeline. Finishes current work, then suspends. Frees CPU/GPU.

```polyglot
[Q] =Q.Default
   [Q] =Q.Pause.Soft
      [=] <CPU.MoreThan;float << 90.0
```

### =Q.Pause.Hard

Pause a running pipeline immediately. Frees CPU/GPU and RAM.

```polyglot
[Q] #Queue:GPUQueue
   [Q] =Q.Pause.Hard
      [=] <RAM.Available.LessThan;float << 3072.0
```

### =Q.Resume

Resume a paused pipeline (soft or hard).

```polyglot
[Q] #Queue:GPUQueue
   [Q] =Q.Resume
      [=] <RAM.Available.MoreThan;float << 5120.0
```

### =Q.Kill.Graceful

Finish current work, run `[/]` cleanup, then terminate.

```polyglot
[Q] =Q.Default
   [Q] =Q.Kill.Graceful
      [=] <ExecutionTime.MoreThan;string << "2h"
```

### =Q.Kill.Hard

Immediate OS-level termination. No cleanup runs.

```polyglot
[Q] =Q.Default
   [Q] =Q.Kill.Hard
      [=] <ExecutionTime.MoreThan;string << "4h"
```

---

## Conditional Parameters

Active queue controls accept conditional parameters that specify when the control activates. These are resource-based or pipeline-state-based conditions.

### Resource Conditions

| Parameter | Type | Description |
|-----------|------|-------------|
| `<CPU.MoreThan;float` | float | CPU usage percentage threshold |
| `<RAM.Available.LessThan;float` | float | RAM available in MB (below = trigger) |
| `<RAM.Available.MoreThan;float` | float | RAM available in MB (above = trigger) |
| `<ExecutionTime.MoreThan;string` | string | Execution time limit (e.g., "30m", "2h") |

### Pipeline State Conditions

| Parameter | Type | Description |
|-----------|------|-------------|
| `<Pipeline.Triggered;string` | string | Activate when named pipeline is triggered |
| `<Pipeline.Running;string` | string | Activate when named pipeline starts running |

### Example: Combined Controls

```polyglot
{Q} #Queue:GPUQueue
   [%] .description << "Queue for GPU-intensive work"
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .maxInstances;int << 1
   [.] .retrigger;#RetriggerStrategy << #Disallow
   [ ] Queue-level default: kill after 4 hours
   [Q] =Q.Kill.Graceful
      [=] <ExecutionTime.MoreThan;string << "4h"

{=} =GPU.RenderFrames
   [=] <frames;array.serial
   [=] >rendered;array.serial ~> {}
   [t] =T.Call
   [Q] #Queue:GPUQueue
      [ ] Pipeline-specific: pause/resume based on RAM
      [Q] =Q.Pause.Hard
         [=] <RAM.Available.LessThan;float << 3072.0
      [Q] =Q.Resume
         [=] <RAM.Available.MoreThan;float << 5120.0
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
