---
audience: developer
type: architecture
updated: 2026-04-02
status: draft
---

# Queue Manager Architecture

<!-- @concepts/pipelines/queue -->
<!-- @Q -->

The Queue Manager is responsible for dispatching triggered pipelines to execution. It manages multiple Dispatch Queues, lifecycle state transitions (pause, resume, kill), and cross-queue constraint enforcement via the Dispatch Coordinator.

## Infrastructure

The Queue Manager runs as a Rust service backed by two external services:

| Service | Role | License |
|---------|------|---------|
| **NATS JetStream** | Messaging — signals, events, inter-service communication | Apache 2.0 |
| **Redis / Valkey** | State — queue ordering, counters, sets, atomic dispatch | Valkey: BSD 3-Clause |

NATS handles communication. Redis handles runtime state. **NoSQL** stores queue definitions and job hierarchy. The Queue Manager logic (Dispatch Coordinator) runs in Rust.

### Storage Split

| Data | Store | Reason |
|------|-------|--------|
| Queue ordering (LIST/ZSET), Executing/Suspended Sets | Redis | Fast atomic operations, microsecond latency |
| Job runtime state (status, timestamps, pid) | Redis | Frequently read/written by dispatch loop |
| Queue definitions (`{Q}` schema fields) | NoSQL | Immutable at runtime, loaded at startup |
| Job hierarchy (parent→children) | NoSQL | Only read by Trigger Monitor for kill propagation |

### Host-Based Dispatch

Each queue has a `.host` field (default: `"localhost"`). **1 queue = 1 host** — the Queue Manager routes dispatch signals via NATS to the Runner on the target host. Offloading work to another host means switching queues (via `=Q.Reassign` or `=Q.Dispatch.Wait.TimeOut.Reassign`).

## Containers in Redis

### Dispatch Queues (one per `{Q}` definition)

Each `{Q}` definition creates a Dispatch Queue. The Redis data structure depends on the queue's strategy:

| Strategy | Redis Structure | Enqueue | Next Candidate |
|----------|-----------------|---------|----------------|
| FIFO | LIST | `RPUSH` | `LINDEX 0` (peek), `LPOP` (dispatch) |
| LIFO | LIST | `RPUSH` | `LINDEX -1` (peek), `RPOP` (dispatch) |
| Priority | SORTED SET | `ZADD` with priority score | `ZREVRANGE 0 0` (peek), `ZPOPMAX` (dispatch) |

```
"queue:dispatch:DefaultQueue"     LIST   [jobA, jobB, jobC]
"queue:dispatch:GPUQueue"         LIST   [jobD, jobE]
"queue:dispatch:BatchQueue"       ZSET   {jobF:99, jobG:50, jobH:10}
```

### Resume Queue

Pipelines moving from Suspended Set back to execution. Always FIFO (resume in unpause order). Participates as an equal peer in Tier 2 round-robin dispatch.

```
"queue:resume"                    LIST   [jobX, jobY]
```

### Teardown Queue

Pipelines that received a graceful kill. Waiting to finish current work and run `[/]` cleanup. Always FIFO.

```
"queue:teardown"                  LIST   [jobZ]
```

### Executing Set

Pipelines currently running. Used for global constraint checks (maxInstances, maxConcurrent).

```
"set:executing"                   SET    {jobA, jobD, jobF}
```

### Suspended Set

Pipelines paused (soft or hard). Tracks pause type for resource accounting.

```
"set:suspended"                   HASH   {jobX: "soft", jobY: "hard"}
```

### Supporting State

```
"counter:instances"               HASH   {ProcessData: 3, GPU.Render: 1}
"job:{jobId}"                     HASH   {pipeline, queue, status,
                                          enqueued_at, dispatched_at,
                                          started_at, suspended_at, pid}
```

Queue definitions (`{Q}` schema fields) and job hierarchy (parent→children) are stored in **NoSQL**, not Redis. Redis only holds runtime state needed by the dispatch loop.

## Dispatch Coordinator

The Dispatch Coordinator is the unified dispatch logic. It runs as an **atomic Redis Lua script** to prevent race conditions between constraint checks and state mutations.

### Two-Tier Round-Robin Dispatch

No queue has absolute priority. The Dispatch Coordinator uses a two-tier round-robin:

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

