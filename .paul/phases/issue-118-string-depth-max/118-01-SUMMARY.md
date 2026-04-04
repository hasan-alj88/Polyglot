---
phase: issue-118-string-depth-max
plan: 01
subsystem: docs
tags: [schema, depth, scalar, leaf, type-system]

requires:
  - phase: none
    provides: n/a
provides:
  - "##Leaf schema (Depth.Max=0) for truly atomic types"
  - "##Scalar redefined to Depth.Max=1 for #String:* and scalar enums"
  - "###ScalarValue and ###ScalarEnum leaf types"
affects: [plan-118-02-propagation]

tech-stack:
  added: []
  patterns:
    - "##Scalar constrains to ###ScalarValue OR ###ScalarEnum"
    - "Fixed fields count toward depth (Depth.Max=1 for records)"

key-files:
  created: []
  modified:
    - docs/user/syntax/types/schema-properties.md
    - docs/technical/spec/metadata-tree/definition-templates.md
    - docs/user/syntax/types/basic-types.md
    - docs/technical/spec/metadata-tree/string-subtypes.md

key-decisions:
  - "##Scalar is Depth.Max=1, not 0 — fixed fields count toward depth"
  - "##Leaf (new) is Depth.Max=0 — reserved for truly atomic types like RawString"
  - "##Scalar specifically for #String:* family and scalar enums, not general-purpose"
  - "###ScalarValue for regex-validated string data; ###ScalarEnum for variant selectors"

patterns-established:
  - "Three-tier depth model: ##Leaf (0), ##Scalar (1), ##Flat (1), ##Deep (-1)"

duration: 10min
started: 2026-04-04
completed: 2026-04-04
---

# Issue #118 Plan 01: Core Schema Redefinition Summary

**Redefined ##Scalar to Depth.Max=1, introduced ##Leaf (Depth.Max=0), and added ###ScalarValue/###ScalarEnum leaf types in 4 core reference files.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 4 completed |
| Files modified | 4 |
| Deviations | 0 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: ##Leaf schema exists with Depth.Max=0 | Pass | In schema-properties.md and definition-templates.md |
| AC-2: ##Scalar redefined to Depth.Max=1 | Pass | Constrained to ###ScalarValue or ###ScalarEnum |
| AC-3: ###ScalarValue and ###ScalarEnum defined | Pass | Added to ### field types table |
| AC-4: #String and #Boolean definitions updated | Pass | #String uses ###ScalarValue, #Boolean uses ###ScalarEnum |
| AC-5: string-subtypes.md contradiction resolved | Pass | Depth consistency note added |
| AC-6: Depth.Max inference rules updated | Pass | Fixed fields → Depth.Max=1, no fields → Depth.Max=0 |

## Accomplishments

- Resolved the core #118 contradiction: ##Scalar now Depth.Max=1, accommodating #String's subtypes
- Introduced ##Leaf (Depth.Max=0) for truly atomic types
- Added ###ScalarValue/###ScalarEnum to constrain ##Scalar to #String:* and scalar enums

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/syntax/types/schema-properties.md | Modified | Core schema + leaf type definitions, inference rules |
| docs/technical/spec/metadata-tree/definition-templates.md | Modified | Schema registry tree, #Boolean template |
| docs/user/syntax/types/basic-types.md | Modified | #String + #Boolean definitions |
| docs/technical/spec/metadata-tree/string-subtypes.md | Modified | Depth consistency note |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- Core definitions established — plan 118-02 can propagate to remaining ~20 files

**Concerns:**
- ~20 files still reference old ###Enum/###Value for scalar types and Depth.Max=0

**Blockers:**
- None

---
*Phase: issue-118-string-depth-max, Plan: 01*
*Completed: 2026-04-04*
