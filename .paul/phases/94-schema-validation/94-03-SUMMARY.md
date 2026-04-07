---
phase: 94-schema-validation
plan: 03
subsystem: spec
tags: [expand-collect, audit, dataframe, compatibility]

provides:
  - "Expand/collect audit complete — all operators compatible with macro-generated types"
  - "Stale ~ForEach.Dataframe.Column references removed from active files"
affects: []

key-files:
  modified:
    - docs/user/pglib/expanders/ForEach/Dataframe.md
    - docs/technical/EBNF.md

key-decisions:
  - "No new operators needed — existing expand/collect signatures are compatible with macro-generated types"

completed: 2026-03-30
---

# Plan 94-03: Expand/Collect Audit — Summary

**Audited all expand/collect operators for macro-generated type compatibility; removed 2 stale ~ForEach.Dataframe.Column references. No structural issues found.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: No Active References to Deprecated Column Expander | Pass | Only match is in Column.md itself (already deprecated) |
| AC-2: Expand/Collect IO Compatible with Macro Types | Pass | Zero old generic syntax; all IO signatures match macro-generated definitions |

## Deviations from Plan

None — plan executed exactly as written.

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/expanders/ForEach/Dataframe.md` | Modified | Marked Column variant as deprecated, linked to =#.Column |
| `docs/technical/EBNF.md` | Modified | Removed ~ForEach.Dataframe.Column from expander signature table |

## Audit Findings

All expander/collector files verified:
- **~ForEach.Array/Map/Serial/Level/Dataframe** — IO signatures clean, no old generic syntax
- **~ForEach.Dataframe** — `>row` output is Map (consistent with row-oriented design)
- ***Into.Array/Map/Serial/Level/Dataframe** — IO signatures clean
- ***Agg.*** — no collection type dependencies
- **No new operators needed** — existing set covers macro-generated type system

## Next Phase Readiness

**Ready:** All three plans (94-01/02/03) complete. Branch ready for merge.

**Blockers:** None

---
*Phase: 94-schema-validation, Plan: 03*
*Completed: 2026-03-30*
