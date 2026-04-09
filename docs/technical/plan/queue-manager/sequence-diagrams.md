---
audience: architect
type: spec
updated: 2026-04-03
---

# Sequence Diagrams

<!-- @queue-manager/end-to-end-flow -->
<!-- @queue-manager/dispatch-coordinator -->

## 1. Normal Dispatch Flow

```mermaid
sequenceDiagram
    participant TM as Trigger Monitor
    participant QH as Queue Handler
    participant DC as Dispatch Coordinator
    participant Redis
    participant Runner

    TM->>QH: command.enqueue {jobId, pipeline, queue}
    QH->>Redis: RPUSH queue:dispatch:{queue} {jobId}
    QH->>Redis: HSET job:{jobId} {pipeline, queue, status: "pending"}
    QH-->>TM: state.queue.{queue}.enqueued {jobId, queue_size}

    Note over DC: Dispatch Coordinator wakes (enqueue event)
    DC->>Redis: Peek all Dispatch Queues (Tier 1 Selection RR)
    DC->>Redis: Read cached constraints from queue:config:{queue}
    DC->>Redis: Read counter:instances, set:executing
    Note over DC: Apply scoped constraints per candidate
    DC->>Redis: LPOP queue, SADD set:executing, HINCRBY counter:instances
    QH-->>TM: state.job.{jobId}.executing
    QH-->>TM: state.executing.count {n}
    QH->>Runner: control.{jobId}.start {pipeline, params}

    Runner-->>QH: runner.started {jobId, pid}
    Runner-->>TM: runner.started {jobId, pid}
    QH->>Redis: HSET job:{jobId} pid, started_at
    QH-->>TM: state.job.{jobId}.running {pid}

    Runner-->>QH: runner.completed {jobId, result}
    Runner-->>TM: runner.completed {jobId, result}
    QH->>Redis: SREM set:executing, HINCRBY counter:instances -1, DEL job
    QH-->>TM: state.job.{jobId}.completed
    QH-->>TM: state.executing.count {n}
    Note over DC: Dispatch Coordinator wakes (slot freed)
```

## 2. Sub-job Creation (parallel branch)

```mermaid
sequenceDiagram
    participant TM as Trigger Monitor
    participant QH as Queue Handler
    participant Redis
    participant NoSQL
    participant Runner as Runner (parent)
    participant SubRunner as Runner (sub-job)

    Note over Runner: Parent job hits [=] parallel branch
    Runner->>TM: trigger.subjob {parentJobId, pipeline, marker: "[=]", branches: 3}
    
    loop For each parallel branch
        TM->>NoSQL: Record job hierarchy (parent→child)
        TM->>QH: command.enqueue {jobId: sub-1, pipeline, queue, parentJobId}
        QH->>Redis: RPUSH queue:dispatch:{queue} {sub-1}
        QH->>Redis: HSET job:{sub-1} {pipeline, queue, status: "pending"}
    end

    Note over QH: Dispatch Coordinator wakes, dispatches sub-jobs
    QH->>SubRunner: control.{sub-1}.start
    QH->>SubRunner: control.{sub-2}.start
    QH->>SubRunner: control.{sub-3}.start

    SubRunner-->>QH: runner.completed {sub-1}
    SubRunner-->>TM: runner.completed {sub-1}
    SubRunner-->>QH: runner.completed {sub-2}
    SubRunner-->>TM: runner.completed {sub-2}
    SubRunner-->>QH: runner.completed {sub-3}
    SubRunner-->>TM: runner.completed {sub-3}

    QH-->>TM: state.job.{sub-1}.completed
    QH-->>TM: state.job.{sub-2}.completed
    QH-->>TM: state.job.{sub-3}.completed

    Note over TM: Collector (*All) sees all sub-jobs done
    TM->>Runner: collector.{parentJobId}.collected {results}
    Note over Runner: Parent job continues to next marker
```

## 3. Kill Propagation (Cascade)

