---
phase: issue-107-object-type-hierarchy
plan: 01
subsystem: docs
tags: [type-hierarchy, triggers, metadata-tree, ebnf]

requires: []
provides:
  - "{T} trigger definition type documented across all spec files"
  - "%T metadata branch with structure and properties"
  - "Subtype relationships annotated on all definition elements"
affects: [issue-108-marker-syntax, issue-109-uppercase-T, issue-111-compiler-rules, issue-114-metadata-T-branch]

key-files:
  modified:
    - docs/user/syntax/blocks.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/user/concepts/pipelines/io-triggers.md
    - docs/technical/spec/metadata-tree.md

key-decisions:
  - "§9.4a placement: between §9.4 (Macro) and §9.4b (Wrapper), following existing naming pattern"
  - "Trigger Definitions section placed before IO as Implicit Triggers in io-triggers.md"

duration: 5min
completed: 2026-03-31
---

# Issue #107 Plan 01: Document Object Type Hierarchy Summary

**Documented two-base-type hierarchy ({#} data, {=} pipeline) with {T} trigger definitions, subtype annotations, EBNF grammar, and %T metadata branch across 4 spec files.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-31 |
| Tasks | 4 completed |
| Files modified | 4 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: {T} row in Definition Elements table | Pass | Row added between {=} and {M} |
| AC-2: Subtype notes on existing rows | Pass | {!}, {M}, {T}, {W}, {Q} all annotated |
| AC-3: Trigger definition grammar in EBNF | Pass | §9.4a with trigger_def production, rules, example |
| AC-4: Trigger Definitions section in io-triggers.md | Pass | Base (=T.Call) and composed (=T.Folder.NewFiles) examples |
| AC-5: %T branch in metadata tree | Pass | type_prefix, table row, Trigger Branch section, definition template |
| AC-6: Tree mapping sentence updated | Pass | {T} at %T added to opening paragraph |

## Accomplishments

- Added `{T}` as a first-class definition type with full EBNF grammar (§9.4a)
- Annotated all definition elements with base type relationships
- Created `%T` metadata branch with structure, IO ports, mandatory `>IsTriggered#bool`, and `live` fields
- Fixed `[{]`/`[}]` descriptions from "in Macros" to "in Wrappers" (stale from {M}→{W} separation)

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/blocks.md` | Modified | {T} row, subtype notes, %T in tree mapping, [{]/[}] fix |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | §9.4a Trigger Definition grammar + rules + example |
| `docs/user/concepts/pipelines/io-triggers.md` | Modified | Trigger Definitions section with {T} explanation and examples |
| `docs/technical/spec/metadata-tree.md` | Modified | %T branch, T in type_prefix, Trigger Branch section, definition template |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| §9.4a between §9.4 and §9.4b | Follows existing naming pattern (§9.4b Wrapper) | Clean section ordering |
| [{]/[}] "Macros" → "Wrappers" fix | Stale text from {M}→{W} separation work | Correctness — minor auto-fix |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Minor text correction |

**Total impact:** Essential fix, no scope creep.

### Auto-fixed Issues

**1. Stale [{]/[}] description**
- **Found during:** Task 1 (blocks.md)
- **Issue:** [{] and [}] still said "in Macros" after {M}→{W} separation
- **Fix:** Changed to "in Wrappers"
- **Planned:** Yes — included in plan action list

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Type hierarchy is now documented — foundational for issues #108, #109, #111, #114
- {T} definition type fully specified in user docs, EBNF, and metadata tree

**Concerns:**
- `[t]` still lowercase everywhere — Issue #109 will do the rename
- No compiler rules yet for trigger constraints — Issue #111's scope

**Blockers:**
- None

---
*Phase: issue-107-object-type-hierarchy, Plan: 01*
*Completed: 2026-03-31*
