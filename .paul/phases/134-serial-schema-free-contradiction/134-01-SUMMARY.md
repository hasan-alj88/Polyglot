---
phase: 134-serial-schema-free-contradiction
plan: 01
subsystem: docs
tags: [serial, schema-properties, terminology]

requires:
  - phase: none
    provides: n/a
provides:
  - Accurate #Serial terminology across all docs
  - Complete #Serial definition with all 6 constraint-removal properties
affects: []

tech-stack:
  added: []
  patterns: [constraint-removal framing for schema properties]

key-files:
  created: []
  modified:
    - docs/user/concepts/collections/serial.md
    - docs/user/jm3lib/types/collections.md
    - docs/technical/edge-cases/24-datatype-defs.md

key-decisions:
  - "Frame each schema property as removing a constraint, not adding one"
  - "Added 3 missing properties (Ordered, Regular, Max) to complete the definition"

patterns-established:
  - "Use 'unconstrained' not 'schema-free' when describing #Serial"

duration: 10min
started: 2026-04-05
completed: 2026-04-05
---

# Issue #134 Plan 01: #Serial Schema-Free Contradiction Fix

**Replaced false "schema-free" claim with constraint-removal explanation across 12 files; completed #Serial definition with 3 missing properties.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Started | 2026-04-05 |
| Completed | 2026-04-05 |
| Tasks | 4 completed |
| Files modified | 12 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: serial.md explains how each property removes a constraint | Pass | Table with 6 rows showing property → constraint removed → effect |
| AC-2: Consistent terminology across all files | Pass | 0 "schema-free" matches outside brainstorming/ |
| AC-3: EC-24.16 edge case title and description updated | Pass | Title: "maximally permissive schemas"; body: "unconstrained data" |
| AC-4: #Serial definition completed with all constraint-removal properties | Pass | Both collections.md and 24-datatype-defs.md have all 6 properties |

## Accomplishments

- Replaced "schema-free" with "unconstrained" across 12 files (only brainstorming/ retains old term)
- Added constraint-removal table to serial.md explaining what each property removes and why
- Completed #Serial jm3lib definition: added %##Children.Ordered, %##Children.Regular, %##Children.Max
- Aligned edge-case EC-24.16 definition with jm3lib (previously inconsistent)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/collections/serial.md` | Modified | Heading + constraint-removal table + complete definition |
| `docs/user/jm3lib/types/collections.md` | Modified | Complete definition + schema table with "Constraint Removed" column |
| `docs/technical/edge-cases/24-datatype-defs.md` | Modified | EC-24.16 title, body, and code example aligned |
| `docs/INDEX.md` | Modified | "schema-free" → "unconstrained" |
| `docs/user/concepts/collections/INDEX.md` | Modified | Two occurrences updated |
| `docs/user/concepts/collections/user-struct.md` | Modified | "schema-free" → "unconstrained" |
| `docs/user/syntax/types/basic-types.md` | Modified | "schema-free" → "unconstrained" |
| `docs/user/syntax/types/hierarchy.md` | Modified | "schema-free" → "unconstrained" |
| `docs/user/syntax/types/conversions.md` | Modified | "schema-free" → "unconstrained" |
| `docs/user/jm3lib/types/types.md` | Modified | "schema-free" → "unconstrained" |
| `docs/technical/COMPILE-RULES.md` | Modified | PGW11003 rationale wording |
| `docs/technical/compile-rules/PGE/PGE04009-unhandled-serial-struct-conversion.md` | Modified | "schema-free" → "unconstrained" |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Frame properties as constraint removal | Each property removes a specific restriction other types have — clearer than "maximally permissive" | serial.md now explains WHY each property exists |
| Add 3 missing properties | Ordered, Regular, Max were never set — those constraints were technically still in effect | #Serial definition is now truly complete |
| Leave brainstorming/ unchanged | Informal content, different context ("treated as serial") | 1 "schema-free" occurrence remains, acceptable |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Essential — definitions were inconsistent |
| Scope additions | 0 | — |
| Deferred | 0 | — |

**Total impact:** Essential fix, no scope creep

### Auto-fixed Issues

**1. Edge-case definition inconsistent with jm3lib definition**
- **Found during:** Task 1b
- **Issue:** EC-24.16 used raw properties (Gap, Ordered, Depth.Max) while jm3lib used ## schema composition (##Deep, ##Sparse, ##Heterogeneous). Both were also missing properties.
- **Fix:** Aligned EC-24.16 to use same ## composition syntax as jm3lib, plus all 3 new properties
- **Verification:** Both definitions now identical

## Issues Encountered

None

## Next Phase Readiness

**Ready:**
- All #Serial documentation is consistent and accurate
- Branch ready for merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 134-serial-schema-free-contradiction, Plan: 01*
*Completed: 2026-04-05*
