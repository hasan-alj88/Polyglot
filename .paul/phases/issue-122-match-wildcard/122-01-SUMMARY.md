---
phase: issue-122-match-wildcard
plan: 01
subsystem: docs
tags: [conditionals, wildcard, match, ebnf, edge-cases]

requires:
  - phase: none
    provides: existing docs with dual wildcard forms
provides:
  - Unified *? wildcard syntax across all docs
affects: []

tech-stack:
  added: []
  patterns: ["*? is the only wildcard form — verbose and match use identical syntax"]

key-files:
  created: []
  modified:
    - docs/user/concepts/conditionals.md
    - docs/technical/ebnf/11-control-flow.md
    - docs/technical/edge-cases/11-control-flow.md
    - docs/technical/compile-rules/PGE/PGE06014-wildcard-only-match.md
    - docs/technical/compile-rules/PGE/PGE06009-conditional-missing-comparison-operator.md

key-decisions:
  - "Standardize on *? everywhere — no bare * in match arms"
  - "Extended scope to PGE06009 and PGE06014 compile rules (discovered during verification)"

patterns-established:
  - "*? reads as 'Otherwise?' — one wildcard form for all conditional contexts"

duration: 5min
started: 2026-04-04
completed: 2026-04-04
---

# Issue #122 Plan 01: Standardize *? wildcard catch-all Summary

**Unified wildcard catch-all to `*?` in all contexts — eliminated bare `*` from match syntax so one form works everywhere.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Tasks | 3 planned + 2 bonus |
| Files modified | 5 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Match syntax uses *? not * | Pass | conditionals.md match examples updated |
| AC-2: EBNF grammar reflects *? in match arms | Pass | match_arm production uses "*?" |
| AC-3: Edge cases use *? in match examples | Pass | 5 replacements in edge-cases/11-control-flow.md |
| AC-4: No stale references to dual-form wildcard | Pass | `grep '[?] *[^?]' docs/` = zero hits |

## Accomplishments

- Standardized `*?` as the single wildcard form across verbose and match conditionals
- Updated EBNF grammar production to reflect unified syntax
- Eliminated desugar note (`[?] * desugars to [?] *?`) — no longer needed

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/conditionals.md` | Modified | Match example `*` to `*?`, rewrite dual-form note, fix "no `*`" reference |
| `docs/technical/ebnf/11-control-flow.md` | Modified | Grammar production `"*"` to `"*?"`, replace desugar note |
| `docs/technical/edge-cases/11-control-flow.md` | Modified | 5 match wildcard examples `*` to `*?` |
| `docs/technical/compile-rules/PGE/PGE06014-wildcard-only-match.md` | Modified | Statement + examples: bare `*` to `*?` |
| `docs/technical/compile-rules/PGE/PGE06009-conditional-missing-comparison-operator.md` | Modified | Match example `*` to `*?` |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| *? everywhere, no bare * | User: "*? reads as 'Otherwise?'" — one mnemonic, one form | Eliminates dual-form confusion flagged by NotebookLM audit |
| Extended scope to compile rules | Final grep found PGE06009/PGE06014 had bare * | Complete fix — zero stale references remain |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 2 files | Essential — would have left stale bare `*` in compile rules |

### Scope Additions

**1. PGE06014-wildcard-only-match.md**
- **Found during:** Final cross-docs verification grep
- **Issue:** Statement and examples used bare `*` in match context
- **Fix:** Updated 4 occurrences to `*?`

**2. PGE06009-conditional-missing-comparison-operator.md**
- **Found during:** Same verification grep
- **Issue:** Match example used bare `*`
- **Fix:** Updated 1 occurrence to `*?`

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #122 fully resolved — ready for commit and merge

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-122-match-wildcard, Plan: 01*
*Completed: 2026-04-04*
