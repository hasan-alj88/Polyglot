---
audience: architect
type: spec
updated: 2026-04-03
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

## command.pause.soft

```text
Input:  {jobId}

State Write:
    SREM set:executing {jobId}
    HSET set:suspended {jobId} "soft"
    HINCRBY counter:instances {pipeline} -1
    HSET job:{jobId} status "suspended.soft" suspended_at {now}

Output: state.job.{jobId}.suspended {type: "soft"}
        state.executing.count {n}
        control.{jobId}.pause.soft → Runner

Wake:   Dispatch Coordinator (slot freed)
```

## command.pause.hard

```text
Input:  {jobId}

State Write:
    SREM set:executing {jobId}
    HSET set:suspended {jobId} "hard"
    HINCRBY counter:instances {pipeline} -1
    HSET job:{jobId} status "suspended.hard" suspended_at {now}

Output: state.job.{jobId}.suspended {type: "hard"}
        state.executing.count {n}
        control.{jobId}.pause.hard → Runner

Wake:   Dispatch Coordinator (slot freed)
```

## command.resume

```text
Input:  {jobId}

State Write:
    HDEL set:suspended {jobId}
    RPUSH queue:resume {jobId}
    HSET job:{jobId} status "resuming"

Output: state.job.{jobId}.resuming
        state.queue.resume.size {n}

Wake:   Dispatch Coordinator (item added to Resume Queue)
```

## command.kill.graceful

```text
Input:  {jobId}

State Write:
    status = HGET job:{jobId} status
    SWITCH status:
        "executing":
            SREM set:executing {jobId}
            HINCRBY counter:instances {pipeline} -1
        "teardown.executing":
            SREM set:executing {jobId}
            HINCRBY counter:instances {pipeline} -1
        "suspended.soft" | "suspended.hard":
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

## command.kill.hard

```text
Input:  {jobId}

State Write:
    status = HGET job:{jobId} status
    SWITCH status:
        "executing" | "teardown.executing":
            SREM set:executing {jobId}
            HINCRBY counter:instances {pipeline} -1
        "suspended.soft" | "suspended.hard":
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
        control.{jobId}.kill.hard → Runner (if was executing or teardown.executing)

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

Hard kill the entire queue — equivalent to `command.kill.hard` for every job that belongs to this queue, regardless of current state. Fire alarm: everyone leaves immediately.

```text
Input:  {queue}

State Write:
    FOR each jobId in queue:dispatch:{queue}:
        LREM/ZREM queue:dispatch:{queue} {jobId}
        DEL job:{jobId}
    FOR each jobId in set:executing WHERE job.queue == {queue}:
        SREM set:executing {jobId}
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
        control.{jobId}.kill.hard → Runner (for each executing/teardown.executing job)

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

```yaml
Input:  {jobId, type}

State Write:
    HSET job:{jobId} confirmed_paused true

Output: state.job.{jobId}.confirmed_suspended {type}
```

## runner.completed

Normal completion only — not for teardown jobs.

```text
Input:  {jobId, result}

State Write:
    SREM set:executing {jobId}
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
    SREM set:executing {jobId}
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
    SREM set:executing {jobId}
    HINCRBY counter:instances {pipeline} -1
    DEL job:{jobId}

Output: state.job.{jobId}.failed {pipeline, error}
        state.executing.count {n}

Wake:   Dispatch Coordinator (slot freed)
```

---

See also: [[dispatch-coordinator]], [[signal-payloads]], [[nats-namespace]]
