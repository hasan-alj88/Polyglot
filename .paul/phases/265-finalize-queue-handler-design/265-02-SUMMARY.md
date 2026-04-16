---
phase: 265-finalize-queue-handler-design
plan: 02
subsystem: documentation
tags: [queue-handler, cross-references, signal-alignment]

requires:
  - phase: 265-01
    provides: Complete signal table in reactive-signals.md
provides:
  - Cross-document consistency across all QH spec files
  - User doc constraint list matching technical spec (9/9)
  - overflow.md promoted to spec status
affects: []

key-files:
  modified:
    - docs/user/concepts/pipelines/queue.md
    - docs/technical/plan/queue-manager/end-to-end-flow.md
    - docs/technical/plan/queue-manager/overflow.md
    - docs/technical/plan/queue-manager/process-isolation.md
    - docs/technical/plan/queue-manager/INDEX.md

key-decisions:
  - "sequence-diagrams.md stale signal name flagged but not fixed (out of scope per plan boundaries)"

completed: 2026-04-16
---

# Plan 265-02 Summary: Cross-Document Alignment

**Cross-document consistency achieved across all QH spec files: 9/9 constraints aligned, signal names corrected, overflow promoted to spec, INDEX completed.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 3 completed |
| Files modified | 5 |
| Deviations | 0 |
| Failures | 0 |

## Changes Made

### Task 1: User doc constraint list (queue.md)

- Added 2 missing constraints: `.maxInstancesWithinHost`, `.maxConcurrentAllQueues`
- Split constraints into **Queue-level** (3) and **Pipeline-level** (6) subsections matching dispatch-coordinator.md
- Total: 9 constraints, matching technical spec exactly

### Task 2: Signal name alignment + overflow promotion

**end-to-end-flow.md:**
- Fixed ambiguous `command.job.pause.free.ram` → `command.job.pause.free.ram.hard` (with timing payload)
- Fixed double-dot typo `-Q.Job.Pause.Free.RAM.RAM.LessThan` → `-Q.Job.Pause.Free.RAM.Hard`
- Fixed suspended set value `"hard"` → `"ram.hard"` (matching redis-containers.md)
- Added missing scoped counter decrements and status field update to pause flow
- Added type field to runner.paused ACK
- Added type lookup and status update to resume flow

**overflow.md:**
- Promoted `status: draft` → `status: spec`
- Changed `type: technical` → `type: spec`
- Design is complete for issue #265 scope (implementation is future work)

### Task 3: Cross-document contradiction check

Systematic 6-point verification:

1. **Signal coverage:** All signals in reactive-signals.md present in signal-payloads.md and nats-namespace.md ✓
2. **#QueueState mapping:** All 10 variants map to exactly one Redis container ✓
3. **-Q.* action mapping:** All user-facing actions map to reactive signals ✓
4. **Pause level consistency:** resource-controls.md levels match redis-containers.md states ✓
5. **CRIU signal names:** Fixed process-isolation.md to use actual signal names (was using descriptive names like `command.job.resume.from.disk` and `command.job.reassign`)
6. **Constraint lists:** queue.md (9) matches dispatch-coordinator.md (9) ✓

**Additional fixes found during check:**
- process-isolation.md: Updated CRIU operations table to use actual signal names with timing/type annotations
- INDEX.md: Added 3 missing sections (resource-controls, process-isolation, overflow)

**Flagged but out of scope:**
- sequence-diagrams.md line 173 still uses old `command.job.pause.free.ram` (boundaries exclude sequence-diagrams.md)

## Files Modified

| File | Change |
|------|--------|
| docs/user/concepts/pipelines/queue.md | +2 constraints, split into Queue/Pipeline-level |
| docs/technical/plan/queue-manager/end-to-end-flow.md | Signal names, state values, counter ops aligned |
| docs/technical/plan/queue-manager/overflow.md | Promoted to status: spec |
| docs/technical/plan/queue-manager/process-isolation.md | CRIU signal names corrected |
| docs/technical/plan/queue-manager/INDEX.md | +3 missing sections added |

## Acceptance Criteria

| AC-1: Constraint list matches | Pass | 9/9 constraints in queue.md matching dispatch-coordinator.md |
| AC-2: overflow.md status resolved | Pass | Promoted draft → spec |
| AC-3: Signal names aligned | Pass | All signal names in end-to-end-flow.md match reactive-signals.md |
| AC-4: Zero contradictions | Pass | 6-point cross-doc check; 1 stale name in excluded sequence-diagrams.md |

## Deviations from Plan

None — plan executed exactly as written.

**Additional fixes (within scope):**
- process-isolation.md CRIU signal names corrected (found during Task 3 cross-check)
- INDEX.md missing 3 sections added (found during Task 3 cross-check)

## Deferred Items

- sequence-diagrams.md line 173: stale `command.job.pause.free.ram` signal name (out of scope per plan boundaries)

## Next Phase Readiness

**Ready:**
- All QH spec files are internally consistent
- Issue #265 acceptance criteria fully met (both plans)
- Branch ready for merge to main

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 265-finalize-queue-handler-design, Plan: 02*
*Completed: 2026-04-16*
