---
phase: issue-272-parameterized-schemas
plan: 04
subsystem: types
tags: [macro-removal, blocks, metadata, concepts, syntax]

requires:
  - phase: 272-01
    provides: "%## / %### properties, #FlexKind, #ActiveKind, #Bound"
  - phase: 272-02
    provides: "All ## schema files (parameterized and static)"
  - phase: 272-03
    provides: "All collection types as generic # types, scalars compose ##String"
provides:
  - "{M} macro block type fully removed from user docs"
  - "[M] invocation marker removed from all block element tables"
  - "%M metadata branch removed"
  - "macros.md and macro-types.md retired as redirect stubs"
  - "Cross-references updated across concept, syntax, and pipeline docs"
affects: [272-05 technical docs — EBNF, compile rules, metadata tree]

key-files:
  modified:
    - docs/user/syntax/blocks.md
    - docs/user/syntax/packages.md
    - docs/user/syntax/types/macro-types.md
    - docs/user/syntax/types/INDEX.md
    - docs/user/syntax/types/prefix-system.md
    - docs/user/syntax/types/arrays.md
    - docs/user/syntax/types/basic-types.md
    - docs/user/syntax/types/hierarchy.md
    - docs/user/syntax/types/schema-properties.md
    - docs/user/concepts/macros.md
    - docs/user/concepts/metadata.md
    - docs/user/concepts/data-is-trees.md
    - docs/user/concepts/permissions.md
    - docs/user/concepts/collections/INDEX.md
    - docs/user/concepts/collections/array.md
    - docs/user/concepts/collections/dataframe.md
    - docs/user/concepts/collections/map.md
    - docs/user/concepts/pipelines/INDEX.md
    - docs/user/concepts/pipelines/wrappers.md
    - docs/user/concepts/pipelines/permissions.md
    - docs/user/jm3lib/pipelines/Schema/INDEX.md

key-decisions:
  - "macro-types.md and macros.md kept as redirect stubs (not deleted) to guide readers"
  - "{M} mentioned only in redirect stubs to explain what replaced it"

patterns-established:
  - "Retired feature → redirect stub pattern (frontmatter + one-line redirect)"

duration: ~45min
completed: 2026-04-09
---

# Plan 272-04: Syntax, Concept, and Block System Summary

**Removed {M} macros from all user-facing syntax, concept, and block system docs — 21 files updated across 6 directories.**

## Performance

| Metric | Value |
|--------|-------|
| Completed | 2026-04-09 |
| Files modified | 21 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: `{M}` removed from docs/user/ active content | Pass | Only appears in redirect stubs (macros.md, macro-types.md) explaining retirement |
| AC-2: `[M]` removed from docs/user/ | Pass | Zero matches |
| AC-3: `%M` branch removed from all tree diagrams | Pass | Zero matches |
| AC-4: blocks.md has no {M} or [M] entries | Pass | Definition elements and block elements tables cleaned |

## Accomplishments

- Retired `{M}` macro block type, `[M]` invocation marker, and `%M` metadata branch from all user docs
- Converted `macro-types.md` and `macros.md` to redirect stubs pointing to generic types / parameterized schemas
- Updated blocks.md definition elements table (removed {M}), block elements table (removed [M]), metadata tree (removed %M)
- Cleaned cross-references in 15+ concept/pipeline/syntax files that mentioned macros

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/blocks.md` | Modified | Removed {M}, [M], %M entries |
| `docs/user/syntax/packages.md` | Modified | Removed {M} references |
| `docs/user/syntax/types/macro-types.md` | Rewritten | Redirect stub to generic types |
| `docs/user/syntax/types/INDEX.md` | Modified | Updated ground truths (removed GT-9 macro reference) |
| `docs/user/syntax/types/prefix-system.md` | Modified | Removed {M} context from [#] roles |
| `docs/user/syntax/types/arrays.md` | Modified | Updated syntax examples |
| `docs/user/syntax/types/basic-types.md` | Modified | Rewrote scalar section without {M} |
| `docs/user/syntax/types/hierarchy.md` | Modified | Rewrote type tree without {M} layer |
| `docs/user/syntax/types/schema-properties.md` | Modified | Further property refinements |
| `docs/user/concepts/macros.md` | Rewritten | Redirect stub |
| `docs/user/concepts/metadata.md` | Modified | Removed %M branch and macro live fields |
| `docs/user/concepts/data-is-trees.md` | Modified | Removed M Macros from tree |
| `docs/user/concepts/permissions.md` | Modified | Removed {M} references |
| `docs/user/concepts/collections/INDEX.md` | Modified | Updated descriptions |
| `docs/user/concepts/collections/array.md` | Modified | Removed {M} references |
| `docs/user/concepts/collections/dataframe.md` | Modified | Removed {M} references |
| `docs/user/concepts/collections/map.md` | Modified | Removed {M} references |
| `docs/user/concepts/pipelines/INDEX.md` | Modified | Removed {M} references |
| `docs/user/concepts/pipelines/wrappers.md` | Modified | Removed {M} references |
| `docs/user/concepts/pipelines/permissions.md` | Modified | Removed {M} references |
| `docs/user/jm3lib/pipelines/Schema/INDEX.md` | Modified | Updated <# references |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Keep macro-types.md / macros.md as redirect stubs | Readers who remember {M} macros need a landing page explaining what replaced them | Stubs contain one mention of {M} in context of explaining retirement |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 0 | — |
| Scope additions | 0 | — |
| Deferred | 0 | — |

**Total impact:** Plan executed as specified.

## Skill Audit

No required skills configured — audit clean.

## Next Phase Readiness

**Ready:**
- All user-facing docs are {M}-free — Plan 272-05 (technical docs) can proceed
- EBNF, compile rules, metadata tree, and edge cases still reference {M} and retired schemas

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-272-parameterized-schemas, Plan: 04*
*Completed: 2026-04-09*
