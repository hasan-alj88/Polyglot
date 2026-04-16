---
phase: 305-ebnf-arithmetic-precedence-self-ref-expressions
plan: 01
subsystem: ebnf
tags: [grammar, arithmetic, inline-data, compile-rules]

requires:
  - phase: none
    provides: n/a
provides:
  - arithmetic_expr and arithmetic_op removed from EBNF
  - inline_value non-recursive production breaks inline_data cycle
  - PGE08013 compile rule for nested inline data
affects: [ebnf, compile-rules, edge-cases]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE08013-nested-inline-data.md
  modified:
    - docs/technical/ebnf/06-operators.md
    - docs/technical/ebnf/08-expressions.md
    - docs/technical/edge-cases/06-operators.md
    - docs/technical/edge-cases/08-expressions.md
    - docs/technical/edge-cases/INDEX.md
    - docs/technical/COMPILE-RULES.md

key-decisions:
  - "No raw arithmetic: removed arithmetic_expr/arithmetic_op from EBNF (PGE04010 already bans)"
  - "No nested inline data: inline_value production breaks inline_data→value_expr cycle"

duration: 5min
completed: 2026-04-16
---

# Issue #305 Plan 01: Arithmetic Precedence + Self-Referential Expressions Summary

**Removed arithmetic productions from EBNF, broke inline_data recursion cycle with flat inline_value, added PGE08013**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: arithmetic_expr/arithmetic_op removed from EBNF | Pass | Zero grep matches in docs/technical/ebnf/ |
| AC-2: inline_data uses non-recursive inline_value | Pass | inline_value excludes inline_data |
| AC-3: EC-6.4 shows arithmetic as INVALID | Pass | All examples marked PGE04010 with -Math.* alternatives |
| AC-4: X.35 and X.36 edge cases documented | Pass | EC-8.7 and EC-8.8 added |
| AC-5: PGE08013 compile rule created | Pass | File created, COMPILE-RULES.md updated |

## Accomplishments

- Removed §6.4 (arithmetic_op) and §8.3 (arithmetic_expr) from EBNF, aligning grammar with PGE04010
- Introduced `inline_value` production — flat subset of `value_expr` that breaks the `inline_data → value_expr → inline_data` cycle
- Rewrote EC-6.4 from valid-looking examples to INVALID with PGE04010 references and correct -Math.* alternatives
- Added EC-8.7 (X.35: arithmetic precedence moot) and EC-8.8 (X.36: nested inline data rejected)
- Created PGE08013 compile rule for nested inline data
- Resolved long-standing deferred issue "EC-6.4 inconsistency" in STATE.md

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/ebnf/06-operators.md | Modified | Removed §6.4 arithmetic_op production |
| docs/technical/ebnf/08-expressions.md | Modified | Removed arithmetic_expr from expression + value_expr; added inline_value; renumbered §8.3 |
| docs/technical/edge-cases/06-operators.md | Modified | EC-6.4 rewritten as INVALID with PGE04010 |
| docs/technical/edge-cases/08-expressions.md | Modified | Added EC-8.7 (X.35) and EC-8.8 (X.36); updated EC-8.1 EBNF ref |
| docs/technical/edge-cases/INDEX.md | Modified | Updated S6/S8 summary lines and EC ranges |
| docs/technical/compile-rules/PGE/PGE08013-nested-inline-data.md | Created | Nested inline data compile error |
| docs/technical/COMPILE-RULES.md | Modified | Added PGE08013 to 8.x index |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- Issue #305 changes complete, ready for /paul:merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 305-ebnf-arithmetic-precedence-self-ref-expressions, Plan: 01*
*Completed: 2026-04-16*
