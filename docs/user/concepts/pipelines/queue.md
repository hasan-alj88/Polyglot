---
audience: automation-builder
type: specification
updated: 2026-04-16
---

<!-- @c:concepts/pipelines/INDEX -->

## Queue

Every pipeline must declare a `[Q]` line — omitting it is a compile error (PGE01006). Polyglot uses a multi-queue execution model with a unified dispatch layer:

- **Dispatch Queues** — one per `{Q} #Queue:Name` definition (plus `-Q.Default`). Each maintains its own ordering strategy (FIFO, LIFO, Priority). Triggered pipelines enter the Dispatch Queue assigned by their `[Q]` declaration.
- **Dispatch Coordinator** — the unified dispatch layer. Reads from all parallel Dispatch Queues simultaneously and dispatches Jobs to the Executing Set. The Dispatch Coordinator is faithful to every Dispatch Queue's ordering and concurrency rules — it never overrides or reorders a queue's internal strategy.

### Three `{Q}` Block Types

`{Q}` is a **multi-purpose block** — the identifier prefix disambiguates its role:

| Block | Purpose | Actions Allowed | Getters Allowed |
|-------|---------|-----------------|-----------------|
| `{Q} #Queue:Name` | Queue configuration (subtype of `{#}`) | None | None |
| `{Q} #QueueRules:Name` | Queue-level policy rules | `-Q.Queue.*` only | `-Q.Queue.*`, `-Q.Host.*`, `-Q.Job.*` (array) |
| `{Q} #JobRules:Name` | Job-level policy rules | `-Q.Job.*` only | `-Q.Job.*`, `-Q.Queue.*`, `-Q.Host.*` |

**Scope rule:** You can read anything, you can only act on your scope.

In `#QueueRules`, Job getters return an **array** of all Jobs on the queue. In `#JobRules`, queue/host getters return single values for the current context.

### Defining a Queue (`{Q} #Queue:Name`)

Custom queues define configuration and load rule sets. The identifier must use the `#Queue:` prefix (PGE01012).

```polyglot
{Q} #Queue:WorkerQueue
   [.] .strategy#QueueStrategy << #FIFO
   [.] .host#URL << "worker-host-01"
   [.] .tickPeriod << #DT"5s"
   [.] .resumeDebounce << #DT"10s"
   [.] .capacity << 50
   [.] .killPropagation#KillPropagation << #Downgrade
   [.] .maxInstancesWithinQueue#UnsignedInt << 10
   [.] .maxConcurrentWithinQueue#UnsignedInt << 4
   [ ] Load rules
   [Q] << #RAMGuard
   [Q] << #HostFailover
```

`-Q.Default` is the only pglib-provided queue and does not require a `{Q}` definition. All other queues must be defined via `{Q}` first. Referencing an undefined queue is a compile error (PGE01014).

Queue data loading supports `[#]` to load and extend a base queue:

```polyglot
{Q} #Queue:BackupHost
   [#] << #Queue.Default
      [.] .host#URL << "alternative.host.com"
```

### Defining Rules (`{Q} #JobRules:Name` / `{Q} #QueueRules:Name`)

<!-- @c:glossary#Trigger Monitor -->

Rules are **parameterized instructions** compiled into the behavior contract's signal map. The Trigger Monitor ([[glossary#Trigger Monitor|c:Trigger Monitor]]) reads the signal map and executes at runtime. They are declarative rules, not function calls.

#### Rule Parameters `(#)`

Rules accept parameters with `(#)` inputs. `<~` sets defaults (overridable), `<<` sets finals.

```polyglot
{Q} #JobRules:RAMGuard
   (#) $value.GB#int <~ 4
   (#) $margin.GB#int <~ 1
```

Parameters support validation inside the rule:

```polyglot
   [?] $margin.GB >? $value.GB
      [!] >> !Queue.InvalidMargin
```

Override parameters when loading a rule into a pipeline:

```polyglot
[Q] #RAMGuard
   (#) <value.GB << 8
   (#) >> $customRAMGuard
[Q] << $customRAMGuard
```

#### Conditionals in Rules

Queue rules use Polyglot's conditional system. No special queue-conditional syntax.

