---
phase: 320-t-call-signal-path
plan: 01
subsystem: spec
tags: [nats, signal-path, t-call, sdk, trigger-monitor, redis]

requires:
  - phase: 319-polyglot-sdk
    provides: SDK call() NATS protocol, type mapping descriptors
provides:
  - TM-side -T.Call signal processing documentation
  - NATS namespace registration for SDK call topics
  - Redis key structure for SDK-initiated jobs
affects: [264-finalize-tm-design, 267-implement-tm]

tech-stack:
  added: []
  patterns: [NATS request-reply for synchronous SDK calls, Redis job:{UID}:port:{name} binding storage]

key-files:
  created: []
  modified:
    - docs/technical/spec/native-dispatch.md
    - docs/technical/plan/queue-manager/nats-namespace.md
    - docs/user/aj3lib/pipelines/T/Call.md
    - docs/technical/spec/polyglot-sdk.md

key-decisions:
  - "TM generates job UID, not SDK — SDK only provides correlation_id"
  - "Redis stores correlation metadata alongside port bindings for result routing"

patterns-established:
  - "SDK-initiated jobs use job:{UID}:meta:* keys for correlation routing"

duration: ~15min
started: 2026-04-19
completed: 2026-04-19
---

# Issue #320 Plan 01: -T.Call Signal Path Summary

**Documented the full -T.Call NATS request-reply signal path: SDK→TM subscription, pipeline matching, Redis binding storage, command.enqueue, result collection, and NATS response — with sequence diagram and comparison table.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-19 |
| Completed | 2026-04-19 |
| Tasks | 2 completed |
| Files modified | 4 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Call.md has signal flow section | Pass | 10-step signal flow diagram + cross-references |
| AC-2: native-dispatch.md has -T.Call protocol | Pass | 8 subsections: subscription, matching, binding storage (Redis keys), enqueue, result collection, sequence diagram, comparison table |
| AC-3: NATS namespace includes SDK Call topics | Pass | New section between Trigger Events and Lifecycle Control |
| AC-4: Cross-references link all documents | Pass | Bidirectional links verified across all 4 files |

## Accomplishments

- Documented TM-side processing of SDK `call()` requests with full NATS request-reply lifecycle
- Established Redis key pattern `job:{UID}:port:{name}` and `job:{UID}:meta:correlation_id` for SDK-initiated job tracking
- Registered `polyglot.call.*` and `polyglot.result.*` in the NATS namespace alongside existing inter-service signals
- Added comparison table showing how -T.Call differs from other triggers (synchronous vs fire-and-forget)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/spec/native-dispatch.md` | Modified | New "-T.Call Signal Path" section with 8 subsections |
| `docs/technical/plan/queue-manager/nats-namespace.md` | Modified | New "SDK Call Signals" section with 2 topics |
| `docs/user/aj3lib/pipelines/T/Call.md` | Modified | New "Signal Flow" section with simplified diagram |
| `docs/technical/spec/polyglot-sdk.md` | Modified | Cross-reference to native-dispatch.md in call() section |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| TM generates job UID, not SDK | SDK should not control internal job identifiers; correlation_id handles result routing | SDK API stays simple; TM owns job lifecycle |
| Redis stores correlation metadata on job hash | Result routing needs caller_topic after async execution completes | Enables TM to publish results without maintaining in-memory state |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Full signal path documented from SDK to TM to Runner and back
- Redis key patterns established for implementation reference
- NATS namespace complete with SDK call topics

**Concerns:**
- None

**Blockers:**
- None — ready for /paul:merge

---
*Phase: 320-t-call-signal-path, Plan: 01*
*Completed: 2026-04-19*
