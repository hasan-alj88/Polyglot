---
phase: issue-112-reconcile-c-inline-code
plan: 01
subsystem: docs
tags: [ebnf, block-elements, foreign-code, rt-pipelines]

requires: []
provides:
  - Unified [C] uppercase foreign code element
  - =RT.* inline code integration pattern
affects: [issue-78-rt-pipelines, issue-111-compiler-rules]

tech-stack:
  added: []
  patterns: [C] inline code via =RT.* pipeline <script input

key-files:
  modified:
    - docs/user/syntax/blocks.md
    - docs/technical/ebnf/05-block-elements.md
    - docs/technical/compile-rules/PGE/PGE01027-empty-foreign-code.md
    - docs/user/concepts/permissions.md
    - docs/technical/edge-cases/10-execution.md
    - docs/technical/ebnf/11-control-flow.md

key-decisions:
  - "Unified [c] → [C] uppercase: consistent with [T]/[W]/[Q] convention"
  - "Removed #Code:Language:Version header: language determined by =RT.* pipeline"
  - "Simplified EBNF: removed foreign_code_header, kept foreign_code_line only"

duration: ~10min
completed: 2026-03-31
---

# Issue #112 Plan 01: Reconcile [C] Inline Code Summary

**Unified `[c]` → `[C]` uppercase foreign code element across 6 files; removed `#Code:` header in favour of `=RT.*` pipeline language selection**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Completed | 2026-03-31 |
| Tasks | 3 completed (1 checkpoint + 2 auto) |
| Files modified | 6 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: [C] uppercase everywhere | Pass | `grep -r '\[c\]' docs/` = zero (excl. archive/brainstorming) |
| AC-2: Foreign Code section rewritten | Pass | blocks.md shows `[C]` with `=RT.Python.Script` example |
| AC-3: EBNF grammar updated | Pass | `foreign_code_elem ::= "[C]"` |
| AC-4: PGE01027 updated | Pass | Examples use `[C]` in `=RT.*` context |
| AC-5: No stale [c] references | Pass | permissions.md, 10-execution.md, 11-control-flow.md all updated |

## Accomplishments

- Unified `[c]` → `[C]` across all active documentation (6 files)
- Removed obsolete `#Code:Language:Version` header pattern — language now determined by `=RT.*` pipeline
- Simplified EBNF grammar: removed `foreign_code_header` production, kept only `foreign_code_line`
- Updated PGE01027 examples to show `[C]` within `=RT.*` pipeline context

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/blocks.md` | Modified | Rewrote Foreign Code section with `[C]` + `=RT.*` example |
| `docs/technical/ebnf/05-block-elements.md` | Modified | `[c]` → `[C]` in element grammar |
| `docs/technical/compile-rules/PGE/PGE01027-empty-foreign-code.md` | Modified | Updated rule text and examples |
| `docs/user/concepts/permissions.md` | Modified | `[c]` → `[C]`, updated example with `=RT.*` |
| `docs/technical/edge-cases/10-execution.md` | Modified | Updated EC-10.16 edge case |
| `docs/technical/ebnf/11-control-flow.md` | Modified | Simplified §11.6 EBNF, removed header grammar |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Unify to `[C]` uppercase | Consistent with `[T]`, `[W]`, `[Q]`; brainstorming resolved this | All foreign code uses single `[C]` element |
| Remove `#Code:Language:Version` header | Language determined by `=RT.*` pipeline name | Simpler syntax, no header declaration needed |
| Simplify EBNF | Header production no longer needed | `foreign_code_block` = just `foreign_code_line+` |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- `[C]` is now the single, unified foreign code element
- Issue #112 ready to merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-112-reconcile-c-inline-code, Plan: 01*
*Completed: 2026-03-31*
