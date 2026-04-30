---
phase: 285-run-shell-pipeline
plan: 01
subsystem: aj3lib
tags: [run, shell, native, permissions, system-capability]

requires:
  - phase: 284-git-triggers-ci-cd
    provides: "-T.Git.* triggers that depend on -Run.Shell"
provides:
  - "-Run.Shell pipeline specification"
  - "System.Shell permission capability"
  - "Process Lifecycle runtime contract (Redis job:UID:process)"
affects: ["runtime implementation", "queue handler process management"]

tech-stack:
  added: []
  patterns: ["Redis job:{UID}:process key for QH process control"]

key-files:
  created:
    - docs/user/aj3lib/pipelines/Run/Shell.md
    - docs/user/aj3lib/permissions/System/Shell.md
  modified:
    - docs/user/aj3lib/pipelines/Run/INDEX.md
    - docs/user/concepts/permissions/capability-enums.md
    - docs/user/aj3lib/permissions/System/INDEX.md
    - docs/user/aj3lib/permissions/INDEX.md

key-decisions:
  - "Shell is language-agnostic (no <Lang> placeholder) — uses -W.Polyglot"
  - "System.Shell is a separate capability from System.Process (higher privilege)"
  - "Process info stored in Redis as job:{UID}:process for QH cross-node management"

patterns-established:
  - "Native functions that spawn child processes write to job:{UID}:process in Redis"
  - "QH uses SIGKILL/SIGTERM/SIGSTOP/SIGCONT via Redis-stored PID for process control"

duration: ~15min
started: 2026-04-16
completed: 2026-04-16
---

# Issue #285 Plan 01: -Run.Shell Pipeline Specification

**Language-agnostic shell command execution pipeline with System.Shell capability, Process Lifecycle runtime contract, and QH signal-based process management via Redis.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-16 |
| Completed | 2026-04-16 |
| Tasks | 2 completed |
| Files modified | 6 (2 created, 4 modified) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Shell pipeline documented | Pass | {N} definition, 5 IO lines, -W.Polyglot, dual permissions |
| AC-2: Run INDEX updated | Pass | Pipelines table (5 rows), Permissions table, Compiler Validation table, Shell vs CLI note |
| AC-3: System.Shell capability exists | Pass | #SystemCapability has 4 capabilities; __System.Shell generic + both INDEX files updated |

## Accomplishments

- Created `-Run.Shell` pipeline spec with {N} native definition, Shell vs CLI comparison table, and full example (git status pipeline)
- Added `System.Shell` as 4th capability to `#SystemCapability` enum with `__System.Shell` permission generic
- Documented Process Lifecycle section: Redis `job:{UID}:process` key structure, QH signal mapping (SIGKILL/SIGTERM/SIGSTOP/SIGCONT)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/aj3lib/pipelines/Run/Shell.md` | Created | Full pipeline spec: definition, IO, Shell vs CLI, example, Process Lifecycle, permissions, errors, metadata |
| `docs/user/aj3lib/permissions/System/Shell.md` | Created | `__System.Shell` capability-level permission generic |
| `docs/user/aj3lib/pipelines/Run/INDEX.md` | Modified | Added Shell to Pipelines, Permissions, Compiler Validation tables; updated intro text |
| `docs/user/concepts/permissions/capability-enums.md` | Modified | Added `Shell` to `#SystemCapability` enum |
| `docs/user/aj3lib/permissions/System/INDEX.md` | Modified | Added `__System.Shell` row |
| `docs/user/aj3lib/permissions/INDEX.md` | Modified | Added `__System.Shell` row to master table |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Shell is language-agnostic (no `<Lang>`) | System shell is not tied to a language runtime | Simpler IO (no `<env`, no `<code`); uses -W.Polyglot |
| System.Shell separate from System.Process | Shell interpretation = higher privilege than known binary | Two capabilities required; permission escalation explicit |
| Process info in Redis `job:{UID}:process` | QH needs cross-node process management | Same pattern for all native functions that spawn child processes |
| QH uses OS signals for process control | Direct SIGKILL/SIGTERM/SIGSTOP/SIGCONT | Maps cleanly to -Q.Kill.*/Pause.*/Resume operations |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 1 | Essential — Process Lifecycle section added per user request |

### Scope Addition

**1. Process Lifecycle section in Shell.md**
- **Reason:** User requested documentation of how the async Rust function exposes PID to QH via Redis
- **Added:** Redis key structure (`job:{UID}:process`), QH signal mapping table, spawn/completion lifecycle
- **Impact:** Establishes a runtime contract pattern reusable by `-Run.<Lang>.CLI` and future native functions

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- `-Run.Shell` fully specified — can be referenced by `-T.Git.Hook` and other automation pipelines
- Process Lifecycle pattern documented — applicable to all process-spawning native functions

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 285-run-shell-pipeline, Plan: 01*
*Completed: 2026-04-16*
