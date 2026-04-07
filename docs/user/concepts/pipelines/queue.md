---
audience: pg-coder
type: specification
updated: 2026-04-03
---

<!-- @concepts/pipelines/INDEX -->

## Queue

Every pipeline must declare a `[Q]` line — omitting it is a compile error (PGE01006). Polyglot uses a multi-queue execution model with a unified dispatch layer:

- **Dispatch Queues** — one per `{Q}` definition (plus `=Q.Default`). Each maintains its own ordering strategy (FIFO, LIFO, Priority). Triggered pipelines enter the Dispatch Queue assigned by their `[Q]` declaration.
- **Dispatch Coordinator** — the unified dispatch layer. Reads from all parallel Dispatch Queues simultaneously and dispatches jobs to the Executing Set. The Dispatch Coordinator is faithful to every Dispatch Queue's ordering and concurrency rules — it never overrides or reorders a queue's internal strategy.

### Defining a Queue (`{Q}`)

Custom queues are defined with `{Q}`, which both defines the queue struct and instantiates it. The identifier must use the `#Queue:` prefix (PGE01012). Queue-level defaults apply to all pipelines assigned to this queue.

```polyglot
{Q} #Queue:GPUQueue
   [.] .strategy#QueueStrategy << #LIFO
   [.] .host#String << "gpu-server-01"
   [.] .maxInstancesWithinQueue#UnsignedInt << 1
   [.] .maxConcurrentWithinQueue#UnsignedInt << 4
   [.] .resourceTagWithinQueue#array:ResourceTag << [#GPU]
   [.] .killPropagation#KillPropagation << #Downgrade
   [.] .maxWaitTime#String << "30m"
   [.] .description#String << "GPU-intensive work"
   [ ] Queue-level default: kill after 4 hours
   [Q] =Q.Kill.Graceful.Time.MoreThan
      [=] <duration << "4h"
```

`=Q.Default` is the only pglib-provided queue and does not require a `{Q}` definition. All other queues must be defined via `{Q}` first. Referencing an undefined queue is a compile error (PGE01014).

### Queue Pipeline Operations (`{Q} =Q.*`)

<!-- @technical/brainstorming/marker-declarations -->

`{Q}` is a **dual-purpose block** — the identifier prefix disambiguates its role:

- **`{Q} #Queue:Name`** — data definition (subtype of `{#}`). Defines and instantiates a queue instance using the `#Queue` schema. Covered above.
- **`{Q} =Q.*`** — pipeline operation (subtype of `{=}`, equivalent to `{=}[Q]`). Defines a queue control pipeline invocable via `[Q]`.

Queue pipeline operations control the Executing Set — pause, resume, and kill behaviors. They are native definitions (`{N}` blocks) backed by host language code:

```polyglot
{N} =Q.Default
   [%] .Kind << #NativeKind.Queue
   [%] .Rust << "QueueDefault"
   [%] .description << "Default FIFO queue strategy"

{N} =Q.Pause.Hard
   [%] .Kind << #NativeKind.Queue
   [%] .Rust << "QueuePauseHard"
   [%] .description << "Hard pause — stop dispatching immediately"
   [=] <condition#string

{N} =Q.Resume
   [%] .Kind << #NativeKind.Queue
   [%] .Rust << "QueueResume"
   [%] .description << "Resume dispatching after pause"
   [=] <condition#string

{N} =Q.Kill.Graceful
   [%] .Kind << #NativeKind.Queue
   [%] .Rust << "QueueKillGraceful"
   [%] .description << "Graceful kill — wait for in-flight jobs"
   [=] <condition#string
```

These pglib queue pipelines do not require `[@]` import — they are built-in like all `=Q.*` pipelines.

### Using a Queue (`[Q]`)

The `[Q]` line in a pipeline declares which queue it uses. It accepts optional `[=]` IO lines for pipeline-level constraints, and nested `[Q]` lines for pipeline-specific active controls:

```polyglot
[Q] =Q.Default
```

