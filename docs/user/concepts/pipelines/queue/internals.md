---
audience: automation-builder
type: specification
updated: 2026-04-26
---

<!-- @c:concepts/pipelines/queue/internals -->

# Queue Handler Internals

## Queues vs Sets

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

## The Dispatch Coordinator

The Dispatch Coordinator is the unified dispatch layer that reads from all parallel Dispatch Queues simultaneously. It is not a separate container — it is the autonomous dispatch loop inside the Queue Handler.

Each Dispatch Queue maintains its own internal ordering (FIFO, LIFO, Priority). Each queue proposes its next candidate based on its strategy. The Dispatch Coordinator dispatches Jobs to the Executing Set while being **faithful** to every Dispatch Queue's rules:

- If GPUQueue (LIFO) says D goes before E, the Dispatch Coordinator will never dispatch E before D
- If DefaultQueue (FIFO) says A goes before B, the Dispatch Coordinator will never dispatch B before A
- Between queues there is no imposed ordering — Jobs from different queues can dispatch in any relative order, as long as each queue's internal order is preserved

The Dispatch Coordinator is event-driven — it wakes whenever queue state changes (enqueue, resume, teardown, slot freed) and cycles until all queues are empty or all candidates fail constraints.

## Two-Tier Round-Robin Dispatch

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

## State Transitions

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

## Host-Based Load Balancing

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Handler dispatches Jobs to the Runner on that host. To distribute work across multiple hosts, define multiple queues with different `.host` values. Offloading work to another host means switching queues — either explicitly via `-Q.Job.Reassign` or automatically via queue rules.

## Cross-Queue Constraints

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

## Resource Freeing Spectrum

Each pause level is a strict superset. Free.RAM includes Free.CPU. Free.All includes Free.RAM.

```text
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

### Resource Effects

| Level | CPU | RAM | FDs | TCP | Locks | State Location |
|-------|-----|-----|-----|-----|-------|---------------|
| **Free.CPU** | Freed | Kept | Open | Open | Held | Process memory |
| **Free.RAM.Soft** | Freed | Best-effort swap | Open | Open | Held | Swap |
| **Free.RAM.Hard** | Freed | Guaranteed freed | Open | Open | Held | Swap |
| **Free.All** | Freed | Freed | Closed (to disk) | Closed (to disk) | Released (to disk) | CRIU image directory |
| **Kill** | Freed | Freed | Closed | Closed | Released | **Gone forever** |

### Timing Modifiers: `.Now` vs `.Wait`

- **`.Now`** — immediate. The runtime freezes the Job instantly. The Job may be mid-operation.
- **`.Wait`** — waits for the current work unit boundary before freezing. The Job finishes its in-progress operation, then stops cleanly. "Work unit boundary" means the smallest atomic unit of work in progress (one loop iteration, one sub-job, one IO operation).

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

```aljam3
{Q} #Queue:MigrateableQueue
   [.] .tcpRepairable << true
```

## See Also

- [[concepts/pipelines/queue/INDEX|Queue Configuration]] — Core queue setup and block types
- [[concepts/pipelines/queue/rules|Queue Rules & Triggers]] — Defining queue rules and the reactive trigger engine
- [[aj3lib/pipelines/Q/INDEX|u:-Q.* Queue Pipelines]] — full pipeline catalog
