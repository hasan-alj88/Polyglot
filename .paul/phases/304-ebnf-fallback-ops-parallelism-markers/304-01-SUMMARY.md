---
phase: 304-ebnf-fallback-ops-parallelism-markers
plan: 01
subsystem: compiler
tags: [ebnf, compile-rules, fallback, parallelism, assignment-operators, collectors]

requires:
  - phase: none
    provides: existing EBNF grammar and compile rule infrastructure
provides:
  - PGE07008 — fallback on non-failable source
  - PGE07009 — unterminated fallback chain
  - PGE01040 — orphan parallel marker
affects: [edge-cases, compile-rules, collection-operators]

tech-stack:
  added: []
  patterns: [source-expression-based validation, forward-sibling pairing]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE07008-fallback-on-non-failable-source.md
    - docs/technical/compile-rules/PGE/PGE07009-unterminated-fallback-chain.md
    - docs/technical/compile-rules/PGE/PGE01040-orphan-parallel-marker.md
  modified:
    - docs/technical/COMPILE-RULES.md
    - docs/technical/ebnf/06-operators.md
    - docs/technical/ebnf/12-collections.md
    - docs/technical/edge-cases/06-operators.md
    - docs/technical/edge-cases/12-collections.md
    - docs/user/syntax/operators.md
    - docs/user/concepts/collections/collect.md

key-decisions:
  - "Fallback validity is source-expression-based, not production-based — no EBNF grammar split needed"
  - "[=]/[b] must pair with next [=]/[b] sibling — forward-only check, inherently sequential markers exempt"

patterns-established:
  - "Semantic compiler rule over grammar restriction — grammar stays permissive, compiler enforces context"

duration: ~15min
started: 2026-04-16
completed: 2026-04-16
---

# Issue #304 Plan 01: Fallback Operators + Parallelism Markers Summary

**3 new compile rules (PGE07008, PGE07009, PGE01040) tightening fallback operator validity and parallel marker pairing — semantic enforcement without EBNF grammar changes.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Tasks | 3 completed |
| Files modified | 10 (3 created, 7 edited) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: PGE07008 — Fallback on non-failable source | Pass | Rule doc with examples; EBNF prose; user docs |
| AC-2: PGE07009 — Unterminated fallback chain | Pass | Rule doc with chain termination examples |
| AC-3: PGE01040 — Orphan parallel marker | Pass | Rule doc with pairing table; EBNF prose; user docs |
| AC-4: EBNF prose and edge cases updated | Pass | EC-6.5 (X.33) and EC-12.17 (X.34) added |

## Accomplishments

- Created PGE07008: fallback operators require failable source (pipeline call); literals/variables cannot error
- Created PGE07009: fallback chains must terminate at non-failable value; compiler walks chain to verify
- Created PGE01040: `[=]`/`[b]` must have forward `[=]`/`[b]` sibling; replaces ambiguous "inter-collector ordering" language
- Key design decision: no EBNF grammar split — constraint is semantic (source-expression-based), not syntactic

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/compile-rules/PGE/PGE07008-fallback-on-non-failable-source.md` | Created | Fallback on non-failable source rule |
| `docs/technical/compile-rules/PGE/PGE07009-unterminated-fallback-chain.md` | Created | Unterminated fallback chain rule |
| `docs/technical/compile-rules/PGE/PGE01040-orphan-parallel-marker.md` | Created | Orphan parallel marker rule |
| `docs/technical/COMPILE-RULES.md` | Modified | 3 new index entries |
| `docs/technical/ebnf/06-operators.md` | Modified | Semantic fallback validity rule in §6.1 |
| `docs/technical/ebnf/12-collections.md` | Modified | Parallel marker pairing rule in §12.2 |
| `docs/technical/edge-cases/06-operators.md` | Modified | EC-6.5 (X.33) edge case |
| `docs/technical/edge-cases/12-collections.md` | Modified | EC-12.17 (X.34) edge case |
| `docs/user/syntax/operators.md` | Modified | Fallback validity note |
| `docs/user/concepts/collections/collect.md` | Modified | Parallel marker pairing note |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Source-expression-based validation, not production-based | User correction: inline pipelines can appear in any context, making fallback valid anywhere a pipeline call is the RHS | No EBNF grammar split needed; simpler compiler rule |
| Collector output `(*) >>` keeps fallback | Collection processes can fail (timeouts, all-Failed) | Exception documented in PGE07008 |
| Forward-only sibling pairing for `[=]`/`[b]` | User correction: parallelism means "parallel with next sibling", not "inter-collector ordering" | Cleaner mental model; PGE01040 replaces ambiguous spec language |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- All 3 compile rules documented and indexed
- Edge cases provide valid/invalid examples for each rule
- User docs reference the new rules

**Blockers:** None

---
*Phase: 304-ebnf-fallback-ops-parallelism-markers, Plan: 01*
*Completed: 2026-04-16*
