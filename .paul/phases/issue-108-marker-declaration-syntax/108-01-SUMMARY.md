---
phase: issue-108-marker-declaration-syntax
plan: 01
subsystem: docs
tags: [marker-decl, exe, pipeline-definition, ebnf]

requires: []
provides:
  - "marker_decl production in EBNF §9.3"
  - "Marker Declarations section in pipelines INDEX.md"
  - "Marker note on {=} in blocks.md"
affects: [issue-111-compiler-rules, issue-110-base-pipelines]

key-files:
  modified:
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/user/syntax/blocks.md
    - docs/user/concepts/pipelines/INDEX.md

key-decisions:
  - "No warning for implicit {=} default — {=} without marker = {=}[exe] silently"
  - "7 marker options: exe, b, r, p, rp, rb, pb"

duration: 5min
completed: 2026-03-31
---

# Issue #108 Plan 01: Document Marker Declaration Syntax Summary

**Added `marker_decl` grammar to EBNF §9.3, marker note to blocks.md, and Marker Declarations section to pipelines INDEX.md — 7 marker options with default and subset semantics.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-31 |
| Tasks | 3 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF §9.3 grammar includes marker_decl | Pass | `marker_decl?` in `pipeline_def`, 7 options, default rule |
| AC-2: blocks.md {=} row mentions marker syntax | Pass | Row updated + note paragraph added |
| AC-3: Pipelines INDEX.md has Marker Declarations section | Pass | Table, examples, cross-references |
| AC-4: Consistency across all three files | Pass | Marker list, default behavior, cross-references all consistent |

## Accomplishments

- Added `marker_decl` production to EBNF `pipeline_def` grammar with all 7 options
- Documented marker declarations on `{=}` in blocks.md with cross-reference to pipelines
- Created comprehensive Marker Declarations section in pipelines INDEX.md with table, 3 examples, and EBNF link

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | `marker_decl` production, rules, `{=}[b]` example |
| `docs/user/syntax/blocks.md` | Modified | `{=}` row updated, marker note paragraph added |
| `docs/user/concepts/pipelines/INDEX.md` | Modified | Marker Declarations section with table + examples |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| No warning for `{=}` without marker | Issue body specifies "no warning", overriding brainstorming draft's "suppressible warning" | Simpler default — `{=}` = `{=}[exe]` silently |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Marker syntax fully documented — foundation for compiler rules (#111)
- `{=}[b]` no-output constraint documented — ready for PGE rule

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-108-marker-declaration-syntax, Plan: 01*
*Completed: 2026-03-31*
