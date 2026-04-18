---
phase: 311-foreign-code-permission-compliance
plan: 01
subsystem: permissions
tags: [compile-rules, foreign-code, ast-analysis, permissions, security]

requires:
  - phase: issue-310
    provides: permission-as-resource model ({_} syntax, decomposed fields)
provides:
  - 8 compile rules for foreign code permission compliance (PGE10011-10014, PGW10002/10003/10005/10006)
  - COMPILE-RULES.md index updated with all new entries
affects: [311-02-technical-docs, foreign-code-analysis, io-registry]

tech-stack:
  added: []
  patterns: [foreign-code-compile-rule format with Detection + banned-constructs tables]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE10011-shell-without-capability.md
    - docs/technical/compile-rules/PGE/PGE10012-code-file-outside-scope.md
    - docs/technical/compile-rules/PGE/PGE10013-foreign-resource-outside-scope.md
    - docs/technical/compile-rules/PGE/PGE10014-ast-invisible-foreign-code.md
    - docs/technical/compile-rules/PGW/PGW10002-unverifiable-foreign-io.md
    - docs/technical/compile-rules/PGW/PGW10003-bind-mode-opacity.md
    - docs/technical/compile-rules/PGW/PGW10005-unrecognized-foreign-call.md
    - docs/technical/compile-rules/PGW/PGW10006-shell-variable-expansion.md
  modified:
    - docs/technical/COMPILE-RULES.md

key-decisions:
  - "PGW10004 intentionally skipped (reserved for future use)"
  - "Rule numbering: PGE 9.25-9.28, PGW 9.4-9.7 (continuing from PGE10010=9.24, PGW10001=9.3)"

patterns-established:
  - "Foreign code compile rules include Detection section with algorithm reference"
  - "Banned constructs documented as per-language table in PGE10014"
  - "Confidence levels: literal=error (PGE10013), variable=warning (PGW10002), unknown=warning (PGW10005)"

duration: ~15min
started: 2026-04-18
completed: 2026-04-18
---

# Plan 311-01: Foreign Code Compile Rules Summary

**8 compile rules (4 errors, 4 warnings) formalizing how the Polyglot compiler enforces permission compliance for foreign code in -Run.* pipelines and [C] blocks.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-18 |
| Completed | 2026-04-18 |
| Tasks | 3 completed |
| Files modified | 9 (8 created + 1 updated) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: All 8 compile rule files exist with correct format | Pass | Each has YAML frontmatter, Statement, Rationale, Detection, See also, VALID, INVALID/WARNING |
| AC-2: COMPILE-RULES.md index updated | Pass | 4 PGE entries in "10 — Permissions" section, 4 PGW entries in warnings section |
| AC-3: Cross-references bidirectional | Pass | All rules reference foreign-code.md and enforcement.md; those docs already reference the rule codes |

## Accomplishments

- Created 4 PGE error rules: PGE10011 (Shell Without Capability), PGE10012 (Code File Outside Scope), PGE10013 (Foreign Resource Outside Scope), PGE10014 (AST-Invisible Foreign Code)
- Created 4 PGW warning rules: PGW10002 (Unverifiable Foreign IO), PGW10003 (Bind Mode Opacity), PGW10005 (Unrecognized Foreign Call), PGW10006 (Shell Variable Expansion)
- Updated COMPILE-RULES.md index with all 8 new entries in correct sections

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/compile-rules/PGE/PGE10011-shell-without-capability.md` | Created | -Run.Shell requires {_} #System.#Shell |
| `docs/technical/compile-rules/PGE/PGE10012-code-file-outside-scope.md` | Created | <code.file path must be within {_} .scope |
| `docs/technical/compile-rules/PGE/PGE10013-foreign-resource-outside-scope.md` | Created | AST-detected IO call resource outside {_} scope |
| `docs/technical/compile-rules/PGE/PGE10014-ast-invisible-foreign-code.md` | Created | eval/exec/importlib/ctypes/dlopen/asm banned |
| `docs/technical/compile-rules/PGW/PGW10002-unverifiable-foreign-io.md` | Created | IO call with unresolvable resource argument |
| `docs/technical/compile-rules/PGW/PGW10003-bind-mode-opacity.md` | Created | -Run.*.Bind mode is fully opaque |
| `docs/technical/compile-rules/PGW/PGW10005-unrecognized-foreign-call.md` | Created | Function not in sink table or known-pure list |
| `docs/technical/compile-rules/PGW/PGW10006-shell-variable-expansion.md` | Created | Shell $VAR in IO context prevents verification |
| `docs/technical/COMPILE-RULES.md` | Modified | Added 8 entries to index tables |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| PGW10004 skipped | Reserved for future use, consistent with gaps in other ranges | No impact — numbering gap documented |
| Confidence-level hierarchy | Errors for definite violations (literals), warnings for uncertain (variables, unknowns) | Establishes graduated enforcement pattern |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 8 compile rules in place for plan 311-02 to cross-reference
- PGE10013 and PGW10002/10005 reference `@c:technical/algorithms/foreign-code-analysis` (to be created in 311-02)

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 311-foreign-code-permission-compliance, Plan: 01*
*Completed: 2026-04-18*
