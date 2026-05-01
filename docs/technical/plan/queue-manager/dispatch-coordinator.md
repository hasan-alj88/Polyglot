---
audience: design
type: spec
updated: 2026-04-15
---

# Dispatch Coordinator

<!-- @c:queue-manager/redis-containers -->
<!-- @c:queue-manager/reactive-signals -->

The Dispatch Coordinator is the autonomous dispatch loop inside the Queue Handler. It runs as an **atomic Redis Lua script** to prevent race conditions between constraint checks and state mutations.

## Dispatch Loop

The Dispatch Coordinator is event-driven — it wakes and cycles whenever queue state changes:

**Wake triggers:**
- Job enqueued (any Dispatch Queue)
- Job pushed to Resume Queue
- Job pushed to Teardown Queue
- Job leaves Running Set (completed, paused, killed)

When woken, it runs one full cycle (Tier 1 Selection → Tier 2 Dispatch → constraint check → dispatch). If items remain and slots are available, it cycles again immediately. When all queues are empty or all candidates fail constraints, it sleeps until the next wake event.

The Dispatch Coordinator is not signal-driven — no external `command.dispatch` triggers it. It is an autonomous deterministic loop: given the same Redis state, it always produces the same output.

## Two-Tier Round-Robin Dispatch

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
Dispatch Constraints → Running Set
```

**Tier 1 (Selection RR):** Round-robins across user-defined Dispatch Queues. Each queue proposes exactly one candidate based on its strategy. The selected candidate enters the Dispatch Queues Coordinator Queue — a single-slot virtual buffer.

**Tier 2 (Dispatch RR):** Round-robins across three equal peers: the Dispatch Queues Coordinator Queue, Resume Queue, and Teardown Queue. All are equal participants. The selected candidate proceeds to constraint checks.

## Faithfulness Principle

The Dispatch Coordinator never overrides or reorders a queue's internal strategy. If GPUQueue (LIFO) says D goes before E, the Dispatch Coordinator will never dispatch E before D. Between queues there is no imposed ordering.

## Constraint Checks (per candidate)

Before dispatching any candidate, the Lua script checks scoped constraints from the Redis config cache:

**Queue-level constraints** (from `{Q}` definition, cached at registration):
1. **maxInstancesWithinQueue** — instances of this pipeline in this queue
2. **maxConcurrentWithinQueue** — total jobs dispatched from this queue
3. **resourceTagWithinQueue** — resource exclusion within this queue

**Pipeline-level constraints** (from `[Q]` declaration, stored in job metadata):
4. **maxInstancesAllQueues** — instances of this pipeline across all queues
5. **maxInstancesWithinHost** — instances of this pipeline on this host
6. **maxConcurrentAllQueues** — total jobs running globally alongside this pipeline
7. **maxConcurrentWithinHost** — total jobs on this host alongside this pipeline
8. **resourceTagAllQueues** — resource exclusion globally
9. **resourceTagWithinHost** — resource exclusion on this host

If a candidate fails constraints, it remains in its queue and the Dispatch Coordinator moves to the next candidate from the next queue.

## Atomic Dispatch Operation

When a candidate passes all constraints, the Lua script dispatches based on the source queue:

**From Dispatch Queue:**
1. Pop candidate from its Dispatch Queue
2. `SADD set:running {jobId}`
3. Increment scoped counters (see below)
4. `HSET job:{jobId} status "executing" dispatched_at {now}`
5. Emit `control.{jobId}.start → Runner`

**From Resume Queue:**
1. `LPOP queue:resume`
2. `SADD set:running {jobId}`
3. Increment scoped counters
4. `HSET job:{jobId} status "executing"`
5. Emit `control.{jobId}.job.resume → Runner`

**From Teardown Queue:**
1. `LPOP queue:teardown`
2. `SADD set:running {jobId}`
3. Increment scoped counters
4. `HSET job:{jobId} status "teardown.executing"`
5. Emit `control.{jobId}.job.kill.with-cleanup → Runner`

**Scoped counter increment** (on every dispatch):
```text
HINCRBY counter:instances {pipeline} 1
HINCRBY counter:instances:queue:{queue} {pipeline} 1
HINCRBY counter:instances:host:{host} {pipeline} 1
```

**Scoped counter decrement** (on every removal from Running Set):
```text
HINCRBY counter:instances {pipeline} -1
HINCRBY counter:instances:queue:{queue} {pipeline} -1
HINCRBY counter:instances:host:{host} {pipeline} -1
```

All steps execute atomically within the Lua script. Counters are always incremented at dispatch, decremented at completion/pause/kill.

---

See also: [[redis-containers]], [[reactive-signals]], [[precomputation]], [[properties]]
