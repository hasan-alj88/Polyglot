---
phase: issue-116-direction-reversal
plan: 01
subsystem: docs
tags: [operators, EBNF, naming, push-pull]

requires:
  - phase: none
    provides: n/a
provides:
  - Unified operator naming (PushLeft/PushRight) across all docs
  - Resolved io.md Push/Pull reversal contradiction
affects: [any future operator documentation, pg:generate, pg:train]

tech-stack:
  added: []
  patterns: [PushLeft/PushRight operator naming convention]

key-files:
  created: []
  modified:
    - docs/technical/ebnf/06-operators.md
    - docs/technical/ebnf/04-type-system.md
    - docs/technical/ebnf/08-expressions.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/technical/ebnf/10-execution.md
    - docs/user/syntax/operators.md
    - docs/user/concepts/variable-lifecycle.md
    - docs/user/syntax/io.md
    - docs/technical/compile-rules/PGE/PGE02010-discard-default-assignment.md
    - docs/technical/edge-cases/08-expressions.md

key-decisions:
  - "Operator names center on Push + Direction — Pull removed as operator name"
  - "Pull remains valid as a verb (pulling from source) — only formal operator names changed"
  - "Rule names using push/pull as verbs (Push-Once, Pull Isolation) left unchanged"

patterns-established:
  - "All assignment operators named Push + direction: PushLeft, PushRight, DefaultPushLeft, etc."

duration: 15min
started: 2026-04-04
completed: 2026-04-04
---

# Issue #116 Plan 01: Rename Assignment Operator Names Summary

**Renamed all 6 assignment operators from Push/Pull to PushLeft/PushRight, resolving the io.md direction contradiction and eliminating naming ambiguity across 10 files.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-04 |
| Completed | 2026-04-04 |
| Tasks | 5 completed |
| Files modified | 10 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: No `final_push`/`final_pull` in docs/ | Pass | Grep returns 0 matches |
| AC-2: No `Push (Final)`/`Pull (Final)` in docs/ | Pass | Grep returns 0 matches |
| AC-3: New EBNF names in all 5 files | Pass | 13 matches across 5 EBNF files |
| AC-4: New display names in user docs | Pass | 12 matches across 3 user doc files |
| AC-5: io.md table consistent | Pass | Headers and descriptions use push-left/push-right |

## Accomplishments

- Renamed 6 EBNF operator symbols: `push_left`, `push_right`, `default_push_left`, `default_push_right`, `fallback_push_left`, `fallback_push_right`
- Updated 3 user-facing operator tables with new display names (PushLeft, PushRight, DefaultPushLeft, etc.)
- Fixed the io.md summary table (the primary bug) — column headers and cell descriptions now consistent
- Updated compile rule and edge case comments to use new operator names

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/ebnf/06-operators.md` | Modified | Primary EBNF definitions — 6 symbols + assignment_op rule |
| `docs/technical/ebnf/04-type-system.md` | Modified | `default_push` → `default_push_left` in macro_param |
| `docs/technical/ebnf/08-expressions.md` | Modified | 4 symbol refs in assignment_expr rule |
| `docs/technical/ebnf/09-definition-blocks.md` | Modified | `final_push` → `push_left` in import_line |
| `docs/technical/ebnf/10-execution.md` | Modified | `fallback_push` → `fallback_push_left` in chain_io_line |
| `docs/user/syntax/operators.md` | Modified | 6 operator display names updated |
| `docs/user/concepts/variable-lifecycle.md` | Modified | 6 operator type/reading columns updated |
| `docs/user/syntax/io.md` | Modified | Summary table headers + descriptions fixed |
| `docs/technical/compile-rules/PGE/PGE02010-discard-default-assignment.md` | Modified | 4 comment labels updated |
| `docs/technical/edge-cases/08-expressions.md` | Modified | 4 comment labels updated |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Center all operator names on "Push" + direction | Both `<<` and `>>` push into target — direction distinguishes them | Eliminates Push/Pull ambiguity permanently |
| Keep "Pull" as verb, remove as operator name | Pulling from source is a real action, but not the operator's identity | Rule names like "Pull Isolation" and "Push-Once" unchanged |
| No change to io.md chain IO table (lines 94-98) | "Push into"/"Pull from" describe caller-perspective IO actions, not operator names | Verb usage preserved |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- All 10 files updated and verified
- Ready for commit and merge

**Concerns:**
- Memory files (`pg_lesson_*.md`) reference old Push/Pull terminology — should be updated if training is re-run

**Blockers:**
- None

---
*Phase: issue-116-direction-reversal, Plan: 01*
*Completed: 2026-04-04*
