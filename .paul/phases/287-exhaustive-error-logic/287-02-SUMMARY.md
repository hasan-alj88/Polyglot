---
phase: 287-exhaustive-error-logic
plan: 02
subsystem: language-spec
tags: [operation-labels, io-mirroring, marker-fix]

requires:
  - phase: 287-01
    provides: grouped (-) $label fallback syntax establishing the correct marker
provides:
  - All operation label declarations use (-) $Label under pipeline calls
  - EBNF operation_label production updated
  - IO mirroring rule documented in operation-labels.md
affects: [compiler-implementation]

key-files:
  modified:
    - docs/user/syntax/operation-labels.md
    - docs/user/syntax/io/operation-labels.md
    - docs/user/syntax/blocks.md
    - docs/audit/reference/glossary.md
    - docs/technical/ebnf/07-io-parameters.md
    - docs/technical/compile-rules/PGE/PGE02012-duplicate-operation-label.md
    - docs/technical/compile-rules/PGE/PGE02013-write-to-label-accessor.md
    - docs/technical/compile-rules/PGE/PGE02014-label-access-before-completion.md
    - docs/technical/compile-rules/PGE/PGE02015-unused-background-label.md
    - docs/technical/compile-rules/PGE/PGE03012-parallel-label-isolation.md

key-decisions:
  - "IO mirroring rule: (-) mirrors [-] pipeline context; ($) is for variable-scope accessor lines"

duration: 10min
completed: 2026-04-16
---

# Issue #287 Plan 02: Operation Label Marker Fix Summary

**Fixed `($) $Label` to `(-) $Label` under pipeline calls across 10 docs — IO mirroring rule enforced**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Operation Labels Use (-) Under Pipeline Calls | Pass | 13 replacements across 5 compile rules + 2 user docs |
| AC-2: EBNF Grammar Updated | Pass | §7 operation_label production uses "(-)" |
| AC-3: Glossary and Blocks Consistent | Pass | Both updated with IO mirroring rule |

## Accomplishments

- 13 `($) $Label` instances changed to `(-) $Label` across 7 files
- EBNF §7 operation_label production updated from `"($)"` to `"(-)"`
- operation-labels.md rewritten with IO mirroring rule section
- Glossary and blocks.md updated to reflect `(-)` as pipeline-scope label marker

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/syntax/operation-labels.md | Rewritten | All examples + new IO Mirroring Rule section |
| docs/user/syntax/io/operation-labels.md | Rewritten | Quick-reference updated |
| docs/user/syntax/blocks.md | Modified | IO bracket table: ($) row split into (-) $Label + ($) accessor |
| docs/audit/reference/glossary.md | Modified | Operation Label + Chain Step Label entries |
| docs/technical/ebnf/07-io-parameters.md | Modified | operation_label production |
| PGE02012 through PGE02015, PGE03012 | Modified | Code examples (13 replacements) |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- Both plans complete — issue #287 ready for merge

**Blockers:**
- None

---
*Phase: 287-exhaustive-error-logic, Plan: 02*
*Completed: 2026-04-16*
