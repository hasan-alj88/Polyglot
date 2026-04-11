---
phase: 14-archive-frontmatter
plan: 01
subsystem: documentation
tags: [frontmatter, deprecation, archive]

requires:
  - phase: 13-deprecated-cross-reference
    provides: "@d: cross-reference type definition"
provides:
  - "replaced_by: field in all 52 archived doc frontmatters"
  - "Full frontmatter on wip/draft.md"
affects: [15-cross-reference-enrichment, 16-triage-decision]

key-files:
  modified:
    - docs/archive/deprecated/**/*.md (51 files)
    - docs/archive/wip/draft.md (1 file)

key-decisions:
  - "replaced_by: none used for files with no current equivalent (3 files)"

patterns-established:
  - "Archive frontmatter standard: status, archived, source_branch, note, replaced_by"

duration: 5min
completed: 2026-04-11
---

# Phase 14 Plan 01: Archive Frontmatter & Deprecation Marking Summary

**Added `replaced_by:` field to all 52 archived docs pointing to current-spec replacements; added full frontmatter to 1 file missing it.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-04-11 |
| Tasks | 3 completed |
| Files modified | 52 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: All archived files have replaced_by | Pass | 52/52 files confirmed via grep |
| AC-2: wip/draft.md has deprecated frontmatter | Pass | All 5 fields present |
| AC-3: Replacement targets are valid paths | Pass | 5 spot-checked, all resolve |

## Accomplishments

- 51 existing deprecated files enriched with replaced_by pointers
- 1 file (wip/draft.md) given full deprecated frontmatter from scratch
- 3 files correctly marked replaced_by: none (no current equivalent)
- Parallel execution via 2 agents completed both groups simultaneously

## Files Created/Modified

| File Group | Count | Change |
|------------|-------|--------|
| docs/archive/deprecated/doc/ + root | 12 | Added replaced_by field |
| docs/archive/deprecated/v0.0.2/ | 39 | Added replaced_by field |
| docs/archive/wip/draft.md | 1 | Full frontmatter added |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| replaced_by: none for 3 files | database-schema, ir-representation, contributing have no current equivalent | Phase 16 triage can flag these for deletion |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 52 files have replaced_by pointers for Phase 15 cross-reference enrichment
- Phase 15 can use replaced_by values to generate @d: cross-references

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 14-archive-frontmatter, Plan: 01*
*Completed: 2026-04-11*
