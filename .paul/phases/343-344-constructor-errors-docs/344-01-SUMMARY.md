---
phase: 343-344-constructor-errors-docs
plan: 344-01
subsystem: docs
tags: [inline-calls, constructors, infrastructure, Path, DT, edge-cases]

requires:
  - phase: 343-344-constructor-errors-docs
    provides: "PGE14xxx error codes for cross-referencing"
provides:
  - "inline-calls.md rewritten as infrastructure-only reference"
  - "PGE12003/12005/12009 scoped to infrastructure with PGE14xxx cross-refs"
  - "strings.md and edge-cases updated with $Path constructor notation"
  - "constructors.md Compile Rules table with all 11 PGE14xxx codes"
affects: [future-docs-maintenance]

tech-stack:
  added: []
  patterns: [three-context-rule, infrastructure-vs-constructor-distinction]

key-files:
  modified:
    - "docs/technical/compile-rules/PGE/PGE12003-invalid-inline-pipeline-argument.md"
    - "docs/technical/compile-rules/PGE/PGE12005-inline-format-mismatch.md"
    - "docs/technical/compile-rules/PGE/PGE12009-template-type-coercion-failure.md"
    - "docs/user/concepts/pipelines/inline-calls.md"
    - "docs/user/syntax/types/strings.md"
    - "docs/technical/edge-cases/04-type-system.md"
    - "docs/user/syntax/constructors.md"

key-decisions:
  - "Cross-Language Inline Calls section kept with infrastructure-context caller examples"
  - "Path.md and DT/INDEX.md already had Three-Context Rule from #341/#342 — minimal changes needed"

patterns-established:
  - "All execution body inline examples replaced with constructor or infrastructure-context equivalents"

duration: 10min
started: 2026-04-22
completed: 2026-04-22
---

# Plan 344-01: Inline-to-Constructor Documentation Transition Summary

**Completed inline→constructor docs transition: PGE12xxx scoped to infrastructure, inline-calls.md rewritten, strings/edge-cases updated with $Path notation, compile rules table in constructors.md**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Tasks | 3 completed |
| Files modified | 7 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: PGE12003/12005/12009 scoped to infrastructure | Pass | All 3 files mention infrastructure lines + PGE14xxx cross-refs |
| AC-2: inline-calls.md rewritten | Pass | Title updated, Mermaid chart uses -T.Daily, no execution body examples |
| AC-3: Path.md updated | Pass | Already had Three-Context Rule from #341 — verified |
| AC-4: strings.md and edge-cases updated | Pass | $Path constructor notation throughout; 6 edge cases updated |
| AC-5: Constructor compile rules reference | Pass | 11 PGE14xxx codes in constructors.md table |

## Accomplishments

- Rewrote inline-calls.md as infrastructure-only reference with constructor redirect
- Scoped PGE12003/12005/12009 to infrastructure lines with PGE14xxx cross-references
- Updated strings.md: `-Path"..."` → `$Path"..."` constructor notation for execution body
- Updated 6 edge cases (EC-4.5, 4.7, 4.8, 4.9, 4.10, 4.11, 4.12) to infrastructure context
- Added Compile Rules section to constructors.md with all 11 PGE14xxx codes

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/compile-rules/PGE/PGE12003-*.md` | Modified | Scoped to infrastructure + PGE14xxx cross-ref |
| `docs/technical/compile-rules/PGE/PGE12005-*.md` | Modified | Scoped to infrastructure + PGE14xxx cross-ref |
| `docs/technical/compile-rules/PGE/PGE12009-*.md` | Modified | Scoped to infrastructure + PGE14xxx cross-ref |
| `docs/user/concepts/pipelines/inline-calls.md` | Modified | Rewritten as infrastructure-only; Mermaid updated |
| `docs/user/syntax/types/strings.md` | Modified | $Path constructor notation |
| `docs/technical/edge-cases/04-type-system.md` | Modified | 6 edge cases updated |
| `docs/user/syntax/constructors.md` | Modified | Added Compile Rules table |

## Deviations from Plan

- Path.md and DT/INDEX.md required no changes — already updated by #341/#342 with Three-Context Rule tables and constructor references. Verified and confirmed.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- No contradictory inline/constructor examples remain in docs
- Constructor compile rules discoverable from constructors.md
- Infrastructure inline calls preserved with clear scoping

**Concerns:** None.
**Blockers:** None.

---
*Phase: 343-344-constructor-errors-docs, Plan: 344-01*
*Completed: 2026-04-22*
