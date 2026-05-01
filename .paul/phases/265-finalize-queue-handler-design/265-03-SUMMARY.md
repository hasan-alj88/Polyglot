---
phase: 265-finalize-queue-handler-design
plan: 03
subsystem: docs
tags: [nats, redis, cgroup, criu, signal-chain, queue-handler]

requires:
  - phase: 265-01
    provides: reactive-signals.md, signal-payloads.md, nats-namespace.md
  - phase: 265-02
    provides: process-isolation.md updates, resource-controls.md cross-refs
provides:
  - Consolidated signal-map.md cross-reference table
  - Runtime Behavior sections in all 15 -Q.* action pipeline docs
  - Source column annotations in all getter/state tables
affects: [queue-handler implementation, runtime architecture]

tech-stack:
  added: []
  patterns:
    - "Runtime Behavior table format: Step/Component/Action per pipeline doc"
    - "Source column in getter tables showing cgroup file or Redis field"

key-files:
  created:
    - docs/technical/plan/queue-manager/signal-map.md
  modified:
    - docs/technical/plan/queue-manager/INDEX.md
    - docs/user/jm3lib/pipelines/Q/INDEX.md
    - 15 action pipeline docs in docs/user/jm3lib/pipelines/Q/

key-decisions: []

patterns-established:
  - "Every -Q.* action doc has a Runtime Behavior section showing the full NATS->QH->Runner->Unix chain"
  - "Every getter/state table includes a Source column identifying the data origin (cgroup file, Redis field, /proc path)"

duration: ~15min
completed: 2026-04-16
---

# Phase 265 Plan 03: Signal Chain Documentation Summary

**Enriched all -Q.* pipeline docs with NATS communication, Redis operations, and Unix mechanisms; created consolidated signal-map.md cross-reference.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Completed | 2026-04-16 |
| Tasks | 3 completed |
| Files modified | 17 |
| Files created | 1 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Every action pipeline doc has Runtime Behavior section | Pass | 15/15 docs verified via grep |
| AC-2: Getter/state docs have Data Source section | Pass | 6 tables in INDEX.md have Source column (30 rows) |
| AC-3: Consolidated signal-map.md exists | Pass | 21 action rows + 30 getter/state rows |
| AC-4: INDEX.md references signal-map.md | Pass | Entry added to section list |

## Accomplishments

- Created signal-map.md consolidating every -Q.* pipeline's full signal chain (NATS subject, payload, Redis write, control signal, Unix mechanism) in one architect-facing table
- Added Runtime Behavior sections to all 15 action pipeline docs, each showing the step-by-step TM->QH->Runner->Unix flow specific to that command
- Added Source column to all 6 getter/state tables in Q/INDEX.md, documenting the exact data origin (cgroup file, Redis field, /proc path, or external tool)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/plan/queue-manager/signal-map.md` | Created | Consolidated cross-reference: action table (21 rows) + getter/state table (30 rows) |
| `docs/technical/plan/queue-manager/INDEX.md` | Modified | Added signal-map.md to section list |
| `docs/user/jm3lib/pipelines/Q/INDEX.md` | Modified | Source column added to 6 getter/state/idle tables |
| `docs/user/jm3lib/pipelines/Q/Job.Pause.Free.CPU.md` | Modified | Runtime Behavior: cgroup.freeze / SIGSTOP |
| `docs/user/jm3lib/pipelines/Q/Job.Pause.Free.RAM.md` | Modified | Runtime Behavior: .Soft (memory.high) and .Hard (memory.max) variants |
| `docs/user/jm3lib/pipelines/Q/Job.Pause.Free.All.md` | Modified | Runtime Behavior: CRIU dump |
| `docs/user/jm3lib/pipelines/Q/Job.Resume.md` | Modified | Runtime Behavior: type-aware routing (cgroup thaw vs criu restore) |
| `docs/user/jm3lib/pipelines/Q/Job.Kill.WithCleanup.md` | Modified | Runtime Behavior: teardown queue + SIGTERM + [/] cleanup |
| `docs/user/jm3lib/pipelines/Q/Job.Kill.Now.md` | Modified | Runtime Behavior: SIGKILL immediate |
| `docs/user/jm3lib/pipelines/Q/Job.Throttle.md` | Modified | Runtime Behavior: throttle (cpu.max/memory.high/io.max) + unthrottle |
| `docs/user/jm3lib/pipelines/Q/Job.Snapshot.md` | Modified | Runtime Behavior: criu dump --leave-running |
| `docs/user/jm3lib/pipelines/Q/Job.Inspect.md` | Modified | Runtime Behavior: Redis read-only, no Unix op |
| `docs/user/jm3lib/pipelines/Q/Job.Reassign.md` | Modified | Runtime Behavior: Lua script + optional CRIU for cross-host |
| `docs/user/jm3lib/pipelines/Q/Priority.Update.md` | Modified | Runtime Behavior: ZADD only |
| `docs/user/jm3lib/pipelines/Q/Queue.Drain.md` | Modified | Runtime Behavior: SADD queues:draining |
| `docs/user/jm3lib/pipelines/Q/Queue.Flush.md` | Modified | Runtime Behavior: per-job SIGKILL |
| `docs/user/jm3lib/pipelines/Q/DoNothing.md` | Modified | Runtime Behavior: no-op note |
| `docs/user/jm3lib/pipelines/Q/Dispatch.Wait.TimeOut.md` | Modified | Runtime Behavior: escalate + variant delegation |

## Decisions Made

None - followed plan as specified.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 3 plans for issue #265 complete (signals, cross-doc, signal chain)
- Every -Q.* pipeline doc is now implementation-ready with full signal chain documentation
- Architect-facing signal-map.md provides single consolidated view

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 265-finalize-queue-handler-design, Plan: 03*
*Completed: 2026-04-16*
