---
phase: 309-ebnf-token-ambiguity-cross-production-io-validation
plan: 01
completed: 2026-04-17
duration: ~10min
key-decisions:
  - "X.45 Accept: lexer disambiguates ?[ vs [?] positionally"
  - "X.46 Accept: schema-typed <Collection.* inputs + PGE04001 cover reassemble IO"
key-files:
  modified:
    - docs/technical/ebnf/06-operators.md
    - docs/technical/ebnf/12-collections.md
    - docs/technical/edge-cases/06-operators.md
    - docs/technical/edge-cases/12-collections.md
    - docs/technical/edge-cases/INDEX.md
---

# Issue #309 Plan 01: Token Ambiguity + Reassemble IO Validation

**Lexer disambiguation rule for `?[` vs `[?]` documented; reassemble IO compatibility confirmed as schema-enforced via existing type system.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Lexer disambiguation rule documented | Pass | Added to EBNF §6.3 |
| AC-2: EC-6.6 edge case for X.45 | Pass | Accept decision with 3 examples |
| AC-3: Reassemble IO schema enforcement note | Pass | Added to EBNF §12.3 |
| AC-4: EC-12.18 edge case for X.46 | Pass | Accept decision with compatible/incompatible examples |
| AC-5: INDEX.md updated | Pass | S6→EC-6.1--6.6, S12→EC-12.1--12.18 |

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/ebnf/06-operators.md | Modified | Lexer Disambiguation note in §6.3 |
| docs/technical/ebnf/12-collections.md | Modified | Schema Enforcement note in §12.3 |
| docs/technical/edge-cases/06-operators.md | Modified | EC-6.6 (X.45 token ambiguity) |
| docs/technical/edge-cases/12-collections.md | Modified | EC-12.18 (X.46 reassemble IO) |
| docs/technical/edge-cases/INDEX.md | Modified | Updated ranges + coverage matrix |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| X.45 Accept — positional lexer disambiguation | `[?]` at line start, `?[` mid-expression, `?` consumed greedily by comparison ops | No grammar change; documented in §6.3 |
| X.46 Accept — schema-enforced IO validation | `<Collection.Array` typed to `##Array`; desugar + PGE04001 catches mismatch | No new compile rule; IO signature table is documentation of existing enforcement |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:** Issue #309 complete, ready for `/paul:merge`

---
*Phase: 309, Plan: 01*
*Completed: 2026-04-17*
