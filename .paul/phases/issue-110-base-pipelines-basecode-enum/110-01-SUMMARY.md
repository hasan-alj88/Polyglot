---
phase: issue-110-base-pipelines-basecode-enum
plan: 01
subsystem: docs
tags: [base-pipeline, derived-pipeline, baseCode, jm3lib, compile-rule]

requires: []
provides:
  - "#BaseCode enum type documentation"
  - ".baseCode reserved metadata field in EBNF and user docs"
  - "Base vs Derived pipeline section in pipelines INDEX.md"
  - "PGE01028 base/derived mutual exclusion compile rule"
affects: [issue-111-compiler-rules]

key-files:
  created:
    - docs/user/jm3lib/types/BaseCode.md
    - docs/technical/compile-rules/PGE/PGE01028-base-derived-mutual-exclusion.md
  modified:
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/user/concepts/metadata.md
    - docs/user/concepts/pipelines/INDEX.md

key-decisions:
  - "PGE01028 uses sub-conditions a-e for different violation types"
  - "{T} and {Q} exempt from bodyless-without-baseCode error (IO-only by design)"

duration: 5min
completed: 2026-03-31
---

# Issue #110 Plan 01: Document Base Pipelines and #BaseCode Enum Summary

**Created `#BaseCode` enum type doc, added `.baseCode` reserved metadata field to EBNF §9.9 and metadata.md, added "Base vs Derived" section to pipelines INDEX.md, and created PGE01028 compile rule with 5 sub-conditions.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-31 |
| Tasks | 3 completed |
| Files modified | 5 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: BaseCode.md type documentation exists | Pass | Full Rust variant tree, ##Scalar/###Enum schema, config documented |
| AC-2: EBNF §9.9 includes .baseCode reserved field | Pass | `metadata_basecode` production added, rules explain base constraints |
| AC-3: metadata.md lists .baseCode in Fixed Fields | Pass | Row added + base/derived note paragraph |
| AC-4: Pipelines INDEX.md has Base vs Derived section | Pass | Table, examples, cross-references, PGE01028 link |
| AC-5: PGE01028 compile rule for base/derived mutual exclusion | Pass | 5 sub-conditions (a-e), valid/invalid examples from brainstorming Rule F |
| AC-6: Consistency across all files | Pass | PGE01028 referenced in 4 files, cross-references verified |

## Accomplishments

- Created `#BaseCode` enum type doc with full Rust variant tree mirroring all jm3lib pipeline names
- Added `metadata_basecode` production to EBNF §9.9 grammar with base pipeline constraint rules
- Added `.baseCode` to metadata.md Fixed Fields with base/derived explanation note
- Created comprehensive "Base vs Derived" section in pipelines INDEX.md with comparison table and examples
- Created PGE01028 with 5 sub-conditions covering all base/derived violation scenarios

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/jm3lib/types/BaseCode.md` | Created | `#BaseCode` enum definition, variant tree, config, usage |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | `metadata_basecode` production + `.baseCode` rules in §9.9 |
| `docs/user/concepts/metadata.md` | Modified | `.baseCode` row in Fixed Fields + base/derived note |
| `docs/user/concepts/pipelines/INDEX.md` | Modified | "Base vs Derived" section with table, examples, cross-refs |
| `docs/technical/compile-rules/PGE/PGE01028-base-derived-mutual-exclusion.md` | Created | 5 sub-conditions, 6 valid + 6 invalid examples |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| PGE01028 uses sub-conditions a-e | Each violation type has distinct trigger and error message | Clearer compiler diagnostics |
| `{T}` and `{Q}` exempt from condition c | They are IO-only by design — bodyless without `.baseCode` is valid | Prevents false positives on trigger/queue definitions |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Base/derived distinction fully documented — foundation for compiler rules (#111)
- `#BaseCode` enum available for jm3lib pipeline files (future issue)
- PGE01028 ready for compiler implementation

**Concerns:**
- jm3lib pipeline `.pg` files don't yet declare `[%] .baseCode` — separate issue needed

**Blockers:**
- None

---
*Phase: issue-110-base-pipelines-basecode-enum, Plan: 01*
*Completed: 2026-03-31*
