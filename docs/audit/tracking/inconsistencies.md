---
type: audit-tracking
scope: inconsistencies
updated: 2026-03-30
---

# Inconsistencies

<!-- @audit/README -->
<!-- @rules/checklist -->
Contradictions found between documentation files. See [[audit/README]] for authority chain and [[rules/checklist]] for quality checks. For coverage gaps (UG/TG/MX/OR), see [[tracking/coverage-gaps]].

**GitHub tracker:** [Issue #95](https://github.com/hasan-alj88/Polyglot/issues/95) — umbrella issue with checklist for all IC items.

## Tags

| Tag | Meaning |
|-----|---------|
| code | Invalid code example — won't compile |
| audience | Audience mismatch or leak |
| wikilink | Broken or non-existent wikilink target |
| type-system | Type definition contradiction |
| convention | Style or rule violation |

## Active Inconsistencies

| ID | Severity | Tag | Description | File(s) | Status |
|----|----------|-----|-------------|---------|--------|
| IC-001 | Critical | code | `#VarState.Ready` used in example but doesn't exist in enum — only `.Declared`, `.Default`, `.Final`, `.Failed`, `.Released` | [[concepts/variable-lifecycle\|variable-lifecycle.md]]:85 | Resolved |
| IC-002 | Critical | audience | Frontmatter declares `audience: developer` but file lives in `docs/user/concepts/` (user-facing) | [[concepts/metadata\|metadata.md]]:2 | Resolved — AD-005 |
| IC-003 | High | type-system | `#Dimension` described as "allows 0D" but regex `^[1-9][0-9]*D$` explicitly excludes 0D | [[stdlib/types/types\|types.md]]:24 vs [[stdlib/types/scalars\|scalars.md]]:117 | Resolved |
| IC-004 | High | wikilink | 12 broken `[[STDLIB]]` wikilinks — no STDLIB.md exists (see details below) | 7 files | Resolved |
| IC-005 | Medium | type-system | `#PipelineStatus`, `#VarState`, `#Boolean` listed as "stdlib structs" but are enums | [[syntax/types/structs\|structs.md]]:17 vs [[stdlib/types/enums\|enums.md]] | Resolved |
| IC-006 | Medium | convention | Missing mandatory `@`-imports (`@vision`, `@glossary`, `@audit/README`) in index files | [[user/SPEC-INDEX]], [[technical/INDEX]] | Resolved |
| IC-007 | Low | convention | TBD stub `└── (live fields TBD)` without tracking reference | [[concepts/metadata\|metadata.md]]:89 | Resolved — AD-005 |
| IC-008 | Low | code | Archived draft.md references PGE-XXX placeholder codes not in COMPILE-RULES.md | [[archive/wip/draft\|draft.md]] | Won't Fix — archived |

## Details

### IC-004: Broken `[[STDLIB]]` wikilinks (Resolved)

All 12 broken `[[STDLIB]]` references were rewritten to point to specific component files. The `metadata.md` references (lines 276–277) were removed during the AD-005 metadata split.

## Resolved Inconsistencies

| ID | Severity | Description | Resolution | Date |
|----|----------|-------------|------------|------|
| IC-001 | Critical | `#VarState.Ready` used in example but doesn't exist in enum | Changed to `#VarState.Default` — valid enum variant | 2026-03-30 |
| IC-002 | Critical | metadata.md had `audience: developer` in user-facing path | Split per AD-005: rewrote as user-facing concept doc (`audience: user`); architectural content already covered by metadata-tree.md | 2026-03-30 |
| IC-003 | High | `#Dimension` regex excluded 0D | Corrected regex to `^[0-9]+D$` in scalars.md (table + definition block), matching EC-24.3 resolution | 2026-03-30 |
| IC-004 | High | 12 broken `[[STDLIB]]` wikilinks across 7 files | Rewrote each link to correct component: `[[stdlib/INDEX\|Standard Library]]`, `[[stdlib/pipelines/Path\|=Path]]`, `[[stdlib/types/boolean\|#Boolean]]`, `[[stdlib/INDEX#Pipeline Namespaces\|Wrappers]]` | 2026-03-30 |
| IC-005 | Medium | Enums mislabeled as structs in structs.md | Already fixed — structs.md no longer lists enum types; confirmed via grep | 2026-03-30 |
| IC-006 | Medium | Missing `@`-imports in index files | Added `@vision`, `@glossary`, `@audit/README` to both SPEC-INDEX.md and technical/INDEX.md | 2026-03-30 |
| IC-007 | Low | TBD stub in metadata.md tree diagram | Tree diagram removed in AD-005 split; macro section now states "not yet defined" explicitly | 2026-03-30 |
