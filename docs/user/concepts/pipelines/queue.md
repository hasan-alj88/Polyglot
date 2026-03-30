---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Queue

Every pipeline must declare a `[Q]` line — omitting it is a compile error (PGE-106). Polyglot uses a two-queue execution model:

- **Pending Queue** — pipelines awaiting dispatch after all triggers fire. Strategies control ordering (FIFO, LIFO, Priority).
- **Active Queue** — pipelines currently executing. Controls include pause, resume, and kill operations.

### Defining a Queue (`{Q}`)

Custom queues are defined with `{Q}`, which both defines the queue struct and instantiates it. The identifier must use the `#Queue:` prefix (PGE-112). Queue-level defaults apply to all pipelines assigned to this queue.

```polyglot
{Q} #Queue:GPUQueue
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .maxInstances#int << 1
   [.] .retrigger;#RetriggerStrategy << #Disallow
   [ ] Queue-level default: kill after 4 hours
   [Q] =Q.Kill.Graceful
      [=] <ExecutionTime.MoreThan#string << "4h"
```

`=Q.Default` is the only stdlib-provided queue and does not require a `{Q}` definition. All other queues must be defined via `{Q}` first. Referencing an undefined queue is a compile error (PGE-114).

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

Pipeline-specific `[Q]` controls must not contradict the queue's `{Q}` defaults (PGE-113). See [[Q]] for the full stdlib queue pipeline catalog.

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[Q]` fits in pipeline element order
- [[concepts/pipelines/wrappers|Wrappers]] — `[W]` wrapper that follows the queue declaration
- [[concepts/pipelines/execution|Execution]] — execution body that runs after queue dispatch
