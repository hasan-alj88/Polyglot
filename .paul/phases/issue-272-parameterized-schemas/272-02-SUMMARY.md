---
phase: issue-272-parameterized-schemas
plan: 02
subsystem: types
tags: [schemas, parameterized-schemas, retired-schemas, branch-descriptors]

requires:
  - phase: 272-01
    provides: "%## / %### property definitions, #FlexKind, #ActiveKind, #Bound"
provides:
  - "10 new ## schema files (Inf, Sorted, Fields, Nullable, Result, String, Map, Array, Set, Dataframe)"
  - "##Homogeneous and ##Heterogeneous retired with redirect stubs"
  - "5 existing schemas updated to new property names"
  - "schemas/INDEX.md fully rewritten with static, parameterized, and retired sections"
affects: [272-03 collection types, 272-04 syntax/concept docs, 272-05 technical docs]

key-files:
  created:
    - docs/user/jm3lib/types/schemas/Inf.md
    - docs/user/jm3lib/types/schemas/Sorted.md
    - docs/user/jm3lib/types/schemas/Fields.md
    - docs/user/jm3lib/types/schemas/Nullable.md
    - docs/user/jm3lib/types/schemas/Result.md
    - docs/user/jm3lib/types/schemas/String.md
    - docs/user/jm3lib/types/schemas/Map.md
    - docs/user/jm3lib/types/schemas/Array.md
    - docs/user/jm3lib/types/schemas/Set.md
    - docs/user/jm3lib/types/schemas/Dataframe.md
  modified:
    - docs/user/jm3lib/types/schemas/INDEX.md
    - docs/user/jm3lib/types/schemas/Enum.md
    - docs/user/jm3lib/types/schemas/Rectangular.md
    - docs/user/jm3lib/types/schemas/Contiguous.md
    - docs/user/jm3lib/types/schemas/Sparse.md
    - docs/user/jm3lib/types/schemas/Deep.md
    - docs/user/jm3lib/types/schemas/Homogeneous.md
    - docs/user/jm3lib/types/schemas/Heterogeneous.md

key-decisions:
  - "Retired schemas kept as redirect stubs (not deleted) for wikilink stability"
  - "##Rectangular shown as parameterized with <Dim (not static)"
  - "##Sorted added as static schema (not used by built-in types yet)"

completed: 2026-04-09
---

# Issue #272 Plan 02: Schema Files — Split, Retire, Add Summary

**One file per ## schema: 10 new parameterized/static schemas created, 2 retired, 5 updated to new property names, INDEX rewritten.**

## Performance

| Metric | Value |
|--------|-------|
| Completed | 2026-04-09 |
| Tasks | 5 completed |
| Files created | 10 |
| Files modified | 8 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Every ## schema has its own file | Pass | 19 static + parameterized schemas, each in own file |
| AC-2: ##Homogeneous, ##Heterogeneous, ##EnumLeafs no longer have definition files | Pass | Homogeneous/Heterogeneous are redirect stubs; ##EnumLeafs never had a file |
| AC-3: INDEX.md lists all schemas with correct cross-references | Pass | Three sections: Static, Parameterized, Retired |
| AC-4: %##Children. in schemas/ = 0 matches | Pass | All references replaced with new property names |

## Accomplishments

- Created 10 new schema files covering all parameterized schemas from the design
- Retired ##Homogeneous and ##Heterogeneous as redirect stubs explaining the replacement (`%###Type`)
- Updated 5 existing schemas (Enum, Rectangular, Contiguous, Sparse, Deep) to use new property names
- Rewrote INDEX.md with clear static/parameterized/retired organization

## Deviations from Plan

None -- plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All ## schema files exist with proper definitions
- Parameterized schemas document their `[#] <param` inputs
- INDEX.md serves as complete registry

**Concerns:**
- Collection type files (Map.md, Array.md, etc. in types/) still use old `%##Children.*` and `{M}` syntax -- scheduled for 272-03

**Blockers:**
- None

---
*Phase: issue-272-parameterized-schemas, Plan: 02*
*Completed: 2026-04-09*
