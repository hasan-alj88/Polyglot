---
phase: issue-121-native-block
plan: 03
subsystem: docs
tags: [native, jm3lib, DT, queue, propagation]

requires:
  - phase: issue-121-native-block/02
    provides: "NativeKind.md type file, PGE01028 rewritten for {N}"
provides:
  - "40 DT.md pipeline definitions converted from {=} to {N} with %Native metadata"
  - "4 queue.md pipeline operations converted from {Q}+.baseCode to {N}"
  - "Terminology aligned: 'base pipeline' → 'native definition' across io-triggers.md and #.md"
affects: []

key-files:
  modified:
    - docs/user/jm3lib/pipelines/DT.md
    - docs/user/concepts/pipelines/queue.md
    - docs/user/concepts/pipelines/io-triggers.md
    - docs/user/jm3lib/pipelines/#.md

key-decisions:
  - "DT.md .Rust names use CamelCase from pipeline path: =DT.From.ISO → DtFromIso"
  - "Queue defs get .description fields added (not present in old .baseCode format)"

duration: 10min
completed: 2026-04-04
---

# Issue #121 Plan 03: Propagate {N} to jm3lib — Summary

**Converted 44 jm3lib pipeline definitions from old `{=}`/`{Q}` + `.baseCode` syntax to `{N}` blocks with `%Native.*` metadata, and aligned all "base pipeline" terminology to "native definition".**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: DT.md uses {N} block syntax | Pass | 40 defs converted; 0 `{=} =DT.*` remain; 0 "Base pipeline" comments |
| AC-2: queue.md uses {N} for queue ops | Pass | 4 defs converted; 0 `.baseCode`/`#BaseCode` remain |
| AC-3: Terminology updated | Pass | io-triggers.md + #.md use "native" not "base pipeline/parser" |
| AC-4: No stale references | Pass | grep confirms 0 matches across all 4 files |

## Accomplishments

- Converted 40 `=DT.*` pipeline definitions to `{N}` with `.Kind << #NativeKind.Execution` and `.Rust` fields
- Converted 4 `=Q.*` queue operations to `{N}` with `.Kind << #NativeKind.Queue` and `.Rust` fields
- Updated "base pipelines backed by native code" → "native definitions" in queue.md and io-triggers.md
- Renamed "Base Parsers" → "Native Parsers" in #.md

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Issue #121 complete.** All 3 plans executed:
- 121-01: Core {N} design (blocks.md, INDEX.md, metadata.md, EBNF)
- 121-02: Retire BaseCode → NativeKind (type file, PGE01028, EBNF, cross-refs)
- 121-03: Propagate {N} to jm3lib (DT.md, queue.md, io-triggers.md, #.md)

**Ready for:** Git commit, branch merge to main, issue closure.

---
*Phase: issue-121-native-block, Plan: 03*
*Completed: 2026-04-04*
