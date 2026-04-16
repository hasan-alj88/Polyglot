---
phase: 265-finalize-queue-handler-design
plan: 01
status: complete
completed: 2026-04-15
---

# Plan 265-01 Summary: Signal Completeness

## What Was Done

Completed the QH signal table to cover all `-Q.*` actions from the resource-freeing spectrum.

### Task 1: Expanded #QueueState enum (redis-containers.md)
- Replaced 2 suspended types ("soft"/"hard") with 4 distinct types: "cpu", "ram.soft", "ram.hard", "all"
- Added `#Executing.Throttled` sub-state (job stays in set:executing with throttle flag)
- Added `throttled`, `throttle_config`, and `images_dir` fields to job:{jobId} HASH
- Total: 10 #QueueState variants (was 7)

### Task 2: Added missing signal handlers (reactive-signals.md)
- Replaced 2 pause signals with 4 level-specific signals, each with `timing` field ("now"|"wait")
- Added: command.job.throttle, command.job.unthrottle, command.job.snapshot, command.job.inspect
- Added: runner.snapshot_completed ACK
- Updated: command.job.resume with type-aware routing (images_dir for Free.All)
- Updated: command.job.kill.* to reference new suspended types + throttled state
- Updated: runner.paused ACK to store images_dir for Free.All
- Added scoped counter decrements (queue + host) to all pause signals

### Task 3: Updated payloads and NATS namespace
- signal-payloads.md: Updated all command, state, control, and runner signal schemas
- nats-namespace.md: Updated all Lifecycle Control, Control Signal, and Runner ACK subjects
- Added command.priority.update NATS subject (was missing)

## Design Decisions Applied
1. 4 distinct suspended types (cpu, ram.soft, ram.hard, all)
2. One signal + timing field (not separate .now/.wait signals)
3. Free.All stays in suspended set (no new container)
4. All operations (throttle/snapshot/inspect) go through QH
5. Throttle = flag on job hash (stays in set:executing)
6. Same resume signal for all types (QH routes by type)
7. Snapshot: QH auto-enqueues fork atomically
8. Inspect: Redis state only (no resource metrics)

## Files Modified
- docs/technical/plan/queue-manager/redis-containers.md
- docs/technical/plan/queue-manager/reactive-signals.md
- docs/technical/plan/queue-manager/signal-payloads.md
- docs/technical/plan/queue-manager/nats-namespace.md

## Deviations
None.
