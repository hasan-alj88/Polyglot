---
phase: 273-three-bracket-symbol-redesign
plan: 02
subsystem: spec
tags: [three-bracket, syntax-migration, user-docs, concept-docs]

requires:
  - phase: 273-01
    provides: authoritative EBNF grammar with three-bracket system
provides:
  - All user-facing syntax docs updated with three-bracket syntax
  - All concept docs (pipelines, collections, remaining) updated
affects: [273-03 compile rules, 273-04 jm3lib, 273-05 source/verification]

tech-stack:
  added: []
  patterns: [three-bracket system {X} define / [X] control / (X) IO]

key-files:
  created: []
  modified:
    - docs/user/syntax/blocks.md
    - docs/user/syntax/io.md
    - docs/user/syntax/identifiers.md
    - docs/user/syntax/operators.md
    - docs/user/syntax/packages.md
    - docs/user/syntax/comments.md
    - docs/user/syntax/types/arrays.md
    - docs/user/syntax/types/conversions.md
    - docs/user/syntax/types/generic-types.md
    - docs/user/syntax/types/hierarchy.md
    - docs/user/syntax/types/INDEX.md
    - docs/user/syntax/types/strings.md
    - docs/user/syntax/types/structs.md
    - docs/user/concepts/pipelines/INDEX.md
    - docs/user/concepts/pipelines/chains.md
    - docs/user/concepts/pipelines/execution.md
    - docs/user/concepts/pipelines/error-handling.md
    - docs/user/concepts/pipelines/inline-calls.md
    - docs/user/concepts/pipelines/io-triggers.md
    - docs/user/concepts/pipelines/metadata.md
    - docs/user/concepts/pipelines/permissions.md
    - docs/user/concepts/pipelines/queue.md
    - docs/user/concepts/pipelines/wrappers.md
    - docs/user/concepts/collections/INDEX.md
    - docs/user/concepts/collections/array.md
    - docs/user/concepts/collections/collect.md
    - docs/user/concepts/collections/dataframe.md
    - docs/user/concepts/collections/examples.md
    - docs/user/concepts/collections/expand.md
    - docs/user/concepts/collections/map.md
    - docs/user/concepts/collections/serial.md
    - docs/user/concepts/collections/user-struct.md
    - docs/user/concepts/conditionals.md
    - docs/user/concepts/data-is-trees.md
    - docs/user/concepts/errors.md
    - docs/user/concepts/metadata.md
    - docs/user/concepts/permissions.md
    - docs/user/concepts/variable-lifecycle.md

key-decisions:
  - "No deviations — all 15 replacement rules applied in strict order"
  - "Context-sensitive replacements handled per-file (pipeline prefix, chain operator, trigger OR)"

patterns-established:
  - "Replacement ordering protocol for symbol-reuse migrations"
  - "Three parallel agents for independent file groups"

duration: ~15min
started: 2026-04-09
completed: 2026-04-09
---

# Plan 273-02: Core Syntax + Concept Docs Summary

**38 user-facing documentation files updated with three-bracket syntax — `{X}` define, `[X]` control, `(X)` IO — plus pipeline prefix `-`, expander prefix `=`, and all marker renames.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Tasks | 3 completed |
| Files modified | 38 |
| Insertions/Deletions | +1091 / -1279 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: No stale markers in syntax docs | Pass | Zero matches for `[r]`, `[p]`, `{=}`, `~ForEach` in docs/user/syntax/ |
| AC-2: No stale markers in concept docs | Pass | Zero matches for same in docs/user/concepts/ |
| AC-3: New syntax present and consistent | Pass | `{-}` (71), `(-)` (242), `[-]` (215), `[=]` (382), `=ForEach` (26), `(*)` (75) |

## Accomplishments