### Faithfulness Principle

The Dispatch Coordinator never overrides or reorders a queue's internal strategy. If GPUQueue (LIFO) says D goes before E, the Dispatch Coordinator will never dispatch E before D. Between queues there is no imposed ordering.

### Constraint Checks (per candidate)

Before dispatching any candidate, the Lua script checks:

1. **maxInstances** — `HGET counter:instances {pipeline}` vs pipeline's limit
2. **maxConcurrent** — `SCARD set:executing` vs pipeline's limit
3. **resourceTag** — resource exclusion (e.g., only one `#GPU`-tagged pipeline at a time)

If a candidate fails constraints, it remains in its queue and the Dispatch Coordinator moves to the next candidate from the next queue.

### Atomic Dispatch Operation

When a candidate passes all constraints:

1. Pop candidate from its queue
2. `SADD set:executing {jobId}`
3. `HINCRBY counter:instances {pipeline} 1`
4. Return job ID for NATS dispatch signal

All steps execute atomically within the Lua script.

## NATS Subject Namespace

All inter-service communication flows through NATS subjects:

### Trigger Events (Trigger Monitor → Queue Manager)

```
polyglot.trigger.fire.{pipeline}         — trigger condition met, enqueue pipeline
```

### Dispatch Events (Queue Manager → Runner)

```
polyglot.queue.dispatched.{jobId}        — pipeline dispatched, start executing
```

### Lifecycle Control (Queue Manager → Runner)

```
polyglot.queue.control.{jobId}.pause.soft     — finish current work, then suspend
polyglot.queue.control.{jobId}.pause.hard     — suspend immediately
polyglot.queue.control.{jobId}.resume         — resume execution
polyglot.queue.control.{jobId}.kill.graceful  — finish work + [/] cleanup, then terminate
polyglot.queue.control.{jobId}.kill.hard      — terminate immediately
```

### Runner Acknowledgments (Runner → Queue Manager)

```
polyglot.runner.started.{jobId}          — pipeline is now executing
polyglot.runner.completed.{jobId}        — pipeline finished successfully
polyglot.runner.failed.{jobId}           — pipeline failed
polyglot.runner.paused.{jobId}           — pipeline suspended (ACK)
```

### Resource Monitoring (Resource Monitor → Queue Manager)

```
polyglot.resource.cpu                    — CPU usage metrics
polyglot.resource.ram                    — RAM availability metrics
```

### Pipeline State (Queue Manager → any subscriber)

```
polyglot.state.{pipeline}.{event}        — state changes for cross-pipeline conditions
```

## End-to-End Flow

### Normal Execution

```
1. Trigger Monitor detects event
   → NATS: publish "polyglot.trigger.fire.ProcessData"

2. Queue Manager receives trigger
   → Redis: RPUSH "queue:dispatch:DefaultQueue" "job:001"
   → Redis: HSET "job:job:001" pipeline ProcessData queue DefaultQueue ...

3. Dispatch Coordinator runs (Lua script)
   → Tier 1 RR: pick candidate from Dispatch Queues → job:001 from DefaultQueue
   → Tier 2 RR: round-robin between candidate, Resume Queue (empty), Teardown Queue (empty) → job:001
   → Check constraints → all clear
   → Redis: LPOP, SADD set:executing, HINCRBY counter:instances
   → NATS: publish "polyglot.queue.dispatched.job:001"

4. Runner starts pipeline
   → NATS: publish "polyglot.runner.started.job:001"
   → NATS: publish "polyglot.state.ProcessData.running"

5. Pipeline completes
   → NATS: publish "polyglot.runner.completed.job:001"
   → Redis: SREM set:executing, HINCRBY counter:instances -1, DEL job:job:001
```

### Pause / Resume Flow

