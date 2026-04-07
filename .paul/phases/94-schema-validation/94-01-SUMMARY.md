---
phase: 94-schema-validation
plan: 01
subsystem: spec
tags: [type-system, macros, generics, dataframe, scalars, ground-truths]

provides:
  - Ground truths preamble (GT-1 through GT-9)
  - "{M}/{W} split — type macros vs wrappers as separate entities"
  - "[M] macro invocation pattern"
  - "Macro-generated collection types (#Map, #Array, #Dataframe)"
  - "Scalars as ## schemas via {M} #String.Subtype"
  - "Row-oriented Dataframe (Array of Map)"
  - "!Alias.Clash error type"
affects: [94-02/04 validation pipelines, 94-03 expand/collect audit]

key-files:
  modified:
    - docs/user/syntax/types.md
    - docs/user/syntax/blocks.md
    - docs/user/pglib/types/collections.md
    - docs/user/pglib/types/scalars.md
    - docs/user/concepts/pipelines.md
    - docs/technical/EBNF.md
    - docs/technical/EDGE-CASES.md
    - docs/technical/plan/decisions/schema-properties.md
    - docs/user/pglib/errors/errors.md
    - docs/user/concepts/collections.md
    - docs/user/pglib/types/types.md
    - docs/user/pglib/expanders/ForEach/Dataframe/Column.md

key-decisions:
  - "Dataframe uses ## composition, not <~ inheritance from Array"
  - "[M] merge = identity — outer {#} names result, macro fills body"
  - "Column extraction via =#.Column pipeline (not expander)"
  - "~ForEach.Dataframe.Column deprecated"

completed: 2026-03-29
---

# Plan 94-01: Ground Truths + Macro-for-Generics Redesign — Summary

**Replaced generic `<param` type parameters with `{M}` type macros across 12 spec files; added 9 ground truth axioms; converted Dataframe to row-oriented; reframed scalars as `##` schemas.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Ground Truths Preamble | Pass | GT-1 through GT-9 in types.md |
| AC-2: Generic Syntax Removed | Pass | Zero `{#} #Type<Param` patterns in primary files |
| AC-3: {M}/{W} Split Documented | Pass | blocks.md has {W} row and [M] row; pipelines.md references {W} |
| AC-4: Collection Types as Macros | Pass | #Map, #Array, #Dataframe as {M} macros; #Serial unchanged |
| AC-5: Scalars as ## Schemas | Pass | All scalars as {#} ##Name via [M]; ##DataTypeString + ##CommaSeparatedList added |
| AC-6: EBNF Updated | Pass | type_param removed; {M}/{W}/[M] productions added |
| AC-7: Cross-References Consistent | Pass | EC-24.11 rewritten; schema-properties updated; !Alias.Clash added |

## Deviations from Plan

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 3 | Essential — stale references in files outside original 9 |
| Deferred | 1 | COMPILE-RULES.md still has 4 old generic references |

### Scope Additions
1. **docs/user/concepts/collections.md** — Had `{#} #Map<KeyType<ValueType` and column-oriented Dataframe examples
2. **docs/user/pglib/types/types.md** — Had `#Map<KeyType<ValueType` in type hierarchy listing
3. **docs/user/pglib/expanders/ForEach/Dataframe/Column.md** — Deprecated (row-oriented Dataframe breaks column expander)

### Deferred
- COMPILE-RULES.md lines 242, 249, 255, 382 still use `#Array<#Array<#float` syntax — update on next touch

## Next Phase Readiness

**Ready:**
- Type system foundation complete for `=#.*` validation pipelines (Plan 94-02/04)
- `<#type` pipeline IO pattern documented in types.md
- !Validation.*, !Field.* error namespaces can be added

**Concerns:**
- COMPILE-RULES.md has 4 stale generic references (deferred)

**Blockers:** None

---
*Phase: 94-schema-validation, Plan: 01*
*Completed: 2026-03-29*
