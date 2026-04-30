---
phase: issue-75-dict-kv
plan: 02
subsystem: docs
tags: [type-system, collections, type-hierarchy, schema-properties]

requires:
  - phase: issue-75-dict-kv/75-01
    provides: Approved type hierarchy in docs/draft.md, all gap resolutions in schema-properties.md
provides:
  - Rewritten types.md with ground-up type hierarchy and # annotations
  - Updated collections.md with #Dict, #Dataframe, array : keys
affects: [75-03 technical spec, 75-04 spec-wide migration, EBNF, compile-rules, aj3lib types]

tech-stack:
  added: []
  patterns:
    - "# type annotation system (replacing ;)"
    - "<~ schema inheritance operator"
    - "[#] % schema properties in {#} blocks"
    - "< generic type parameters with [<] constraints"

key-files:
  modified:
    - docs/user/syntax/types.md
    - docs/user/concepts/collections.md

key-decisions:
  - "Included full {#} definitions for all scalar subtypes in types.md"
  - "Added Type Hierarchy Summary section at bottom of types.md"
  - "collections.md gets #Dict and #Dataframe inline sections rather than separate files"

patterns-established:
  - "#type annotation throughout user-facing spec files"
  - "Schema property documentation pattern ([#] % in tables)"

completed: 2026-03-27
---

# Issue #75 Plan 02: Core Type System Spec Rewrite

**Rewrote types.md with complete ground-up type hierarchy (RawString → #String → scalars → collections) and updated collections.md with #Dict, #Dataframe, and array `:` key migration.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 3 completed (including 1 checkpoint) |
| Files modified | 2 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Type Hierarchy Complete | Pass | All layers (0-3) documented with full {#} definitions |
| AC-2: # Annotation System | Pass | Zero ; annotations remaining; character role table updated |
| AC-3: Schema Properties and Generics | Pass | %Key.Type, %Key.Gap, %Ordered, %Depth.Max, %Alias documented; [<] constraints; #* wildcard |
| AC-4: Collections with #Dict and #Dataframe | Pass | 5-entry table; #Dict and #Dataframe with usage examples; array uses : keys |

## Accomplishments

- Complete types.md rewrite: 4-layer hierarchy, all scalar subtypes (#Int, #UnsignedInt, #Float, #Sci, #Eng, #Dimension), #Boolean, schema properties, generic type params, [<] constraints
- collections.md updated with #Dict (typed K-V), #Dataframe (tabular), expanded collection table, and "assembled at once" description
- Zero ; type annotations remain in either file
- Zero %Open references remain

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/types.md` | Rewritten | Ground-up type hierarchy, # annotations, schema properties, generics |
| `docs/user/concepts/collections.md` | Modified | #Dict, #Dataframe, collection table, array : keys, # annotations |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Full {#} definitions inline in types.md | Single source of truth for type hierarchy | Readers see complete definitions without jumping to aj3lib |
| Type Hierarchy Summary section added | Quick reference for the full inheritance tree | aids navigation and understanding |
| #Dict/#Dataframe as inline sections in collections.md | Small enough to not warrant separate files | Keeps collection docs consolidated |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- types.md and collections.md are the authoritative source of truth for # annotations
- All downstream files can reference these for correct syntax patterns
- docs/draft.md content has been placed into spec — draft can be cleaned up

**Concerns:**
- ~120 remaining files still use ; annotations — Plan 75-04 needed
- aj3lib/types/types.md missing core type definitions (#String, #Int, #Array, #Dict, etc.) — include in Plan 75-03
- type-identity.md still uses ; annotations and old array . syntax

**Blockers:**
None.

---
*Phase: issue-75-dict-kv, Plan: 02*
*Completed: 2026-03-27*
