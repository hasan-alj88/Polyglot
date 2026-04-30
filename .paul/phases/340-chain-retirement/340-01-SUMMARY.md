---
phase: 340-chain-retirement
plan: 01
subsystem: language-spec
tags: [ebnf, chain-operator, compile-rules, syntax-retirement]

requires:
  - phase: brainstorming-session-2026-04-21-002
    provides: decision to retire -> chain operator
provides:
  - EBNF §10.3 chain grammar removed
  - chains.md rewritten as retirement notice
  - chain-io.md retired as stub
  - 6 chain-specific compile rules retired
  - COMPILE-RULES.md index updated
  - glossary chain entry updated
affects: [340-02 propagation, issue-345 wildcard auto-wire]

key-files:
  modified:
    - docs/technical/ebnf/10-execution.md
    - docs/user/concepts/pipelines/chains.md
    - docs/user/syntax/io/chain-io.md
    - docs/technical/compile-rules/PGE/PGE07002-chain-error-scoping.md
    - docs/technical/compile-rules/PGE/PGE08004-ambiguous-step-reference.md
    - docs/technical/compile-rules/PGE/PGE08005-unresolved-step-reference.md
    - docs/technical/compile-rules/PGE/PGE08006-non-pipeline-step-in-chain.md
    - docs/technical/compile-rules/PGE/PGE08012-self-chain-requires-indexing.md
    - docs/technical/compile-rules/PGE/PGE10007-chain-step-label-overflow.md
    - docs/technical/COMPILE-RULES.md
    - docs/audit/reference/glossary.md

key-decisions:
  - "Retire -> chain operator entirely rather than refine multi-line syntax"
  - "Labeled [-] calls with (-) $Label and $Label>output addressing replace chains"

duration: 10min
completed: 2026-04-22
---

# Issue #340 Plan 01: Chain Operator Core Retirement Summary

**Retired the `->` chain operator from Polyglot — removed EBNF grammar, rewrote user docs as retirement notice with before/after examples, retired 6 chain-specific compile rules.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Completed | 2026-04-22 |
| Tasks | 3 completed |
| Files modified | 11 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF no chain productions | Pass | chain_call removed from exec_expr; §10.3 deleted; §10.4→§10.3 renumbered |
| AC-2: chains.md retirement notice | Pass | Before/after example, replacement table, retirement rationale |
| AC-3: chain-io.md retired stub | Pass | Redirect to operation-labels.md |
| AC-4: 6 compile rules retired | Pass | PGE07002, PGE08004, PGE08005, PGE08006, PGE08012, PGE10007 all status: retired |
| AC-5: Glossary chain entry updated | Pass | "Chain Step Label (Retired)" with redirect |

## Accomplishments

- Removed `chain_call`, `chain_label_block`, `step_ref`, `chain_io_param`, `chain_io_line`, `chain_error_block` EBNF productions
- Rewrote chains.md with concrete before/after showing labeled `[-]` calls replacing `->` chains
- Retired 6 compile rules to stubs with redirect to chains.md
- COMPILE-RULES.md index shows 6 "(Retired)" entries
- Glossary entry updated

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/ebnf/10-execution.md | Modified | Removed §10.3 chain grammar; updated exec_expr; renumbered §10.4→§10.3 |
| docs/user/concepts/pipelines/chains.md | Rewritten | Retirement notice with before/after example |
| docs/user/syntax/io/chain-io.md | Rewritten | Retired stub redirecting to operation-labels |
| docs/technical/compile-rules/PGE/PGE07002-*.md | Rewritten | Retired stub |
| docs/technical/compile-rules/PGE/PGE08004-*.md | Rewritten | Retired stub |
| docs/technical/compile-rules/PGE/PGE08005-*.md | Rewritten | Retired stub |
| docs/technical/compile-rules/PGE/PGE08006-*.md | Rewritten | Retired stub |
| docs/technical/compile-rules/PGE/PGE08012-*.md | Rewritten | Retired stub |
| docs/technical/compile-rules/PGE/PGE10007-*.md | Rewritten | Retired stub |
| docs/technical/COMPILE-RULES.md | Modified | 6 rules marked "(Retired)" |
| docs/audit/reference/glossary.md | Modified | Chain Step Label marked retired |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- Core chain grammar and docs retired
- Replacement pattern documented with before/after

**Deferred to plan 340-02 (propagation):**
- ~80 files across docs/ still reference chains in passing
- Edge cases (10-execution, 06-operators, 11-control-flow, 23-stress-tests, 24-datatype-defs) need chain examples removed
- 08-expressions.md EBNF has minor chain_call reference
- operation-labels.md may need "replaces chains" note
- Various scenarios, aj3lib docs, and technical docs mention chains

**Blockers:** None

---
*Phase: 340-chain-retirement, Plan: 01*
*Completed: 2026-04-22*
