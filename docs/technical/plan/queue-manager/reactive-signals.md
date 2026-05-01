---
audience: design
type: spec
updated: 2026-04-15
---

# Reactive Signal Table

<!-- @c:queue-manager/dispatch-coordinator -->
<!-- @u:queue-manager/signal-payloads -->

`f(signal, state) → (new_state, output_signals)`

Every signal is a command from the Trigger Monitor (or ACK from Runner). The Queue Handler just obeys.

## command.enqueue

```text
Input:  {jobId, pipeline, queue, params, priority?, parentJobId?, marker?,
         pipelineConstraints?: {maxInstancesAllQueues?, maxInstancesWithinHost?,
                                maxConcurrentAllQueues?, maxConcurrentWithinHost?,
                                resourceTagAllQueues?, resourceTagWithinHost?}}

Precondition:
    IF queue IN queues:draining → reject

State Write:
    IF strategy == Priority:
        ZADD queue:dispatch:{queue} {priority} {jobId}
    ELSE:
        RPUSH queue:dispatch:{queue} {jobId}
    HSET job:{jobId} {pipeline, queue, status: "pending", enqueued_at}
    IF pipelineConstraints AND NOT EXISTS pipeline:config:{pipeline}:
        HSET pipeline:config:{pipeline} {pipelineConstraints}

Output: state.queue.{queue}.enqueued {jobId, pipeline, queue_size}

Wake:   Dispatch Coordinator
```

## Pause Signals

All four pause signals share the same QH state write pattern — the only differences are the suspended type written to Redis and the control signal forwarded to the Runner. The `timing` field ("now" or "wait") does not affect the QH state write; it is passed through to the Runner control signal, which determines whether the freeze is immediate or waits for a work unit boundary.

### command.job.pause.free.cpu

```text
Input:  {jobId, timing: "now"|"wait"}

State Write:
    SREM set:running {jobId}
    HSET set:suspended {jobId} "cpu"
    HINCRBY counter:instances {pipeline} -1
    HINCRBY counter:instances:queue:{queue} {pipeline} -1
    HINCRBY counter:instances:host:{host} {pipeline} -1
    HSET job:{jobId} status "suspended.cpu" suspended_at {now}

Output: state.job.{jobId}.suspended {type: "cpu"}
        state.executing.count {n}
        control.{jobId}.job.pause.free.cpu {timing} → Runner

Wake:   Dispatch Coordinator (slot freed)
```

### command.job.pause.free.ram.soft

```text
Input:  {jobId, timing: "now"|"wait"}

State Write:
    SREM set:running {jobId}
    HSET set:suspended {jobId} "ram.soft"
    HINCRBY counter:instances {pipeline} -1
    HINCRBY counter:instances:queue:{queue} {pipeline} -1
    HINCRBY counter:instances:host:{host} {pipeline} -1
    HSET job:{jobId} status "suspended.ram.soft" suspended_at {now}

Output: state.job.{jobId}.suspended {type: "ram.soft"}
        state.executing.count {n}
        control.{jobId}.job.pause.free.ram.soft {timing} → Runner

Wake:   Dispatch Coordinator (slot freed)
```

### command.job.pause.free.ram.hard

```text
Input:  {jobId, timing: "now"|"wait"}

State Write:
    SREM set:running {jobId}
    HSET set:suspended {jobId} "ram.hard"
    HINCRBY counter:instances {pipeline} -1
    HINCRBY counter:instances:queue:{queue} {pipeline} -1
    HINCRBY counter:instances:host:{host} {pipeline} -1
    HSET job:{jobId} status "suspended.ram.hard" suspended_at {now}

Output: state.job.{jobId}.suspended {type: "ram.hard"}
        state.executing.count {n}
        control.{jobId}.job.pause.free.ram.hard {timing} → Runner

Wake:   Dispatch Coordinator (slot freed)
```

### command.job.pause.free.all

```text
Input:  {jobId, timing: "now"|"wait"}

State Write:
    SREM set:running {jobId}
    HSET set:suspended {jobId} "all"
    HINCRBY counter:instances {pipeline} -1
    HINCRBY counter:instances:queue:{queue} {pipeline} -1
    HINCRBY counter:instances:host:{host} {pipeline} -1
    HSET job:{jobId} status "suspended.all" suspended_at {now}

Output: state.job.{jobId}.suspended {type: "all"}
        state.executing.count {n}
        control.{jobId}.job.pause.free.all {timing} → Runner

Wake:   Dispatch Coordinator (slot freed)
```

