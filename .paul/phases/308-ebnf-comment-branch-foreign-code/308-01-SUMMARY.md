---
phase: 308-ebnf-comment-branch-foreign-code
plan: 01
subsystem: design
tags: [ebnf, edge-cases, compile-rules, foreign-code, conditionals]

requires:
  - phase: none
    provides: n/a
provides:
  - EC-11.11 (X.43 Accept — comment-only conditional branch)
  - EC-22.5 (X.44 PGW — orphaned [C] foreign code warning)
  - PGW01004 compile warning for orphaned foreign code
affects: [compiler-implementation, ebnf-edge-cases]

tech-stack:
  added: []
  patterns: [semantic-over-grammar enforcement for context-dependent rules]

key-files:
  created:
    - docs/technical/compile-rules/PGW/PGW01004-orphaned-foreign-code.md
  modified:
    - docs/technical/edge-cases/11-control-flow.md
    - docs/technical/edge-cases/22-control-flow-gaps.md
    - docs/technical/edge-cases/INDEX.md
    - docs/technical/COMPILE-RULES.md
    - docs/technical/ebnf/11-control-flow.md

key-decisions:
  - "X.43 Accept: PGE06010 is sufficient for comment-only branches — no grammar tightening"
  - "X.44 PGW: Semantic warning (PGW01004) for orphaned [C] lines — grammar cannot enforce pipeline-type scoping"

patterns-established: []

duration: ~5min
started: 2026-04-17
completed: 2026-04-17
---

# Issue #308 Plan 01: Comment-only branches + orphaned foreign code Summary

**Resolved two EBNF edge cases: X.43 Accept (comment-only branches caught by PGE06010) and X.44 PGW01004 (orphaned [C] lines outside -RT.* scope).**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Tasks | 3 completed |
| Files modified | 6 |
| Files created | 1 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: X.43 Accept decision documented | Pass | EC-11.11 in 11-control-flow.md with PGE06010 cross-ref |
| AC-2: X.44 orphaned [C] warning documented | Pass | PGW01004 created + EC-22.5 with 3 examples |
| AC-3: EBNF section 11.6 updated with scoping note | Pass | PGW01004 note added after Rule paragraph |
| AC-4: INDEX.md counts updated | Pass | S11: 11.1--11.11, S22: 22.1--22.5, total: 137 |

## Accomplishments

- Documented X.43 Accept decision: grammar permissiveness for comment-only branches is fine because PGE06010 catches them semantically
- Created PGW01004 warning for orphaned `[C]` foreign code lines not scoped under `-RT.*` calls
- Added 2 edge cases (EC-11.11, EC-22.5) bringing total to 137 across 24 sections

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/edge-cases/11-control-flow.md` | Modified | Added EC-11.11 (X.43 Accept) |
| `docs/technical/edge-cases/22-control-flow-gaps.md` | Modified | Added EC-22.5 (X.44 PGW) |
| `docs/technical/compile-rules/PGW/PGW01004-orphaned-foreign-code.md` | Created | New warning rule for orphaned [C] |
| `docs/technical/COMPILE-RULES.md` | Modified | Added PGW01004 row |
| `docs/technical/ebnf/11-control-flow.md` | Modified | Added PGW01004 scoping note to section 11.6 |
| `docs/technical/edge-cases/INDEX.md` | Modified | Updated S11/S22 counts and total |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| X.43 Accept — no grammar tightening | PGE06010 already catches comment-only branches; tightening grammar adds complexity without benefit | No EBNF change needed |
| X.44 PGW semantic warning | Grammar cannot distinguish -RT.* from other pipelines; semantic check is practical | PGW01004 warns at compile time |

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #308 fully resolved, ready for merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 308-ebnf-comment-branch-foreign-code, Plan: 01*
*Completed: 2026-04-17*
