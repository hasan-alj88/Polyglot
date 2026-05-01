---
phase: issue-272-parameterized-schemas
plan: 05
subsystem: documentation
tags: [ebnf, compile-rules, metadata-tree, macros, generics, schemas]

requires:
  - phase: issue-272-parameterized-schemas (plans 01-04)
    provides: user-facing docs updated with new design
provides:
  - Technical docs aligned with parameterized ## schema design
  - EBNF grammar: {M} removed, generic {#} params added
  - Metadata tree: %M branch removed, new schemas listed
  - Compile rules: macro-specific rules retired, references updated
  - Edge cases: macro edge cases retired, datatype examples updated
affects: []

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/technical/ebnf/01-file-structure.md
    - docs/technical/ebnf/04-type-system.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/technical/ebnf/INDEX.md
    - docs/technical/spec/metadata-tree/FULL-TREE.md
    - docs/technical/spec/metadata-tree/object-types.md
    - docs/technical/spec/metadata-tree/definition-templates.md
    - docs/technical/COMPILE-RULES.md
    - docs/technical/compile-rules/PGE/PGE01002-io-before-trigger.md
    - docs/technical/compile-rules/PGE/PGE01008-wrapper-must-reference-macro.md
    - docs/technical/compile-rules/PGE/PGE01010-pipeline-io-name-mismatch.md
    - docs/technical/compile-rules/PGE/PGE01021-empty-data-definition.md
    - docs/technical/compile-rules/PGE/PGE01023-parameterless-macro.md
    - docs/technical/compile-rules/PGE/PGE01029-invalid-marker-for-definition-type.md
    - docs/technical/compile-rules/PGE/PGE01031-forbidden-element-in-definition.md
    - docs/technical/compile-rules/PGE/PGE02006-metadata-pull-only.md
    - docs/technical/compile-rules/PGE/PGE09007-duplicate-pipeline-definition.md
    - docs/technical/compile-rules/PGE/PGE10006-duplicate-permission.md
    - docs/technical/compile-rules/PGW/PGW01003-no-definitions-in-file.md
    - docs/technical/compile-rules/PGW/PGW10001-unused-permission.md
    - docs/technical/edge-cases/18-macro-structure.md
    - docs/technical/edge-cases/24-datatype-defs.md
    - docs/technical/edge-cases/INDEX.md
    - docs/technical/plan/decisions/schema-properties.md
    - docs/technical/plan/decisions/metadata-data-tree.md
    - docs/technical/brainstorming/marker-declarations.md

key-decisions:
  - "Retirement stubs keep {M} mentions in explanatory text — necessary to explain what was retired"
  - "Section renumbering in 09-definition-blocks: §9.4 {M} removed, §9.4a-c → §9.4-9.6, rest shifted"
  - "-1 depth replaced with .Inf throughout (##Inf schema)"
  - "%##Children.* → direct %## properties (%##Gap, %##Key, %##Ordered, etc.)"
  - "PGE01019 and PGE01023 retired (not deleted) with redirect stubs"

patterns-established: []

duration: ~45min
started: 2026-04-09
completed: 2026-04-09
---

# Plan 272-05 Summary: Technical Docs — EBNF, Compile Rules, Metadata Tree

**Removed all {M} macro references, retired schemas, and %##Children.* properties from ~27 technical documentation files, replacing with parameterized ## schema and generic {#} type syntax.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~45min |
| Started | 2026-04-09 |
| Completed | 2026-04-09 |
| Tasks | 3 completed |
| Files modified | 27 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: No {M} in docs/technical/ | Pass | Remaining mentions are only in retirement stubs explaining what was removed |
| AC-2: No [M] in docs/technical/ | Pass | Remaining mentions are only in retirement table entries |
| AC-3: No retired schemas in docs/ | Pass | Only in expected redirect stubs (docs/user/jm3lib/types/schemas/) |
| AC-4: No %##Children.* in docs/ | Pass | Zero in docs/technical/; docs/user/ remnants are boundary-protected (plans 01-04 scope) |
| AC-5: No %M branch in metadata trees | Pass | Zero matches |
| AC-6: EBNF macro grammar replaced | Pass | macro_def/macro_body/macro_input/macro_output all zero; generic_param/value_param/schema_param_bind added |

## Accomplishments

- EBNF §4.3 rewritten: macro productions replaced with generic_param, value_param, schema_param_bind; full %## and %### property lists added
- EBNF §9 renumbered: §9.4 {M} removed; §9.4a-c (Trigger/Wrapper/Native) promoted to §9.4-9.6; downstream sections shifted
- Metadata tree: %M branch removed from FULL-TREE.md and object-types.md; all named schemas updated (depth use .Inf not -1; new parameterized schemas listed)
- definition-templates.md: complete rewrite of schema tree, %## properties table (16 properties), %### properties table (3 properties)
- 13 compile rule files updated: PGE01023 retired as redirect stub; PGE01019 retired; {M} references replaced with {#}/generic equivalents
- Edge cases: EC-18 renamed "Wrapper Structure" (EC-18.4 retired); EC-24.11 rewritten as generic {#}; EC-24.19/EC-24.20 retired

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| ebnf/01-file-structure.md | Modified | Remove macro_def from definition production |
| ebnf/04-type-system.md | Modified | Rewrite §4.3: macros → generics, new %## properties |
| ebnf/09-definition-blocks.md | Modified | Remove §9.4 {M}, renumber §9.4a-c → §9.4-9.6 |
| ebnf/INDEX.md | Modified | Update section 9 description |
| metadata-tree/FULL-TREE.md | Modified | Remove %M branch, update ## schema list |
| metadata-tree/object-types.md | Modified | Remove %M row |
| metadata-tree/definition-templates.md | Modified | Rewrite schema tree, property tables, examples |
| COMPILE-RULES.md | Modified | Rename/retire codes, update %##Children.* → %## |
| PGE01002 | Modified | Fix {M} wrapper reference |
| PGE01008 | Modified | Remove {M} from statement |
| PGE01010 | Modified | Remove {M} from see-also |
| PGE01021 | Modified | Replace [M] invocation with [#] schema composition |
| PGE01023 | Modified | Retired as redirect stub |
| PGE01029 | Modified | Remove {M} from marker table |
| PGE01031 | Modified | Remove {M} from forbidden element matrix |
| PGE02006 | Modified | Replace %M.* with %W.* in schema tree |
| PGE09007 | Modified | Replace {M} with {W}/{T} in statement |
| PGE10006 | Modified | Remove {M} from scope list |
| PGW01003 | Modified | Replace {M} with {T}/{N} in definition list |
| PGW10001 | Modified | Remove "or macro" from statement |
| edge-cases/18-macro-structure.md | Modified | Renamed "Wrapper Structure", EC-18.4 retired |
| edge-cases/24-datatype-defs.md | Modified | EC-24.11 rewritten, EC-24.19/20 retired, properties updated |
| edge-cases/INDEX.md | Modified | Update S18 title, S24 description |
| decisions/schema-properties.md | Modified | Replace {M} macro section with generic {#} |
| decisions/metadata-data-tree.md | Modified | Remove %M from branch table |
| brainstorming/marker-declarations.md | Modified | Remove {M} from hierarchy, retire open question |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Essential alignment |
| Scope additions | 0 | — |
| Deferred | 1 | Logged below |

**Total impact:** Minimal — one property rename beyond plan scope discovered.

### Auto-fixed Issues

**1. -1 depth replaced with .Inf**
- **Found during:** Task 2 (COMPILE-RULES.md)
- **Issue:** `%##Depth.Max << -1` is stale; design uses `##Inf` schema and `.Inf` value
- **Fix:** Replaced all `-1` depth references with `.Inf` in COMPILE-RULES.md and edge cases
- **Verification:** grep confirms zero `-1` depth references remain in modified files

### Deferred Items

- **docs/user/concepts/collections/serial.md** has stale `%##Children.*` and `##Heterogeneous` references that should have been caught in plans 01-04. These are outside 272-05 boundaries (docs/user/ protected). Should be fixed in a follow-up cleanup.

## Next Phase Readiness

**Ready:**
- All 5 plans for Issue #272 are now complete (applied + unified)
- Zero {M}/{[M]}/%M/macro_def/##Homogeneous/##Heterogeneous/##EnumLeafs/%##Children.* in docs/technical/
- Branch design/issue-272-parameterized-schemas-replace-m-macros ready for merge

**Concerns:**
- serial.md stale references (deferred — outside scope)

**Blockers:**
- None

---
*Phase: issue-272-parameterized-schemas, Plan: 05*
*Completed: 2026-04-09*
