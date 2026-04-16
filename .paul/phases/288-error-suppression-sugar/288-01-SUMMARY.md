---
phase: 288-error-suppression-sugar
plan: 01
subsystem: language-design
tags: [error-handling, syntax-sugar, ebnf, compile-rules]

requires:
  - phase: none
    provides: existing error handling docs and EBNF grammar
provides:
  - "[!] !*- syntax sugar formalized in EBNF"
  - "PGW07010 compile warning for consumed outputs under suppress"
  - "User-facing documentation for error suppression sugar"
affects: []

tech-stack:
  added: []
  patterns:
    - "error_wildcard as distinct EBNF production (not error_id)"
    - "bodyless error block alternative (error_suppress)"

key-files:
  created:
    - docs/technical/compile-rules/PGW/PGW07010-suppress-on-consumed-output.md
  modified:
    - docs/technical/ebnf/11-control-flow.md
    - docs/user/concepts/errors.md
    - docs/technical/COMPILE-RULES.md

key-decisions:
  - "error_wildcard is a separate production from error_id — !* is not a dotted_name"
  - "error_suppress is bodyless (no exec_line children) — distinct alternative in error_block"

patterns-established: []

duration: ~5min
started: 2026-04-16
completed: 2026-04-16
---

# Issue #288 Plan 01: Error Suppression Sugar Summary

**Added `[!] !*-` bodyless syntax sugar for fire-and-forget error suppression, formalized `error_wildcard` in EBNF, and created PGW07010 warning for consumed outputs under suppress.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Started | 2026-04-16 |
| Completed | 2026-04-16 |
| Tasks | 3 completed |
| Files modified | 4 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF grammar formalizes error wildcard and suppress syntax | Pass | `error_wildcard` and `error_suppress` productions added to Section 11.2 |
| AC-2: User docs explain [!] !*- with example and distinction | Pass | New section in errors.md with issue example and complementary tools note |
| AC-3: PGW07010 compile warning exists and is indexed | Pass | Rule file created, COMPILE-RULES.md updated |
| AC-4: errors.md compile rules table includes PGW07010 | Pass | Row added after PGW07004 |

## Accomplishments

- Formalized `error_wildcard` (`!*`) as a distinct EBNF production separate from `error_id` — previously used by convention but not in the grammar
- Added `error_suppress` (`[!] !*-`) as a bodyless alternative in `error_block`, with desugaring rule to `[!] !* / [-] -DoNothing`
- Created PGW07010 with VALID/WARNING examples, fix options, and diagnostic message
- Documented the complementary relationship between `(<) !<` (input-level) and `[!] !*-` (call-level)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/ebnf/11-control-flow.md` | Modified | Added `error_wildcard`, `error_suppress` productions and 2 rules |
| `docs/user/concepts/errors.md` | Modified | New "Error Suppression Sugar" section + PGW07010 in compile rules table |
| `docs/technical/compile-rules/PGW/PGW07010-suppress-on-consumed-output.md` | Created | Warning rule for suppress on consumed output |
| `docs/technical/COMPILE-RULES.md` | Modified | Added PGW07010 to warnings index |

## Decisions Made

None — followed plan as specified.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #288 fully addressed — ready for commit and merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 288-error-suppression-sugar, Plan: 01*
*Completed: 2026-04-16*