| Marker | Meaning |
|--------|---------|
| `[?]` | IF (starts a conditional branch) |
| `[&]` | AND (must also be true) |
| `[+]` | OR (alternative condition) |
| `*?` | ELSE (wildcard catch-all, mandatory) |

#### State Guards

State guards are required on action blocks. Without them, the compiler errors — forcing explicit temporal assumptions:

| Guard | Meaning |
|-------|---------|
| `-Q.Job.Is.Active` | Job is currently running |
| `-Q.Job.Is.Paused` | Job is currently paused (any level) |
| `-Q.Job.Is.Throttled` | Job is currently throttled |

#### Job Rules Example

```polyglot
{Q} #JobRules:RAMGuard
   (#) $value.GB#int <~ 4
   (#) $margin.GB#int <~ 1
   [?] $margin.GB >? $value.GB
      [!] >> !Queue.InvalidMargin
   [?] -Q.Job.Is.Active
   [&] -Q.Job.Get.RAM.GB">? {$value.GB}"
      [Q] -Q.Job.Pause.Free.RAM.Wait
   [?] -Q.Job.Is.Paused
   [&] -Q.Job.Get.RAM.GB"<? {$value.GB}-{$margin.GB}"
      [Q] -Q.Job.Resume
   [?] *?
      [Q] -Q.DoNothing
```

#### Queue Rules Example

```polyglot
[ ] Queue-level rule using reassemble operator
{Q} #QueueRules:OverloadProtection
   [Q] =*Agg.Sum
      (=) <array << -Q.Queue.Jobs.Get.RAM.GB
      (*) >sum >> $TotalRAM
   [?] $TotalRAM >? 32.0
      [Q] -Q.Queue.Drain
   [?] *?
      [Q] -Q.DoNothing
```

### Using a Queue (`[Q]`)

The `[Q]` line in a pipeline declares which queue it uses.

#### Queue Assignment

Three equivalent syntaxes:

```polyglot
[Q] >> #WorkerQueue
[Q] -Q.Assign"{#WorkerQueue}"
[Q] -Q.Assign
   (-) <Queue << #WorkerQueue
```

#### Rule Loading

Three equivalent syntaxes:

```polyglot
[Q] << #RAMGuard
[Q] -Q.Load.Job.Rules"{#CPUGuard}"
[Q] -Q.Load.Job.Rules
   (-) <rules << #IdleHibernation
```

#### Full Pipeline Example

```polyglot
{-} ProcessData
   [T] ...
   [ ] Assign Queue
   [Q] >> #WorkerQueue
   [ ] Load job rules
   [Q] << #RAMGuard
   [Q] << #CPUGuard
   [Q] << #IdleHibernation
   [W] -W.Polyglot
   ...
```

#### Pipeline with Custom Threshold Override

```polyglot
{-} HeavyJob
   [T] ...
   [Q] >> #WorkerQueue
   [Q] #RAMGuard
      (#) <value.GB << 8
      (#) >> $customRAMGuard
   [Q] << $customRAMGuard
   [W] -W.Polyglot
   ...
```

### Resource Freeing Spectrum

Each pause level is a strict superset. Free.RAM includes Free.CPU. Free.All includes Free.RAM.

```
Running ---- full CPU, RAM, FDs, TCP, Locks
  |
  v  -Q.Job.Throttle
Throttled -- reduced CPU/RAM/IO, still running
  |
  v  -Q.Job.Pause.Free.CPU
Paused.CPU - CPU freed, RAM+FDs+TCP+Locks kept
  |
  v  -Q.Job.Pause.Free.RAM.Soft
Paused.RAM.Soft - CPU freed, RAM best-effort swapped
  |
  v  -Q.Job.Pause.Free.RAM.Hard
Paused.RAM.Hard - CPU freed, RAM guaranteed freed (OOM-kill risk)
  |
  v  -Q.Job.Pause.Free.All
Paused.All - ALL freed -> serialized to disk files
  |
  v  -Q.Job.Kill.WithCleanup
Dying -------  cleanup [/] running, then ALL freed -> gone
  |
  v  -Q.Job.Kill.Now
Dead --------  ALL freed -> gone (no cleanup)
```