```
1. Resource Monitor: RAM drops below threshold
   → NATS: publish "polyglot.resource.ram" {available: 2800}

2. Queue Manager checks conditions → matches =Q.Pause.Hard for job:001
   → NATS: publish "polyglot.queue.control.job:001.pause.hard"

3. Runner suspends process, frees CPU+RAM
   → NATS: publish "polyglot.runner.paused.job:001"

4. Queue Manager updates state
   → Redis: SREM set:executing job:001
   → Redis: HSET set:suspended job:001 "hard"
   → Redis: HINCRBY counter:instances ProcessData -1

5. RAM recovers above threshold
   → NATS: publish "polyglot.resource.ram" {available: 5500}

6. Queue Manager checks resume conditions → matches =Q.Resume
   → Redis: HDEL set:suspended job:001
   → Redis: RPUSH queue:resume job:001

7. Dispatch Coordinator runs
   → Tier 2 RR includes Resume Queue → job:001
   → Constraints re-checked → all clear
   → Redis: LPOP queue:resume, SADD set:executing, HINCRBY counter:instances
   → NATS: publish "polyglot.queue.control.job:001.resume"
```

### Graceful Kill Flow

```
1. Execution time exceeds threshold
   → Queue Manager: matches =Q.Kill.Graceful for job:001

2. Queue Manager updates state
   → Redis: SREM set:executing job:001
   → Redis: RPUSH queue:teardown job:001
   → Redis: HINCRBY counter:instances ProcessData -1

3. Dispatch Coordinator dispatches from Teardown Queue
   → NATS: publish "polyglot.queue.control.job:001.kill.graceful"

4. Runner finishes current work, runs [/] cleanup, terminates
   → NATS: publish "polyglot.runner.completed.job:001"
   → Redis: LPOP queue:teardown, DEL job:job:001
```

### Sub-Job Flow

When a pipeline hits a `[p]`, `[r]`, or `[b]` marker, the Runner sends a `trigger.subjob` signal to the Trigger Monitor (not the Queue Manager). The Trigger Monitor creates job IDs, records parent→child relationships in NoSQL, and sends `command.enqueue` with `parentJobId` to the Queue Manager. Sub-jobs go through the normal dispatch flow.

```
1. Runner hits [p] marker in job:001
   → NATS: publish "trigger.subjob" {parentJobId: "job:001", pipeline, marker, params}

2. Trigger Monitor creates sub-job
   → NoSQL: record job:002 as child of job:001
   → NATS: publish "command.enqueue" {jobId: "job:002", pipeline, queue, parentJobId: "job:001"}

3. Queue Manager enqueues sub-job
   → Redis: RPUSH queue:dispatch:{queue} job:002
   → Redis: HSET job:job:002 {pipeline, queue, status: "pending", ...}

4. Normal dispatch flow — sub-jobs are treated like any other job
```

**Kill propagation:** When a parent job is killed, the Trigger Monitor pre-computes the full descendant list from NoSQL and sends individual `command.kill` signals for each descendant. The Queue Manager never queries NoSQL — it only processes the kill commands it receives. The queue's `#KillPropagation` setting (`#Cascade` or `#Downgrade`) determines whether sub-jobs receive the same kill type or a downgraded one.

## Design Rationale

### Why Redis for State

- **Atomic Lua scripts** — Dispatch Coordinator constraint checks and state mutations execute as one atomic operation. No race conditions.
- **Data structure fit** — LIST for FIFO/LIFO, SORTED SET for Priority, SET for Executing, HASH for Suspended and metadata.
- **Dynamic priority** — `ZADD` / `ZINCRBY` allow real-time reprioritization triggered by events.
- **Lightweight** — single binary, in-memory, microsecond latency.

### Why NATS for Messaging

- **Decoupled services** — Trigger Monitor, Queue Manager, Runner, Resource Monitor communicate only via NATS subjects.
- **Subject-based routing** — `polyglot.queue.control.{jobId}.pause.soft` routes precisely.
- **JetStream persistence** — critical events can be persisted for crash recovery and replay.
- **Embeddable** — NATS server can embed in the Polyglot runtime process.
- **Lightweight** — ~20MB, no external dependencies.

### Why This Split

| Concern | Wrong Service | Right Service |
|---------|--------------|---------------|
| Queue ordering | NATS (no ordering primitives) | Redis (LIST, ZSET) |
| Atomic constraint checks | NATS (no transactions) | Redis (Lua scripts) |
| Event signals | Redis (pub/sub is fire-and-forget) | NATS (persistent, routable) |
| Cross-service communication | Redis (not designed for this) | NATS (designed for this) |

Redis handles state and ordering. NATS handles communication. Neither duplicates the other's work.
