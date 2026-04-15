---
phase: issue-283-reassemble-operator
plan: 01
subsystem: docs
tags: [ebnf, grammar, operators, reassemble]
requires: []
provides:
  - EBNF §12.3 reassemble grammar productions
  - =* row in operators.md
affects: [compiler-rules, ebnf-tooling]
key-files:
  modified:
    - docs/technical/ebnf/12-collections.md
    - docs/user/syntax/operators.md
key-decisions:
  - "=* compiles to equivalent = + * pair — no new runtime instruction"
patterns-established:
  - "Reassemble IO uses dual markers: (=) for expander input, (*) for collector output"
duration: 3min
completed: 2026-04-15
---

# Issue #283 Plan 01: EBNF Grammar + Operators Reference Summary

**Added §12.3 Reassemble Operators to EBNF grammar (9 operators, 5 rules) and =* row to operators.md**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF grammar defines =* syntax | Pass | §12.3 with 7 productions, IO signatures table, 5 compiler rules |
| AC-2: Operators reference includes =* | Pass | Collection Operators table has =* row with link |

## Accomplishments

- EBNF §12.3 defines full reassemble grammar: `reassemble_line`, `reassemble_invocation`, `reassemble_operator`, `reassemble_agg`, `reassemble_into`, `reassemble_expand_io`, `reassemble_collect_io`
- IO Signatures table covers all 9 operators with inputs, outputs, and equivalent expand-collect pair
- operators.md Collection Operators table includes =* with concept page link

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/ebnf/12-collections.md` | Modified | Added §12.3 Reassemble Operators |
| `docs/user/syntax/operators.md` | Modified | Added =* row to Collection Operators |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:** Grammar is defined, operators reference updated. Future compiler rules (PGE codes) can reference §12.3.

---
*Phase: issue-283-reassemble-operator, Plan: 01*
*Completed: 2026-04-15*
