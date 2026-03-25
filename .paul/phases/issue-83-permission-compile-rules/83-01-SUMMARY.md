---
phase: issue-83-permission-compile-rules
plan: 01
subsystem: compiler
tags: [permissions, compile-rules, PGE, PGW]

requires:
  - phase: issue-80
    provides: _ permission prefix and [_] block marker
  - phase: issue-81
    provides: Package ceiling (PGE-915/916) and pipeline-level [_]
provides:
  - PGE-917 Unknown Permission Category rule
  - PGE-918 Undeclared Permission rule (call graph tracing)
  - PGE-919 Permission Output rule
  - PGE-920 Duplicate Permission rule
  - PGW-903 Unused Permission warning
affects: []

tech-stack:
  added: []
  patterns: [compile-rule-file-format]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE-917-unknown-permission-category.md
    - docs/technical/compile-rules/PGE/PGE-918-undeclared-permission.md
    - docs/technical/compile-rules/PGE/PGE-919-permission-output.md
    - docs/technical/compile-rules/PGE/PGE-920-duplicate-permission.md
    - docs/technical/compile-rules/PGW/PGW-903-unused-permission.md
  modified:
    - docs/technical/COMPILE-RULES.md

key-decisions:
  - "PGE-915/916 kept as packages.md defines them; issue's Duplicate Permission renumbered to PGE-920"
  - "All permission rules extend 9.x (Packages) category range"

patterns-established: []

duration: 5min
completed: 2026-03-25
---

# Issue #83 Plan 01: Permission Compile Rules Summary

**Added 4 PGE error rules (917–920) and 1 PGW warning (903) for compile-time permission enforcement with full examples.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-25 |
| Tasks | 3 completed |
| Files modified | 6 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: COMPILE-RULES.md Reference Tables Updated | Pass | PGE-915–920 + PGW-903 all present in tables |
| AC-2: Individual Rule Files Created | Pass | 5 files with correct frontmatter and format |
| AC-3: Rule Content Matches Permission Spec | Pass | All rules reference 8 categories, call graph tracing, IO form |
| AC-4: Cross-References Complete | Pass | All files have [[permissions]] wikilinks and See also links |

## Accomplishments

- Added PGE-915/916 to COMPILE-RULES.md reference table (were missing despite being defined in packages.md from #81)
- Created 4 PGE rule files with Statement, Rationale, Detection, See also, VALID/INVALID examples
- Created 1 PGW rule file with Statement, Rationale, Detection, See also, VALID/WARNING examples
- PGE-918 documents call graph tracing including transitive calls

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/COMPILE-RULES.md` | Modified | Added PGE-915–920 and PGW-903 to reference tables |
| `docs/technical/compile-rules/PGE/PGE-917-unknown-permission-category.md` | Created | Unknown category/subfield in [_] declaration |
| `docs/technical/compile-rules/PGE/PGE-918-undeclared-permission.md` | Created | IO call without matching [_] permission |
| `docs/technical/compile-rules/PGE/PGE-919-permission-output.md` | Created | Output direction in permission IO block |
| `docs/technical/compile-rules/PGE/PGE-920-duplicate-permission.md` | Created | Same permission declared twice in one scope |
| `docs/technical/compile-rules/PGW/PGW-903-unused-permission.md` | Created | Permission declared but never exercised |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Renumbered issue's "Duplicate Permission" from PGE-916 to PGE-920 | PGE-916 already taken by "Imported Package Exceeds Importer Permission Ceiling" from #81 | No conflict with existing codes |
| Added PGE-915/916 to COMPILE-RULES.md table | Were defined in packages.md but missing from the central reference table | Reference table now complete for all 9.x rules |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All permission compile rules documented
- Ready for /paul:merge to close issue #83

**Concerns:**
- PGE-915/916 have individual rule files in packages.md but no dedicated rule files in compile-rules/PGE/ (could be a future issue)

**Blockers:**
- None

---
*Phase: issue-83-permission-compile-rules, Plan: 01*
*Completed: 2026-03-25*
