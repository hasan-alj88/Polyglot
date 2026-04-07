---
phase: 152-continue-collector-classification
plan: 01
subsystem: docs
tags: [error-handling, collectors, compiler-rules, design-change]

requires: []
provides:
  - "Compiler-enforced exhaustive error handling model"
  - "*Continue removal across all docs"
  - "PGE02005 rewrite: Failed must resolve to Final"
  - "PGE02007 retired"
affects: []

tech-stack:
  added: []
  patterns:
    - "Compiler-enforced error handling: every failable call must have [!] or <!/> ! fallback"

key-files:
  created: []
  modified:
    - docs/technical/compile-rules/PGE/PGE02005-failed-is-terminal.md
    - docs/technical/compile-rules/PGW/PGW02004-failed-variable-usage.md
    - docs/user/concepts/errors.md
    - docs/user/concepts/pipelines/error-handling.md
    - docs/technical/ebnf/12-collections.md
    - docs/technical/COMPILE-RULES.md
    - docs/technical/compile-rules/PGE/PGE04009-unhandled-serial-struct-conversion.md
    - docs/user/syntax/types/conversions.md
    - docs/technical/brainstorming/serial-to-struct-matching.md
    - docs/technical/compile-rules/PGE/PGE04007-invalid-path-string.md
    - docs/technical/compile-rules/PGE/PGE07002-chain-error-scoping.md
    - docs/INDEX.md
    - docs/user/pglib/INDEX.md
    - docs/user/PGLIB.md
    - docs/technical/spec/type-identity.md

key-decisions:
  - "ALL errors must be compiler-enforced handled — design change from opt-in to mandatory"
  - "*Continue removed entirely — both >FallBack and >IsFailed obsolete"
  - "Failed state kept as runtime trigger; compiler ensures exhaustive handling at compile time"
  - "<!/> ! fallback operators are primary recovery alongside [!] blocks"
  - "[>] <! placement: under [=] output lines, not inside [!] blocks"

patterns-established:
  - "Error handling pattern: [!] block replacement OR [>] <! fallback on IO lines"

duration: 15min
started: 2026-04-07
completed: 2026-04-07
---

# Issue #152 Plan 01-03: *Continue Removal & Compiler-Enforced Error Handling Summary

**Removed *Continue collector entirely; established compiler-enforced exhaustive error handling model where every failable call must have [!] block or <!/> ! fallback.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | 15min |
| Tasks | 12 completed (across 3 plans) |
| Files deleted | 2 |
| Files modified | 15 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: *Continue files deleted | Pass | Continue.md and PGE02007 removed |
| AC-2: Error model compiler-enforced | Pass | PGE02005 rewritten |
| AC-3: Error patterns table updated | Pass | 3-pattern table in errors.md |
| AC-4: EBNF grammar updated | Pass | error_operator removed from 12-collections.md |

## Accomplishments

- Removed *Continue collector and PGE02007 compile rule (2 files deleted)
- Rewrote PGE02005: "Failed is terminal" → "Failed must resolve to Final; compiler enforces exhaustive handling"
- Updated 15 files to replace all *Continue references with <!/> ! fallback operators
- Fixed <! fallback placement to follow io.md convention (under [=] output lines)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| collectors/Continue.md | Deleted | *Continue collector removed |
| PGE02007-continue-after-error.md | Deleted | Rule retired |
| PGE02005-failed-is-terminal.md | Rewritten | Compiler-enforced exhaustive handling |
| PGW02004-failed-variable-usage.md | Rewritten | Updated for new model |
| errors.md | Edited | 3-pattern table, no *Continue |
| error-handling.md | Edited | No *Continue, compiler note added |
| 12-collections.md | Edited | EBNF error_operator removed |
| COMPILE-RULES.md | Edited | PGE02007 marked retired |
| PGE04009, PGE04007, PGE07002 | Edited | *Continue → <! fallback |
| conversions.md, serial-to-struct-matching.md | Edited | *Continue → <! fallback |
| INDEX.md, pglib/INDEX.md, PGLIB.md | Edited | *Continue entries removed |
| type-identity.md | Edited | *Continue → <! fallback |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| ALL errors compiler-enforced | User design decision — shift from opt-in to mandatory | Every failable call must have handler |
| *Continue fully removed | Replaced by <!/> ! fallback operators | Both >FallBack and >IsFailed gone |
| Failed state kept | Runtime trigger for fallback operators | Compiler ensures resolution at compile time |
| [>] <! under [=] lines | Matches existing io.md convention | Consistent with all existing examples |

## Deviations from Plan

### Auto-fixed Issues

**1. <! fallback placement**
- **Found during:** User review
- **Issue:** Initially placed [>] <! inside [!] blocks
- **Fix:** Moved to under [=] output lines per io.md convention
- **Files:** PGE07002, PGE04009, PGE04007, conversions.md, serial-to-struct-matching.md
- **Verification:** All examples match io.md pattern

## Next Phase Readiness

**Ready:**
- Issue #152 complete, ready for commit and merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 152-continue-collector-classification, Plan: 01*
*Completed: 2026-04-07*
