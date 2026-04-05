---
phase: issue-140-permission-object-block
plan: 03
subsystem: docs
tags: [permissions, compile-rules, stdlib, grammar]

requires:
  - phase: issue-140-permission-object-block/01
    provides: "User-facing {_} permission object docs"
  - phase: issue-140-permission-object-block/02
    provides: "EBNF grammar + metadata tree for {_}"
provides:
  - "Compile rules PGE10003-10006 + PGW10001 rewritten for {_} objects"
  - "Stdlib permission references updated across 7 files"
affects: ["issue-140 merge"]

key-files:
  modified:
    - docs/technical/compile-rules/PGE/PGE10003-unknown-permission-category.md
    - docs/technical/compile-rules/PGE/PGE10004-undeclared-permission.md
    - docs/technical/compile-rules/PGE/PGE10005-permission-output.md
    - docs/technical/compile-rules/PGE/PGE10006-duplicate-permission.md
    - docs/technical/compile-rules/PGW/PGW10001-unused-permission.md
    - docs/technical/COMPILE-RULES.md
    - docs/user/stdlib/pipelines/DT.md
    - docs/user/stdlib/pipelines/File.md
    - docs/user/stdlib/pipelines/W.md
    - docs/user/stdlib/pipelines/T.md
    - docs/user/stdlib/pipelines/RT.md
    - docs/user/stdlib/pipelines/Sys.md
    - docs/user/stdlib/pipelines/#.md

key-decisions:
  - "PGE10005 renamed from 'Permission Output' to 'Invalid Permission Block Marker'"
  - "DT.md [_] _None lines removed (pure computation = no permission line)"
  - "DT.md =DT.Now requires System.Env (not _IO.Read — IO is not a category)"
  - "Stdlib {N} defs don't have [_] lines — capabilities are compiler-internal"

duration: ~15min
completed: 2026-04-05
---

# Issue #140 Plan 03: Compile Rules + Stdlib Permission Refs Summary

**Rewrote 5 compile rules and 7 stdlib permission sections for {_} named permission object system.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Completed | 2026-04-05 |
| Tasks | 2 completed |
| Files modified | 13 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Compile rules use {_} named object syntax | Pass | All 5 rules rewritten with {_} blocks, [.] field lines, [_] _ObjectName references |
| AC-2: Stdlib permission tables use Category.Capability format | Pass | All 7 files updated; old _Category.subfield and Inline/IO columns removed |
| AC-3: DT.md [_] lines removed from {N} definitions | Pass | ~40 [_] _None removed, [_] _IO.Read removed; table shows System.Env |
| AC-4: RT.md example code uses named {_} objects | Pass | Both examples now show {_} ceiling + grant blocks with [_] references |

## Accomplishments

- PGE10003: Now validates `[.] .Category.Capability` field lines in `{_}` blocks (not inline `[_]` declarations)
- PGE10004: Now checks `[_] _ObjectName` references resolve to `{_}` objects granting needed capabilities
- PGE10005: Renamed to "Invalid Permission Block Marker" — validates `{_}` blocks contain only `[.]` lines
- PGE10006: Now covers two scopes — duplicate `[_]` references AND duplicate capabilities within `{_}` blocks
- PGW10001: Now warns when referenced `{_}` object's capabilities go unexercised
- COMPILE-RULES.md index updated with PGE10005 name change
- All 7 stdlib files use `Category.Capability` format in permission tables
- DT.md cleaned of ~40 `[_] _None` lines and 1 `[_] _IO.Read`
- RT.md examples now demonstrate full {_} ceiling/grant + {@}/{=} reference pattern

## Deviations from Plan

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 2 | Minor scope additions |

**1. COMPILE-RULES.md index update**
- Not in plan's files_modified list but required for PGE10005 name change
- One-line edit: "Permission Output" → "Invalid Permission Block Marker"

**2. DT.md See Also reference update**
- Bottom of file had `[_] declarations` text → changed to `{_} permission objects`
- Discovered during verification grep

## Next Phase Readiness

**Ready:** All 3 plans complete — issue #140 is ready for /paul:merge
**Concerns:** None
**Blockers:** None

---
*Phase: issue-140-permission-object-block, Plan: 03*
*Completed: 2026-04-05*
