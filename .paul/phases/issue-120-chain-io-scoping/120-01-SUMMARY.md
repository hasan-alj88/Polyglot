---
phase: issue-120-chain-io-scoping
plan: 01
subsystem: docs
tags: [io, chains, ebnf, terminology]

requires:
  - phase: none
    provides: n/a
provides:
  - Consistent "pipeline perspective" terminology across IO docs
  - Clarifying one-line scoping rule for </'>' prefixes
affects: []

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/user/syntax/io.md
    - docs/user/concepts/pipelines/chains.md
    - docs/technical/ebnf/10-execution.md

key-decisions:
  - "Terminology is 'pipeline perspective' not 'caller perspective' — </'>' describes the port from the pipeline's own viewpoint"

patterns-established: []

duration: 5min
started: 2026-04-04
completed: 2026-04-04
---

# Issue #120 Plan 01: Fix IO perspective terminology — Summary

**Replaced misleading "caller perspective" with "pipeline perspective" across 3 files and added a one-line scoping rule clarifying that `<`/`>` always describes the port from the pipeline's own viewpoint.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Started | 2026-04-04 |
| Completed | 2026-04-04 |
| Tasks | 3 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: No "caller perspective" remains | Pass | grep returns 0 matches across all 3 files |
| AC-2: "pipeline perspective" replaces all instances | Pass | 7 matches across all 3 files |
| AC-3: Clarifying rule present in io.md | Pass | One-liner added before direction table in Chain IO Addressing |

## Accomplishments

- Replaced "caller perspective" / "caller-perspective" with "pipeline perspective" / "pipeline-perspective" in all 7 occurrences across 3 files
- Added clarifying one-liner to io.md: "`<` and `>` always describe the port from the pipeline's own viewpoint — `<` marks the pipeline's input, `>` marks its output — whether in a definition, a call site, or a chain step reference."
- Expanded chains.md explanation to explicitly state the viewpoint belongs to the step

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/io.md` | Modified | Added scoping rule + 2 terminology replacements |
| `docs/user/concepts/pipelines/chains.md` | Modified | 3 terminology replacements + expanded explanation |
| `docs/technical/ebnf/10-execution.md` | Modified | 2 terminology replacements |

## Decisions Made

None — followed plan as specified.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #120 fix complete, ready for commit and merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-120-chain-io-scoping, Plan: 01*
*Completed: 2026-04-04*
