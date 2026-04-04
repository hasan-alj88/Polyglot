---
phase: issue-121-native-block
plan: 01
subsystem: docs
tags: [native, blocks, ebnf, metadata, NativeKind]

requires:
  - phase: none
    provides: n/a
provides:
  - "{N} block type for compiler-native definitions"
  - "#NativeKind enum (Trigger, Queue, Wrapper, Execution, Intrinsic)"
  - "%Native.* metadata scope with .Kind, .<Language>, .description"
  - "EBNF §9.4c grammar for {N}"
affects: ["121-02 (BaseCode retirement)", "121-03 (stdlib propagation)"]

key-files:
  modified:
    - docs/user/syntax/blocks.md
    - docs/user/concepts/pipelines/INDEX.md
    - docs/user/concepts/metadata.md
    - docs/technical/ebnf/09-definition-blocks.md

key-decisions:
  - "{N} is a separate block type, not a {=} subtype"
  - "[%] under {N} implicitly scopes to %Native.* — all fixed . fields"
  - "#NativeKind replaces #BaseCode (5 variants: Trigger, Queue, Wrapper, Execution, Intrinsic)"
  - ".<Language> field holds native function name per supported language"
  - "{T}/{Q}/{W} remain as user-extendable {=} subtypes — {N} is compiler-only"

duration: 15min
completed: 2026-04-04
---

# Issue #121 Plan 01: Core {N} Design — Summary

**Introduced `{N}` block type for compiler-native definitions with `#NativeKind` enum and `%Native.*` metadata scope across 4 core reference files.**

## Acceptance Criteria Results

| Criterion | Status |
|-----------|--------|
| AC-1: {N} in blocks.md definition table | Pass |
| AC-2: Native vs Derived section in INDEX.md | Pass |
| AC-3: %Native metadata in metadata.md | Pass |
| AC-4: EBNF §9.4c grammar | Pass |

## Accomplishments

- Added `{N}` to block system with `%Native` metadata tree branch
- Replaced "Base vs Derived" with "Native vs Derived" in INDEX.md — complete rewrite with {N} examples, #NativeKind enum table, configuration section
- Added structure table footnote clarifying [T]/[Q]/[W] are derived-only
- Created EBNF §9.4c with full `native_def` grammar
- Updated §9.9 metadata rules to reference {N} instead of .baseCode
- Added {N} live fields section to metadata.md

## Next Phase Readiness

**Ready:**
- Core {N} design established in 4 reference files
- #NativeKind enum defined, ready for type file creation

**Remaining plans:**
- 121-02: Replace #BaseCode with #NativeKind (retire BaseCode.md, update PGE01028, cross-refs)
- 121-03: Propagate {N} to stdlib pipeline definitions (replace {=}[exe] + .baseCode examples)

---
*Phase: issue-121-native-block, Plan: 01*
*Completed: 2026-04-04*