```polyglot
[Q] =Q.Assign"GPUQueue"
   [=] <maxInstancesAllQueues#UnsignedInt << 3
   [=] <maxConcurrentAllQueues#UnsignedInt << 10
   [=] <maxConcurrentWithinHost#UnsignedInt << 5
   [ ] Pipeline-specific: pause/resume based on RAM
   [Q] =Q.Pause.Hard.RAM.LessThan
      [=] <mb << 3072.0
   [Q] =Q.Resume.RAM.MoreThan
      [=] <mb << 5120.0
```

Pipeline-specific `[Q]` controls must not contradict the queue's `{Q}` defaults (PGE01013). See [[Q]] for the full pglib queue pipeline catalog.

### Job-Level Queue Conditions

Pipelines break into **jobs** at IO boundaries — each `[r]`, `[p]`, or `[b]` marker creates a job. `[Q]` lines can be nested under these markers to scope queue conditions to that specific job and its sub-jobs (the branch subtree), rather than the entire pipeline.

Pipeline-level `[Q]` applies to all jobs. Job-level `[Q]` **extends** the pipeline-level defaults for that branch only — it does not replace them. Contradictions between job-level and pipeline-level `[Q]` raise PGE01013.

```polyglot
{=} =ProcessData
   [T] =T.Call
   [=] <input#Serial
   [=] >output#Serial
   [ ] Pipeline-level: all jobs use default queue
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =FetchData
      [=] <in << $input
      [=] >out >> $fetched
   [p] =Transform
      [=] << $fetched
      [=] >> $transformed
      [ ] Job-level: pause only this branch when RAM is low
      [Q] =Q.Pause.Soft.RAM.LessThan
         [=] <mb << 2048.0
   [p] =Validate
      [=] << $fetched
      [=] >> $validated
   [*] *All
      [*] << $transformed
      [*] << $validated
      [*] >> $results
   [r] =Save
      [=] << $results
      [=] >> $output
```

In this example, all jobs inherit `=Q.Default`. The `=Transform` branch additionally pauses when RAM drops below 2048 MB — this condition only affects that `[p]` branch and any sub-jobs it spawns, not the `=Validate` branch or the `[r]` jobs.

## Queue Handler Internals

### Queues vs Sets

The Queue Handler uses two container types:

- **Queue** — jobs waiting in line to DO something (ordered, dispatched one at a time)
- **Set** — a record of jobs currently IN a state (not waiting, actively being something)

| Container | Type | Purpose |
|-----------|------|---------|
| Dispatch Queue (per `{Q}`) | Queue | Waiting to be dispatched, ordered by queue strategy |
| Dispatch Queues Coordinator Queue | Queue | Virtual single-slot buffer — holds Tier 1 Selection RR's candidate for Tier 2 |
| Resume Queue | Queue | Resumed jobs waiting for a dispatch slot |
| Teardown Queue | Queue | Jobs waiting for a cleanup slot to run `[/]` cleanup before termination |
| Executing Set | Set | Currently executing |
| Suspended Set | Set | Suspended, not consuming CPU (soft) or CPU+RAM (hard) |

### The Dispatch Coordinator

The Dispatch Coordinator is the unified dispatch layer that reads from all parallel Dispatch Queues simultaneously. It is not a separate container — it is the autonomous dispatch loop inside the Queue Handler.

Each Dispatch Queue maintains its own internal ordering (FIFO, LIFO, Priority). Each queue proposes its next candidate based on its strategy. The Dispatch Coordinator dispatches jobs to the Executing Set while being **faithful** to every Dispatch Queue's rules:

- If GPUQueue (LIFO) says D goes before E, the Dispatch Coordinator will never dispatch E before D
- If DefaultQueue (FIFO) says A goes before B, the Dispatch Coordinator will never dispatch B before A
- Between queues there is no imposed ordering — jobs from different queues can dispatch in any relative order, as long as each queue's internal order is preserved

