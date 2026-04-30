---
phase: issue-95-96-doc-audit-inconsistencies
plan: 03
subsystem: docs
tags: [datetime, aj3lib, pipelines, compile-rules]
requires:
  - phase: 95-01
    provides: IC-005 fix, #DateTime references valid
  - phase: 95-02
    provides: 55 {#} type definitions
provides:
  - 40 {=} pipeline definitions for =DT.*
  - 3 compile rules (PGE04026-04028)
  - INDEX.md entries for #DateTime and =DT.*
  - Full IC verification sweep
affects: [future aj3lib work, compiler implementation]
key-files:
  created: [docs/user/aj3lib/pipelines/DT.md, docs/technical/compile-rules/PGE/PGE04026-invalid-iana-timezone.md, docs/technical/compile-rules/PGE/PGE04027-missing-required-datetime-subfield.md, docs/technical/compile-rules/PGE/PGE04028-invalid-epoch-value.md]
  modified: [docs/technical/COMPILE-RULES.md, docs/user/aj3lib/INDEX.md]
key-decisions:
  - "=DT.Now is the only pipeline needing _IO.Read (system clock); all others _None"
  - "3 compile rules focused on type validation; runtime rules deferred"
duration: 10min
completed: 2026-04-01
---

# Plan 95-03: =DT.* Pipelines, Compile Rules, and Verification

**Created 40 =DT.* pipeline definitions, 3 compile rules, updated INDEX; all IC items verified**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: All =DT.* pipelines defined | Pass | 40 {=} definitions with [T], [Q], [W], [_], IO |
| AC-2: Compile rules cover #DateTime validation | Pass | PGE04026 (IANA tz), PGE04027 (missing epoch), PGE04028 (epoch range) |
| AC-3: INDEX files updated | Pass | #DateTime in types, =DT.* in pipelines |
| AC-4: All IC items verified | Pass | IC-001-007 resolved, IC-008 Won't Fix |

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/aj3lib/pipelines/DT.md | Created | 40 {=} pipeline definitions |
| docs/technical/compile-rules/PGE/PGE04026-invalid-iana-timezone.md | Created | Invalid IANA timezone string rule |
| docs/technical/compile-rules/PGE/PGE04027-missing-required-datetime-subfield.md | Created | Missing .Instant.epoch rule |
| docs/technical/compile-rules/PGE/PGE04028-invalid-epoch-value.md | Created | Out-of-range epoch literal rule |
| docs/technical/COMPILE-RULES.md | Modified | Added PGE04026-04028 to index table |
| docs/user/aj3lib/INDEX.md | Modified | Added #DateTime type + =DT.* pipeline entries |

## Pipeline Groups

| Group | Count |
|-------|-------|
| Construction | 4 (Now, From.Epoch, From.ISO, From.Parts) |
| Calendar Conversion | 11 (To.Gregorian through To.Custom) |
| Time Unit Conversion | 3 |
| Arithmetic | 3 |
| Comparison | 4 |
| Extraction | 7 |
| Zone | 2 |
| Formatting | 3 |
| Business | 3 |

## Deviations from Plan

| Type | Count | Impact |
|------|-------|--------|
| Minor | 1 | Pipeline count is 40 (plan estimated ~41) — brainstorming has 40 distinct pipelines |

---
*Phase: issue-95-96-doc-audit-inconsistencies, Plan: 03*
*Completed: 2026-04-01*
