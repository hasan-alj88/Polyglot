---
audience: architect
type: spec
updated: 2026-04-16
---

# End-to-End Flow

<!-- @c:queue-manager/dispatch-coordinator -->
<!-- @c:queue-manager/reactive-signals -->

## Normal Execution

```text
1. Trigger Monitor detects event
   → NATS: publish "polyglot.trigger.fire.ProcessData"
   → TM evaluates retrigger policy, creates job ID, records hierarchy in NoSQL
   → NATS: publish "command.enqueue" {jobId, pipeline, queue, params}

2. Queue Handler receives command.enqueue
   → Redis: RPUSH "queue:dispatch:DefaultQueue" "job:001"
   → Redis: HSET "job:job:001" pipeline ProcessData queue DefaultQueue status pending ...

3. Dispatch Coordinator wakes (enqueue event)
   → Tier 1 Selection RR: pick candidate from Dispatch Queues → job:001 from DefaultQueue
   → Tier 2 Dispatch RR: round-robin between Coordinator Queue, Resume Queue (empty), Teardown Queue (empty) → job:001
   → Check scoped constraints → all clear
   → Redis: LPOP, SADD set:executing, HINCRBY counter:instances
   → NATS: publish "polyglot.queue.control.job:001.start"

4. Runner starts pipeline
   → NATS: publish "polyglot.runner.started.job:001" → QH + TM

5. Pipeline completes
   → NATS: publish "polyglot.runner.completed.job:001" → QH + TM
   → QH Redis: SREM set:executing, HINCRBY counter:instances -1, DEL job:job:001
   → Dispatch Coordinator wakes (slot freed)
```

## Pause / Resume Flow

```polyglot
1. Resource Monitor: RAM drops below threshold
   → NATS: publish "polyglot.resource.ram" {available: 2800}

2. Trigger Monitor evaluates -Q.Job.Pause.Free.RAM.Hard condition → met for job:001
   → NATS: publish "command.job.pause.free.ram.hard" {jobId: job:001, timing: "wait"}

3. Queue Handler receives command.job.pause.free.ram.hard
   → Redis: SREM set:executing job:001
   → Redis: HSET set:suspended job:001 "ram.hard"
   → Redis: HINCRBY counter:instances ProcessData -1
   → Redis: HINCRBY counter:instances:queue:DefaultQueue ProcessData -1
   → Redis: HINCRBY counter:instances:host:{host} ProcessData -1
   → Redis: HSET job:job:001 status "suspended.ram.hard" suspended_at {now}
   → NATS: publish "polyglot.queue.control.job:001.job.pause.free.ram.hard"
   → Dispatch Coordinator wakes (slot freed)

4. Runner suspends process, frees CPU+RAM
   → NATS: publish "polyglot.runner.paused.job:001" {type: "ram.hard"} → QH + TM

5. Queue Handler updates state
   → Redis: HSET job:job:001 confirmed_paused true

6. RAM recovers above threshold
   → NATS: publish "polyglot.resource.ram" {available: 5500}

7. Trigger Monitor evaluates -Q.Job.Resume.RAM.MoreThan → condition met for job:001
   → NATS: publish "command.job.resume" {jobId: job:001}

8. Queue Handler receives command.job.resume
   → Redis: type = HGET set:suspended job:001 → "ram.hard"
   → Redis: HDEL set:suspended job:001
   → Redis: RPUSH queue:resume job:001
   → Redis: HSET job:job:001 status "resuming"
   → Dispatch Coordinator wakes (item added to Resume Queue)

9. Dispatch Coordinator cycles
   → Tier 2 RR includes Resume Queue → job:001
   → Constraints re-checked → all clear
   → Redis: LPOP queue:resume, SADD set:executing, HINCRBY counter:instances
   → NATS: publish "polyglot.queue.control.job:001.job.resume"
```

## Graceful Kill Flow

```polyglot
1. Trigger Monitor evaluates -Q.Job.Kill.WithCleanup condition → met for job:001
   → NATS: publish "command.job.kill.with-cleanup" {jobId: job:001}

2. Queue Handler receives command.job.kill.with-cleanup (job status: executing)
   → Redis: SREM set:executing job:001
   → Redis: RPUSH queue:teardown job:001
   → Redis: HINCRBY counter:instances ProcessData -1
   → Redis: HSET job:job:001 status "teardown.pending"
   → (NO control signal to Runner yet — job waits for cleanup slot)
   → Dispatch Coordinator wakes (slot freed + item in Teardown Queue)

3. Dispatch Coordinator dispatches from Teardown Queue
   → Redis: LPOP queue:teardown, SADD set:executing, HINCRBY counter:instances
   → Redis: HSET job:job:001 status "teardown.executing"
   → NATS: publish "polyglot.queue.control.job:001.job.kill.with-cleanup"

4. Runner finishes current work, runs [/] cleanup, terminates
   → NATS: publish "polyglot.runner.teardown_completed.job:001" → QH + TM
   → QH Redis: SREM set:executing, HINCRBY counter:instances -1, DEL job:job:001
   → Dispatch Coordinator wakes (slot freed)
```

## Sub-Job Flow

When a pipeline hits a `[=]`, `[-]`, or `[b]` marker, the Runner sends a `trigger.subjob` signal to the Trigger Monitor (not the Queue Handler). The Trigger Monitor creates job IDs, records parent→child relationships in NoSQL, and sends `command.enqueue` with `parentJobId` to the Queue Handler. Sub-jobs go through the normal dispatch flow.

```text
1. Runner hits [=] marker in job:001
   → NATS: publish "trigger.subjob" {parentJobId: "job:001", pipeline, marker, params}

2. Trigger Monitor creates sub-job
   → NoSQL: record job:002 as child of job:001
   → NATS: publish "command.enqueue" {jobId: "job:002", pipeline, queue, parentJobId: "job:001"}

3. Queue Handler enqueues sub-job
   → Redis: RPUSH queue:dispatch:{queue} job:002
   → Redis: HSET job:job:002 {pipeline, queue, status: "pending", ...}

4. Normal dispatch flow — sub-jobs are treated like any other job
```

**Kill propagation:** When a parent job is killed, the Trigger Monitor pre-computes the full descendant list from NoSQL and sends individual kill signals for each descendant. The Queue Handler never queries NoSQL — it only processes the kill commands it receives. The queue's `#KillPropagation` setting (`#Cascade` or `#Downgrade`) determines whether sub-jobs receive the same kill type or a downgraded one.

## Dispatch Wait Timeout

```polyglot
1. Trigger Monitor checks maxWaitTime periodically
   → Read job enqueued_at from QH state signals
   → Read queue maxWaitTime from NoSQL
   → enqueued_at + maxWaitTime < now → TIMEOUT

2. Default behavior: strategy-aware escalation
   → NATS: publish "command.dispatch.escalate" {jobId, queue}
   → QH moves job to next-to-dispatch position (strategy-dependent)

3. Alternative: -Q.Dispatch.Wait.TimeOut.Job.Kill.WithCleanup
   → NATS: publish "command.job.kill.with-cleanup" {jobId}

4. Alternative: -Q.Dispatch.Wait.TimeOut.Reassign
   → NATS: publish "command.reassign" {jobId, fromQueue, toQueue}
```

---

See also: [[dispatch-coordinator]], [[reactive-signals]], [[sequence-diagrams]]