### Timing Modifiers: `.Now` vs `.Wait`

- **`.Now`** — immediate. The runtime freezes the Job instantly. The Job may be mid-operation.
- **`.Wait`** — waits for the current work unit boundary before freezing. The Job finishes its in-progress operation, then stops cleanly. "Work unit boundary" means the smallest atomic unit of work in progress (one loop iteration, one sub-job, one IO operation).

### Temporal Behavior: Pause Reason Set

When multiple rules exist, Resume means "this rule no longer wants the Job paused." The Job only actually resumes when **no rule** wants it paused.

The Trigger Monitor keeps a **pause reason set** per Job. Each rule adds/removes its reason. The Job resumes when the set is empty.

Example: RAM guard pauses (set = `{RAM}`), CPU guard resumes (set still = `{RAM}`) — Job stays paused. When RAM drops below threshold — set becomes `{}` — Job actually resumes.

Conflicting pause levels: the **highest level wins** (Free.All > Free.RAM > Free.CPU).

### Anti-Flap Mechanisms

| Mechanism | Type | How |
|-----------|------|-----|
| `<margin` parameter | Spatial hysteresis | Different thresholds for pause vs resume |
| `.resumeDebounce` | Temporal hysteresis | Set must be empty for N duration before resuming |
| `[?] -Q.Job.Is.Paused` / `Is.Active` | State guards | Compiler-enforced temporal awareness |

### Evaluation Model

All `[Q]` rules evaluate in the same tick (atomic). The pause reason set is rebuilt from scratch each cycle. The tick period is configurable:

```polyglot
{Q} #Queue:WorkerQueue
   [.] .tickPeriod << #DT"5s"
```

### GPU Jobs: Automatic Level Capping

The compiler verifies resource capabilities at compile time. Using an action beyond the max level is a compile error.

| Job Uses | Max Pause Level | Max Spatial | Reason |
|----------|----------------|-------------|--------|
| CPU only | Free.All | Cross-host Reassign, Snapshot | Full CRIU support |
| CPU + TCP (not repairable) | Free.RAM | Snapshot only | TCP can't survive Free.All |
| CPU + TCP (repairable) | Free.All | Cross-host Reassign, Snapshot | TCP\_REPAIR enabled |
| GPU (with vendor plugin) | Free.All | Cross-host Reassign, Snapshot | Plugin handles VRAM |
| GPU (no plugin) | Free.RAM | None | CRIU can't serialize GPU state |
| POSIX named semaphores | Free.RAM | None | CRIU can't checkpoint these |

TCP repairability is declared on the queue:

```polyglot
{Q} #Queue:MigrateableQueue
   [.] .tcpRepairable << true
```

### Default Queue Behaviors

Every queue includes built-in rules unless explicitly overridden:

| Default | Condition | Action |
|---------|-----------|--------|
| RAM overflow prevention | Job RAM approaching host available | `-Q.Job.Pause.Free.RAM.Wait` |
| Disk space guard | Host disk < 1GB | `-Q.Job.Pause.Free.All.Wait` |
| Runaway CPU | Job CPU > 100% for > 5m (single-threaded) | `-Q.Job.Throttle` |
| Zombie cleanup | Job status = finished but process alive | `-Q.Job.Kill.Now` |
| Orphan detection | Job's parent pipeline terminated | `-Q.Job.Kill.WithCleanup` |
| CRIU image cleanup | Paused.All Job not resumed in 24h | Notify + optional Kill |

Override by setting the default to false:

```polyglot
{Q} #Queue:MyQueue
   [.] .defaults.ramOverflow << false
```

## Queue Handler Internals

### Queues vs Sets

The Queue Handler uses two container types:

- **Queue** — Jobs waiting in line to DO something (ordered, dispatched one at a time)
- **Set** — a record of Jobs currently IN a state (not waiting, actively being something)

