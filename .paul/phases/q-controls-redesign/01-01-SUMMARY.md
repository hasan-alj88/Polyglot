---
phase: q-controls-redesign
plan: 01
subsystem: docs
tags: [queue, jm3lib, Q-pipelines, controls-redesign]

requires:
  - phase: brainstorm
    provides: docs/draft.md accepted brainstorm with full -Q.* redesign

provides:
  - Rewritten queue concepts doc (docs/user/concepts/pipelines/queue.md)
  - Rewritten Q pipeline catalog (docs/user/jm3lib/pipelines/Q/INDEX.md)

affects: [01-02 per-pipeline specs, 01-03 technical docs]

tech-stack:
  added: []
  patterns: [three {Q} block types, scoped naming, conditional rule system]

key-files:
  created: []
  modified:
    - docs/user/concepts/pipelines/queue.md
    - docs/user/jm3lib/pipelines/Q/INDEX.md

key-decisions:
  - "Three {Q} block types: #Queue, #QueueRules, #JobRules"
  - "Scope rule: read anything, act only on your scope"
  - "[?]/[&]/[+] conditionals replace pipeline name suffixes"
  - "State guards mandatory — compiler error without them"
  - "(#) parameters with <~ defaults, capturable as $ variable objects"
  - "Free.RAM split: .Soft (memory.high hint) vs .Hard (memory.max guarantee)"
  - "Pause reason set: resume only when ALL rules agree"
  - "Cross-queue constraints moved from pipeline IO to #Queue properties"
  - "Dispatch.Wait.TimeOut redesigned into #QueueRules model"

patterns-established:
  - "Scoped naming: -Q.Job.*, -Q.Host.*, -Q.Queue.*, -Q.Queue.Jobs.*"
  - "Three pipeline kinds: Getter, State Guard, Action"
  - "@d: deprecation markers in tables pointing to old files"

duration: ~45min (across two sessions)
started: 2026-04-15
completed: 2026-04-15
---

# Phase q-controls-redesign Plan 01: Core Docs Rewrite Summary

**Rewrote queue.md concepts file and Q/INDEX.md pipeline catalog with three {Q} block types, scoped -Q.Job/Host/Queue naming, [?]/[&]/[+] conditionals, and full resource freeing spectrum.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~45min (two sessions) |
| Started | 2026-04-15 |
| Completed | 2026-04-15 |
| Tasks | 2 completed |
| Files modified | 2 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Queue concepts uses new three-block model | Pass | Three block types, new naming, [?]/[&]/[+] conditionals, old names only in deprecation section |
| AC-2: INDEX lists new pipeline catalog | Pass | All getter/state/action pipelines listed, composition syntax shown, valid frontmatter, deprecation table |
| AC-3: Audit compliance | Pass | YAML frontmatter, no H4+, code blocks tagged, @-imports with typed prefixes, [[wikilinks]] paired |

## Accomplishments

- Rewrote queue.md with three `{Q}` block types (`#Queue`, `#QueueRules`, `#JobRules`), scoped naming, temporal behavior model, resource freeing spectrum, and default queue behaviors
- Rewrote INDEX.md with complete pipeline catalog organized by kind (getter, state guard, action) and scope (Job, Host, Queue, Queue.Jobs)
- Both files use `[?]`/`[&]`/`[+]` conditional syntax throughout — no old pipeline name suffixes
- Deprecation tables with `@d:` markers link old names to new equivalents

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/pipelines/queue.md` | Rewritten | Three {Q} block types, composition model, temporal behavior, resource freeing spectrum |
| `docs/user/jm3lib/pipelines/Q/INDEX.md` | Rewritten | Full pipeline catalog with getters, state guards, actions, examples, deprecation table |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Audience is automation-builder only | No Unix internals (cgroups, CRIU, SIGSTOP) — those go in Plan 03 technical docs | Clean separation of concerns |
| Kept -Q.Job.Branch as-is | Still serves purpose of naming marker subtrees | No breaking change |
| Dispatch.Wait.TimeOut redesigned into #QueueRules | Fits naturally as queue-level conditional rule | Old file gets @d: marker in Plan 02 |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Skill Audit

No required skills configured in SPECIAL-FLOWS.md. Audit: N/A.

## Next Phase Readiness

**Ready:**
- queue.md and INDEX.md establish all new terminology and structure
- Plans 01-02 and 01-03 can now reference the new naming consistently
- Deprecation table provides mapping for old file updates

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: q-controls-redesign, Plan: 01*
*Completed: 2026-04-15*
