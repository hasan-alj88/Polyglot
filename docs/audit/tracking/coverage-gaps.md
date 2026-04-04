---
type: audit-tracking
scope: documentation-coverage
updated: 2026-03-30
---

# Documentation Coverage Gaps

<!-- @audit/README -->
<!-- @rules/checklist -->
Track structural coverage gaps between user documentation and technical documentation. See [[audit/README]] for authority chain and [[rules/checklist]] for quality checks. For inconsistencies (IC), see [[tracking/inconsistencies]].

## Gap Categories

| Code | Category | Meaning |
|------|----------|---------|
| UG | User Gap | User-facing feature lacks technical background |
| TG | Tech Gap | Technical spec lacks user-facing documentation |
| IC | Inconsistency | Two docs contradict each other — **tracked in [[tracking/inconsistencies]]** |
| MX | Missing Cross-ref | Feature mentioned without link to related docs |
| OR | Orphan | File not linked from any index |

## Active Gaps

| ID | Cat | Description | Source | Missing In | Priority | Status |
|----|-----|-------------|--------|-----------|----------|--------|
| GAP-001 | TG | Macro {M} has edge cases (§18) but no dedicated user concept doc | edge-cases/18-macro-structure.md | user/concepts/ | High | Resolved |
| GAP-002 | UG | permissions.md references permission model but no compile rules enforce it | user/concepts/permissions.md | technical/compile-rules/ | Medium | Resolved |
| GAP-003 | MX | EBNF §9 Definition Blocks covers {=},{#},{M},{@} but user docs split across separate files without back-links | technical/ebnf/09-definition-blocks.md | user/ concept files | Medium | Resolved |
| GAP-004 | TG | type-identity.md spec has no user-facing explanation of structural matching | technical/spec/type-identity.md | user/syntax/types/ | Medium | Resolved |
| GAP-005 | MX | Compile-rule PGE files lack systematic links back to user docs and edge cases | technical/compile-rules/PGE/ | cross-refs | Low | Resolved |
| GAP-006 | TG | String regex subfields decision (technical/plan/decisions/) has no user doc coverage | technical/plan/decisions/string-re-subfields.md | user/syntax/types/ | Low | Resolved |
| GAP-007 | IC | draft.md (now archived) references PGE-XXX placeholder codes not in COMPILE-RULES.md | archive/wip/draft.md | technical/COMPILE-RULES.md | Low | Moved → IC-008 |
| GAP-008 | TG | EBNF §8 Expressions defines 16 expression types but no unified user doc covers them — concepts scattered across operators.md, line-structure.md, blocks.md | technical/ebnf/08-expressions.md | user/syntax/ (no expressions.md) | Medium | Resolved |
| GAP-009 | MX | PGE01004 (Macro Structural Constraints) has no "See also" back-link to user docs, though wrappers.md does reference PGE01004 forward | technical/compile-rules/PGE/PGE01004-macro-structural-constraints.md | cross-ref to user/concepts/pipelines/wrappers.md | Low | Resolved |
| GAP-010 | MX | PGE03001 (No Push Across Parallel) has no "See also" back-link to user docs, though collections docs reference PGE03001 forward | technical/compile-rules/PGE/PGE03001-no-push-across-parallel.md | cross-ref to user/concepts/collections/ | Low | Resolved |

## Resolved Gaps

| ID | Cat | Description | Resolution | Date |
|----|-----|-------------|------------|------|
| GAP-001 | TG | Macro {M} has no user concept doc | Created user/concepts/macros.md — brief consumer-focused intro, stdlib macro list, links to syntax/types/macro-types.md for full reference | 2026-03-30 |
| GAP-002 | UG | permissions.md references permission model but no compile rules enforce it | PGE10003, PGE10004, PGE10005, PGE10006 now enforce permission model | 2026-03-30 |
| GAP-003 | MX | EBNF §9 missing back-links to user docs | Added "Related User Documentation" table to 09-definition-blocks.md mapping all §9 subsections to user concept/syntax files | 2026-03-30 |
| GAP-004 | TG | type-identity.md has no user-facing explanation | Added "Type Identity" section to user/syntax/types/conversions.md — structural matching, no implicit coercion, link to technical spec | 2026-03-30 |
| GAP-009 | MX | PGE01004 missing back-link | Added "See Also" section with links to wrappers.md and macros.md | 2026-03-30 |
| GAP-010 | MX | PGE03001 missing back-link | Added "See Also" section with link to collections/ | 2026-03-30 |
| GAP-006 | TG | String regex subfields decision has no user doc | User coverage already exists in string.md + scalars.md + basic-types.md. Renamed field `.re` → `.regex` (alias `.re`) across all docs | 2026-03-30 |
| GAP-008 | TG | EBNF §8 expressions not in user docs | Expressions are a grammar abstraction, not a user concept. All 16 types already covered by user docs organized by mental model: operators.md (assignment, comparison, arithmetic), blocks.md (structural elements), io.md (IO + expand/collect), collections/ (expand/collect semantics) | 2026-03-30 |
| GAP-005 | MX | PGE files lack back-links | Added "See Also" sections to 90 PGE compile rule files with wikilinks to referencing user docs and edge cases | 2026-03-30 |

## Audit Process

1. When splitting a file, check: does every H2 section have both a user doc AND a technical doc?
2. When adding a new compile rule, check: does the user doc mention this constraint?
3. When adding a new stdlib pipeline, check: does an edge case exist for it?
4. Run coverage check: for each EBNF section, verify a user doc section covers the same concept
5. Update this file whenever a gap is discovered or resolved
