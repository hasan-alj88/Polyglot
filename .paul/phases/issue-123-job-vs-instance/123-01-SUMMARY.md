---
phase: issue-123-job-vs-instance
plan: 01
subsystem: docs
tags: [job, instance, metadata-tree, glossary, pglib-types]

requires:
  - phase: none
    provides: existing glossary, structs.md, branches.md, data-is-trees.md
provides:
  - "{#} #Job pglib struct type"
  - "Instance glossary entry"
  - "Pipeline Branch metadata tree spec with .jobs:UID"
  - "Job vs Instance clarification in data-is-trees.md"
affects: [queue-manager, compiler, runtime]

tech-stack:
  added: []
  patterns: ["UID-keyed jobs under sequential pipeline instances"]

key-files:
  created: []
  modified:
    - docs/user/pglib/types/structs.md
    - docs/audit/reference/glossary.md
    - docs/technical/spec/metadata-tree/branches.md
    - docs/user/concepts/data-is-trees.md

key-decisions:
  - "Jobs nest under pipeline instances (%=:Pipeline:N.jobs:UID), not a separate top-level branch"
  - "Jobs are UID-keyed (not sequential), instances are sequential-numbered"
  - "#Job is ##Flat (full runtime struct with 9 fields)"

patterns-established:
  - "Instance = concurrent pipeline run (sequential :N); Job = unit of work within instance (UID-keyed)"

duration: ~10min
started: 2026-04-05
completed: 2026-04-05
---

# Issue #123 Plan 01: Job vs Instance Terminology Summary

**Defined the relationship between Job (UID-keyed work unit) and Instance (sequential pipeline run), added {#} #Job struct, Pipeline Branch spec, and glossary entries.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Started | 2026-04-05 |
| Completed | 2026-04-05 |
| Tasks | 3 completed |
| Files modified | 4 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: #Job struct type defined | Pass | 9 fields, Fields table, context note in structs.md |
| AC-2: Glossary distinguishes Job from Instance | Pass | Both entries with cross-references in "NOT this" column |
| AC-3: Jobs nest under pipeline instances | Pass | Pipeline Branch in branches.md + clarification in data-is-trees.md |

## Accomplishments

- Defined `{#} #Job` as full runtime struct with PID, status, pipeline, queue, parent, hasChildren, hierarchy, marker, killPropagation
- Added Pipeline Branch (`%=`) to metadata tree spec showing `.jobs:UID#Job` structure
- Clarified: Instance = the Nth concurrent run (`:0`, `:1`, `:2`); Job = unit of work within instance (UID-keyed at IO boundaries)
- Eliminated the old conflating glossary definition ("single enqueued instance of a pipeline")

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/types/structs.md` | Modified | Added `## #Job` section with definition, Fields table, context note |
| `docs/audit/reference/glossary.md` | Modified | Revised "Job" entry, added "Instance" entry with cross-references |
| `docs/technical/spec/metadata-tree/branches.md` | Modified | Added `## Pipeline Branch` with `.jobs:UID#Job` structure |
| `docs/user/concepts/data-is-trees.md` | Modified | Added clarification paragraph after Instance numbering text |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Jobs under pipeline instances, not top-level | Keeps tree consistent — all branches use `%{type}:{ref}:{instance}` pattern | Queue system references `%=:Pipeline:N.jobs:UID` |
| #Job as ##Flat with full runtime fields | User chose full runtime struct over minimal OS-only | #Job captures everything needed for OS control + queue management |
| UID-keyed jobs, sequential instances | TM generates UIDs; metadata tree uses sequential :N | Two identification schemes now formally documented |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Job/Instance terminology is unambiguous across all docs
- #Job type available for queue-manager and compiler references
- Pipeline Branch spec provides foundation for runtime implementation

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-123-job-vs-instance, Plan: 01*
*Completed: 2026-04-05*
