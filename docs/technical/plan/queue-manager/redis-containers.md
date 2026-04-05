---
audience: architect
type: spec
updated: 2026-04-03
---

# Containers in Redis

<!-- @queue-manager/infrastructure -->

## Dispatch Queues (one per `{Q}` definition)

Each `{Q}` definition creates a Dispatch Queue. The Redis data structure depends on the queue's strategy:

| Strategy | Redis Structure | Enqueue | Next Candidate |
|----------|-----------------|---------|----------------|
| FIFO | LIST | `RPUSH` | `LINDEX 0` (peek), `LPOP` (dispatch) |
| LIFO | LIST | `RPUSH` | `LINDEX -1` (peek), `RPOP` (dispatch) |
| Priority | SORTED SET | `ZADD` with priority score | `ZREVRANGE 0 0` (peek), `ZPOPMAX` (dispatch) |

```text
"queue:dispatch:DefaultQueue"     LIST   [jobA, jobB, jobC]
"queue:dispatch:GPUQueue"         LIST   [jobD, jobE]
"queue:dispatch:BatchQueue"       ZSET   {jobF:99, jobG:50, jobH:10}
```

## Dispatch Queues Coordinator Queue

Single-slot virtual queue holding the one candidate selected by Tier 1 Selection RR across all Dispatch Queues. This is the bridge between Tier 1 and Tier 2 dispatch. Not a Redis container — exists only within the Dispatch Coordinator's Lua script execution.

## Resume Queue

Jobs moving from Suspended Set back to execution. Always FIFO (resume in unpause order). Participates as an equal peer in Tier 2 Dispatch RR.

```text
"queue:resume"                    LIST   [jobX, jobY]
```

## Teardown Queue

Jobs that received a graceful kill. Waiting for a cleanup slot to be dispatched for `[/]` cleanup execution. Always FIFO.

```text
"queue:teardown"                  LIST   [jobZ]
```

## Executing Set

Jobs currently running. Used for constraint checks (scoped maxInstances, maxConcurrent, resourceTag).

```text
"set:executing"                   SET    {jobA, jobD, jobF}
```

## Suspended Set

Jobs paused (soft or hard). Tracks pause type for resource accounting.

```text
"set:suspended"                   HASH   {jobX: "soft", jobY: "hard"}
```

## Supporting State

```text
"counter:instances"               HASH   {ProcessData: 3, GPU.Render: 1}
"counter:instances:queue:{name}"  HASH   {ProcessData: 1}
"counter:instances:host:{host}"   HASH   {ProcessData: 2}
"queues:registered"               SET    {DefaultQueue, GPUQueue, BatchQueue}
"queues:draining"                 SET    {BatchQueue}
```

## pipeline:config:{name} (HASH)

Pipeline-level constraints cached at first enqueue. Read by the Dispatch Coordinator during constraint checks.

```yaml
maxInstancesAllQueues:    int?
maxInstancesWithinHost:   int?
maxConcurrentAllQueues:   int?
maxConcurrentWithinHost:  int?
resourceTagAllQueues:     string[]?
resourceTagWithinHost:    string[]?
```

## job:{jobId} (HASH)

```polyglot
pipeline:           string    — pipeline name
queue:              string    — assigned Dispatch Queue
status:             string    — current #QueueState variant
params:             string    — serialized pipeline input parameters
enqueued_at:        string    — ISO timestamp
dispatched_at:      string?   — ISO timestamp
started_at:         string?   — ISO timestamp
suspended_at:       string?   — ISO timestamp
pid:                int?      — OS process ID (from Runner ACK)
confirmed_paused:   bool?     — Runner confirmed suspension
```

## Job ID Format

Each job has a composite identifier:

| Component | Purpose | Example |
|-----------|---------|---------|
| **UID** | Globally unique number | `a8f3c2` |
| **Path** | Position in job hierarchy | `ProcessData/job1/job3` |

The path doubles as the job's address in the hierarchy — parent/child relationships can be derived from it.

## #QueueState

Every job has a `status` field with one of these variants:

```polyglot
#QueueState (7 variants)
├── #Pending              — in a Dispatch Queue, waiting for dispatch
├── #Executing            — in Executing Set, actively running
├── #Suspended.Soft       — in Suspended Set, CPU freed, RAM kept
├── #Suspended.Hard       — in Suspended Set, CPU+RAM freed
├── #Resuming             — in Resume Queue, waiting for dispatch slot
├── #Teardown.Pending     — in Teardown Queue, waiting for cleanup slot
└── #Teardown.Executing   — in Executing Set, running [/] cleanup
```

Each state maps to exactly one container. No `#Killed` or `#Completed` — the job hash is DELeted on those transitions.

Job hierarchy (parentJobId, children) lives ONLY in NoSQL — not in Redis. For kill propagation, the Trigger Monitor reads the hierarchy from NoSQL, pre-computes the full descendant list, and sends individual kill signals for each.

---

See also: [[infrastructure]], [[nosql-schema]], [[dispatch-coordinator]]
