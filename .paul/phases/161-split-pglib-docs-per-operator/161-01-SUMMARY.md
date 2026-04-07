---
phase: 161-split-pglib-docs-per-operator
plan: 01
subsystem: docs
tags: [pglib, collectors, doc-template]

requires:
  - phase: 160-split-dt-pipeline-template
    provides: pipeline doc template pattern and DT/ folder reference
provides:
  - 15 individual collector operator files (6 Agg, 4 Sync, 5 Into reformatted)
  - 4 INDEX files (collectors, Agg, Sync, Into)
  - Collector doc template (## Syntax variant)
affects: [161-02 expanders, 161-05 wikilinks]

tech-stack:
  added: []
  patterns: [collector-doc-template]

key-files:
  created:
    - docs/user/pglib/collectors/INDEX.md
    - docs/user/pglib/collectors/Agg/INDEX.md
    - docs/user/pglib/collectors/Sync/INDEX.md
    - docs/user/pglib/collectors/Into/INDEX.md
  modified:
    - docs/user/pglib/collectors/Into/Array.md
    - docs/user/pglib/collectors/Into/Map.md
    - docs/user/pglib/collectors/Into/Serial.md
    - docs/user/pglib/collectors/Into/Level.md
    - docs/user/pglib/collectors/Into/Dataframe.md

key-decisions:
  - "*Second documented as alias inside Nth.md, not separate file"
  - "$* inline discard documented in Sync/INDEX.md (language construct, not operator)"

patterns-established:
  - "Collector template: frontmatter → description → ## Syntax → ## Inputs → ## Outputs → ## Errors → ## Permissions → ## Related"

duration: 10min
completed: 2026-04-07
---

# Phase 161 Plan 01: Collectors Summary

**Split Agg.md and Sync.md into per-operator files, reformatted Into/*.md to collector template, created 4 INDEX files**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Completed | 2026-04-07 |
| Tasks | 3 completed |
| Files created | 14 |
| Files modified | 5 |
| Files deleted | 2 |
| Total in collectors/ | 19 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Individual collector files exist | Pass | 6 Agg + 4 Sync files created; Agg.md and Sync.md deleted |
| AC-2: All collector files follow template | Pass | All 15 operator files have Syntax/Inputs/Outputs/Errors/Permissions/Related |
| AC-3: INDEX files provide navigation | Pass | 4 INDEX files with wikilinks to all operators |
| AC-4: Into/*.md files reformatted | Pass | IO Signature → Inputs+Outputs tables; Usage → Syntax; missing sections added |

## Accomplishments

- Split 2 monolithic files into 10 individual operator files
- Reformatted 5 existing Into/ files to match collector doc template
- Created 4 INDEX files providing full navigation hierarchy
- Established collector doc template pattern (## Syntax replaces ## Definition)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `collectors/INDEX.md` | Created | Top-level collector overview |
| `collectors/Agg/INDEX.md` | Created | Aggregation collector listing |
| `collectors/Agg/Sum.md` | Created | *Agg.Sum operator |
| `collectors/Agg/Count.md` | Created | *Agg.Count operator |
| `collectors/Agg/Average.md` | Created | *Agg.Average operator |
| `collectors/Agg/Max.md` | Created | *Agg.Max operator |
| `collectors/Agg/Min.md` | Created | *Agg.Min operator |
| `collectors/Agg/Concatenate.md` | Created | *Agg.Concatenate operator |
| `collectors/Sync/INDEX.md` | Created | Collect-all & race listing |
| `collectors/Sync/All.md` | Created | *All operator |
| `collectors/Sync/First.md` | Created | *First operator |
| `collectors/Sync/Nth.md` | Created | *Nth operator (includes *Second alias) |
| `collectors/Sync/Ignore.md` | Created | *Ignore operator |
| `collectors/Into/INDEX.md` | Created | Data collector listing |
| `collectors/Into/Array.md` | Modified | Reformatted to template |
| `collectors/Into/Map.md` | Modified | Reformatted to template |
| `collectors/Into/Serial.md` | Modified | Reformatted to template |
| `collectors/Into/Level.md` | Modified | Reformatted to template |
| `collectors/Into/Dataframe.md` | Modified | Reformatted to template |
| `collectors/Agg.md` | Deleted | Replaced by Agg/ folder |
| `collectors/Sync.md` | Deleted | Replaced by Sync/ folder |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| *Second as alias in Nth.md | Sugar for *Nth n=2, not distinct operator | Avoids near-duplicate file |
| $* in Sync/INDEX.md not own file | Language construct, not collector operator | Keeps operator files pure |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Negligible |

**Total impact:** Trivial count error, no scope change.

**1. File count arithmetic**
- **Issue:** Plan predicted 18 .md files, actual is 19
- **Cause:** Miscounted INDEX files (4 not 3)
- **Impact:** None -- all files correct

## Next Phase Readiness

**Ready:**
- Collector doc template validated and applied
- Same pattern ready for expanders (Plan 161-02)

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 161-split-pglib-docs-per-operator, Plan: 01*
*Completed: 2026-04-07*