- Updated 13 syntax reference docs (blocks, io, identifiers, operators, packages, comments, 7 type files)
- Updated 10 pipeline concept docs (chains, execution, error-handling, inline-calls, io-triggers, metadata, permissions, queue, wrappers, INDEX)
- Updated 15 collection + remaining concept docs (8 collection files, conditionals, data-is-trees, errors, metadata, permissions, variable-lifecycle)
- All 15 replacement rules applied in strict order to avoid symbol-reuse collisions

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/blocks.md` | Modified | All 15 replacement rules applied |
| `docs/user/syntax/io.md` | Modified | IO markers `(-)`, `(=)`, `(*)`, `(>)`, `(<)` |
| `docs/user/syntax/identifiers.md` | Modified | Pipeline prefix `=` → `-` |
| `docs/user/syntax/operators.md` | Modified | Chain `=>` → `->`, pipeline refs |
| `docs/user/syntax/packages.md` | Modified | `{-}` pipelines, `@alias-Pipeline` |
| `docs/user/syntax/comments.md` | Modified | `(-)` IO, `[-]` sequential |
| `docs/user/syntax/types/arrays.md` | Modified | `(-)` IO, `[-]` sequential |
| `docs/user/syntax/types/conversions.md` | Modified | `[-]`, `(>)` |
| `docs/user/syntax/types/generic-types.md` | Modified | `{-}`, `(-)`, `(<)`, pipeline refs |
| `docs/user/syntax/types/hierarchy.md` | Modified | `{-}`, `[-]` |
| `docs/user/syntax/types/INDEX.md` | Modified | `[-]`, pipeline refs |
| `docs/user/syntax/types/strings.md` | Modified | `[-]`, pipeline refs |
| `docs/user/syntax/types/structs.md` | Modified | `[-]`, `(-)` |
| `docs/user/concepts/pipelines/INDEX.md` | Modified | Full marker table rewrite, all examples |
| `docs/user/concepts/pipelines/chains.md` | Modified | Chain `=>` → `->`, pipeline chains |
| `docs/user/concepts/pipelines/execution.md` | Modified | `[-]`/`[=]`/`[b]` execution model |
| `docs/user/concepts/pipelines/error-handling.md` | Modified | `[-]`, `(-)`, `(>)` |
| `docs/user/concepts/pipelines/inline-calls.md` | Modified | `-Pipeline"string"` syntax |
| `docs/user/concepts/pipelines/io-triggers.md` | Modified | `-T.*` triggers, `(-)` IO |
| `docs/user/concepts/pipelines/metadata.md` | Modified | `%-:{name}` metadata |
| `docs/user/concepts/pipelines/permissions.md` | Modified | `{-}`, `(-)`, pipeline refs |
| `docs/user/concepts/pipelines/queue.md` | Modified | `-Q.*` queues, `(-)`, `(*)` |
| `docs/user/concepts/pipelines/wrappers.md` | Modified | `[=]` parallel, `(*)`, `-W.*` |
| `docs/user/concepts/collections/INDEX.md` | Modified | Prose references only |
| `docs/user/concepts/collections/array.md` | Modified | `[-]`, `=ForEach.Array` |
| `docs/user/concepts/collections/collect.md` | Modified | `(*)`, `[-]`, `[=]`, `(-)` |
| `docs/user/concepts/collections/dataframe.md` | Modified | `[-]`, `-#.Column` |
| `docs/user/concepts/collections/examples.md` | Modified | Full expand/collect examples |
| `docs/user/concepts/collections/expand.md` | Modified | `=ForEach.*`, `(=)`, `.=` suffix |
| `docs/user/concepts/collections/map.md` | Modified | `[-]`, `=ForEach.Map` |
| `docs/user/concepts/collections/serial.md` | Modified | `[-]`, `=ForEach.*` |
| `docs/user/concepts/collections/user-struct.md` | Modified | `[-]` |
| `docs/user/concepts/conditionals.md` | Modified | `[-]` match, `-DoNothing` |
| `docs/user/concepts/data-is-trees.md` | Modified | `%-`, `{-}`, pipeline tree |
| `docs/user/concepts/errors.md` | Modified | `{-}`, `(-)`, `[-]`, `(>)`, `->` chains |
| `docs/user/concepts/metadata.md` | Modified | `%-:`, `{-}`, pipeline refs |
| `docs/user/concepts/permissions.md` | Modified | `{-}`, `(-)`, pipeline refs |
| `docs/user/concepts/variable-lifecycle.md` | Modified | `{-}`, `(-)`, `[-]` |

## Decisions Made

None — followed plan as specified.

## Deviations from Plan

None — plan executed exactly as written. All 15 replacement rules applied in strict order across all 38 files.

## Next Phase Readiness

**Ready:**
- All user-facing docs consistent with EBNF grammar from 273-01
- Replacement ordering protocol proven for remaining plans

**Concerns:**
- jm3lib docs still use old syntax (~99 files with old `[=]` marker — plan 273-04 scope)
- Compile rules still reference old markers (plan 273-03 scope)

**Blockers:**
- None

---
*Phase: 273-three-bracket-symbol-redesign, Plan: 02*
*Completed: 2026-04-09*
