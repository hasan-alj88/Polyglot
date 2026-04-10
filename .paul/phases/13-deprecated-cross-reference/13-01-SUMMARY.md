---
phase: 13-deprecated-cross-reference
plan: 01
subsystem: documentation
tags: [cross-references, deprecation, audit]

requires:
  - phase: typed-cross-references
    provides: "@c: and @u: cross-reference system"
provides:
  - "@d: deprecated cross-reference type definition"
  - "Updated conventions, checklist, and classification for @d:"
affects: [14-archive-frontmatter, 15-cross-reference-enrichment]

tech-stack:
  added: []
  patterns: ["@d: informational marker (not mandatory import)"]

key-files:
  created: []
  modified:
    - docs/audit/README.md
    - docs/audit/rules/conventions.md
    - docs/audit/rules/checklist.md
    - docs/audit/tracking/ref-classification.md

key-decisions:
  - "@d: is informational, not mandatory read-before-write like @c:/@u:"

patterns-established:
  - "@d: marks deprecated content with pointer to replacement"
  - "d: prefix available for wikilink display text"

duration: 5min
completed: 2026-04-10
---

# Phase 13 Plan 01: @d: Deprecated Cross-Reference Summary

**Added `@d:` (deprecated) as third typed cross-reference prefix — informational marker for superseded content with pointer to replacement.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-04-10 |
| Tasks | 3 completed |
| Files modified | 4 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: @d: prefix defined in README.md | Pass | Type table, examples, semantics all added |
| AC-2: Conventions updated for @d: | Pass | typed-refs + deprecated-refs rules added |
| AC-3: Checklist includes @d: verification | Pass | deprecated-target-check added |
| AC-4: Classification updated with @d: category | Pass | @d: row with 0 count in both summary and migration tables |

## Accomplishments

- Defined @d: as informational (not mandatory import) — distinct from @c:/@u: semantics
- Added deprecated reference examples in both @-import and wikilink formats
- Extended all 4 audit system files consistently

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/audit/README.md` | Modified | @d: type row, examples, semantics in How @-imports work |
| `docs/audit/rules/conventions.md` | Modified | typed-refs updated + deprecated-refs rule added |
| `docs/audit/rules/checklist.md` | Modified | deprecated-target-check added |
| `docs/audit/tracking/ref-classification.md` | Modified | @d: category in summary + migration results |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| @d: is informational, not mandatory import | Deprecated content should not pollute context; Claude should use replacements instead | Simpler handling — no read-before-write overhead for archived docs |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- @d: prefix fully defined and available for use
- Phase 14 can use @d: in archive frontmatter and cross-references

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 13-deprecated-cross-reference, Plan: 01*
*Completed: 2026-04-10*
