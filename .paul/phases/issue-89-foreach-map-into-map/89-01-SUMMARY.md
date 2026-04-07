---
phase: issue-89-foreach-map-into-map
plan: 01
subsystem: pglib
tags: [expand, collect, map, foreach, into]

requires:
  - phase: issue-88-schema-properties
    provides: "##" schema property system used by #Map definition
provides:
  - ~ForEach.Map expand operator for #Map iteration
  - "*Into.Map collect operator for #Map assembly"
  - Folder-per-variant convention for ForEach/ and Into/
affects: [issue-90-dataframe-operators, pglib-restructure]

tech-stack:
  added: []
  patterns:
    - "Dot-to-folder convention: ~ForEach.Array → ForEach/Array.md"

key-files:
  created:
    - docs/user/pglib/expanders/ForEach/Array.md
    - docs/user/pglib/expanders/ForEach/Array/Enumerate.md
    - docs/user/pglib/expanders/ForEach/Serial.md
    - docs/user/pglib/expanders/ForEach/Level.md
    - docs/user/pglib/expanders/ForEach/Map.md
    - docs/user/pglib/collectors/Into/Array.md
    - docs/user/pglib/collectors/Into/Serial.md
    - docs/user/pglib/collectors/Into/Level.md
    - docs/user/pglib/collectors/Into/Map.md
  modified:
    - docs/user/concepts/collections.md
    - docs/user/pglib/INDEX.md
    - docs/user/PGLIB.md
    - docs/technical/EBNF.md
    - docs/technical/compile-rules/PGE/PGE03007-expand-operator-input-mismatch.md
    - docs/technical/compile-rules/PGE/PGE03008-collect-operator-io-mismatch.md
    - docs/user/pglib/types/types.md
    - docs/user/syntax/types.md
    - docs/technical/plan/decisions/schema-properties.md

key-decisions:
  - "Renamed #Dict to #Map — unified duplicate collection type"
  - "Dot-to-folder convention: each operator variant gets its own file"
  - "No ~ForEach.Map.Enumerate — Map is unordered"

patterns-established:
  - "pglib namespace dot = folder hierarchy (ForEach.Array → ForEach/Array.md)"

duration: ~15min
completed: 2026-03-28
---

# Issue #89 Plan 01: Add ~ForEach.Map and *Into.Map Summary

**Added first-class Map expand/collect operators and restructured ForEach + Into into folder-per-variant hierarchy.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Completed | 2026-03-28 |
| Tasks | 3 completed |
| Files created | 9 |
| Files modified | 9 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: ForEach split into folder hierarchy | Pass | 5 files in ForEach/, old ForEach.md deleted |
| AC-2: Into split into folder hierarchy | Pass | 4 files in Into/, old Into.md deleted |
| AC-3: ForEach.Map documented | Pass | IO: <Map, >key, >item; no .Enumerate |
| AC-4: Into.Map documented | Pass | IO: <key, <value, >Map; duplicate keys = runtime error |
| AC-5: collections.md updated | Pass | Both tables + Map example with =Math.Multiply |
| AC-6: EBNF grammar updated | Pass | Both productions + IO signature tables |
| AC-7: Compile rule tables updated | Pass | PGE03007 and PGE03008 both have Map entries |
| AC-8: INDEX.md links updated | Pass | Folder links, descriptions mention "maps" |

## Accomplishments

- Added `~ForEach.Map` and `*Into.Map` operators completing the collection operator matrix for `#Map`
- Restructured ForEach.md → ForEach/ folder (5 files) and Into.md → Into/ folder (4 files) establishing dot-to-folder convention
- Unified `#Dict` → `#Map` across 3 files (types/types.md, syntax/types.md, schema-properties.md)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/expanders/ForEach/Array.md` | Created | ~ForEach.Array spec |
| `docs/user/pglib/expanders/ForEach/Array/Enumerate.md` | Created | ~ForEach.Array.Enumerate spec |
| `docs/user/pglib/expanders/ForEach/Serial.md` | Created | ~ForEach.Serial spec |
| `docs/user/pglib/expanders/ForEach/Level.md` | Created | ~ForEach.Level spec |
| `docs/user/pglib/expanders/ForEach/Map.md` | Created | ~ForEach.Map spec (NEW) |
| `docs/user/pglib/collectors/Into/Array.md` | Created | *Into.Array spec |
| `docs/user/pglib/collectors/Into/Serial.md` | Created | *Into.Serial spec |
| `docs/user/pglib/collectors/Into/Level.md` | Created | *Into.Level spec |
| `docs/user/pglib/collectors/Into/Map.md` | Created | *Into.Map spec (NEW) |
| `docs/user/pglib/expanders/ForEach.md` | Deleted | Replaced by folder hierarchy |
| `docs/user/pglib/collectors/Into.md` | Deleted | Replaced by folder hierarchy |
| `docs/user/concepts/collections.md` | Modified | Added Map rows to tables + example |
| `docs/technical/EBNF.md` | Modified | Added Map to grammar + IO tables |
| `docs/user/pglib/INDEX.md` | Modified | Updated links to folders |
| `docs/user/PGLIB.md` | Modified | Updated links to folders |
| `docs/technical/compile-rules/PGE/PGE03007-*.md` | Modified | Added ForEach.Map IO signature |
| `docs/technical/compile-rules/PGE/PGE03008-*.md` | Modified | Added Into.Map IO signature |
| `docs/user/pglib/types/types.md` | Modified | #Dict → #Map rename |
| `docs/user/syntax/types.md` | Modified | #Dict → #Map rename |
| `docs/technical/plan/decisions/schema-properties.md` | Modified | #Dict → #Map rename |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Rename #Dict to #Map | Duplicate type with #Map in collections.md; #Map uses newer ## schema system | All #Dict references removed; #Dataframe rows now reference #Map |
| Dot-to-folder convention | User requested each variant in its own file matching namespace hierarchy | Establishes pattern for remaining pglib restructure (Agg, Sync, pipelines) |
| No ForEach.Map.Enumerate | Map is unordered (%Ordered = #False); positional index misleading | Consistent with Serial which also has no .Enumerate |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Stale link fix |
| Scope additions | 1 | #Dict → #Map rename (pre-plan) |
| Deferred | 0 | None |

**Total impact:** Essential fixes, no scope creep

### Auto-fixed Issues

**1. Stale PGLIB.md links**
- **Found during:** Task 3 verification
- **Issue:** PGLIB.md still linked to deleted ForEach.md and Into.md
- **Fix:** Updated links to ForEach/ and Into/ folders
- **Files:** docs/user/PGLIB.md

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Map operators fully integrated across spec, grammar, and compile rules
- Folder-per-variant pattern established for future pglib restructure

**Concerns:**
- Remaining pglib files (Agg, Sync, Continue, pipelines) still use single-file pattern — needs separate issue for restructure

**Blockers:**
- None

---
*Phase: issue-89-foreach-map-into-map, Plan: 01*
*Completed: 2026-03-28*
