---
phase: issue-74-block-markers
plan: 01
subsystem: docs
tags: [block-markers, EBNF, conditionals, language-design]

requires:
  - phase: none
    provides: n/a
provides:
  - "[+] line continuation marker in spec"
  - "[|] OR logical marker (replacing [+])"
  - "[c] foreign code injection marker in spec"
affects: ["compile-rules migration (Plan 02)", "EDGE-CASES update"]

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/user/syntax/blocks.md
    - docs/user/concepts/conditionals.md
    - docs/technical/EBNF.md

key-decisions:
  - "Reordered Logical table: [&] AND, [|] OR, [-] NOT, [^] XOR"
  - "Line Continuation and Foreign Code as separate sections after Logical"

patterns-established:
  - "Escape literal pipes in markdown table cells with backslash"

duration: ~5min
completed: 2026-03-24
---

# Issue #74 Plan 01: Core Spec Files — Block Marker Updates

**Reassigned `[+]` from OR to line continuation, introduced `[|]` as OR, and added `[c]` foreign code injection across blocks.md, conditionals.md, and EBNF.md.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-03-24 |
| Tasks | 3 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Block registry updated | Pass | `[+]` = continuation, `[\|]` = OR, `[c]` = foreign code; new sections added |
| AC-2: Conditionals use `[\|]` for OR | Pass | Section, example, and exhaustiveness table all migrated |
| AC-3: EBNF grammar defines all three | Pass | `logical_or` uses `[\|]`, new §11.5 continuation, §11.6 foreign code |

## Accomplishments

- Reassigned `[+]` from OR to line continuation in all three core spec files
- Introduced `[\|]` as the dedicated OR logical operator
- Added `[c]` foreign code injection with `#Code:<Language>:<Version>` header pattern
- Added EBNF grammar rules for continuation (`continuation_line`) and foreign code (`foreign_code_block`)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/blocks.md` | Modified | Updated Logical table, added Line Continuation and Foreign Code sections |
| `docs/user/concepts/conditionals.md` | Modified | Migrated OR section and examples from `[+]` to `[\|]` |
| `docs/technical/EBNF.md` | Modified | Updated `logical_elem`/`logical_or`, added `continuation_elem`, `foreign_code_elem`, §11.5, §11.6 |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Placed Line Continuation and Foreign Code as separate sections after Logical | They are distinct categories, not logical operators | Clear taxonomy in blocks.md |
| Added `continuation_elem` and `foreign_code_elem` to block_element alternatives | EBNF must list all valid block elements | Grammar completeness |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Core spec files updated — blocks.md, conditionals.md, EBNF.md consistent
- Plan 02 can proceed with compile rules migration (`[+]` → `[\|]` in PGE files, algorithms, EDGE-CASES)

**Concerns:**
- ~7 compile-rule and algorithm files still reference `[+]` as OR — must be migrated in Plan 02

**Blockers:**
- None

---
*Phase: issue-74-block-markers, Plan: 01*
*Completed: 2026-03-24*
