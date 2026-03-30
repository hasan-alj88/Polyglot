---
phase: issue-97-ebnf-bare-expressions
plan: 01
subsystem: compiler
tags: [ebnf, compile-rules, edge-cases, PGE01020]

requires:
  - phase: none
    provides: existing EBNF and compile-rules infrastructure
provides:
  - PGE01020 compile rule for effectless execution expressions
  - Tightened exec_expr EBNF production
  - Edge cases EC-10.12, EC-10.13
affects: []

tech-stack:
  added: []
  patterns: []

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE01020-effectless-execution-expression.md
  modified:
    - docs/technical/ebnf/10-execution.md
    - docs/technical/COMPILE-RULES.md
    - docs/technical/edge-cases/10-execution.md
    - docs/technical/edge-cases/INDEX.md

key-decisions:
  - "X.1 (bare literals) and X.2 (non-pipeline identifiers) are compile errors, not warnings"
  - "X.3 (bare standalone expression) resolves automatically — no standalone path remains after exec_expr tightening"
  - "Single PGE01020 rule covers both X.1 and X.2"

patterns-established: []

duration: 5min
completed: 2026-03-30
---

# Issue #97 Plan 01: Effectless Execution Expression Summary

**Tightened `exec_expr` EBNF to reject bare literals and non-pipeline identifiers; added PGE01020 compile error rule with edge cases EC-10.12/EC-10.13.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 2 completed |
| Files modified | 5 |
| Deviations | 0 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: exec_expr rejects bare identifiers | Pass | `\| identifier` removed from production |
| AC-2: PGE01020 rule file exists | Pass | Frontmatter + VALID/INVALID examples |
| AC-3: COMPILE-RULES.md index includes PGE01020 | Pass | Added to 01 — Pipeline Structure |
| AC-4: EC-10.12 and EC-10.13 exist | Pass | Bare literals and non-pipeline identifiers |
| AC-5: INDEX reflects updated counts | Pass | EC-10.1--10.13, total 104 |

## Accomplishments

- Removed `| identifier` catch-all from `exec_expr` EBNF production, closing the grammar permissiveness gap
- Created PGE01020 with comprehensive VALID/INVALID examples covering bare int, bare string, #type, $var, and @alias
- Added 2 edge cases (EC-10.12, EC-10.13) documenting the INVALID patterns with PGE01020 references

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/ebnf/10-execution.md` | Modified | Removed `\| identifier` from exec_expr |
| `docs/technical/compile-rules/PGE/PGE01020-effectless-execution-expression.md` | Created | New compile error rule |
| `docs/technical/COMPILE-RULES.md` | Modified | Added PGE01020 to 01 category index |
| `docs/technical/edge-cases/10-execution.md` | Modified | Added EC-10.12, EC-10.13 |
| `docs/technical/edge-cases/INDEX.md` | Modified | Updated S10 range and total count |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| PGE (error) not PGW (warning) for both X.1 and X.2 | Bare literals/identifiers are never intentional — always a bug | Strict: no effectless expressions pass compilation |
| Single rule PGE01020 covers both cases | Same root cause (effectless exec_expr) and same fix (grammar restriction) | Simpler rule set, one code to remember |
| X.3 auto-resolves, no explicit rule needed | After removing `\| identifier` from exec_expr, no standalone path remains for bare expressions to produce no effect | No additional grammar or rule changes needed |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #97 fully resolved — all 3 sub-cases addressed
- Ready for commit and merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-97-ebnf-bare-expressions, Plan: 01*
*Completed: 2026-03-30*
