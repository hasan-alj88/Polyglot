---
phase: 94-schema-validation
plan: 02
subsystem: spec
tags: [aj3lib, pipelines, validation, schema, file-io, errors]

provides:
  - "=File.Serial.Read/Write/Read.Field pipelines"
  - "=#.Match/Validate/Describe/Coerce validation pipelines"
  - "=#.Field safe field extraction pipeline"
  - "=#.Column dataframe column extraction pipeline"
  - "=#.JSON/YAML/TOML.Parse base parsers (compiler intrinsics)"
  - "<#type pipeline IO pattern (extends <# from macros to pipelines)"
  - "!Validation.Schema/.Type/.Regex, !Field.NotFound/.PathError, !File.ParseError error leaves"
affects: [94-03 expand/collect audit]

key-files:
  created:
    - docs/user/aj3lib/pipelines/#.md
  modified:
    - docs/user/aj3lib/pipelines/File.md
    - docs/user/aj3lib/errors/errors.md
    - docs/user/aj3lib/INDEX.md
    - docs/user/syntax/types.md
    - docs/user/concepts/pipelines.md
    - docs/technical/EBNF.md

key-decisions:
  - "Base parsers (=#.JSON/YAML/TOML.Parse) are compiler intrinsics, not user-definable"
  - "Validation pipelines (=#.Match/Validate/Describe/Coerce) are non-failable — report via outputs, not errors"
  - "<#type extends <# from {M} macros to {=} pipeline IO — same mechanism, runtime context"
  - "!Validation.Error replaced by .Schema/.Type/.Regex for granular validation reporting"

completed: 2026-03-30
---

# Plan 94-02: Serial File Loading + Schema Validation Pipelines — Summary

**Added 10 `=#.*` pipelines (base parsers, validation, field/column extraction), 3 `=File.Serial.*` pipelines, `<#type` pipeline IO pattern, and 6 new error leaves across 7 spec files.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: File Serial Pipelines Specified | Pass | =File.Serial.Read/Write/Read.Field in File.md with IO tables, errors, permissions |
| AC-2: Schema Validation Pipelines Specified | Pass | New #.md with all 10 pipelines (3 base + 4 validation + 2 field + 1 column) |
| AC-3: Error Leaves Added | Pass | !File.ParseError, !Validation.Schema/.Type/.Regex (replacing .Error), !Field.NotFound/.PathError |
| AC-4: INDEX.md Updated | Pass | =# row added; =File description updated; error prefix list updated |
| AC-5: <#type Pipeline IO Documented | Pass | New subsection in types.md with tier table and example |
| AC-6: EBNF Updated | Pass | type_input_line production added to io_section grammar |
| AC-7: Cross-References Consistent | Pass | Wikilinks between types.md, pipelines.md, #.md, errors.md all resolve |

## Deviations from Plan

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 0 | N/A |
| Deferred | 0 | N/A |

None — plan executed exactly as written.

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/aj3lib/pipelines/#.md` | Created | 10 pipelines: 3 base parsers, 4 validation, 2 field/column extraction |
| `docs/user/aj3lib/pipelines/File.md` | Modified | Added =File.Serial.* branch (3 pipelines), permissions, errors, implementation status |
| `docs/user/aj3lib/errors/errors.md` | Modified | Added !File.ParseError, replaced !Validation.Error with .Schema/.Type/.Regex, added !Field namespace, 5 new pipeline error associations |
| `docs/user/aj3lib/INDEX.md` | Modified | Added =# row, updated =File description, updated error prefix list |
| `docs/user/syntax/types.md` | Modified | Added "<#type in Pipeline IO" subsection with tier table and example |
| `docs/user/concepts/pipelines.md` | Modified | Added <#type cross-reference paragraph in IO section |
| `docs/technical/EBNF.md` | Modified | Added type_input_line production to io_section grammar |

## Next Phase Readiness

**Ready:**
- All `=#.*` and `=File.Serial.*` pipelines fully specified
- `<#type` IO pattern available for future pipelines
- Error tree complete for validation and field extraction
- Foundation set for Plan 94-03 (expand/collect audit, issue #91)

**Concerns:**
- COMPILE-RULES.md still has 4 stale generic references from 94-01 (deferred)

**Blockers:** None

---
*Phase: 94-schema-validation, Plan: 02*
*Completed: 2026-03-30*