The Dispatch Coordinator is event-driven — it wakes whenever queue state changes (enqueue, resume, teardown, slot freed) and cycles until all queues are empty or all candidates fail constraints.

### Two-Tier Round-Robin Dispatch

The Dispatch Coordinator uses a two-tier round-robin algorithm:

```text
Tier 1 — Selection RR (across user Dispatch Queues):
   ┌─ Queue A (FIFO)
   ├─ Queue B (LIFO)
   └─ Queue C (Priority)
       │ round-robin picks one candidate from each,
       │ selects one → Dispatch Queues Coordinator Queue
       ▼

Tier 2 — Dispatch RR (three equal peers):
   ┌─ Dispatch Queues Coordinator Queue (from Tier 1)
   ├─ Resume Queue
   └─ Teardown Queue
       │ round-robin between all three
       ▼
Dispatch Constraints → Executing Set
```

**Tier 1 (Selection RR):** Round-robins across user-defined Dispatch Queues to select one candidate. Each queue proposes exactly one candidate. The selected candidate enters the Dispatch Queues Coordinator Queue.

**Tier 2 (Dispatch RR):** Round-robins across three equal peers: the Dispatch Queues Coordinator Queue, Resume Queue, and Teardown Queue. The selected candidate proceeds to constraint checks and dispatch.

### State Transitions

Triggered pipelines enter the Dispatch Queue assigned by their `[Q]` declaration. The Dispatch Coordinator dispatches them to the Executing Set when resources and concurrency constraints allow.

- **Trigger** → Dispatch Queue (per `[Q]` config)
- **Dispatch Queue** → dispatched by Dispatch Coordinator → Executing Set
- **Executing Set** → pause → Suspended Set (soft: frees CPU; hard: frees CPU+RAM)
- **Suspended Set** → unpause → Resume Queue
- **Resume Queue** → dispatched by Dispatch Coordinator → Executing Set (resumes from suspended state)
- **Executing Set** → graceful kill → Teardown Queue (waits for cleanup slot)
- **Teardown Queue** → dispatched by Dispatch Coordinator → Executing Set (runs `[/]` cleanup, terminates)
- **Executing Set** → hard kill → immediately removed, no cleanup

### Host-Based Load Balancing

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Handler dispatches jobs to the Runner on that host. To distribute work across multiple hosts, define multiple queues with different `.host` values. Offloading work to another host means switching queues — either explicitly via `=Q.Reassign` or automatically via `=Q.Dispatch.Wait.TimeOut.Reassign`.

### Cross-Queue Constraints

The Dispatch Coordinator enforces constraints at multiple scopes:

**Queue-level** (declared on `{Q}` definition):
- `maxInstancesWithinQueue` — max instances of a specific pipeline within this queue
- `maxConcurrentWithinQueue` — max total jobs dispatched from this queue
- `resourceTagWithinQueue` — resource exclusion within this queue (e.g., only one `#GPU`-tagged job)

**Pipeline-level** (declared on `[Q]` usage):
- `maxInstancesAllQueues` — max instances of this pipeline across all queues
- `maxInstancesWithinHost` — max instances of this pipeline on a specific host
- `maxConcurrentAllQueues` — max total jobs running globally alongside this pipeline
- `maxConcurrentWithinHost` — max total jobs on a specific host alongside this pipeline
- `resourceTagAllQueues` — resource exclusion globally
- `resourceTagWithinHost` — resource exclusion per host

### Resource Effects

| Action | CPU | RAM |
|--------|-----|-----|
| Pause Soft | Freed | Kept |
| Pause Hard | Freed | Freed |
| Running | Used | Used |
| Kill Graceful | Used (finishing) | Used (finishing) |
| Kill Hard | Freed immediately | Freed immediately |

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[Q]` fits in pipeline element order
- [[concepts/pipelines/wrappers|Wrappers]] — `[W]` wrapper that follows the queue declaration
- [[concepts/pipelines/execution|Execution]] — execution body that runs after queue dispatch
