---
phase: issue-98-ebnf-discard-defaults-self-assignment
plan: 01
subsystem: design
tags: [ebnf, compile-rules, edge-cases, assignment, discard]

requires: []
provides:
  - PGE02010 compile rule (discard default assignment)
  - PGE08011 compile rule (self-assignment)
  - 3 new edge cases (EC-8.4, EC-8.5, EC-8.6)
affects: []

tech-stack:
  added: []
  patterns: [semantic restriction via EBNF comments + compile rules]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE02010-discard-default-assignment.md
    - docs/technical/compile-rules/PGE/PGE08011-self-assignment.md
  modified:
    - docs/technical/COMPILE-RULES.md
    - docs/technical/ebnf/08-expressions.md
    - docs/technical/edge-cases/08-expressions.md
    - docs/technical/edge-cases/INDEX.md

key-decisions:
  - "PGE02010 in category 02 (Lifecycle): $* discard is a lifecycle concept"
  - "PGE08011 in category 08 (Chains & IO Wiring): self-assignment is a wiring error"
  - "EBNF restrictions as comments, not production changes: patterns are syntactically valid but semantically meaningless"

duration: 5min
completed: 2026-03-30
---

# Issue #98 Plan 01: Discard Defaults and Self-Assignment Summary

**Two new compile errors (PGE02010, PGE08011) catching three no-op assignment patterns, with EBNF restriction comments and 3 edge cases.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-30 |
| Tasks | 2 completed |
| Files modified | 6 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Discard default is compile error | Pass | PGE02010 with VALID/INVALID examples |
| AC-2: Self-assignment via same output param | Pass | PGE08011 covers `>x << >x` |
| AC-3: Variable self-assignment | Pass | PGE08011 covers `$x << $x` |
| AC-4: EBNF restricts $* to final operators | Pass | Comment on `$*` line references PGE02010 |
| AC-5: Edge cases and indexes updated | Pass | EC-8.4/8.5/8.6, INDEX total 107 |

## Accomplishments

- PGE02010 blocks `$* <~` and `$* ~>` (discard has no lifecycle to default)
- PGE08011 blocks same-identifier self-assignment for both output params and variables
- EBNF 08-expressions.md annotated with restriction comments (no production changes needed)
- Edge case total: 104 → 107

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/compile-rules/PGE/PGE02010-discard-default-assignment.md` | Created | Compile error for default assignment to discard |
| `docs/technical/compile-rules/PGE/PGE08011-self-assignment.md` | Created | Compile error for self-assignment no-ops |
| `docs/technical/COMPILE-RULES.md` | Modified | Added PGE02010 and PGE08011 to index |
| `docs/technical/ebnf/08-expressions.md` | Modified | Restriction comments on $* and self-assignment |
| `docs/technical/edge-cases/08-expressions.md` | Modified | Added EC-8.4, EC-8.5, EC-8.6 |
| `docs/technical/edge-cases/INDEX.md` | Modified | S8 row and total updated to 107 |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Restrictions as comments, not grammar changes | Patterns are syntactically valid EBNF; semantic analysis catches them | No EBNF production refactoring needed |
| Self-assignment scoped to same operation | Cross-port wiring (`>out1 << >out2`) is valid | PGE08011 only fires on identical identifiers |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #98 fully addressed — all 3 sub-cases (X.4, X.5, X.6) resolved
- Ready for /paul:merge

**Concerns:** None.

**Blockers:** None.

---
*Phase: issue-98-ebnf-discard-defaults-self-assignment, Plan: 01*
*Completed: 2026-03-30*
