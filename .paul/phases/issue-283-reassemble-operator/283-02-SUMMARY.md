---
phase: issue-283-reassemble-operator
plan: 02
subsystem: docs
tags: [cross-references, draft-cleanup, reassemble]
requires:
  - phase: issue-283-reassemble-operator/01
    provides: concept and aj3lib files to link to
provides:
  - Cross-references from expand.md and collect.md to reassemble.md
  - draft.md =* section marked as placed
affects: []
key-files:
  modified:
    - docs/user/concepts/collections/expand.md
    - docs/user/concepts/collections/collect.md
    - docs/draft.md
key-decisions: []
duration: 2min
completed: 2026-04-15
---

# Issue #283 Plan 02: Cross-References + Draft Cleanup Summary

**Wired reassemble cross-references into expand/collect docs and marked draft.md =* brainstorm as placed**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Expand docs link to reassemble | Pass | See Also has reassemble link |
| AC-2: Collect docs link to reassemble | Pass | See Also has reassemble link |
| AC-3: Draft.md =* section marked as placed | Pass | Placed notice with 3 links, original preserved |

## Accomplishments

- expand.md and collect.md See Also sections now link to reassemble.md
- draft.md Reassemble Operator section has placed notice pointing to concept, aj3lib, and EBNF docs
- Original brainstorm content preserved below the notice

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/collections/expand.md` | Modified | Added reassemble link to See Also |
| `docs/user/concepts/collections/collect.md` | Modified | Added reassemble link to See Also |
| `docs/draft.md` | Modified | Added placed notice to =* section |

## Deviations from Plan

None — plan executed exactly as written.

---
*Phase: issue-283-reassemble-operator, Plan: 02*
*Completed: 2026-04-15*
