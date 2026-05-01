---
phase: 10-operators-control-flow
plan: 01
subsystem: docs
tags: [operators, conditionals, exhaustiveness, arithmetic, negation, PGE-6xx]

requires:
  - phase: 09-core-language-type-system
    provides: types.md, blocks.md, identifiers.md at status: complete
provides:
  - operators.md promoted to status: complete (arithmetic, negation, type-operator compatibility)
  - conditionals.md new spec at status: complete (exhaustiveness, logical operators, nesting)
  - SPEC-INDEX.md updated with conditionals.md entry
affects: [11-pipelines-concurrency, 12-package-system-jm3lib]

key-files:
  created:
    - docs/user/concepts/conditionals.md
  modified:
    - docs/user/syntax/operators.md
    - docs/user/SPEC-INDEX.md

key-decisions:
  - "Arithmetic uses =Math.* jm3lib pipelines, not raw operators (PGE04010 confirms)"
  - "conditionals.md placed in concepts/ not syntax/ — conditionals are behavioral, not lexical"

completed: 2026-03-24
---

# Phase 10 Plan 01: Operators & Control Flow Spec Completion

**Completed operators.md (arithmetic via =Math.*, negation detail, type-operator compatibility) and created conditionals.md (exhaustiveness rules, logical operators, nested branching) — both at status: complete with full PGE cross-references.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Operators spec complete with arithmetic and negation | Pass | Arithmetic section documents =Math.* pipeline model per PGE04010; negation operators expanded with examples; type-operator compatibility table added; PGE04005/410/411/412/415/601/609 all referenced |
| AC-2: Conditionals spec covers exhaustiveness and logical operators | Pass | Full exhaustiveness rules by type (enum, numeric, string, flexible, compound); logical operators [&]/[+]/[-]/[^]; nested conditionals; metadata switching; all 13 PGE-6xx rules referenced |
| AC-3: SPEC-INDEX updated | Pass | conditionals.md inserted as #10 in Phase 2: Core Concepts; all subsequent entries renumbered (11-16) |

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/syntax/operators.md | Modified | Added Arithmetic section (=Math.*), Negation Operators subsection, Type-Operator Compatibility subsection, PGE refs; status → complete |
| docs/user/concepts/conditionals.md | Created | Full conditionals spec: chains, exhaustiveness, logical operators, nesting, metadata switching, wildcard rules, PGE-6xx reference table |
| docs/user/SPEC-INDEX.md | Modified | Added conditionals.md as #10, renumbered subsequent entries, updated description for operators.md |

## Deviations from Plan

### Key Finding: Arithmetic Model

The plan originally described arithmetic as "operators (+, -, *, / with type rules)". During execution, PGE04010 revealed that **raw arithmetic tokens are compile errors** — all arithmetic is performed through `=Math.*` jm3lib pipelines. The Arithmetic section was written to reflect this authoritative compile rule rather than the EBNF grammar productions (which exist only for error detection purposes).

**Impact:** More accurate spec. EC-6.4 in EDGE-CASES shows raw arithmetic syntax that conflicts with PGE04010 — this is a known inconsistency in the technical reference material.

### Scope Note: Loops

The ROADMAP lists "Loops" in Phase 10 scope. Polyglot has no traditional loop construct — iteration uses expand/collect (`~ForEach.*`), which is Phase 11 scope. No loop spec was created. This is correct scoping, not a gap.

## Next Phase Readiness

**Ready:**
- All operator and conditional semantics fully specified
- PGE cross-references in place for downstream phases
- Phase 11 can reference operators.md for pipeline body expressions and conditionals.md for branching

**Concerns:**
- EC-6.4 (arithmetic in assignment) uses raw `*`, `/`, `-` syntax that PGE04010 rejects — technical reference inconsistency worth noting for future EBNF audit

**Blockers:**
- None

---
*Phase: 10-operators-control-flow, Plan: 01*
*Completed: 2026-03-24*
