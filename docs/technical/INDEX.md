---
type: spec-index
audience: developer
updated: 2026-03-19
---

# Polyglot Technical Documentation

Internal specifications for Polyglot project contributors. These documents describe how Polyglot Code is parsed, validated, and processed.

## Specifications (docs/technical/)

| File | Covers |
|------|--------|
| EBNF.md | Complete formal grammar in Extended Backus-Naur Form |
| EDGE-CASES.md | 42 edge cases organized by EBNF section for validation |
| COMPILE-RULES.md | Error/warning code lookup tables, legend, and rule format template |

## Compiler Rules (compile-rules/)

Semantic constraints enforced at compile time — beyond what EBNF grammar captures. Each rule lives in its own file under `compile-rules/PGE/` (errors) or `compile-rules/PGW/` (warnings).

**Numbering convention:**
- Rules use `N.M` — category dot sequence (e.g., 3.5)
- Errors use `PGE-NMM` — category hundred + sequence (e.g., PGE-305)
- Warnings use `PGW-NMM` — same scheme, mirroring PGE ranges

**Categories:**

| Cat | Name | Code Range | Rules |
|-----|------|------------|-------|
| 1 | Execution Order & Structure | PGE-1xx | 1.1–1.4 |
| 2 | Variable Lifecycle | PGE-2xx, PGW-2xx | 2.1–2.7 |
| 3 | Parallel Execution | PGE-3xx, PGW-3xx | 3.1–3.6 |
| 4 | Type & Schema Rules | PGE-4xx | 4.1–4.4 |
| 5 | Identifier & Serialization | PGE-5xx | 5.1–5.2 |
| 6 | Conditional Exhaustiveness | PGE-6xx | 6.1–6.4 |
| 7 | Error Handling | PGE-7xx | 7.1 |
| 8 | Chain Execution | PGE-8xx, PGW-8xx | 8.1–8.2 |

## Compiler Issues (docs/technical/compiler_issues/)

Deferred compiler enhancements and investigation items.

| File | Summary |
|------|---------|
| 001-static-failed-detection.md | Statically detect when `[!]` handlers don't provide replacement values for output variables |
| 002-metadata-schema-tree.md | Document the full `%` metadata schema tree — block types, instance references, `live` vs non-live fields |
| 003-metadata-access-before-collection.md | Can `live` metadata be inspected before parallel collection? (Rule 3.3) |
| 004-setup-internal-parallel-collection.md | Can `[\]` setup have both `[p]` and `[*]` internally? (Rule 3.4) |
| 005-race-type-matching-semantics.md | Race collector type matching — exact vs structural (Rule 3.6) |
| 006-compound-condition-exhaustiveness.md | Boolean algebra analysis for compound condition exhaustiveness (Rule 6.1) |
| 007-pge601-duplicate-content.md | PGE-601 contains duplicated type-specific content from PGE-602/603 |
| 008-pge203-pge204-overlap.md | PGE-203 and PGE-204 have overlapping scope |
| 009-pge301-pge303-push-redundancy.md | PGE-303 push case is redundant with PGE-301 |
| 010-pge101-split-failure-modes.md | PGE-101 covers 5 distinct failure modes under one code |
| 011-pge104-incomplete-macro-whitelist.md | PGE-104 macro element whitelist is incomplete |
| 012-pge205-continue-mechanism-split.md | PGE-205 *Continue mechanism warrants its own rule |
| 013-pge401-extract-type-definition.md | PGE-401 canonical "same type" definition should be shared |
| 014-pge601-restructure-type-sections.md | PGE-601 needs per-type sub-rules (PGE-605/606/607) |
| 015-pge701-chain-error-expansion.md | PGE-701 chain error addressing needs explicit coverage |

## Language Reference

See [[user/SPEC-INDEX|docs/user/SPEC-INDEX.md]] for the user-facing language reference (syntax, concepts, stdlib).