Note: For Free.All, the Runner performs a CRIU checkpoint and ACKs with `runner.paused {jobId, type: "all", images_dir}`. The QH then stores `images_dir` on the job hash (see `runner.paused` ACK below).

## command.job.resume

Handles all suspended types. The QH checks the suspended type to determine which control signal to send to the Runner when the job is dispatched from the Resume Queue.

```text
Input:  {jobId}

State Write:
    type = HGET set:suspended {jobId}
    HDEL set:suspended {jobId}
    RPUSH queue:resume {jobId}
    HSET job:{jobId} status "resuming"

Output: state.job.{jobId}.resuming
        state.queue.resume.size {n}

Wake:   Dispatch Coordinator (item added to Resume Queue)
```

When the Dispatch Coordinator dispatches this job from the Resume Queue, the control signal varies by prior suspended type:
- `"cpu"`, `"ram.soft"`, `"ram.hard"` → `control.{jobId}.job.resume` (cgroup thaw)
- `"all"` → `control.{jobId}.job.resume {images_dir}` (criu restore from disk)

## command.job.kill.with-cleanup

```text
Input:  {jobId}

State Write:
    status = HGET job:{jobId} status
    SWITCH status:
        "executing" | "executing.throttled":
            SREM set:running {jobId}
            HINCRBY counter:instances {pipeline} -1
            HINCRBY counter:instances:queue:{queue} {pipeline} -1
            HINCRBY counter:instances:host:{host} {pipeline} -1
        "teardown.executing":
            SREM set:running {jobId}
            HINCRBY counter:instances {pipeline} -1
            HINCRBY counter:instances:queue:{queue} {pipeline} -1
            HINCRBY counter:instances:host:{host} {pipeline} -1
        "suspended.cpu" | "suspended.ram.soft" | "suspended.ram.hard" | "suspended.all":
            HDEL set:suspended {jobId}
        "resuming":
            LREM queue:resume {jobId}
        "pending":
            LREM/ZREM queue:dispatch:{queue} {jobId}
    RPUSH queue:teardown {jobId}
    HSET job:{jobId} status "teardown.pending"

Output: state.job.{jobId}.teardown.pending
        state.executing.count {n} (if was executing)
        (NO control signal — Runner not told yet, job waits for cleanup slot)

Wake:   Dispatch Coordinator (item added to Teardown Queue; slot freed if was executing)
```

## command.job.kill.now

```text
Input:  {jobId}

State Write:
    status = HGET job:{jobId} status
    SWITCH status:
        "executing" | "executing.throttled" | "teardown.executing":
            SREM set:running {jobId}
            HINCRBY counter:instances {pipeline} -1
            HINCRBY counter:instances:queue:{queue} {pipeline} -1
            HINCRBY counter:instances:host:{host} {pipeline} -1
        "suspended.cpu" | "suspended.ram.soft" | "suspended.ram.hard" | "suspended.all":
            HDEL set:suspended {jobId}
        "teardown.pending":
            LREM queue:teardown {jobId}
        "resuming":
            LREM queue:resume {jobId}
        "pending":
            LREM/ZREM queue:dispatch:{queue} {jobId}
    DEL job:{jobId}

Output: state.job.{jobId}.killed
        state.executing.count {n} (if was executing)
        control.{jobId}.job.kill.now → Runner (if was executing or teardown.executing)

Wake:   Dispatch Coordinator (slot freed if was executing)
```

## command.priority.update

Only valid for Priority strategy queues.

```yaml
Input:  {jobId, score}

State Write:
    ZADD queue:dispatch:{queue} {score} {jobId}

Output: state.queue.{queue}.updated {jobId, new_score}
```

## command.dispatch.escalate

Strategy-aware timeout escalation. Moves a job to the next-to-dispatch position in its queue.

