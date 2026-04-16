---
audience: architect
type: spec
updated: 2026-04-15
---

# Signal Data Payloads

<!-- @c:queue-manager/reactive-signals -->
<!-- @u:queue-manager/nats-namespace -->

## Command Signals (Trigger Monitor → Queue Handler)

```yaml
command.enqueue
{
    jobId:          string    — unique job identifier (UID + hierarchy path)
    pipeline:       string    — pipeline name (e.g., "GPU.RenderFrames")
    queue:          string    — target Dispatch Queue name
    params:         serial    — pipeline input parameters
    priority:       int?      — priority score (Priority queues only)
    parentJobId:    string?   — parent job ID (for sub-jobs)
    marker:         string?   — originating marker: [-], [=], [b], [?]
    pipelineConstraints: {    — pipeline-level constraints (cached on first enqueue)
        maxInstancesAllQueues:    int?
        maxInstancesWithinHost:   int?
        maxConcurrentAllQueues:   int?
        maxConcurrentWithinHost:  int?
        resourceTagAllQueues:     string[]?
        resourceTagWithinHost:    string[]?
    }?
}

command.job.pause.free.cpu       { jobId: string, timing: "now"|"wait" }
command.job.pause.free.ram.soft  { jobId: string, timing: "now"|"wait" }
command.job.pause.free.ram.hard  { jobId: string, timing: "now"|"wait" }
command.job.pause.free.all       { jobId: string, timing: "now"|"wait" }
command.job.resume               { jobId: string }
command.job.kill.with-cleanup    { jobId: string }
command.job.kill.now             { jobId: string }

command.job.throttle
{
    jobId:          string    — job to throttle
    cpu:            string?   — cpu.max value (e.g., "50000 100000" = 50%)
    memory:         string?   — memory.high value in bytes
    io:             string?   — io.max value (e.g., "{major}:{minor} rbps={n} wbps={n}")
}

command.job.unthrottle           { jobId: string }

command.job.snapshot
{
    jobId:          string    — job to snapshot (must be executing)
    targetQueue:    string?   — queue for the fork (defaults to same queue as original)
}

command.job.inspect              { jobId: string }

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

```text
state.job.{jobId}.executing     { jobId, pipeline, queue }
state.job.{jobId}.suspended     { jobId, pipeline, type: "cpu"|"ram.soft"|"ram.hard"|"all" }
state.job.{jobId}.resuming      { jobId, pipeline }
state.job.{jobId}.teardown.pending   { jobId, pipeline }
state.job.{jobId}.teardown.completed { jobId, pipeline }
state.job.{jobId}.completed     { jobId, pipeline, result? }
state.job.{jobId}.failed        { jobId, pipeline, error }
state.job.{jobId}.killed        { jobId, pipeline }
state.job.{jobId}.running       { jobId, pipeline, pid }
state.job.{jobId}.confirmed_suspended { jobId, type: "cpu"|"ram.soft"|"ram.hard"|"all" }
state.job.{jobId}.throttled     { jobId, cpu?, memory?, io? }
state.job.{jobId}.unthrottled   { jobId }
state.job.{jobId}.inspected     { jobId, status, queue, pipeline, enqueued_at, dispatched_at,
                                  started_at, suspended_at, pid, throttled, throttle_config,
                                  confirmed_paused, images_dir }
state.job.{jobId}.snapshot_ready { jobId (forkId), images_dir }

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

```text
control.{jobId}.start                    { jobId, pipeline, params }
control.{jobId}.job.resume               { jobId, images_dir? }
control.{jobId}.job.pause.free.cpu       { jobId, timing: "now"|"wait" }
control.{jobId}.job.pause.free.ram.soft  { jobId, timing: "now"|"wait" }
control.{jobId}.job.pause.free.ram.hard  { jobId, timing: "now"|"wait" }
control.{jobId}.job.pause.free.all       { jobId, timing: "now"|"wait" }
control.{jobId}.job.throttle             { jobId, cpu?, memory?, io? }
control.{jobId}.job.unthrottle           { jobId }
control.{jobId}.job.snapshot             { jobId, forkId }
control.{jobId}.job.kill.with-cleanup    { jobId }
control.{jobId}.job.kill.now             { jobId }
```

`images_dir` is included in `control.{jobId}.job.resume` only when resuming from Free.All (type "all"). The Runner uses it to locate the CRIU image directory for `criu restore`.

## Runner Signals (Runner → Queue Handler + Trigger Monitor)

All `runner.*` signals are published to both the Queue Handler and Trigger Monitor. Both subscribe independently — the QH reacts (state updates), the TM calculates (next decisions).

```text
runner.started              { jobId, pid }
runner.completed            { jobId, result? }
runner.teardown_completed   { jobId, pipeline }
runner.failed               { jobId, error }
runner.paused               { jobId, type: "cpu"|"ram.soft"|"ram.hard"|"all", images_dir? }
runner.snapshot_completed   { jobId, forkId, images_dir }
```

## Collector Signals (Trigger Monitor → Runner)

```polyglot
collector.{jobId}.collected
{
    jobId:          string    — parent job ID
    results:        serial[]  — collected sub-job results
    marker:         string    — originating marker: [=], [b]
    collector:      string    — *All, *First, *Nth
}
```

### Collector Reconciliation Logic (TM-internal)

<!-- @c:glossary#Reconciliation -->
Collectors are **Trigger Monitor programs** — algorithms that run inside the TM to determine output selection strategy and job lifecycle policy. The QH has no concept of collectors; it only receives `command.job.kill.*` and `collector.*.collected` signals.

**Core rule:** The TM sends `command.job.kill.with-cleanup` to a job only when **all** collector claims on that job have been released. Each collector independently decides when to release its claim (based on its own algorithm). The TM tracks the claim count per job and acts only when it reaches zero.

**How the TM processes collectors:**

1. When a `[=]` parallel scope is entered, the TM registers all collector programs referencing that scope's job variables
2. When `runner.completed` arrives for a sub-job, the TM evaluates every active collector referencing that job:
   - `*All`: marks this job's output as received; when all referenced jobs report, emits `collector.*.collected`
   - `*First` / `*Nth`: checks if the Nth result has arrived; if so, emits `collector.*.collected` and **releases its claim** on remaining jobs
   - `*Into.*` / `*Agg.*`: accumulates the per-item result; emits `collector.*.collected` when the expand scope completes
3. After evaluating collectors, the TM checks each remaining in-flight job: if **zero** collectors still reference it, the TM emits `command.job.kill.with-cleanup` to the QH
4. If all collectors referencing a job still need it, no kill signal is sent — the job continues

**Compound collector example:** `*First` + `*All` on the same variables. When the first job completes, `*First` is satisfied and releases its claims on the remaining jobs. But `*All` still holds claims on them. The TM sees non-zero claim count → no kill signal. Only when `*All` is also satisfied (all jobs complete) are all claims released.

The Polyglot Code gives the TM foreknowledge of all collector rules at compile time. The compiler validates that collector usage is consistent (e.g., all parallel outputs are collected per PGE03002). The TM implements these rules at runtime exactly as described.

## Trigger Monitor-exclusive signals

Only `trigger.subjob` is Trigger Monitor-exclusive (Runner → TM only):

```yaml
trigger.subjob
{
    parentJobId:    string    — parent job requesting sub-jobs
    pipeline:       string    — pipeline name for sub-jobs
    marker:         string    — [-], [=], [b], [?]
    branches:       int       — number of parallel branches (for [=])
    params:         serial[]  — parameters for each branch
}
```

---

See also: [[reactive-signals]], [[nats-namespace]], [[end-to-end-flow]]
