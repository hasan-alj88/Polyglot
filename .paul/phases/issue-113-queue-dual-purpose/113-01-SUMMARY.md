---
phase: issue-113-queue-dual-purpose
plan: 01
subsystem: docs
tags: [queue, dual-purpose, blocks, ebnf, pipeline]

requires:
  - phase: issue-107-object-type-hierarchy
    provides: "{T} as first-class type, object hierarchy established"
provides:
  - "Unified {Q} dual-purpose documentation across user docs, concept docs, and EBNF"
affects: []

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/user/syntax/blocks.md
    - docs/user/concepts/pipelines/queue.md
    - docs/technical/ebnf/09-definition-blocks.md

key-decisions:
  - "Cross-reference EBNF rather than duplicate grammar for {Q} =Q.* form"

patterns-established: []

duration: 5min
started: 2026-04-01
completed: 2026-04-01
---

# Issue #113 Plan 01: Unify {Q} Dual-Purpose Documentation

**Unified `{Q}` dual-purpose behavior across blocks.md, queue.md, and EBNF spec — data form (`{Q} #Queue:Name`) vs pipeline form (`{Q} =Q.*`) now documented consistently with identifier prefix disambiguation rule.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Tasks | 3 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: blocks.md {Q} row explains dual-purpose | Pass | Row expanded with both forms and prefix disambiguation rule |
| AC-2: queue.md documents pipeline operation form | Pass | New section with disambiguation rule + 4 aj3lib examples |
| AC-3: EBNF S9.5 covers both forms | Pass | Dual-purpose paragraph added with S9.3 cross-reference |

## Accomplishments

- Expanded `{Q}` row in blocks.md from terse parenthetical to clear dual-purpose explanation
- Added "Queue Pipeline Operations (`{Q} =Q.*`)" section to queue.md with all 4 aj3lib queue pipelines (`=Q.Default`, `=Q.Pause.Hard`, `=Q.Resume`, `=Q.Kill.Graceful`)
- Added EBNF S9.5 clarification cross-referencing S9.3 for the pipeline form grammar

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/blocks.md` | Modified | Expanded `{Q}` row with full dual-purpose explanation |
| `docs/user/concepts/pipelines/queue.md` | Modified | Added "Queue Pipeline Operations" section with disambiguation and examples |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | Added "Dual-purpose" paragraph to S9.5 with S9.3 cross-reference |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Cross-reference S9.3 rather than add new EBNF grammar | `{Q} =Q.*` is syntactic sugar for `{=}[Q]` — duplicating the pipeline grammar would create maintenance burden | Clean; single source of grammar truth |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #113 fully addressed — all three action items from the issue complete
- Consistent with marker-declarations.md S0 source of truth

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-113-queue-dual-purpose, Plan: 01*
*Completed: 2026-04-01*