```text
Input:  {jobId, queue}

State Write:
    IF strategy == Priority:
        ZADD queue:dispatch:{queue} MAX_SCORE {jobId}
    ELIF strategy == FIFO:
        LREM queue:dispatch:{queue} {jobId}
        LPUSH queue:dispatch:{queue} {jobId}
    ELIF strategy == LIFO:
        LREM queue:dispatch:{queue} {jobId}
        RPUSH queue:dispatch:{queue} {jobId}

Output: state.queue.{queue}.escalated {jobId, strategy}
```

## command.reassign

Atomic queue transfer. Replaces the former `command.dequeue` + `command.enqueue` pair.

```text
Input:  {jobId, fromQueue, toQueue, priority?}

State Write (single Lua script):
    LREM/ZREM queue:dispatch:{fromQueue} {jobId}
    IF toQueue strategy == Priority:
        ZADD queue:dispatch:{toQueue} {priority} {jobId}
    ELSE:
        RPUSH queue:dispatch:{toQueue} {jobId}
    HSET job:{jobId} queue {toQueue}

Output: state.queue.{fromQueue}.dequeued {jobId}
        state.queue.{toQueue}.enqueued {jobId}

Wake:   Dispatch Coordinator
```

Used by `=Q.Dispatch.Wait.TimeOut.Reassign` — the Trigger Monitor sends a single `command.reassign` to move a job between queues atomically.

## command.queue.register

```text
Input:  {name, strategy, host, maxInstancesWithinQueue, maxConcurrentWithinQueue,
         resourceTagWithinQueue, killPropagation, maxWaitTime, description}

State Write:
    SADD queues:registered {name}
    HSET queue:config:{name} {all constraint fields}
    Create Redis container (LIST or ZSET based on strategy)

Output: state.queue.{name}.registered {strategy}
```

The Trigger Monitor reads the full definition from NoSQL and sends all properties in the signal payload. The Queue Handler never queries NoSQL.

## command.queue.update

```yaml
Input:  {name, ...changed fields}

State Write:
    HSET queue:config:{name} {changed fields}

Output: state.queue.{name}.updated {changed_fields}
```

Sent by the Trigger Monitor when queue properties change at runtime.

## command.drain

Stop accepting new enqueues to a queue. Existing jobs continue to dispatch and complete normally.

```yaml
Input:  {queue}

State Write:
    SADD queues:draining {queue}

Output: state.queue.{queue}.draining
```

Subsequent `command.enqueue` to a draining queue is rejected.

## command.flush

Hard kill the entire queue — equivalent to `command.job.kill.now` for every job that belongs to this queue, regardless of current state. Fire alarm: everyone leaves immediately.

```text
Input:  {queue}

State Write:
    FOR each jobId in queue:dispatch:{queue}:
        LREM/ZREM queue:dispatch:{queue} {jobId}
        DEL job:{jobId}
    FOR each jobId in set:running WHERE job.queue == {queue}:
        SREM set:running {jobId}
        Decrement scoped counters
        DEL job:{jobId}
    FOR each jobId in set:suspended WHERE job.queue == {queue}:
        HDEL set:suspended {jobId}
        DEL job:{jobId}
    FOR each jobId in queue:resume WHERE job.queue == {queue}:
        LREM queue:resume {jobId}
        DEL job:{jobId}
    FOR each jobId in queue:teardown WHERE job.queue == {queue}:
        LREM queue:teardown {jobId}
        DEL job:{jobId}
    DEL queue:dispatch:{queue}
    SREM queues:registered {queue}

Output: state.queue.{queue}.flushed {killed_count}
        control.{jobId}.job.kill.now → Runner (for each executing/teardown.executing job)

Wake:   Dispatch Coordinator (slots freed)
```

## runner.started (ACK)

```yaml
Input:  {jobId, pid}

State Write:
    HSET job:{jobId} pid {pid} started_at {now}

Output: state.job.{jobId}.running {pipeline, pid}
```

## runner.paused (ACK)

```text
Input:  {jobId, type: "cpu"|"ram.soft"|"ram.hard"|"all", images_dir?: string}

State Write:
    HSET job:{jobId} confirmed_paused true
    IF type == "all" AND images_dir:
        HSET job:{jobId} images_dir {images_dir}

Output: state.job.{jobId}.confirmed_suspended {type}
```

For Free.All, the Runner includes `images_dir` — the path to the CRIU image directory. The QH stores this on the job hash so it can be included in the resume control signal later.

