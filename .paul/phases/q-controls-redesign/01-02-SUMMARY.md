---
phase: q-controls-redesign
plan: 02
subsystem: docs
tags: [queue, aj3lib, Q-pipelines, per-pipeline-specs, deprecation]

requires:
  - phase: q-controls-redesign/01-01
    provides: Rewritten queue.md and INDEX.md with new naming and structure

provides:
  - 4 deprecated old spec files with @d: markers
  - 13 new per-pipeline spec files for redesigned -Q.* system
  - 9 updated existing spec files with new naming and links

affects: [01-03 technical docs]

tech-stack:
  added: []
  patterns: [per-pipeline spec pattern with Resource Effects and Timing Variants sections]

key-files:
  created:
    - docs/user/aj3lib/pipelines/Q/Job.Pause.Free.CPU.md
    - docs/user/aj3lib/pipelines/Q/Job.Pause.Free.RAM.md
    - docs/user/aj3lib/pipelines/Q/Job.Pause.Free.All.md
    - docs/user/aj3lib/pipelines/Q/Job.Kill.WithCleanup.md
    - docs/user/aj3lib/pipelines/Q/Job.Kill.Now.md
    - docs/user/aj3lib/pipelines/Q/Job.Resume.md
    - docs/user/aj3lib/pipelines/Q/Job.Throttle.md
    - docs/user/aj3lib/pipelines/Q/Job.Snapshot.md
    - docs/user/aj3lib/pipelines/Q/Job.Inspect.md
    - docs/user/aj3lib/pipelines/Q/Job.Reassign.md
    - docs/user/aj3lib/pipelines/Q/Queue.Drain.md
    - docs/user/aj3lib/pipelines/Q/Queue.Flush.md
    - docs/user/aj3lib/pipelines/Q/DoNothing.md
  modified:
    - docs/user/aj3lib/pipelines/Q/Pause.Soft.md
    - docs/user/aj3lib/pipelines/Q/Pause.Hard.md
    - docs/user/aj3lib/pipelines/Q/Kill.Graceful.md
    - docs/user/aj3lib/pipelines/Q/Kill.Hard.md
    - docs/user/aj3lib/pipelines/Q/Resume.md
    - docs/user/aj3lib/pipelines/Q/Reassign.md
    - docs/user/aj3lib/pipelines/Q/Drain.md
    - docs/user/aj3lib/pipelines/Q/Flush.md
    - docs/user/aj3lib/pipelines/Q/Priority.Update.md
    - docs/user/aj3lib/pipelines/Q/Default.md
    - docs/user/aj3lib/pipelines/Q/Assign.md
    - docs/user/aj3lib/pipelines/Q/Dispatch.Wait.TimeOut.md
    - docs/user/aj3lib/pipelines/Q/Job.Branch.md

key-decisions:
  - "Resume.md, Reassign.md, Drain.md, Flush.md, Priority.Update.md deprecated with @d: markers pointing to new scoped names"
  - "Dispatch.Wait.TimeOut redesigned to show #QueueRules conditional model instead of pipeline variants"
  - "New spec files include Resource Effects and Timing Variants sections beyond base pattern"

patterns-established:
  - "Deprecated files keep original content intact below @d: notice"
  - "New pause specs document both .Now/.Wait timing and .Soft/.Hard precision variants"
  - "Compiler safety rules documented for Snapshot and Free.All"

duration: ~15min
started: 2026-04-15
completed: 2026-04-15
---

# Phase q-controls-redesign Plan 02: Per-Pipeline Specs Summary

**Deprecated 4 old spec files, created 13 new per-pipeline specs for redesigned -Q.Job/Queue system, and updated 9 existing specs with new naming and deprecation markers.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-15 |
| Completed | 2026-04-15 |
| Tasks | 3 completed |
| Files created | 13 |
| Files modified | 13 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Old files deprecated | Pass | 4 files have status: deprecated + @d: markers pointing to replacements |
| AC-2: New per-pipeline specs created | Pass | 13 new files with valid frontmatter, spec pattern, no Unix internals |
| AC-3: Existing files updated | Pass | 9 files updated with new naming, deprecation markers, and Related links to new files |

## Accomplishments

- Deprecated Pause.Soft, Pause.Hard, Kill.Graceful, Kill.Hard with `@d:` pointers to new equivalents
- Created 13 new spec files covering all redesigned action pipelines (Pause.Free.CPU/RAM/All, Kill.WithCleanup/Now, Resume, Throttle, Snapshot, Inspect, Reassign, Queue.Drain, Queue.Flush, DoNothing)
- Updated 9 existing files: Resume, Reassign, Drain, Flush, Priority.Update marked deprecated; Default, Assign, Job.Branch updated with new links; Dispatch.Wait.TimeOut fully redesigned into `#QueueRules` model
- All 27 Q/*.md files verified to have valid `audience: automation-builder` frontmatter

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 5 | Resume, Reassign, Drain, Flush, Priority.Update also deprecated (plan said "update" but they have new scoped replacements) |

**Total impact:** Essential — these files have new scoped replacements (Job.Resume, Job.Reassign, Queue.Drain, Queue.Flush) so deprecation markers are correct. The original names no longer exist in the redesigned system.

## Issues Encountered

None.

## Skill Audit

No required skills configured in SPECIAL-FLOWS.md. Audit: N/A.

## Next Phase Readiness

**Ready:**
- All per-pipeline specs in place for the redesigned -Q.* system
- Plan 01-03 (technical docs) can proceed — user docs foundation complete

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: q-controls-redesign, Plan: 02*
*Completed: 2026-04-15*
