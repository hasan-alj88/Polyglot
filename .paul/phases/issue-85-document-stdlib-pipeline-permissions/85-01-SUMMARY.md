---
phase: issue-85-document-stdlib-pipeline-permissions
plan: 01
subsystem: stdlib
tags: [permissions, stdlib, pipelines, documentation]

requires:
  - phase: issue-80-84
    provides: Permission system ([_] blocks, categories, compile rules, !Permission error tree)
provides:
  - Permission declarations in all 7 stdlib pipeline reference files
  - Permission column in stdlib INDEX.md
affects: []

tech-stack:
  added: []
  patterns: ["## Permissions section in stdlib pipeline files", "Permission column in INDEX.md"]

key-files:
  created: []
  modified:
    - docs/user/stdlib/pipelines/File.md
    - docs/user/stdlib/pipelines/T.md
    - docs/user/stdlib/pipelines/W.md
    - docs/user/stdlib/pipelines/Math.md
    - docs/user/stdlib/pipelines/Path.md
    - docs/user/stdlib/pipelines/Sys.md
    - docs/user/stdlib/pipelines/Q.md
    - docs/user/stdlib/INDEX.md

key-decisions:
  - "Include full permission map from issue #85, even for pipelines not yet fully specified"
  - "W.Log.Context and W.Polyglot listed as None (no IO); W.File.TempDir added as _File.write"

patterns-established:
  - "## Permissions section pattern: table with Pipeline/Permission/Type columns for IO files; prose for pure files"

duration: 5min
started: 2026-03-25
completed: 2026-03-25
---

# Issue #85 Plan 01: Document Stdlib Pipeline Permission Declarations — Summary

**Added [_] permission declarations to all 7 stdlib pipeline files and Permission column to INDEX.md, completing the permission documentation chain (#80-#85).**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5 min |
| Tasks | 3 completed |
| Files modified | 8 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Permission declarations in pipeline files | Pass | All 7 files have ## Permissions section |
| AC-2: INDEX.md Permission column | Pass | 5-column table with Permission values |
| AC-3: Permission map matches issue specification | Pass | All pipelines from issue map covered |

## Accomplishments

- Added per-pipeline permission tables to File.md (8 rows), T.md (5 rows), W.md (12 rows), Sys.md (1 row)
- Added "No permissions required" to Math.md, Path.md, Q.md
- Added Permission column to INDEX.md pipeline table with `_File.*`, `_System.env`, Mixed, None values
- Added `!Permission` to INDEX.md error namespace table

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/stdlib/pipelines/File.md` | Modified | 8-row permission table for all File operations |
| `docs/user/stdlib/pipelines/T.md` | Modified | 5-row permission table (3 None, 2 IO) |
| `docs/user/stdlib/pipelines/W.md` | Modified | 12-row permission table (2 None, 10 IO) |
| `docs/user/stdlib/pipelines/Math.md` | Modified | No permissions required (pure computation) |
| `docs/user/stdlib/pipelines/Path.md` | Modified | No permissions required (pure computation) |
| `docs/user/stdlib/pipelines/Sys.md` | Modified | 1-row permission table (_System.env) |
| `docs/user/stdlib/pipelines/Q.md` | Modified | No permissions required (queue scheduling) |
| `docs/user/stdlib/INDEX.md` | Modified | Permission column + !Permission in error table |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Include full permission map (option B) | User chose to document all mappings from issue, even for not-yet-specified pipelines | Complete reference; future specs inherit permissions |
| W.Log.Context = None | Structured logging scope is in-process, not IO | Consistent with pure-computation classification |
| W.File.TempDir = _File.write | Creates filesystem directory | Not in original issue map but logically correct |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Minor |
| Scope additions | 0 | — |
| Deferred | 0 | — |

**Total impact:** Minimal — one wrapper not in the original issue map was added.

### Auto-fixed Issues

**1. W.File.TempDir missing from issue map**
- **Found during:** Task 1 (W.md permissions)
- **Issue:** Plan listed `W.File.Lock` but not `W.File.TempDir` which also does filesystem IO
- **Fix:** Added `W.File.TempDir` → `_File.write` (creates temp directory)
- **Verification:** Consistent with File permission category

## Issues Encountered

None

## Next Phase Readiness

**Ready:**
- Permission documentation chain complete (#80 → #81 → #82 → #83 → #84 → #85)
- All stdlib pipeline files now document their permission requirements
- Ready for /paul:merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-85-document-stdlib-pipeline-permissions, Plan: 01*
*Completed: 2026-03-25*