## runner.completed

Normal completion only — not for teardown jobs.

```text
Input:  {jobId, result}

State Write:
    SREM set:running {jobId}
    HINCRBY counter:instances {pipeline} -1
    DEL job:{jobId}

Output: state.job.{jobId}.completed {pipeline, result}
        state.executing.count {n}

Wake:   Dispatch Coordinator (slot freed)
```

## runner.teardown_completed

Completion of `[/]` cleanup after graceful kill.

```text
Input:  {jobId, pipeline}

State Write:
    SREM set:running {jobId}
    HINCRBY counter:instances {pipeline} -1
    DEL job:{jobId}

Output: state.job.{jobId}.teardown.completed {pipeline}
        state.executing.count {n}

Wake:   Dispatch Coordinator (slot freed)
```

## runner.failed

```text
Input:  {jobId, error}

State Write:
    SREM set:running {jobId}
    HINCRBY counter:instances {pipeline} -1
    DEL job:{jobId}

Output: state.job.{jobId}.failed {pipeline, error}
        state.executing.count {n}

Wake:   Dispatch Coordinator (slot freed)
```

## command.job.throttle

Reduces resource allocation without pausing. The job stays in `set:running` and keeps its dispatch slot.

```text
Input:  {jobId, cpu?, memory?, io?}

Precondition:
    status = HGET job:{jobId} status
    IF status != "executing" → reject (can only throttle executing jobs)

State Write:
    HSET job:{jobId} status "executing.throttled"
                     throttled true
                     throttle_config {cpu, memory, io}

Output: state.job.{jobId}.throttled {cpu, memory, io}
        control.{jobId}.job.throttle {cpu, memory, io} → Runner
```

No Wake — no slot change (job remains executing).

## command.job.unthrottle

Restores full resource allocation.

```text
Input:  {jobId}

State Write:
    HSET job:{jobId} status "executing"
                     throttled false
    HDEL job:{jobId} throttle_config

Output: state.job.{jobId}.unthrottled
        control.{jobId}.job.unthrottle → Runner
```

## command.job.snapshot

Point-in-time state fork. The original job continues running. The QH auto-enqueues the fork as a new job.

```text
Input:  {jobId, targetQueue?}

Precondition:
    status = HGET job:{jobId} status
    IF status != "executing" → reject (can only snapshot executing jobs)

State Write:
    targetQueue = targetQueue ?? HGET job:{jobId} queue
    forkId = generate_uid()
    pipeline = HGET job:{jobId} pipeline
    IF targetQueue strategy == Priority:
        ZADD queue:dispatch:{targetQueue} {default_priority} {forkId}
    ELSE:
        RPUSH queue:dispatch:{targetQueue} {forkId}
    HSET job:{forkId} pipeline {pipeline} queue {targetQueue}
                       status "pending" enqueued_at {now}
                       forked_from {jobId}

Output: state.job.{forkId}.enqueued {pipeline, queue: targetQueue, forked_from: jobId}
        state.queue.{targetQueue}.enqueued {forkId, pipeline, queue_size}
        control.{jobId}.job.snapshot {forkId} → Runner

Wake:   Dispatch Coordinator (item enqueued)
```

The Runner performs `criu dump --leave-running` and ACKs with `runner.snapshot_completed`. The fork's CRIU images are associated with `forkId` — when `forkId` is dispatched, the Runner uses `criu restore` to start it.

## runner.snapshot_completed (ACK)

```text
Input:  {jobId, forkId, images_dir}

State Write:
    HSET job:{forkId} images_dir {images_dir}

Output: state.job.{forkId}.snapshot_ready {images_dir}
```

## command.job.inspect

Read-only query — no state mutation. Returns the job's Redis hash contents.

```text
Input:  {jobId}

State Write:
    (none — read-only)

Output: state.job.{jobId}.inspected {
            status, queue, pipeline, enqueued_at, dispatched_at,
            started_at, suspended_at, pid, throttled, throttle_config,
            confirmed_paused, images_dir
        }
```

Resource metrics (CPU, RAM, IO, disk) are not included — those come from the Resource Monitor via separate `aljam3.resource.*` subjects.

---

See also: [[dispatch-coordinator]], [[signal-payloads]], [[nats-namespace]]
