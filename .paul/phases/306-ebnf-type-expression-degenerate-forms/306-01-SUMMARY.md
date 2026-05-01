---
phase: 306-ebnf-type-expression-degenerate-forms
plan: 01
subsystem: ebnf
tags: [type-system, grammar, edge-cases, wildcard, live-type, array, version]
provides:
  - concrete_type_expr production (no wildcard, no nested live)
  - element_type_param production (mandatory array element type)
  - multi-digit version segments
  - EC-4.20 through EC-4.23 edge cases
affects: type annotations, array declarations, version parsing
key-files:
  modified:
    - docs/technical/ebnf/04-type-system.md
    - docs/technical/ebnf/03-identifiers.md
    - docs/technical/edge-cases/04-type-system.md
key-decisions:
  - "X.37: Remove wildcard_type from grammar — ## schemas cover multi-type constraints"
  - "X.38: Mandatory array element type in grammar (Option B) — PGE04025 becomes stub"
  - "X.39: Break live_type recursion via concrete_type_expr"
  - "X.40: Expand version segments to multi-digit"
completed: 2026-04-16
---

# Issue #306 Plan 01: Type Expression Degenerate Forms Summary

**Tightened EBNF §4.1 and §3.3 to reject 4 degenerate type expression forms at grammar level; cleaned stale wildcard references from 6 files.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Wildcard type removed from EBNF | Pass | wildcard_type production deleted; removed from type_expr and type_param |
| AC-2: Array element type mandatory in grammar | Pass | element_type_param created; array_type requires it (not optional) |
| AC-3: Live type recursion broken | Pass | concrete_type_expr added; live_type wraps concrete only |
| AC-4: Version segments multi-digit | Pass | digit { digit } for all segments |
| AC-5: Edge cases added | Pass | EC-4.20 through EC-4.23; INDEX.md updated |
| AC-6: Stale references cleaned | Pass | Zero [:] :*#* matches; zero #* type wildcard references |

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/ebnf/04-type-system.md | Modified | concrete_type_expr, element_type_param, live recursion fix, wildcard removal |
| docs/technical/ebnf/03-identifiers.md | Modified | version segments multi-digit |
| docs/technical/edge-cases/04-type-system.md | Modified | EC-4.20 through EC-4.23 added; EC-4.19 updated |
| docs/technical/edge-cases/INDEX.md | Modified | EC-4.1--4.19 → EC-4.1--4.23 |
| docs/user/syntax/types/generic-types.md | Modified | #* Wildcard Type section → Multi-Type Constraints |
| docs/user/jm3lib/types/Serial.md | Modified | [:] :*#* removed |
| docs/user/concepts/collections/serial.md | Modified | [:] :*#* removed |
| docs/technical/edge-cases/24-datatype-defs.md | Modified | [:] :*#* removed (2 occurrences) |
| docs/technical/plan/decisions/schema-properties.md | Modified | [:] :*#* removed; Gap #2 updated |
| docs/technical/compile-rules/PGE/PGE04025-untyped-array.md | Modified | Reduced to stub (grammar-enforced) |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Remove wildcard_type entirely | ## schemas already cover multi-type constraints; #* had no legitimate use case | No more ambiguous "any type" annotations |
| Array element type mandatory in grammar (Option B) | Grammar-level enforcement is cleaner than semantic PGE04025 | PGE04025 becomes stub redirect |
| Break live recursion via concrete_type_expr | Nested live is meaningless; live is internal only | #live live X is grammar error |
| Multi-digit version segments | Single-digit cap at v9.9.9.9 was unintentional | Real-world semver now supported |

## Deviations from Plan

None — plan executed exactly as written.

---
*Phase: 306-ebnf-type-expression-degenerate-forms, Plan: 01*
*Completed: 2026-04-16*