```mermaid
sequenceDiagram
    participant TM as Trigger Monitor
    participant QH as Queue Handler
    participant Redis
    participant NoSQL
    participant Runner as Runner (parent)
    participant SubRunner as Runner (sub-jobs)

    Note over TM: Decision to kill parent job
    TM->>NoSQL: Read job hierarchy → {sub-1, sub-2}
    TM->>NoSQL: Read killPropagation → #Cascade

    Note over TM: Cascade: same kill type to all descendants
    TM->>QH: command.kill.graceful {jobId: parent}
    TM->>QH: command.kill.graceful {jobId: sub-1}
    TM->>QH: command.kill.graceful {jobId: sub-2}

    QH->>Redis: SREM set:executing {parent}, HINCRBY -1
    QH->>Redis: RPUSH queue:teardown {parent}
    
    QH->>Redis: SREM set:executing {sub-1}, HINCRBY -1
    QH->>Redis: RPUSH queue:teardown {sub-1}

    QH->>Redis: SREM set:executing {sub-2}, HINCRBY -1
    QH->>Redis: RPUSH queue:teardown {sub-2}

    QH-->>TM: state.job.{parent}.teardown.pending
    QH-->>TM: state.job.{sub-1}.teardown.pending
    QH-->>TM: state.job.{sub-2}.teardown.pending

    Note over QH: Dispatch Coordinator dispatches from Teardown Queue
    QH->>SubRunner: control.{sub-1}.kill.graceful
    QH->>SubRunner: control.{sub-2}.kill.graceful
    QH->>Runner: control.{parent}.kill.graceful

    SubRunner-->>QH: runner.teardown_completed {sub-1}
    SubRunner-->>QH: runner.teardown_completed {sub-2}
    Runner-->>QH: runner.teardown_completed {parent}
```

## 4. Dispatch Wait Timeout

```mermaid
sequenceDiagram
    participant TM as Trigger Monitor
    participant QH as Queue Handler
    participant Redis

    TM->>QH: command.enqueue {jobId, pipeline, queue}
    QH->>Redis: RPUSH queue:dispatch:{queue} {jobId}
    QH->>Redis: HSET job:{jobId} enqueued_at: {now}

    Note over QH: Job waits in queue...

    Note over TM: Trigger Monitor checks maxWaitTime periodically
    Note over TM: enqueued_at + maxWaitTime < now → TIMEOUT

    alt Default behavior: strategy-aware escalation
        TM->>QH: command.dispatch.escalate {jobId, queue}
        QH->>Redis: Move job to next-to-dispatch position (strategy-dependent)
        QH-->>TM: state.queue.{queue}.escalated {jobId}
    else -Q.Dispatch.Wait.TimeOut.Kill.Graceful
        TM->>QH: command.kill.graceful {jobId}
    else -Q.Dispatch.Wait.TimeOut.Reassign
        TM->>QH: command.reassign {jobId, from: queue, to: other}
    end
```

## 5. Pause / Resume Flow

```mermaid
sequenceDiagram
    participant TM as Trigger Monitor
    participant QH as Queue Handler
    participant Redis
    participant Runner

    Note over TM: RAM drops below threshold → -Q.Pause.Hard.RAM.LessThan fires
    TM->>QH: command.pause.hard {jobId}
    QH->>Redis: SREM set:executing {jobId}
    QH->>Redis: HSET set:suspended {jobId} "hard"
    QH->>Redis: HINCRBY counter:instances {pipeline} -1
    QH->>Runner: control.{jobId}.pause.hard
    QH-->>TM: state.job.{jobId}.suspended {type: "hard"}
    QH-->>TM: state.executing.count {n}

    Runner-->>QH: runner.paused {jobId, type: "hard"}
    Runner-->>TM: runner.paused {jobId, type: "hard"}
    QH->>Redis: HSET job:{jobId} confirmed_paused true
    QH-->>TM: state.job.{jobId}.confirmed_suspended

    Note over TM: RAM recovers → -Q.Resume.RAM.MoreThan fires
    TM->>QH: command.resume {jobId}
    QH->>Redis: HDEL set:suspended {jobId}
    QH->>Redis: RPUSH queue:resume {jobId}
    QH-->>TM: state.job.{jobId}.resuming

    Note over QH: Dispatch Coordinator wakes: Tier 2 RR includes Resume Queue
    QH->>Redis: LPOP queue:resume, SADD set:executing, HINCRBY counter:instances
    QH->>Runner: control.{jobId}.resume
    QH-->>TM: state.job.{jobId}.executing
```

---

See also: [[end-to-end-flow]], [[dispatch-coordinator]], [[reactive-signals]]
