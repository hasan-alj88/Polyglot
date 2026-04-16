---
phase: 307-ebnf-field-path-separator-match-value-redundancy
plan: 01
subsystem: ebnf
tags: [field_path, match_value, cross_pkg_enum, PGE05001, edge-cases]

requires:
  - phase: none
    provides: n/a
provides:
  - field_path EBNF comment documenting cross-level mixed separators
  - match_value production tightened (cross_pkg_enum removed)
  - EC-3.8 and EC-11.10 edge cases
  - PGE05001 scope clarification

key-files:
  modified:
    - docs/technical/ebnf/03-identifiers.md
    - docs/technical/ebnf/11-control-flow.md
    - docs/technical/edge-cases/03-identifiers.md
    - docs/technical/edge-cases/11-control-flow.md
    - docs/technical/edge-cases/INDEX.md
    - docs/technical/compile-rules/PGE/PGE05001-sibling-separator-homogeneity.md

key-decisions:
  - "X.41 Accept: mixed separators in field_path valid across level boundaries"
  - "X.42 Remove: cross_pkg_enum from match_value (redundant with identifier)"

duration: 10min
completed: 2026-04-16
---

# Issue #307 Plan 01: EBNF field path separator + match value redundancy

**Documented mixed-separator validity in field_path, removed redundant cross_pkg_enum from match_value, added 2 edge cases**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: field_path EBNF documents mixed-separator validity | Pass | 4-line EBNF comment added to §3.2 |
| AC-2: match_value removes cross_pkg_enum redundancy | Pass | Production now `literal \| identifier` only |
| AC-3: Edge cases document both decisions | Pass | EC-3.8 and EC-11.10 added |
| AC-4: PGE05001 clarifies scope | Pass | Scope paragraph added after Detection |

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/ebnf/03-identifiers.md | Modified | field_path EBNF comment: mixed separators valid across levels |
| docs/technical/ebnf/11-control-flow.md | Modified | match_value: removed cross_pkg_enum alternative |
| docs/technical/edge-cases/03-identifiers.md | Modified | Added EC-3.8: mixed separator cross-level paths |
| docs/technical/edge-cases/11-control-flow.md | Modified | Added EC-11.10: cross_pkg_enum via identifier; updated EC-11.8 ref |
| docs/technical/edge-cases/INDEX.md | Modified | Updated S3/S11 counts; total now 135 |
| docs/technical/compile-rules/PGE/PGE05001-sibling-separator-homogeneity.md | Modified | Added Scope paragraph: per-sibling-level not per-path |

## Deviations from Plan

None - plan executed exactly as written.

## Next Phase Readiness

**Ready:** Issue #307 complete. Branch ready for merge to main.
**Blockers:** None.

---
*Phase: 307-ebnf-field-path-separator-match-value-redundancy, Plan: 01*
*Completed: 2026-04-16*
