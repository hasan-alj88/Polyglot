---
phase: issue-75-dict-kv
plan: 04
subsystem: docs
tags: [type-system, migration, batch-replace]

requires:
  - phase: issue-75-dict-kv/75-03
    provides: Authoritative # annotation system in types.md, EBNF, metadata-tree, pglib
provides:
  - Zero ; type annotations remaining in docs/ (excluding decision records and draft.md)
  - Complete # type annotation consistency across entire spec
affects: [issue-75 merge, future spec editing]

tech-stack:
  added: []
  patterns:
    - "# type annotation universal across all spec files"

key-files:
  modified:
    - "109 files across docs/ (see breakdown below)"

key-decisions:
  - "Batch Python regex migration rather than manual file-by-file editing"

completed: 2026-03-27
---

# Issue #75 Plan 04: Spec-Wide ; → # Migration

**Migrated 1,339 type annotations from `;` to `#` across 109 documentation files — zero `;` type annotations remain.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 3 completed (batch via script) |
| Files modified | 109 |
| Total replacements | 1,339 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: User-Facing Docs Migrated | Pass | grep returns zero matches for ;type patterns in docs/user/ |
| AC-2: Technical Spec Files Migrated | Pass | grep returns zero matches in docs/technical/ (excluding EBNF terminals) |
| AC-3: Compile Rules Migrated | Pass | grep returns zero matches in docs/technical/compile-rules/ |

## Accomplishments

- 109 files migrated with zero manual intervention — batch Python regex script
- All ;type → #type, ;array.Type → #array:Type (separator change), ;CapitalType → #CapitalType
- Verified: zero ; type annotation patterns remain in docs/ (excluding read-only decision records and draft.md)

## Files Modified (by directory)

| Directory | Files | Replacements |
|-----------|-------|-------------|
| docs/user/syntax/ | 5 | 31 |
| docs/user/concepts/ | 3 | 31 |
| docs/user/pglib/ | 8 | 86 |
| docs/technical/ (EBNF, EDGE-CASES, brainstorming) | 4 | 340 |
| docs/technical/spec/ | (already migrated in 75-03, remaining sections caught) | — |
| docs/technical/compile-rules/PGE/ | 64 | 667 |
| docs/technical/compile-rules/PGW/ | 18 | 142 |
| docs/technical/compile-rules/algorithms/ | 1 | 1 |

## Deviations from Plan

None — executed as planned using batch script rather than 3 separate subagents (more efficient).

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 4 plans for Issue #75 complete
- Entire spec uses consistent # type annotations
- Ready for /paul:merge to close issue #75

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-75-dict-kv, Plan: 04*
*Completed: 2026-03-27*
