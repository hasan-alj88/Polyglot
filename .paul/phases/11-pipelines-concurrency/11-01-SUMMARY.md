---
phase: 11-pipelines-concurrency
plan: 01
subsystem: docs
tags: [pipelines, concurrency, errors, collections, parallel, chains, auto-wire, expand, collect]

requires:
  - phase: 09-core-language-type-system
    provides: types.md, blocks.md, identifiers.md at status: complete
  - phase: 10-operators-control-flow
    provides: operators.md, conditionals.md at status: complete
provides:
  - pipelines.md promoted to status: complete (PGE-1xx, PGE-8xx compile rules)
  - collections.md promoted to status: complete (PGE-3xx compile rules)
  - errors.md promoted to status: complete (PGE-7xx compile rules)
affects: [12-package-system-aj3lib]

key-files:
  modified:
    - docs/user/concepts/pipelines.md
    - docs/user/concepts/collections.md
    - docs/user/concepts/errors.md

key-decisions:
  - "No new spec files created — Phase 11 completed existing drafts only"
  - "PGE01003 excluded from pipelines.md — belongs in blocks.md/packages.md"

completed: 2026-03-24
---

# Phase 11 Plan 01: Pipelines & Concurrency Spec Completion

**Audited and completed pipelines.md (31 compile rules), collections.md (12 compile rules), and errors.md (12 compile rules) — all promoted to status: complete with full PGE/PGW cross-reference tables.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: pipelines.md complete with PGE-1xx + PGE-8xx | Pass | 27 PGE refs (101-118 minus 103, 801-810) + 4 PGW refs (701, 801, 808, 809); new Execution Rules and Call Site Rules subsections; compile rules table added |
| AC-2: collections.md complete with PGE-3xx | Pass | 10 PGE refs (301-309, 311) + 2 PGW refs (301, 302); new Parallel Boundaries subsection; expand/collect validation notes; compile rules table added |
| AC-3: errors.md complete with PGE-7xx | Pass | 7 PGE refs (701-707) + 5 PGW refs (205, 701-704); missing PGE07001/702/704 integrated inline; compile rules table added |

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/concepts/pipelines.md | Modified | Added PGE-1xx/8xx inline refs, Execution Rules subsection, Call Site Rules section, compile rules table; status → complete |
| docs/user/concepts/collections.md | Modified | Added PGE-3xx inline refs, Parallel Boundaries subsection, expand/collect validation notes, compile rules table; status → complete |
| docs/user/concepts/errors.md | Modified | Added PGE07001/702/704 inline refs, compile rules table; status → complete |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- All pipeline, concurrency, and error semantics fully specified
- Compile rule cross-references in place for Phase 12 (Package System & aj3lib)
- Phase 12 can reference pipelines.md for aj3lib pipeline definitions and errors.md for error tree contracts

**Concerns:**
- EC-6.4 inconsistency (raw arithmetic in EDGE-CASES vs PGE04010) noted in Phase 10 — still unresolved in technical references

**Blockers:**
- None

---
*Phase: 11-pipelines-concurrency, Plan: 01*
*Completed: 2026-03-24*
