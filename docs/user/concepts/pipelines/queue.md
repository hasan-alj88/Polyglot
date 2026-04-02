---
audience: user
type: specification
updated: 2026-04-02
---

<!-- @concepts/pipelines/INDEX -->

## Queue

Every pipeline must declare a `[Q]` line — omitting it is a compile error (PGE01006). Polyglot uses a multi-queue execution model with a unified dispatch layer:

- **Dispatch Queues** — one per `{Q}` definition (plus `=Q.Default`). Each maintains its own ordering strategy (FIFO, LIFO, Priority). Triggered pipelines enter the Dispatch Queue assigned by their `[Q]` declaration.
- **Dispatch Coordinator** — the unified dispatch layer. Reads from all parallel Dispatch Queues simultaneously and dispatches pipelines to the Executing Set. The Dispatch Coordinator is faithful to every Dispatch Queue's ordering and concurrency rules — it never overrides or reorders a queue's internal strategy.

### Defining a Queue (`{Q}`)

Custom queues are defined with `{Q}`, which both defines the queue struct and instantiates it. The identifier must use the `#Queue:` prefix (PGE01012). Queue-level defaults apply to all pipelines assigned to this queue.

```polyglot
{Q} #Queue:GPUQueue
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .host;#String << "gpu-server-01"
   [.] .maxInstances;#UnsignedInt << 1
   [.] .killPropagation;#KillPropagation << #Downgrade
   [.] .resourceTags;#Array:ResourceTag << [#GPU]
   [.] .maxWaitTime;#String << "30m"
   [.] .description;#String << "GPU-intensive work"
   [ ] Queue-level default: kill after 4 hours
   [Q] =Q.Kill.Graceful.Time.MoreThan
      [=] <duration << "4h"
```

`=Q.Default` is the only stdlib-provided queue and does not require a `{Q}` definition. All other queues must be defined via `{Q}` first. Referencing an undefined queue is a compile error (PGE01014).

### Queue Pipeline Operations (`{Q} =Q.*`)

<!-- @technical/brainstorming/marker-declarations -->

`{Q}` is a **dual-purpose block** — the identifier prefix disambiguates its role:

- **`{Q} #Queue:Name`** — data definition (subtype of `{#}`). Defines and instantiates a queue instance using the `#Queue` schema. Covered above.
- **`{Q} =Q.*`** — pipeline operation (subtype of `{=}`, equivalent to `{=}[Q]`). Defines a queue control pipeline invocable via `[Q]`.

Queue pipeline operations control the Executing Set — pause, resume, and kill behaviors. They are base pipelines backed by native code:

```polyglot
{Q} =Q.Default
   [%] .baseCode << #BaseCode.Rust.Q.Default

{Q} =Q.Pause.Hard
   [%] .baseCode << #BaseCode.Rust.Q.Pause.Hard
   [=] <condition#string

{Q} =Q.Resume
   [%] .baseCode << #BaseCode.Rust.Q.Resume
   [=] <condition#string

{Q} =Q.Kill.Graceful
   [%] .baseCode << #BaseCode.Rust.Q.Kill.Graceful
   [=] <condition#string
```

These stdlib queue pipelines do not require `[@]` import — they are built-in like all `=Q.*` pipelines.

### Using a Queue (`[Q]`)

The `[Q]` line in a pipeline declares which queue it uses. It accepts optional `[=]` IO lines and nested `[Q]` lines for pipeline-specific active controls:

```polyglot
[Q] =Q.Default
```

```polyglot
[Q] =Q.Assign"GPUQueue"
   [ ] Pipeline-specific: pause/resume based on RAM
   [Q] =Q.Pause.Hard.RAM.LessThan
      [=] <mb << 3072.0
   [Q] =Q.Resume.RAM.MoreThan
      [=] <mb << 5120.0
```

Pipeline-specific `[Q]` controls must not contradict the queue's `{Q}` defaults (PGE01013). See [[Q]] for the full stdlib queue pipeline catalog.

## Queue Manager Internals

### Queues vs Sets

The Queue Manager uses two container types:

