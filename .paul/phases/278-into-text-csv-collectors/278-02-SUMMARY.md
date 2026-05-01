---
phase: 278-into-text-csv-collectors
plan: 02
subsystem: docs
tags: [jm3lib, text, collectors, expanders, pipelines, diff, merge]

requires:
  - phase: 278-01
    provides: foundation types (#TextDiff, #DiffOp, #DiffStats, #MergeConflict, #MergeResult, #MergeStrategy, #CollectOrder), error types (!Text.*, !Storage.Space), PPTD overflow docs
provides:
  - =Text.Diff comparator pipeline documentation
  - =ForEach.Text.Lines expander documentation
  - *Into.Text.Append collector documentation
  - *Into.Text.Merge collector documentation with workflow example
affects: [278-03-csv-operators]

tech-stack:
  added: []
  patterns: [Text namespace under pipelines/, text operators in existing ForEach/Into INDEX structure]

key-files:
  created: [pipelines/Text/Diff.md, expanders/ForEach/Text.Lines.md, collectors/Into/Text.Append.md, collectors/Into/Text.Merge.md]
  modified: [expanders/ForEach/INDEX.md, collectors/Into/INDEX.md]

key-decisions:
  - "pipelines/Text/ new folder for =Text.Diff — first non-File text pipeline namespace"
  - "Text operators use flat files (Text.Lines.md, Text.Append.md) not subfolders — no variants yet"

patterns-established:
  - "Text operator docs follow same structure as Array/Map/Serial counterparts"
  - "PPTD overflow referenced (not re-documented) in collector files"

completed: 2026-04-11
---

# Plan 278-02: Text Operators

**4 text operator docs (=Text.Diff, =ForEach.Text.Lines, *Into.Text.Append, *Into.Text.Merge) with INDEX updates — completing the expand/collect pattern for text data.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: =Text.Diff pipeline documented | Pass | {N} definition, <original/<modified inputs, >diffs/>stats outputs, !Text.Diff.EmptyInput, metadata paths |
| AC-2: =ForEach.Text.Lines expander documented | Pass | Expand syntax, <text input, >line/>index outputs, !Text.Lines.Empty |
| AC-3: *Into.Text.Append collector documented | Pass | <fragment/<separator/<order inputs, >text output, !Text.Append.EmptyResult, PPTD note |
| AC-4: *Into.Text.Merge collector documented | Pass | <diffs/<base/<conflict inputs, >result (#MergeResult) output, errors, workflow, PPTD note, _File.TempWrite |
| AC-5: INDEX files updated | Pass | ForEach/INDEX.md has Text section; Into/INDEX.md has Text Collectors section |

## Accomplishments

- =Text.Diff comparator pipeline with #NativeKind.Computation, producing #TextDiffs and #DiffStats
- =ForEach.Text.Lines expander with >line and >index outputs for parallel line processing
- *Into.Text.Append collector with separator, ordering (#CollectOrder), and PPTD overflow support
- *Into.Text.Merge collector with k-way diff merge, #MergeStrategy conflict resolution, canonical workflow example

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/jm3lib/pipelines/Text/Diff.md` | Created | =Text.Diff comparator — compares two texts, outputs line-level diffs |
| `docs/user/jm3lib/expanders/ForEach/Text.Lines.md` | Created | =ForEach.Text.Lines — expand text into lines with index |
| `docs/user/jm3lib/collectors/Into/Text.Append.md` | Created | *Into.Text.Append — concatenate fragments with separator and ordering |
| `docs/user/jm3lib/collectors/Into/Text.Merge.md` | Created | *Into.Text.Merge — k-way merge diffs against base with conflict resolution |
| `docs/user/jm3lib/expanders/ForEach/INDEX.md` | Modified | Added Text section with =ForEach.Text.Lines entry |
| `docs/user/jm3lib/collectors/Into/INDEX.md` | Modified | Added Text Collectors section with Append and Merge entries |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| New pipelines/Text/ folder | =Text.Diff is computation, not file I/O — separate from File/ namespace | Establishes Text/ as utility pipeline namespace |
| Flat files, not subfolders | No variants for any of these operators yet | Can add subfolders later if variants emerge |
| Workflow example in Text.Merge | K-way merge pattern is non-obvious — canonical example shows =Text.Diff → *Into.Text.Merge pipeline | Users can copy/adapt the pattern |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All text operators documented for 278-03 (CSV operators can reference text patterns)
- INDEX structure established — 278-03 adds CSV entries to same INDEX files
- pipelines/Text/ namespace available if CSV needs comparator pipelines

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 278-into-text-csv-collectors, Plan: 02*
*Completed: 2026-04-11*
