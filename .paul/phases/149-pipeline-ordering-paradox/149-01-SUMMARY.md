---
phase: 149-pipeline-ordering-paradox
plan: 01
subsystem: docs
tags: [ebnf, compile-rules, pipeline-ordering, PGE01001, PGE01002]

requires:
  - phase: none
    provides: existing docs with [T],[=] ordering
provides:
  - consistent [=],[T] ordering across all documentation
  - EBNF grammar enforcing IO before triggers
affects: []

tech-stack:
  added: []
  patterns: ["[=] IO before [T] trigger as positional rule"]

key-files:
  created: []
  modified:
    - docs/technical/compile-rules/PGE/PGE01001-pipeline-execution-order.md
    - docs/technical/compile-rules/PGE/PGE01002-io-before-trigger.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/user/concepts/pipelines/INDEX.md
    - docs/user/concepts/pipelines/wrappers.md
    - docs/user/jm3lib/pipelines/W.md
    - docs/technical/edge-cases/16-trigger-io-wiring.md

key-decisions:
  - "IO [=] before triggers [T] is a positional rule, not just semantic"
  - "EBNF trigger_io_section enforces ordered production, not unordered"

patterns-established:
  - "Pipeline section order: [=],[T] → [Q] → setup → body → cleanup"

duration: 5min
started: 2026-04-06
completed: 2026-04-06
---

# Issue #149 Plan 01: Pipeline Ordering Paradox Fix Summary

**Resolved [T],[=] vs [=],[T] ordering paradox — established IO-first as positional rule across 7 files**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Tasks | 3 completed |
| Files modified | 7 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: PGE01001 says [=],[T] | Pass | Statement + diagnostic updated |
| AC-2: PGE01002 positional rule | Pass | "positionally before all [T] trigger lines" |
| AC-3: EBNF enforces order | Pass | Comment says "order IS strict"; grammar splits IO/triggers into ordered productions |
| AC-4: Downstream references | Pass | 0 remaining `[T],[=]` in docs/ |

## Accomplishments

- Flipped all `[T],[=]` → `[=],[T]` notation across 6 locations
- Rewrote PGE01002 statement from scoped semantic rule to universal positional rule
- Split EBNF `trigger_io_section` from single unordered production into two ordered productions (IO first, triggers second)
- Updated edge-case 16 EBNF reference to match new grammar

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/compile-rules/PGE/PGE01001-pipeline-execution-order.md` | Modified | `[T],[=]` → `[=],[T]` in statement + diagnostic |
| `docs/technical/compile-rules/PGE/PGE01002-io-before-trigger.md` | Modified | Reworded as positional ordering rule |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | EBNF comment + grammar + execution order reference |
| `docs/user/concepts/pipelines/INDEX.md` | Unchanged | Already said "IO before trigger" — no `[T],[=]` notation to fix |
| `docs/user/concepts/pipelines/wrappers.md` | Modified | Execution order + Mermaid diagram label |
| `docs/user/jm3lib/pipelines/W.md` | Modified | Execution order line |
| `docs/technical/edge-cases/16-trigger-io-wiring.md` | Modified | EBNF reference updated to match new production |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| IO before triggers is positional, not semantic | User directive — simplifies compiler rule, removes ambiguity | All future pipelines must declare [=] IO before [T] triggers |
| EBNF uses ordered production | Unordered grammar contradicted compile rules | Grammar now authoritative for ordering |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #149 fully resolved, branch ready for merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 149-pipeline-ordering-paradox, Plan: 01*
*Completed: 2026-04-06*