- **Queue** — pipelines waiting in line to DO something (ordered, dispatched one at a time)
- **Set** — a record of pipelines currently IN a state (not waiting, actively being something)

| Container | Type | Purpose |
|-----------|------|---------|
| Dispatch Queue (per `{Q}`) | Queue | Waiting to be dispatched, ordered by queue strategy |
| Resume Queue | Queue | Resumed pipelines waiting for a slot (higher dispatch priority) |
| Teardown Queue | Queue | Pipelines finishing current work + `[/]` cleanup before termination |
| Executing Set | Set | Currently executing |
| Suspended Set | Set | Suspended, not consuming CPU (soft) or CPU+RAM (hard) |

### The Dispatch Coordinator

The Dispatch Coordinator is the unified dispatch layer that reads from all parallel Dispatch Queues simultaneously. It is not a separate container — it is the dispatch logic that spans all queues.

Each Dispatch Queue maintains its own internal ordering (FIFO, LIFO, Priority). Each queue proposes its next candidate based on its strategy. The Dispatch Coordinator dispatches pipelines to the Executing Set while being **faithful** to every Dispatch Queue's rules:

- If GPUQueue (LIFO) says D goes before E, the Dispatch Coordinator will never dispatch E before D
- If DefaultQueue (FIFO) says A goes before B, the Dispatch Coordinator will never dispatch B before A
- Between queues there is no imposed ordering — pipelines from different queues can dispatch in any relative order, as long as each queue's internal order is preserved

The Dispatch Coordinator is a **constraint solver**: it takes all queue strategies, all pipeline constraints, and the Executing Set state as input, and outputs the set of pipelines that can be dispatched right now without violating any queue's rules.

### State Transitions

Triggered pipelines enter the Dispatch Queue assigned by their `[Q]` declaration. The Dispatch Coordinator dispatches them to the Executing Set when resources and concurrency constraints allow.

- **Trigger** → Dispatch Queue (per `[Q]` config)
- **Dispatch Queue** → dispatched by Dispatch Coordinator → Executing Set
- **Executing Set** → pause → Suspended Set (soft: frees CPU; hard: frees CPU+RAM)
- **Suspended Set** → unpause → Resume Queue (higher dispatch priority)
- **Executing Set** → graceful kill → Teardown Queue (finish work, `[/]` cleanup, terminate)
- **Executing Set** → hard kill → immediately removed, no cleanup

### Host-Based Load Balancing

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Manager dispatches jobs to the Runner on that host. To distribute work across multiple hosts, define multiple queues with different `.host` values. Offloading work to another host means switching queues — either explicitly via `=Q.Reassign` or automatically via `=Q.Dispatch.Wait.TimeOut.Reassign`.

### Two-Tier Round-Robin Dispatch

The Dispatch Coordinator uses a two-tier round-robin algorithm. No queue has absolute priority — all are equal participants.

```
Tier 1 RR — User Dispatch Queues:
   ┌─ Queue A (FIFO)
   ├─ Queue B (LIFO)
   └─ Queue C (Priority)
       │ round-robin picks one candidate
       ▼

Tier 2 RR — All peers, fair:
   ┌─ Dispatch candidate (from Tier 1)
   ├─ Resume Queue
   └─ Teardown Queue
       │ round-robin between all three
       ▼
Dispatch Constraints → Executing Set
```

Tier 1 round-robins across user-defined Dispatch Queues to pick one candidate. Tier 2 round-robins that candidate against the Resume and Teardown queues — all are equal participants.

### Cross-Queue Constraints

The Dispatch Coordinator enforces constraints across ALL queues — no queue dispatches independently:

- `maxInstances` counts instances of a specific pipeline across all queues and the Executing Set
- `maxConcurrent` counts other pipelines running alongside this one across ALL queues and the Executing Set
- `resourceTag` enforces resource exclusion (e.g., only one `#GPU`-tagged pipeline at a time)
- A pipeline declaring `maxConcurrent << 0` requires it to be the only pipeline running globally — the Dispatch Coordinator holds all queues until the constraint is satisfiable

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
