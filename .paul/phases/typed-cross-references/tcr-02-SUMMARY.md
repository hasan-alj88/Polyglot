---
phase: typed-cross-references
plan: 02
status: complete
completed: 2026-04-10
---

# Summary: Classify Existing Refs + Coverage Gaps

## What Was Done

Analyzed all 409 `@` references across docs/ and produced a classification table with coverage gap inventory.

### Files Created
- `docs/audit/tracking/ref-classification.md` — 101 unique targets classified, 13 coverage gap constructs identified

## Classification Results
- @c: (concept): 52 targets
- @u: (usage): 43 targets
- Untyped (INDEX/navigation): 20 targets
- Ambiguous resolved: 2 (`@blocks` → @c:, `@Q` → @c:)

## Coverage Gap Findings
- compile-rules/: 161 of 165 files had zero cross-references
- Largest gaps: [Q] (~71), [W] (~67), [=] (~47), [?] (~36)

## Decisions
- `@blocks` classified as @c: (block definitions are concepts, not syntax usage)
- `@source:*` datetime internal refs stay untyped (sub-document navigation)
- User approved classification before migration

## Deviations
None.
