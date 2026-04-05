---
phase: 135-error-extensibility-contradiction
plan: 01
subsystem: docs
tags: [error-handling, extensibility, ebnf, metadata-tree]

requires:
  - phase: none
    provides: n/a
provides:
  - Consistent documentation of {!} implicit !Error: nesting rule
  - Complete stdlib error namespace table (9 namespaces)
  - EBNF grammar supporting both [.] and [:] in error definitions
affects: []

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/user/concepts/errors.md
    - docs/user/stdlib/errors/errors.md
    - docs/technical/ebnf/09-definition-blocks.md

key-decisions:
  - "User {!} !Name implicitly creates !Error:Name.* in the metadata tree"
  - "Stdlib error namespaces are 9, not 7 (was missing !Field, !Alias, !RT)"

patterns-established:
  - "Error extensibility: only !Error uses [:] flexible children; all others use [.] fixed leaves"

duration: 5min
started: 2026-04-05
completed: 2026-04-05
---

# Issue #135 Plan 01: Error Extensibility Contradiction Fix — Summary

**Clarified that user `{!} !Name` implicitly nests under `!Error:Name.*` in the metadata tree; fixed misleading examples and incomplete stdlib namespace table across 3 files.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Started | 2026-04-05 |
| Completed | 2026-04-05 |
| Tasks | 3 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: User {!} examples show implicit !Error: nesting | Pass | `{!} !Validation` replaced with `{!} !Error` using `[:]` children in concepts/errors.md |
| AC-2: Stdlib namespace table is complete | Pass | Table now has 10 rows (9 stdlib + 1 user-extensible); added !Field, !Alias, !RT |
| AC-3: EBNF grammar models user !Error: extension | Pass | `error_body_line` now includes `[:]` flexible_field alternative; dual examples added |
| AC-4: Stdlib errors.md aligns with concepts doc | Pass | "Defining Custom Errors" section now shows `{!} !Error` with `[:]` children; metadata path corrected |

## Accomplishments

- Replaced misleading `{!} !Validation` user example with `{!} !Error` using `[:]` extensible branches in both user-facing docs
- Added 3 missing stdlib namespaces (!Field, !Alias, !RT) to Standard Error Trees table — now complete at 10 rows
- Extended EBNF `error_body_line` grammar to allow `[:]` flexible_field for `!Error` extension, with PGE05001 sibling homogeneity note

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/errors.md` | Modified | Replaced {!} !Validation example with {!} !Error using [:]; completed stdlib table 7→10 rows |
| `docs/user/stdlib/errors/errors.md` | Modified | Updated "Defining Custom Errors" section to show {!} !Error pattern; fixed metadata path description |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | Added [:] to error_body_line grammar; dual stdlib/user examples; implicit nesting documentation |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| User `{!} !Name` → `!Error:Name.*` implicit nesting | Confirmed by user — stdlib namespaces are fixed, user errors always nest under !Error | Core design rule now explicitly documented in 3 files |
| Stdlib has 9 namespaces (not 7) | Original table omitted !Field, !Alias, !RT which exist in stdlib/errors/errors.md | Concepts doc now matches stdlib doc |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 3 files consistent on implicit !Error: nesting rule
- path-grammar.md already correct (was verified, not changed)
- Ready for /paul:merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 135-error-extensibility-contradiction, Plan: 01*
*Completed: 2026-04-05*
