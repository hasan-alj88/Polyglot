---
phase: 342-jm3lib-constructors
plan: 01
subsystem: jm3lib
tags: [constructors, datetime, path, jm3lib, compile-time]

requires:
  - phase: issue-341
    provides: "{$} constructor block syntax and EBNF grammar"
provides:
  - "$DT constructor with 7 overloads (3 string-parsing + 4 keyword)"
  - "$Path constructor with 3 overloads (1 string-parsing + 2 keyword)"
  - "-DT.Parse and -Path.Parse runtime parsing pipelines"
  - "jm3lib constructors/ directory with registry INDEX"
affects: [342-02 Tier 2 constructors, 343 compile error codes, 344 docs updates]

tech-stack:
  added: []
  patterns: ["{$} constructor overload pattern", "Three-Context Rule documentation pattern"]

key-files:
  created:
    - docs/user/jm3lib/constructors/INDEX.md
    - docs/user/jm3lib/constructors/DT.md
    - docs/user/jm3lib/constructors/Path.md
    - docs/user/jm3lib/pipelines/DT/Parse.md
    - docs/user/jm3lib/pipelines/Path.Parse.md
  modified:
    - docs/user/jm3lib/pipelines/DT/INDEX.md
    - docs/user/jm3lib/pipelines/Path.md

key-decisions:
  - "Used actual #Date/#Time field names (.hour/.minute/.second) from core-components.md, not illustrative names from constructors.md spec example"
  - "$DT 'Now' overload uses [.] << $now whole-tree assignment from -DT.Now pipeline result"

patterns-established:
  - "Constructor doc structure: string-parsing overloads first, keyword overloads second, overload resolution table, usage examples with parse pipeline contrast"
  - "Parse pipeline doc structure: {N} native definition, IO table, error table, constructor cross-reference"
  - "Three-Context Rule table in existing pipeline INDEX/docs for constructor-eligible types"

duration: ~15min
started: 2026-04-22
completed: 2026-04-22
---

# Plan 342-01: Tier 1 {$} Constructors ($DT + $Path) Summary

**Defined $DT (7 overloads) and $Path (3 overloads) constructors with companion -DT.Parse/-Path.Parse runtime pipelines, establishing the jm3lib constructors/ directory**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-22 |
| Completed | 2026-04-22 |
| Tasks | 3 completed |
| Files modified | 7 (5 created + 2 edited) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Constructor directory established | Pass | INDEX.md with registry table listing $DT and $Path |
| AC-2: $DT constructor fully defined | Pass | 7 overloads, all ($) have .re regex, [$] binds correct types, [.] maps actual type fields |
| AC-3: $Path constructor fully defined | Pass | 3 overloads (path string + "." + ".."), OS-aware field population |
| AC-4: Parse pipelines defined | Pass | {N} native pipelines with <raw#string input, typed output, [!] error handling |
| AC-5: Existing pipeline docs updated | Pass | Three-Context Rule tables in DT/INDEX.md and Path.md |

## Accomplishments

- Established `docs/user/jm3lib/constructors/` as the jm3lib constructor catalog with registry INDEX
- Defined complete `$DT` constructor with ISO-8601 full, date-only, time-only string-parsing overloads and Today/Yesterday/Tomorrow/Now keyword overloads
- Defined `$Path` constructor with path string, `.` (CWD), and `..` (parent) overloads
- Created `-DT.Parse` and `-Path.Parse` as runtime counterparts with proper error types
- Updated existing DT and Path pipeline docs with Three-Context Rule clarification

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/jm3lib/constructors/INDEX.md` | Created | Constructor registry with $DT and $Path entries |
| `docs/user/jm3lib/constructors/DT.md` | Created | $DT constructor — 7 overloads with regex, type binding, field mapping |
| `docs/user/jm3lib/constructors/Path.md` | Created | $Path constructor — 3 overloads with OS-aware path handling |
| `docs/user/jm3lib/pipelines/DT/Parse.md` | Created | -DT.Parse runtime pipeline for dynamic datetime strings |
| `docs/user/jm3lib/pipelines/Path.Parse.md` | Created | -Path.Parse runtime pipeline for dynamic path strings |
| `docs/user/jm3lib/pipelines/DT/INDEX.md` | Modified | Three-Context Rule table, -DT.Parse added to Construction listing |
| `docs/user/jm3lib/pipelines/Path.md` | Modified | Three-Context Rule section with constructor and parse pipeline refs |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Used actual type field names (.hour, .minute, .second) | core-components.md is authoritative for field names; constructors.md spec used illustrative names | Constructor field mappings match real type definitions |
| $DT "Now" uses whole-tree assignment `[.] << $now` | -DT.Now produces complete #DateTime; field-by-field mapping would be redundant | Cleaner native pipeline overload pattern |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Constructor catalog foundation established for Tier 2 types (plan 342-02)
- Parse pipeline pattern established for future type parsers
- Three-Context Rule documentation pattern ready for reuse

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 342-jm3lib-constructors, Plan: 01*
*Completed: 2026-04-22*
