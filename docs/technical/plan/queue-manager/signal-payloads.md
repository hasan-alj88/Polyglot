---
audience: architect
type: spec
updated: 2026-04-03
---

# Signal Data Payloads

<!-- @queue-manager/reactive-signals -->
<!-- @queue-manager/nats-namespace -->

## Command Signals (Trigger Monitor → Queue Handler)

```
command.enqueue
{
    jobId:          string    — unique job identifier (UID + hierarchy path)
    pipeline:       string    — pipeline name (e.g., "GPU.RenderFrames")
    queue:          string    — target Dispatch Queue name
    params:         serial    — pipeline input parameters
    priority:       int?      — priority score (Priority queues only)
    parentJobId:    string?   — parent job ID (for sub-jobs)
    marker:         string?   — originating marker: [r], [p], [b], [?]
    pipelineConstraints: {    — pipeline-level constraints (cached on first enqueue)
        maxInstancesAllQueues:    int?
        maxInstancesWithinHost:   int?
        maxConcurrentAllQueues:   int?
        maxConcurrentWithinHost:  int?
        resourceTagAllQueues:     string[]?
        resourceTagWithinHost:    string[]?
    }?
}

command.pause.soft  { jobId: string }
command.pause.hard  { jobId: string }
command.resume      { jobId: string }
command.kill.graceful { jobId: string }
command.kill.hard   { jobId: string }

command.priority.update
{
    jobId:          string    — job to reprioritize (Priority queues only)
    score:          int       — new priority score
}

command.dispatch.escalate
{
    jobId:          string    — job to escalate
    queue:          string    — queue containing the job
}

command.reassign
{
    jobId:          string    — job to move
    fromQueue:      string    — source queue
    toQueue:        string    — destination queue
    priority:       int?      — priority score (if destination is Priority queue)
}

command.queue.register
{
    name:           string    — queue name (e.g., "GPUQueue")
    strategy:       string    — FIFO | LIFO | Priority
    host:           string    — target host
    maxInstancesWithinQueue:  int
    maxConcurrentWithinQueue: int
    resourceTagWithinQueue:   string[]
    killPropagation: string   — Cascade | Downgrade
    maxWaitTime:    string    — max queue wait time
    description:    string    — human-readable
}

command.queue.update
{
    name:           string    — queue name
    ...changed fields
}

command.drain   { queue: string }
command.flush   { queue: string }
```

## State Signals (Queue Handler → Trigger Monitor)

```
state.job.{jobId}.executing     { jobId, pipeline, queue }
state.job.{jobId}.suspended     { jobId, pipeline, type: "soft"|"hard" }
state.job.{jobId}.resuming      { jobId, pipeline }
state.job.{jobId}.teardown.pending   { jobId, pipeline }
state.job.{jobId}.teardown.completed { jobId, pipeline }
state.job.{jobId}.completed     { jobId, pipeline, result? }
state.job.{jobId}.failed        { jobId, pipeline, error }
state.job.{jobId}.killed        { jobId, pipeline }
state.job.{jobId}.running       { jobId, pipeline, pid }
state.job.{jobId}.confirmed_suspended { jobId, type: "soft"|"hard" }

state.executing.count           { count, members: string[] }

state.queue.{queue}.enqueued    { jobId, pipeline, queue_size }
state.queue.{queue}.dequeued    { jobId }
state.queue.{queue}.updated     { jobId, new_score }
state.queue.{queue}.escalated   { jobId, strategy }
state.queue.{queue}.registered  { name, strategy }
state.queue.{queue}.draining    { name }
state.queue.{queue}.flushed     { name, killed_count }
state.queue.resume.size         { count }
```

## Control Signals (Queue Handler → Runner)

```
control.{jobId}.start           { jobId, pipeline, params }
control.{jobId}.resume          { jobId }
control.{jobId}.pause.soft      { jobId }
control.{jobId}.pause.hard      { jobId }
control.{jobId}.kill.graceful   { jobId }
control.{jobId}.kill.hard       { jobId }
```

## Runner Signals (Runner → Queue Handler + Trigger Monitor)

All `runner.*` signals are published to both the Queue Handler and Trigger Monitor. Both subscribe independently — the QH reacts (state updates), the TM calculates (next decisions).

```
runner.started              { jobId, pid }
runner.completed            { jobId, result? }
runner.teardown_completed   { jobId, pipeline }
runner.failed               { jobId, error }
runner.paused               { jobId, type: "soft"|"hard" }
```

## Collector Signals (Trigger Monitor → Runner)

```
collector.{jobId}.collected
{
    jobId:          string    — parent job ID
    results:        serial[]  — collected sub-job results
    marker:         string    — originating marker: [p], [b]
    collector:      string    — *All, *First, *Nth
}
```

## Trigger Monitor-exclusive signals

Only `trigger.subjob` is Trigger Monitor-exclusive (Runner → TM only):

```
trigger.subjob
{
    parentJobId:    string    — parent job requesting sub-jobs
    pipeline:       string    — pipeline name for sub-jobs
    marker:         string    — [r], [p], [b], [?]
    branches:       int       — number of parallel branches (for [p])
    params:         serial[]  — parameters for each branch
}
```

---

See also: [[reactive-signals]], [[nats-namespace]], [[end-to-end-flow]]
