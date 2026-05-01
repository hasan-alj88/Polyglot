---
phase: 341-constructor-blocks
plan: 01
subsystem: language-design
tags: [constructor, block-type, inline-pipeline, error-philosophy]

requires:
  - phase: brainstorming-session-2026-04-21-001
    provides: "{$} design decisions, syntax, overload resolution"
provides:
  - "{$} constructor specification (constructors.md)"
  - "Block registry updated with {$}, ($) dual-context, [$]"
  - "Inline calls scoped to infrastructure lines"
  - "Constructor glossary entry"
affects: [341-02-ebnf-metadata, 342-jm3lib-constructors, 343-compile-rules, 344-doc-updates]

tech-stack:
  added: []
  patterns: ["three-context rule (infrastructure/constructor/pipeline)"]

key-files:
  created: [docs/user/syntax/constructors.md]
  modified: [docs/user/syntax/blocks.md, docs/user/concepts/pipelines/inline-calls.md, docs/audit/reference/glossary.md, docs/user/SPEC-INDEX.md]

key-decisions:
  - "($) has dual context: variable-scope accessor under (-) $Label, constructor IO under {$}"
  - "%InlineString retired from {-} pipelines; only {T}/{Q}/{W} keep it"
  - "Scoped extension model (C): cross-package constructors visible via [@] imports"

patterns-established:
  - "Three-context rule: infrastructure inline, body constructor, body pipeline call"
  - "($) IO + [$] action + [.] field mapping ordering in {$} blocks"

duration: ~15min
completed: 2026-04-22
---

# Issue #341 Plan 01: {$} Constructor Specification Summary

**Created {$} constructor block spec — compile-time guaranteed Final values with no error surface, replacing inline pipelines in execution body**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Completed | 2026-04-22 |
| Tasks | 3 completed |
| Files modified | 5 (1 created, 4 modified) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Constructor specification exists | Pass | constructors.md with 15 sections |
| AC-2: Block type registry updated | Pass | {$}, ($) dual-context, [$] in blocks.md |
| AC-3: Inline calls scoped to infrastructure | Pass | Header + %InlineString scoped |
| AC-4: Glossary and index updated | Pass | Constructor entry + SPEC-INDEX 4b link |

## Accomplishments

- Created full {$} constructor specification: syntax, contract, overload resolution, interpolation rules, cross-package scoping, three-context rule
- Documented ($) dual-context disambiguation (variable-scope vs constructor IO) — existing ($) usage preserved
- Scoped inline-calls.md to infrastructure lines only with redirect to constructors.md
- No boundary violations: zero PGE/PGW codes, zero jm3lib definitions

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/syntax/constructors.md | Created | Full {$} constructor specification |
| docs/user/syntax/blocks.md | Modified | Added {$}, ($) dual-context, [$] entries; %$ in metadata list |
| docs/user/concepts/pipelines/inline-calls.md | Modified | Scoped to infrastructure lines; %InlineString noted as {T}/{Q}/{W}-only |
| docs/audit/reference/glossary.md | Modified | Added Constructor entry |
| docs/user/SPEC-INDEX.md | Modified | Added constructors.md at position 4b |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| ($) dual-context rather than new marker | ($) already used for variable-scope; context disambiguates (under {$} vs (-) $Label) — same pattern as [*] dual meaning | No new marker needed |
| %InlineString retired from {-} only | Triggers/queues/wrappers still need inline config; only execution body use is replaced | Clean separation; #344 can update jm3lib docs |
| Scoped extension model (C) | [@] imports already scope visibility; ambiguity caught at import site | No orphan rule needed |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- constructors.md provides the authoritative spec for plan 341-02 (EBNF + metadata tree)
- Child issues #342, #343, #344 can proceed independently after 341-02

**Concerns:**
- inline-calls.md still contains execution-body examples (e.g., cross-language section) — #344 should clean these up

**Blockers:**
- None

---
*Phase: 341-constructor-blocks, Plan: 01*
*Completed: 2026-04-22*
