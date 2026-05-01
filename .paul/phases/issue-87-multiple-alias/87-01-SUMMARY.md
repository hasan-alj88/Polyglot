---
phase: issue-87-multiple-alias
plan: 01
subsystem: design
tags: [metadata, alias, types, compile-rules, EBNF]

requires:
  - phase: issue-88-schema-properties
    provides: schema property system (%##), #IndexString type, metadata tree structure
provides:
  - "#KeyString type (renamed from #IndexString)"
  - "#NestedKeyString type (allows . and : in alias paths)"
  - "Flexible %alias field with array semantics"
  - "PGE12002 duplicate alias compile rule"
affects: [compiler implementation, future type system work]

tech-stack:
  added: []
  patterns:
    - "Flexible metadata fields use % prefix with [:] children"

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE12002-duplicate-alias.md
  modified:
    - docs/user/syntax/types.md
    - docs/user/concepts/metadata.md
    - docs/user/concepts/collections.md
    - docs/user/syntax/blocks.md
    - docs/user/jm3lib/types/types.md
    - docs/technical/EBNF.md
    - docs/technical/COMPILE-RULES.md
    - docs/technical/compile-rules/PGE/PGE01015-duplicate-metadata-field.md
    - docs/technical/spec/metadata-tree.md
    - docs/technical/EDGE-CASES.md
    - docs/draft.md

key-decisions:
  - "Renamed #IndexString to #KeyString for clearer terminology"
  - "#NestedKeyString allows . and : but excludes whitespace, <, >"
  - "%alias is flexible field with #Array.NestedKeyString type"
  - "PGE12002 in 10xx range (metadata rules)"
  - "%##Alias schema property type changed from #string to #NestedKeyString"

patterns-established:
  - "Flexible metadata fields use %name prefix (not .name fixed prefix)"

duration: ~15min
started: 2026-03-28
completed: 2026-03-28
---

# Issue #87 Plan 01: Multiple Alias Declarations Summary

**Renamed #IndexString → #KeyString, added #NestedKeyString type, converted singular .alias to flexible %alias with array semantics, created PGE12002 for globally unique alias enforcement.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-03-28 |
| Completed | 2026-03-28 |
| Tasks | 3 completed |
| Files modified | 12 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: #IndexString renamed to #KeyString globally | Pass | Zero occurrences of #IndexString remain; 5 files updated |
| AC-2: #NestedKeyString type exists | Pass | Defined in types.md and draft.md; regex `^[^\s<>]+$` |
| AC-3: %alias is flexible field with array semantics | Pass | All .alias << syntax replaced with %alias + [:] children |
| AC-4: PGE01015 no longer covers .alias | Pass | .alias removed from fixed-field list; note added pointing to PGE12002 |
| AC-5: PGE12002 enforces globally unique aliases | Pass | New rule file with VALID/INVALID examples; added to COMPILE-RULES.md table |
| AC-6: %##Alias uses #NestedKeyString | Pass | Updated in types.md, metadata-tree.md, and draft.md schema property tables |

## Accomplishments

- Renamed `#IndexString` → `#KeyString` across entire spec (5 files, consistent terminology)
- Added `#NestedKeyString` type for alias paths that include `.` and `:` separators
- Converted `.alias` from singular fixed field to flexible `%alias#Array.NestedKeyString` with `[:]` children across all spec and jm3lib files
- Created PGE12002 (Duplicate Alias) with intra-definition and inter-definition duplicate detection
- Updated EBNF `metadata_alias` production to match new array syntax

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/types.md` | Modified | #KeyString rename, #NestedKeyString definition, %##Alias type update |
| `docs/user/concepts/metadata.md` | Modified | %alias flexible field, metadata tree diagram update |
| `docs/user/concepts/collections.md` | Modified | #KeyString rename |
| `docs/user/syntax/blocks.md` | Modified | %alias description |
| `docs/user/jm3lib/types/types.md` | Modified | All enum .alias → %alias syntax (26 aliases converted) |
| `docs/technical/EBNF.md` | Modified | metadata_alias production, #KeyString, rules text, parse tree example |
| `docs/technical/COMPILE-RULES.md` | Modified | PGE12002 table entry, #KeyString rename |
| `docs/technical/compile-rules/PGE/PGE01015-duplicate-metadata-field.md` | Modified | Removed .alias from fixed-field list |
| `docs/technical/compile-rules/PGE/PGE12002-duplicate-alias.md` | Created | New compile rule for globally unique aliases |
| `docs/technical/spec/metadata-tree.md` | Modified | #KeyString rename, %##Alias type update |
| `docs/technical/EDGE-CASES.md` | Modified | EC-15.2 rewritten for multi-alias, EC-15.2b added for nested paths |
| `docs/draft.md` | Modified | #KeyString, #NestedKeyString, %##Alias type, Decision #15 update |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Renamed #IndexString → #KeyString | "Key" is more descriptive and consistent with its role as tree child key type | All references updated; no functional change |
| #NestedKeyString regex `^[^\s<>]+$` | Must allow `.` and `:` for nested alias paths while still excluding parse-ambiguous chars | Enables aliases like "File.Permission.Denied" |
| %alias as flexible field, not fixed | Array semantics require flexible children; fixed fields are singular by design | Consistent with existing flexible field pattern (:info) |
| PGE12002 in 10xx range | Metadata rules live in 10xx; 1001 already exists for undefined metadata access | Follows existing numbering convention |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Spec fully updated for multi-alias support
- All cross-references consistent
- Ready for /paul:merge to commit and merge to main

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-87-multiple-alias, Plan: 01*
*Completed: 2026-03-28*
