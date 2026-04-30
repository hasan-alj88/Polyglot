---
phase: 273-three-bracket-symbol-redesign
plan: 01
subsystem: spec
tags: [ebnf, grammar, three-bracket, syntax-redesign]

requires:
  - phase: none
    provides: existing EBNF grammar
provides:
  - Authoritative EBNF grammar rewritten with three-bracket system
  - New IO bracket category (X) for data binding
  - Pipeline prefix - replaces =
  - Expander prefix = replaces ~
affects: [273-02 core docs, 273-03 compile rules, 273-04 aj3lib docs, 273-05 source files]

tech-stack:
  added: []
  patterns: [three-bracket system {X} define / [X] control / (X) IO]

key-files:
  created: []
  modified:
    - docs/technical/ebnf/03-identifiers.md
    - docs/technical/ebnf/05-block-elements.md
    - docs/technical/ebnf/06-operators.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/technical/ebnf/10-execution.md
    - docs/technical/ebnf/11-control-flow.md
    - docs/technical/ebnf/12-collections.md
    - docs/technical/ebnf/15-example.md
    - docs/technical/ebnf/INDEX.md

key-decisions:
  - "[-] reclaimed from logical_elem for sequential execution (negation uses ! in comparison ops)"
  - "[+] added to control_flow_elem as OR trigger scope (distinct from [|] logical OR)"
  - "New io_bracket production added to §5 for round-bracket (X) markers"

patterns-established:
  - "Three-bracket principle: {X} define, [X] control, (X) IO"
  - "IO bracket matches parent operator prefix: (-) for pipelines, (=) for expanders, (*) for collectors"

duration: 15min
started: 2026-04-09
completed: 2026-04-09
---

# Plan 273-01: EBNF Grammar Rewrite Summary

**All 9 EBNF grammar files rewritten with three-bracket system — `{X}` define, `[X]` control, `(X)` IO — plus pipeline prefix `=` → `-` and expander prefix `~` → `=`.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Tasks | 3 completed |
| Files modified | 9 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Three-bracket taxonomy | Pass | New io_bracket category with `(-)`, `(=)`, `(*)`, `(>)`, `(<)` |
| AC-2: Pipeline prefix = → - | Pass | pipeline_id, cross_pkg_pipeline, all refs updated |
| AC-3: Execution markers | Pass | `[-]` sequential, `[=]` parallel, `->` chains, `(-)` IO |
| AC-4: Collection markers | Pass | `=ForEach.*`, `(=)` IO, `(*)` collect, `.=` level suffix |
| AC-5: Control flow updated | Pass | `[-]` match, `[~]` continuation, `[+]` OR trigger |
| AC-6: Complete example | Pass | 15-example.md fully rewritten |

## Accomplishments

- Restructured block element taxonomy: removed `data_flow_elem`, added `io_bracket` production, reclaimed `[-]` from `logical_elem`
- Rewrote all pipeline definition grammar (`{-}`, marker_decl `[-]`/`[=]`/`[-=]`/`[-b]`/`[=b]`)
- Rewrote all execution grammar (sequential `[-]`, parallel `[=]`, chain `->`)
- Rewrote all collection grammar (expander `=ForEach.*` with `(=)` IO, collector `(*)` IO)
- Updated control flow: match `[-]`, continuation `[~]`, error raise `(-)` / `(>)`

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/ebnf/03-identifiers.md` | Modified | pipeline_id `'='` → `'-'`, cross_pkg_pipeline comment |
| `docs/technical/ebnf/05-block-elements.md` | Rewritten | Full taxonomy restructure + new §5.2 IO Brackets |
| `docs/technical/ebnf/06-operators.md` | Modified | Fallback reference `(>)`/`(<)` |
| `docs/technical/ebnf/09-definition-blocks.md` | Rewritten | `{-}` pipelines, `(-)` IO, all pipeline refs `-` prefix |
| `docs/technical/ebnf/10-execution.md` | Rewritten | `[-]`/`[=]`/`[b]` execution, `->` chains, `(-)` call IO |
| `docs/technical/ebnf/11-control-flow.md` | Rewritten | `[-]` match, `[~]` continuation, `(-)` error raise |
| `docs/technical/ebnf/12-collections.md` | Rewritten | `=ForEach.*`, `(=)` expand IO, `(*)` collect IO |
| `docs/technical/ebnf/15-example.md` | Rewritten | Complete example with all new markers |
| `docs/technical/ebnf/INDEX.md` | Modified | `{-}` in §9 table entry |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Reclaim `[-]` from logical_elem | EBNF already noted "[-] NOT is not needed" — negation uses `!` in comparison ops | Clean reuse, no functionality lost |
| Add `[+]` to control_flow_elem | Replaces `[⏐]`/`[|]` for trigger OR scope; `[|]` stays as logical OR | Clear separation: `[+]` = trigger OR, `[|]` = conditional OR |
| New §5.2 for IO brackets | Round brackets `(X)` are a new syntactic category distinct from block elements | Three-bracket principle fully encoded in grammar |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- Authoritative EBNF grammar establishes all new syntax
- Subsequent plans can use EBNF as reference for bulk replacements

**Concerns:**
- ~8,800 remaining occurrences across 250+ non-EBNF files still use old syntax
- Replacement ordering matters due to symbol reuse (e.g., `[=]` old IO vs `[=]` new parallel)

**Blockers:**
- None

---
*Phase: 273-three-bracket-symbol-redesign, Plan: 01*
*Completed: 2026-04-09*
