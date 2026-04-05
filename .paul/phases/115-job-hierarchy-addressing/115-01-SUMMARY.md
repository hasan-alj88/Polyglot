---
phase: 115-job-hierarchy-addressing
plan: 01
subsystem: spec
tags: [queue, job-hierarchy, EBNF, metadata-tree]

requires:
  - phase: issue-123
    provides: "#Job struct definition, Job vs Instance terminology"
provides:
  - "Nested [Q] under markers for job-level queue scoping"
  - "=Q.Job.Branch stdlib pipeline stub"
  - "Positional job path grammar in metadata tree"
  - "EBNF grammar update for job-level [Q]"
affects: [queue-manager, compiler, runtime]

tech-stack:
  added: []
  patterns: ["indentation-based [Q] scoping for job-level conditions"]

key-files:
  created: []
  modified:
    - docs/user/concepts/pipelines/queue.md
    - docs/user/stdlib/pipelines/Q.md
    - docs/technical/spec/metadata-tree/branches.md
    - docs/technical/spec/metadata-tree/path-grammar.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/technical/ebnf/10-execution.md

key-decisions:
  - "Nested [Q] under markers extends pipeline-level [Q], does not replace"
  - "Positional paths (r.0, p.1) are compiler-internal only"
  - "=Q.Job.Branch reserved as stub for future branch operations"

patterns-established:
  - "[Q] dual context: pipeline header (all jobs) vs execution body (per-job)"

duration: ~15min
completed: 2026-04-05
---

# Issue #115 Plan 01: Job Hierarchy Addressing Summary

**Nested [Q] scoping under execution markers, positional job paths in metadata tree, and EBNF grammar updates for job-level queue conditions.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 3 completed |
| Files modified | 6 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Nested [Q] under markers documented | Pass | queue.md "Job-Level Queue Conditions" section with full example |
| AC-2: =Q.Job.Branch stdlib pipeline documented | Pass | Q.md "Job Addressing" section |
| AC-3: Positional job paths in metadata tree | Pass | branches.md subsection + path-grammar.md `job_path` production |
| AC-4: EBNF allows nested [Q] in execution section | Pass | `pipeline_call` includes `queue_control_line`; dual context note in 09 |

## Accomplishments

- Documented job-level `[Q]` scoping: nested under `[r]`/`[p]`/`[b]` markers to target specific jobs
- Added `job_path` grammar with `marker_type` (r/p/b) and zero-based index as compiler-internal form
- Updated EBNF `pipeline_call` production to accept `queue_control_line` in execution body
- Added dual-context note to 09-definition-blocks.md explaining `[Q]` in pipeline header vs execution body

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/pipelines/queue.md` | Modified | Added "Job-Level Queue Conditions" subsection with example |
| `docs/user/stdlib/pipelines/Q.md` | Modified | Added "Job Addressing" section with `=Q.Job.Branch` |
| `docs/technical/spec/metadata-tree/branches.md` | Modified | Added "Job Positional Addressing" subsection |
| `docs/technical/spec/metadata-tree/path-grammar.md` | Modified | Added `job_path`, `marker_type` grammar productions |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | Added dual-context note for `[Q]` |
| `docs/technical/ebnf/10-execution.md` | Modified | Added `queue_control_line` to `pipeline_call`; job-level `[Q]` note + example |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Nested [Q] extends, not replaces | Consistent with how pipeline-level defaults work; PGE01013 handles contradictions | No new compile rules needed |
| Positional paths compiler-internal | Fragile for users (reordering breaks); users access via `.jobs:UID` | Clean separation of user vs compiler concerns |
| =Q.Job.Branch as stub | Full branch-level operations not yet needed; nested [Q] covers current use cases | Future work when branch operations are designed |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 1 | Minor — also updated 10-execution.md (not in original files_modified) |

The EBNF change required updating both 09-definition-blocks.md (dual context note) and 10-execution.md (pipeline_call production + example). The plan listed only 09; both were necessary.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Job hierarchy addressing syntax fully documented
- Ready for `/paul:merge` to close issue #115

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 115-job-hierarchy-addressing, Plan: 01*
*Completed: 2026-04-05*
