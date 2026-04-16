---
phase: 287-exhaustive-error-logic
plan: 01
subsystem: language-spec
tags: [error-handling, exhaustiveness, PGE07007, fallback, compiler-rules]

requires:
  - phase: none
    provides: existing PGE07007/PGE02005 rule docs
provides:
  - Exhaustiveness algorithm for PGE07007 (5-step, implementation-ready)
  - Grouped (-) $label fallback syntax for multi-output pipelines
  - PGE02005/PGE07007 role separation
affects: [compiler-implementation, 287-02]

tech-stack:
  added: []
  patterns: [exhaustiveness-algorithm, grouped-fallback, io-mirroring]

key-files:
  created: []
  modified:
    - docs/technical/compile-rules/PGE/PGE07007-error-handling-must-be-exhaustive.md
    - docs/user/concepts/errors.md
    - docs/technical/ebnf/10-execution.md
    - docs/technical/compile-rules/PGE/PGE02005-failed-is-terminal.md
    - docs/user/concepts/pipelines/error-handling.md
    - docs/technical/compile-rules/PGW/PGW02004-failed-variable-usage.md

key-decisions:
  - "PGE07007 owns exhaustiveness algorithm; PGE02005 is secondary safety net for variable lifecycle"
  - "Global [!] !* covers all chain steps; .N!* is step-scoped"
  - "All outputs must have coverage — partial output coverage is PGE07007"
  - "Grouped (-) $label and scattered (>) forms are both valid; compiler unions them"

patterns-established:
  - "Exhaustiveness algorithm: collect E, compute H ∪ F ∪ G per output, check Coverage(O) = E"
  - "IO mirroring rule: (-) under [-] pipeline calls, ($) for variable-scope accessors"

duration: 15min
completed: 2026-04-16
---

# Issue #287 Plan 01: Exhaustiveness Algorithm + Grouped Fallback Summary

**5-step exhaustiveness algorithm for PGE07007, grouped `(-) $label` fallback syntax, and PGE02005/PGE07007 role separation**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Exhaustiveness Algorithm Specified | Pass | 5-step algorithm with set-theoretic notation |
| AC-2: Strict No-Failed-State Rule | Pass | Statement added to PGE07007 |
| AC-3: Chain Error Exhaustiveness | Pass | Global !* vs .N!* scoping documented with examples |
| AC-4: Grouped Fallback Syntax | Pass | errors.md section + EBNF grouped_fallback production |
| AC-5: PGE02005/PGE07007 Overlap Resolved | Pass | PGE02005 narrowed, PGW02004 updated, cross-refs fixed |
| AC-6: Multi-Output Coverage | Pass | Per-output coverage rule with VALID/INVALID examples |

## Accomplishments

- PGE07007 expanded from 1-page rule to full algorithm spec with chain, multi-output, and diagnostic sections
- Grouped `(-) $label` fallback syntax added to errors.md with 4 examples + EBNF grammar
- All PGE02005-for-exhaustiveness references corrected to PGE07007 (errors.md, error-handling.md, PGW02004)

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/compile-rules/PGE/PGE07007-*.md | Rewritten | Algorithm, chain rules, multi-output, diagnostics |
| docs/user/concepts/errors.md | Modified | Grouped fallback section, PGE02005→PGE07007 fix |
| docs/technical/ebnf/10-execution.md | Modified | grouped_fallback + grouped_fallback_line productions |
| docs/technical/compile-rules/PGE/PGE02005-*.md | Modified | Narrowed to lifecycle, delegates to PGE07007 |
| docs/user/concepts/pipelines/error-handling.md | Modified | PGE02005→PGE07007 reference |
| docs/technical/compile-rules/PGW/PGW02004-*.md | Modified | PGE02005→PGE07007 reference |

## Deviations from Plan

### Auto-fixed Issues

**1. PGW02004 cross-reference**
- **Found during:** Task 3
- **Issue:** PGW02004 also referenced PGE02005 for exhaustiveness (not listed in plan files)
- **Fix:** Changed reference to PGE07007
- **Verification:** grep confirms zero PGE02005-exhaustiveness refs outside PGE02005 itself

### Deferred Items

None.

## Next Phase Readiness

**Ready:**
- Plan 287-02 (operation label marker fix) can proceed

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 287-exhaustive-error-logic, Plan: 01*
*Completed: 2026-04-16*
