---
phase: 278-into-text-csv-collectors
plan: 03
subsystem: docs
tags: [pglib, csv, collectors, expanders, merge, index]

requires:
  - phase: 278-01
    provides: foundation types (#TextDiff, #MergeResult, #MergeStrategy, #CollectOrder), error types (!CSV.*, !Storage.Space, !Text.Merge.*)
  - phase: 278-02
    provides: text operator conventions (=Text.Diff, =ForEach.Text.Lines, *Into.Text.Append, *Into.Text.Merge)
provides:
  - =ForEach.CSV.Rows expander documentation
  - *Into.CSV.Rows collector documentation
  - *Into.CSV.Merge collector documentation
  - Full pglib registry integration for all #278 operators
affects: []

tech-stack:
  added: []
  patterns: [CSV operators follow text operator conventions from 278-02]

key-files:
  created: [expanders/ForEach/CSV.Rows.md, collectors/Into/CSV.Rows.md, collectors/Into/CSV.Merge.md]
  modified: [expanders/ForEach/INDEX.md, collectors/Into/INDEX.md, collectors/INDEX.md, pglib/INDEX.md]

key-decisions:
  - "CSV.Merge reuses !Text.Merge.* errors plus adds !CSV.Merge.HeaderConflict"
  - "-Text namespace added to pglib/INDEX.md pipeline table"

patterns-established:
  - "CSV operators mirror text operator doc structure established in 278-02"

completed: 2026-04-11
---

# Plan 278-03: CSV Operators + Integration

**3 CSV operator docs (=ForEach.CSV.Rows, *Into.CSV.Rows, *Into.CSV.Merge) and full pglib registry integration for all issue #278 operators.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: =ForEach.CSV.Rows documented | Pass | 3 inputs (<csv, <delimiter, <has_header), 2 outputs (>row ##Record, >index), 3 errors |
| AC-2: *Into.CSV.Rows documented | Pass | 3 inputs (<row, <delimiter, <order), 1 output (>csv), 3 errors, PPTD note |
| AC-3: *Into.CSV.Merge documented | Pass | 3 inputs (<diffs, <base, <conflict), 1 output (>result #MergeResult), 4 errors, PPTD note |
| AC-4: All INDEX files updated | Pass | ForEach/INDEX CSV section, Into/INDEX CSV section, collectors/INDEX description, pglib/INDEX -Text namespace + types + errors |

## Accomplishments

- =ForEach.CSV.Rows expander with header-aware row parsing and ##Record output
- *Into.CSV.Rows collector with schema validation and header generation
- *Into.CSV.Merge collector reusing Text.Merge infrastructure with header protection
- Full pglib/INDEX.md integration: -Text pipeline namespace, text/CSV types row, error namespaces

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/expanders/ForEach/CSV.Rows.md` | Created | =ForEach.CSV.Rows — expand CSV into rows with index |
| `docs/user/pglib/collectors/Into/CSV.Rows.md` | Created | *Into.CSV.Rows — collect rows into CSV text |
| `docs/user/pglib/collectors/Into/CSV.Merge.md` | Created | *Into.CSV.Merge — k-way merge CSV diffs with header preservation |
| `docs/user/pglib/expanders/ForEach/INDEX.md` | Modified | Added CSV section |
| `docs/user/pglib/collectors/Into/INDEX.md` | Modified | Added CSV Collectors section |
| `docs/user/pglib/collectors/INDEX.md` | Modified | Updated *Into description to include text/CSV |
| `docs/user/pglib/INDEX.md` | Modified | Added -Text namespace, text/CSV types, !Storage/!Text/!CSV errors |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| CSV.Merge reuses !Text.Merge errors | Same merge machinery, CSV adds header-specific error only | Consistent error model across text/CSV merge |
| ##Record as CSV row output type | Header field names become record keys; positional fallback when no header | Type-safe CSV row access |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #278 fully documented: 7 types, 1 pipeline, 2 expanders, 4 collectors, 12 errors, PPTD overflow
- All operators registered in pglib INDEX hierarchy
- All files status: draft (promote to stable after issue review)

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 278-into-text-csv-collectors, Plan: 03*
*Completed: 2026-04-11*
