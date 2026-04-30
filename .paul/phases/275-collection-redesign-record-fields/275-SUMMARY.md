---
phase: 275-collection-redesign-record-fields
plan: all (5 plans)
completed: 2026-04-10
---

# Issue #275: Collection Redesign — ##Record, %##Fields, retire #Map/#Set

**Replaced #Map/#Set with ##Record, unified child fields under %##Fields, retired 6 schemas, updated schema composition syntax across ~60 files.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| ##Record schema defined | Pass | Parameterized with <#Fields << ##Enum, <#ValueType |
| #FieldsDescriptor type defined | Pass | .Range and .Enum variants, aliased as #Range/#Enum |
| #Bound.Inf aliased as #Inf | Pass | Follows #Boolean.True → #True pattern |
| ##Array uses %##Fields << #Range | Pass | Replaces ##Contiguous/##Rectangular |
| ##Dataframe uses %##Level.2 ##Record | Pass | Two-level: L1 range rows, L2 record columns |
| #Serial uses #Inf | Pass | %##Depth.Max << #Inf, %##Count << #Inf |
| %## property tables updated | Pass | Branch/Tree/Leaf tables match issue spec |
| Schema composition syntax | Pass | [#] ##Schema (no <<) across all docs/ |
| Retired types removed | Pass | #Map, #Set, ##Map, ##Set, ##Contiguous, ##Rectangular, ##Sparse, ##Deep |
| Full-codebase verification | Pass | 0 occurrences of [#] << ## in docs/ |

## Plans Executed

| Plan | Scope | Files | Commits |
|------|-------|-------|---------|
| 275-01 | Core definitions + property tables | 5 | (pre-agent) |
| 275-02 | Collection user docs (aj3lib + concepts) | 13 | 5 |
| 275-03 | Schema files + composition syntax | 10 | 4 |
| 275-04 | Technical docs (EBNF, rules, metadata) | 7 | 5 |
| 275-05 | Remaining propagation + verification | 42 | 4 |

## Deviations

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 3 | Essential fixes — scalars.md %##Key ref, Serial.md %##Regular, #FlexKind refs |
| Scope additions | 1 | 275-05 found 42 files needing syntax update vs 16 planned |
| Deferred | 0 | None |

## Decisions

None — straightforward execution of issue #275 specification.

## Next

Ready for `/paul:merge` to merge branch to main and close issue #275.

---
*Completed: 2026-04-10*
