---
phase: issue-121-native-block
plan: 02
subsystem: docs
tags: [native, NativeKind, BaseCode, EBNF, compile-rules]

requires:
  - phase: issue-121-native-block/01
    provides: "{N} block type, #NativeKind enum in INDEX.md/blocks.md/metadata.md/EBNF"
provides:
  - "NativeKind.md stdlib type file (replaces BaseCode.md)"
  - "PGE01028 rewritten for {N} vs {=} mutual exclusion"
  - "EBNF metadata_basecode rule removed"
  - "Cross-references updated: schema-properties.md, definition-templates.md"
affects: ["121-03 (stdlib propagation)"]

key-files:
  created:
    - docs/user/stdlib/types/NativeKind.md
  modified:
    - docs/technical/compile-rules/PGE/PGE01028-base-derived-mutual-exclusion.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/user/syntax/types/schema-properties.md
    - docs/technical/spec/metadata-tree/definition-templates.md
  deleted:
    - docs/user/stdlib/types/BaseCode.md

key-decisions:
  - "PGE01028 sub-condition (c) now enforces %Native.Kind presence on {N}, not .baseCode on {=}"
  - "#NativeKind.Intrinsic added for compiler built-ins like =DoNothing (no host function needed)"

duration: 10min
completed: 2026-04-04
---

# Issue #121 Plan 02: Retire BaseCode → NativeKind — Summary

**Replaced `#BaseCode` enum and `.baseCode` metadata with `#NativeKind` type file, rewrote PGE01028 for `{N}` vs `{=}` block type separation, and cleaned all stale references from EBNF and cross-ref files.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: NativeKind.md replaces BaseCode.md | Pass | 5-variant ##Scalar ###ScalarEnum created; BaseCode.md deleted |
| AC-2: PGE01028 rewritten for {N} vs {=} | Pass | 5 sub-conditions (a-e), all examples use {N} syntax |
| AC-3: EBNF grammar updated | Pass | metadata_basecode rule + reference removed |
| AC-4: Cross-references updated | Pass | #BaseCode → #NativeKind in schema-properties + definition-templates |

## Accomplishments

- Created `NativeKind.md` with 5 variants (Trigger, Queue, Wrapper, Execution, Intrinsic) and usage/config documentation
- Rewrote PGE01028 from `.baseCode` vs body enforcement to `{N}` vs `{=}` block type separation with 7 valid + 5 invalid examples
- Removed `metadata_basecode` EBNF production rule and its reference in `metadata_expr`
- Updated ###ScalarEnum examples in schema-properties.md and definition-templates.md

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- #NativeKind type file established as stdlib reference
- PGE01028 enforces {N} block semantics
- All core + cross-ref files aligned on #NativeKind

**Remaining plans:**
- 121-03: Propagate {N} to stdlib pipeline definitions (replace {=}[exe] + .baseCode examples with {N} syntax)

---
*Phase: issue-121-native-block, Plan: 02*
*Completed: 2026-04-04*
