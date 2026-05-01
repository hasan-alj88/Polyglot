---
audience: design
type: spec
updated: 2026-04-03
---

# Precomputation

<!-- @c:queue-manager/dispatch-coordinator -->
<!-- @c:queue-manager/properties -->

Because every signal handler is `f(signal, state) → (new_state, output_signals)` — a pure function — the Queue Handler can optimize without changing behavior.

## 1. Constraint Pre-evaluation

When a job is enqueued, the Queue Handler has all constraint data in Redis (cached at queue registration). It can precompute constraint results and cache them:

```text
On enqueue(jobId):
    constraint_result = check(scoped constraints from queue:config + job metadata)
    HSET job:{jobId} constraint_precomputed {pass|fail|reason}
```

When the Dispatch Coordinator cycles, the cached result is read instead of re-evaluated. Caches invalidate only when the Running Set changes (job starts, completes, pauses, or resumes) — a known, finite set of events.

Invalidation trigger: any change to `set:running` → recompute constraints for all waiting candidates.

## 2. Short-circuit on State Broadcast

When the Queue Handler broadcasts `state.executing.count`, constraint checks can short-circuit:

- If `executing_count < min(maxConcurrentWithinQueue)` across all waiting candidates → ALL pass without per-candidate evaluation
- If `instances[pipeline] < maxInstancesAllQueues` for a pipeline → ALL instances of that pipeline pass without checking each one
- If no tagged job is executing → ALL candidates with that tag pass resourceTag check

This turns O(n) constraint checks into O(1) for common cases.

## What Does NOT Precompute

The Queue Handler never precomputes **conditions** (RAM thresholds, CPU usage, time limits). Those live in `=Q.Pause.Hard.RAM.LessThan` and similar pipelines — the Trigger Monitor evaluates conditions and sends commands. The Queue Handler only precomputes state-derivable facts: constraint pass/fail, queue ordering, instance counts.

---

See also: [[dispatch-coordinator]], [[properties]], [[reactive-signals]]
