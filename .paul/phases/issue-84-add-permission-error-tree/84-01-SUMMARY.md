---
phase: issue-84-add-permission-error-tree
plan: 01
subsystem: stdlib
tags: [permissions, errors, error-tree, runtime]

requires:
  - phase: issue-80
    provides: _ permission prefix and [_] block marker
  - phase: issue-82
    provides: %_ metadata tree branch for permissions
provides:
  - "!Permission error namespace with 8 .Denied leaves"
  - "Pipeline error associations for =File.Text.Read/Write/Append"
affects: []

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/user/stdlib/errors/errors.md
    - docs/user/concepts/errors.md

key-decisions:
  - "Trimmed tree: only .Denied leaves; overlapping .NotFound/.Timeout stay in existing namespaces"
  - "Explicit pipeline associations: File IO pipelines declare [=] !Permission.File.Denied"
  - "Alias deferred to issue #87 (multi-alias support for [%] .alias)"
  - "Fixed pre-existing bugs: missing !Math in concepts table, missing !Validation in inline list"

patterns-established: []

duration: 5min
completed: 2026-03-25
---

# Issue #84 Plan 01: Add !Permission.* Error Tree Summary

**Added `!Permission` error namespace with 8 `.Denied` leaves for runtime system denials, plus File IO pipeline error associations.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-25 |
| Tasks | 3 completed |
| Files modified | 2 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: `{!} !Permission` block in stdlib errors | Pass | 8 `.Denied` leaves: File, Web, Database, System, Crypto, IPC, Device, Memory |
| AC-2: Pipeline error associations updated | Pass | `=File.Text.Read/Write/Append` all declare `[=] !Permission.File.Denied` |
| AC-3: Concepts errors updated | Pass | Count → six, inline list includes all 6, table has all 6 rows |
| AC-4: Pre-existing bugs fixed | Pass | Added missing `!Math` to table, `!Validation` to inline list |

## Accomplishments

- Added `{!} !Permission` block with 8 `.Denied` leaves matching the 8 permission categories from `[_]` system
- Added `[=] !Permission.File.Denied` to all 3 File IO pipeline error associations
- Fixed pre-existing omissions in concepts/errors.md (missing `!Math` row in table, missing `!Validation` in inline list)
- Created issue #87 for multi-alias `[%] .alias` support (deferred from this issue)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/stdlib/errors/errors.md` | Modified | Added `{!} !Permission` block + pipeline error associations |
| `docs/user/concepts/errors.md` | Modified | Updated namespace count, inline list, and summary table |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Trimmed tree to `.Denied` only | `.NotFound`, `.Timeout`, `.Unavailable` already exist in `!File`, `!Timeout` namespaces | No duplication; clean separation of concerns |
| Explicit pipeline associations | IO pipelines should declare all errors they can raise | `=File.Text.*` pipelines now declare permission errors for `[!]` handling |
| Alias deferred to #87 | Current `[%] .alias` only supports single alias; multi-alias needed for `!File.Permission` shorthand | Issue #87 created; no blocking impact |

## Deviations from Plan

### Auto-fixed Issues

**1. Pre-existing bug: Missing `!Math` in concepts table**
- **Found during:** Task 2
- **Issue:** Table at line 184-189 listed 4 namespaces but text said "five" — `!Math` was missing
- **Fix:** Added `!Math` row to table
- **Verification:** Table now has 6 rows matching "six root namespaces"

**2. Pre-existing bug: Missing `!Validation` in inline list**
- **Found during:** Task 2
- **Issue:** Line 28 listed `!File`, `!No`, `!Timeout`, `!Math` but omitted `!Validation`
- **Fix:** Added `!Validation` and `!Permission` to inline list
- **Verification:** Inline list now shows all 6 namespaces

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- `!Permission` error tree complete — ready for `/paul:merge` to close issue #84

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-84-add-permission-error-tree, Plan: 01*
*Completed: 2026-03-25*
