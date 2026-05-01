---
phase: 15-cross-reference-enrichment
plan: 01
subsystem: documentation
tags: [cross-references, deprecation, archive]

requires:
  - phase: 13-deprecated-cross-reference
    provides: "@d: cross-reference type definition"
  - phase: 14-archive-frontmatter
    provides: "replaced_by: field in all 52 archived doc frontmatters"
provides:
  - "Cross-references (@d:, @c:, @u:) in 13 non-v0.0.2 archived docs"
affects: [16-triage-decision]

tech-stack:
  added: []
  patterns: ["@d: deprecation notice + blockquote after frontmatter", "@c:/@u: HTML comments near first concept/syntax mention"]

key-files:
  created: []
  modified:
    - docs/archive/deprecated/README-original.md
    - docs/archive/deprecated/architecture.md
    - docs/archive/deprecated/doc/04-package-management.md
    - docs/archive/deprecated/doc/05-standard-library.md
    - docs/archive/deprecated/doc/06-error-handling.md
    - docs/archive/deprecated/doc/07-flow-control.md
    - docs/archive/deprecated/doc/09-architecture.md
    - docs/archive/deprecated/doc/12-development-roadmap.md
    - docs/archive/deprecated/doc/13-contributing.md
    - docs/archive/deprecated/doc/DOCUMENTATION_INDEX.md
    - docs/archive/deprecated/doc/getting-started.md
    - "docs/archive/deprecated/doc/Syntax Reference.md"
    - docs/archive/wip/draft.md

key-decisions:
  - "HTML comment form only for @c:/@u: in archive files — no wikilinks in deprecated content"

patterns-established:
  - "@d: notice immediately after frontmatter closing --- with blockquote"
  - "replaced_by: none files get <!-- no current replacement --> instead of @d:"

duration: 3min
completed: 2026-04-11
---

# Phase 15 Plan 01: Cross-Reference Enrichment (doc/ + wip) Summary

**Added @d:, @c:, and @u: cross-references to 13 non-v0.0.2 archived docs — linking deprecated content to current spec replacements.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~3min |
| Completed | 2026-04-11 |
| Tasks | 2 completed |
| Files modified | 13 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Every file has @d: reference | Pass | 12 files with @d:, 1 file (13-contributing) with "no current replacement" |
| AC-2: Files with replaced_by: none have no @d: | Pass | 13-contributing.md correctly uses <!-- no current replacement --> |
| AC-3: @c:/@u: references added where applicable | Pass | 52 lines added total; 0-4 @c:/@u: per file |

## Accomplishments

- 12 files enriched with @d: pointing to replaced_by targets
- 1 file (13-contributing.md) correctly marked with "no current replacement"
- Targeted @c:/@u: added for glossary terms (TM, QM, pipeline, jm3lib) and syntax constructs (blocks, operators, errors)

## Files Created/Modified

| File Group | Count | Change |
|------------|-------|--------|
| docs/archive/deprecated/doc/ | 10 | @d: + @c:/@u: cross-references |
| docs/archive/deprecated/ (root) | 2 | @d: + @c:/@u: cross-references |
| docs/archive/wip/ | 1 | @d: cross-reference |

## Decisions Made

None — followed plan as specified.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 13 non-v0.0.2 files enriched for Phase 16 triage

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 15-cross-reference-enrichment, Plan: 01*
*Completed: 2026-04-11*
