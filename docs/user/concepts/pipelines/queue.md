---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Queue

Every pipeline must declare a `[Q]` line — omitting it is a compile error (PGE01006). Polyglot uses a two-queue execution model:

- **Pending Queue** — pipelines awaiting dispatch after all triggers fire. Strategies control ordering (FIFO, LIFO, Priority).
- **Active Queue** — pipelines currently executing. Controls include pause, resume, and kill operations.

### Defining a Queue (`{Q}`)

Custom queues are defined with `{Q}`, which both defines the queue struct and instantiates it. The identifier must use the `#Queue:` prefix (PGE01012). Queue-level defaults apply to all pipelines assigned to this queue.

```polyglot
{Q} #Queue:GPUQueue
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .maxInstances#int << 1
   [.] .retrigger;#RetriggerStrategy << #Disallow
   [ ] Queue-level default: kill after 4 hours
   [Q] =Q.Kill.Graceful
      [=] <ExecutionTime.MoreThan#string << "4h"
```

`=Q.Default` is the only stdlib-provided queue and does not require a `{Q}` definition. All other queues must be defined via `{Q}` first. Referencing an undefined queue is a compile error (PGE01014).

### Queue Pipeline Operations (`{Q} =Q.*`)

<!-- @technical/brainstorming/marker-declarations -->

`{Q}` is a **dual-purpose block** — the identifier prefix disambiguates its role:

- **`{Q} #Queue:Name`** — data definition (subtype of `{#}`). Defines and instantiates a queue instance using the `#Queue` schema. Covered above.
- **`{Q} =Q.*`** — pipeline operation (subtype of `{=}`, equivalent to `{=}[Q]`). Defines a queue control pipeline invocable via `[Q]`.

Queue pipeline operations control the active queue — pause, resume, and kill behaviors. They are base pipelines backed by native code:

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
[Q] #Queue:GPUQueue
   [=] <maxConcurrent#int << 2
   [ ] Pipeline-specific: pause/resume based on RAM
   [Q] =Q.Pause.Hard
      [=] <RAM.Available.LessThan#float << 3072.0
   [Q] =Q.Resume
      [=] <RAM.Available.MoreThan#float << 5120.0
```

| IO Parameter | Type | Description |
|-------------|------|-------------|
| `<maxInstances#int` | int | Max parallel instances of this pipeline |
| `<maxConcurrent#int` | int | Max other pipelines running alongside |
| `<retrigger;#RetriggerStrategy` | enum | Behavior on re-trigger while active |

Pipeline-specific `[Q]` controls must not contradict the queue's `{Q}` defaults (PGE01013). See [[Q]] for the full stdlib queue pipeline catalog.

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[Q]` fits in pipeline element order
- [[concepts/pipelines/wrappers|Wrappers]] — `[W]` wrapper that follows the queue declaration
- [[concepts/pipelines/execution|Execution]] — execution body that runs after queue dispatch