| Container | Type | Purpose |
|-----------|------|---------|
| Dispatch Queue (per `{Q}`) | Queue | Waiting to be dispatched, ordered by queue strategy |
| Dispatch Queues Coordinator Queue | Queue | Virtual single-slot buffer — holds Tier 1 Selection RR's candidate for Tier 2 |
| Resume Queue | Queue | Resumed Jobs waiting for a dispatch slot |
| Teardown Queue | Queue | Jobs waiting for a cleanup slot to run `[/]` cleanup before termination |
| Executing Set | Set | Currently executing |
| Suspended Set | Set | Suspended — resource freeing level varies per Job |

### The Dispatch Coordinator

The Dispatch Coordinator is the unified dispatch layer that reads from all parallel Dispatch Queues simultaneously. It is not a separate container — it is the autonomous dispatch loop inside the Queue Handler.

Each Dispatch Queue maintains its own internal ordering (FIFO, LIFO, Priority). Each queue proposes its next candidate based on its strategy. The Dispatch Coordinator dispatches Jobs to the Executing Set while being **faithful** to every Dispatch Queue's rules:

- If GPUQueue (LIFO) says D goes before E, the Dispatch Coordinator will never dispatch E before D
- If DefaultQueue (FIFO) says A goes before B, the Dispatch Coordinator will never dispatch B before A
- Between queues there is no imposed ordering — Jobs from different queues can dispatch in any relative order, as long as each queue's internal order is preserved

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
- **Executing Set** → throttle → Throttled (still in Executing Set, reduced resources)
- **Executing Set** → pause (any level) → Suspended Set (resource freeing per level)
- **Suspended Set** → resume → Resume Queue
- **Resume Queue** → dispatched by Dispatch Coordinator → Executing Set (resumes from suspended state)
- **Executing Set** → Kill.WithCleanup → Teardown Queue (waits for cleanup slot)
- **Teardown Queue** → dispatched by Dispatch Coordinator → Executing Set (runs `[/]` cleanup, terminates)
- **Executing Set** → Kill.Now → immediately removed, no cleanup

### Host-Based Load Balancing

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Handler dispatches Jobs to the Runner on that host. To distribute work across multiple hosts, define multiple queues with different `.host` values. Offloading work to another host means switching queues — either explicitly via `-Q.Job.Reassign` or automatically via queue rules.

### Cross-Queue Constraints

The Dispatch Coordinator enforces constraints at multiple scopes. These are declared as `#Queue` properties:

**Queue-level** (declared on `{Q} #Queue:Name` definition):
- `.maxInstancesWithinQueue` — max instances of a specific pipeline within this queue
- `.maxConcurrentWithinQueue` — max total Jobs dispatched from this queue
- `.resourceTagWithinQueue` — resource exclusion within this queue (e.g., only one `#GPU`-tagged Job)

**Pipeline-level** (declared on `[Q]` in the pipeline):
- `.maxInstancesAllQueues` — max instances of a pipeline across all queues
- `.maxInstancesWithinHost` — max instances of a pipeline on a specific host
- `.maxConcurrentAllQueues` — max total Jobs running globally alongside this pipeline
- `.maxConcurrentWithinHost` — max total Jobs on a specific host alongside this pipeline
- `.resourceTagAllQueues` — resource exclusion globally
- `.resourceTagWithinHost` — resource exclusion per host

### Resource Effects

| Level | CPU | RAM | FDs | TCP | Locks | State Location |
|-------|-----|-----|-----|-----|-------|---------------|
| **Free.CPU** | Freed | Kept | Open | Open | Held | Process memory |
| **Free.RAM.Soft** | Freed | Best-effort swap | Open | Open | Held | Swap |
| **Free.RAM.Hard** | Freed | Guaranteed freed | Open | Open | Held | Swap |
| **Free.All** | Freed | Freed | Closed (to disk) | Closed (to disk) | Released (to disk) | CRIU image directory |
| **Kill** | Freed | Freed | Closed | Closed | Released | **Gone forever** |

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[Q]` fits in pipeline element order
- [[concepts/pipelines/wrappers|Wrappers]] — `[W]` wrapper that follows the queue declaration
- [[concepts/pipelines/execution|Execution]] — execution body that runs after queue dispatch
- [[pglib/pipelines/Q/INDEX|u:-Q.* Queue Pipelines]] — full pipeline catalog
