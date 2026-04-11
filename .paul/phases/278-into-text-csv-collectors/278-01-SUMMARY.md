---
phase: 278-into-text-csv-collectors
plan: 01
subsystem: docs
tags: [pglib, types, enums, collectors, overflow, errors]

requires:
  - phase: none
    provides: existing type/collector doc conventions
provides:
  - 7 data types for text diff/merge operations
  - PPTD overflow infrastructure documentation
  - *Agg.Concatenate separator parameter
  - 12 error types across 3 new namespaces
affects: [278-02-text-operators, 278-03-csv-operators]

tech-stack:
  added: []
  patterns: [type doc with metadata_definition/metadata_instance frontmatter]

key-files:
  created: [types/TextDiff.md, types/DiffOp.md, types/DiffStats.md, types/MergeConflict.md, types/MergeResult.md, types/MergeStrategy.md, types/CollectOrder.md, queue-manager/overflow.md]
  modified: [types/types.md, types/enums.md, Agg/Concatenate.md, errors/errors.md]

key-decisions:
  - "#TextDiffs is alias for #Array.TextDiff, not separate type file"
  - "enums.md gets own Text & Merge Enums section"
  - "All new type files status: draft until full issue completion"

patterns-established:
  - "Text/merge types follow existing pglib type doc conventions"

completed: 2026-04-11
---

# Plan 278-01: Foundation Types, PPTD, Concatenate, Errors

**7 data types, PPTD overflow docs, *Agg.Concatenate separator, and 12 error types across 3 new namespaces — foundation for text/CSV collection operators.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Data types documented | Pass | 7 files, all with frontmatter + field tables; types.md and enums.md updated |
| AC-2: *Agg.Concatenate updated | Pass | <separator added, backwards-compatible |
| AC-3: PPTD overflow documented | Pass | overflow.md with 5 -Q.Overflow.* params, permission, error |
| AC-4: Error types registered | Pass | !Storage, !Text, !CSV namespaces + pipeline associations |

## Accomplishments

- 7 data type definitions (#TextDiff, #DiffOp, #DiffStats, #MergeConflict, #MergeResult, #MergeStrategy, #CollectOrder)
- PPTD three-tier overflow infrastructure documented (RAM → PPTD → batch collection)
- *Agg.Concatenate backwards-compatible separator update
- 12 error types across 3 new namespaces with pipeline error associations

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/types/TextDiff.md` | Created | #TextDiff struct + #TextDiffs alias |
| `docs/user/pglib/types/DiffOp.md` | Created | #DiffOp enum (Add/Delete/Replace) |
| `docs/user/pglib/types/DiffStats.md` | Created | #DiffStats struct (additions/deletions/unchanged) |
| `docs/user/pglib/types/MergeConflict.md` | Created | #MergeConflict struct |
| `docs/user/pglib/types/MergeResult.md` | Created | #MergeResult struct (.text + .conflicts) |
| `docs/user/pglib/types/MergeStrategy.md` | Created | #MergeStrategy enum (5 strategies) |
| `docs/user/pglib/types/CollectOrder.md` | Created | #CollectOrder enum (ExpandIndex/Arrival) |
| `docs/user/pglib/types/types.md` | Modified | Added Text & Merge category |
| `docs/user/pglib/types/enums.md` | Modified | Added Text & Merge Enums section |
| `docs/user/pglib/collectors/Agg/Concatenate.md` | Modified | Added <separator parameter |
| `docs/technical/plan/queue-manager/overflow.md` | Created | PPTD overflow documentation |
| `docs/user/pglib/errors/errors.md` | Modified | 3 namespaces + pipeline associations |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| #TextDiffs as alias, not separate file | It's just #Array.TextDiff — documented in TextDiff.md Alias section | Avoids redundant file |
| enums.md own section for text/merge | Keeps enum groups organized by domain | Future enums follow same pattern |
| All new files status: draft | Will promote to stable after full issue (278-02, 278-03) completion | Consistent lifecycle |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All foundation types available for 278-02 (text operators) and 278-03 (CSV operators)
- Error namespaces registered and pipeline associations pre-declared
- PPTD overflow documented for collector permission references

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 278-into-text-csv-collectors, Plan: 01*
*Completed: 2026-04-11*
