---
phase: issue-92-datatype-edge-case-audit
plan: 92-02
subsystem: language-spec
tags: [types, compile-rules, edge-cases, regex, ###None]

requires:
  - phase: 92-01
    provides: 18 edge cases documented in EDGE-CASES.md §24, pglib type files split

provides:
  - Resolved 4 edge cases with spec corrections and new compile rules
  - PGE11005 (Final Field Override via Inheritance)
  - PGE04021 (Empty String on Non-None Type)
  - ###None as third field type
  - 0D array semantics

affects: [compile-rules, type-system]

tech-stack:
  added: []
  patterns: [###None nullable field type, 0D scalar container]

key-files:
  created:
    - .paul/phases/issue-92-datatype-edge-case-audit/92-02-PLAN.md
  modified:
    - docs/user/syntax/types.md
    - docs/user/pglib/types/scalars.md
    - docs/user/pglib/types/boolean.md
    - docs/technical/COMPILE-RULES.md
    - docs/technical/EDGE-CASES.md

key-decisions:
  - "#Dimension regex stores D suffix: ^[0-9]+D$ not ^[0-9]+$"
  - "###None is third field type for nullable — empty string only valid for ###None types"
  - "PGE04021 rejects empty string on non-###None types"
  - "PGE11005 prevents overriding << final fields via <~ inheritance"
  - "0D array = scalar container, direct access, PGE04017 on index"
  - "#Dataframe status remains TBD"

patterns-established:
  - "###None: nullable field type — no fields, empty string is only valid value"

duration: ~30min
completed: 2026-03-28
---

# Issue #92 Plan 02: Resolve Datatype Edge Cases — Summary

**Reviewed all 18 EC-24.x edge cases; resolved 4 with regex fixes, 2 new PGEs, and ###None field type; 13 clean, 1 deferred.**

## Performance

| Metric | Value |
|--------|-------|
| Completed | 2026-03-28 |
| Tasks | 6 completed |
| Files modified | 5 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: All 18 edge cases reviewed | Pass | 4 fixed, 13 clean, 1 TBD |
| AC-2: #Dimension regex consistent | Pass | `"^[0-9]+D$"` in both files |
| AC-3: PGE11005 in COMPILE-RULES.md | Pass | Index + detailed rule section |
| AC-4: ###None in types.md | Pass | Field types table, inference rules, hierarchy |
| AC-5: PGE04021 in COMPILE-RULES.md | Pass | Index entry added |
| AC-6: 0D array semantics | Pass | Multidimensional Arrays section updated |
| AC-7: EDGE-CASES.md updated | Pass | 4 edge cases marked RESOLVED |

## Accomplishments

- Added `###None` as third `###` field type — nullable types where empty string `""` is the only valid value
- Added PGE11005 (Final Field Override via Inheritance) — prevents child types from overriding `<<` final fields
- Added PGE04021 (Empty String on Non-None Type) — enforces `###None` exclusivity for empty strings
- Fixed #Dimension regex to `"^[0-9]+D$"` across authoritative spec and pglib
- Documented 0D array semantics: scalar container, direct access, no indexing

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/types.md` | Modified | ###None field type, #Dimension regex, 0D array semantics, hierarchy |
| `docs/user/pglib/types/scalars.md` | Modified | #Dimension regex + examples |
| `docs/user/pglib/types/boolean.md` | Modified | #None definition with ###None |
| `docs/technical/COMPILE-RULES.md` | Modified | PGE11005 + PGE04021 |
| `docs/technical/EDGE-CASES.md` | Modified | EC-24.3/7/10/13 marked RESOLVED |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| #Dimension stores D suffix | User confirmed D is part of stored value, not syntax sugar | Regex `"^[0-9]+D$"` |
| ###None as third field type | #None needs explicit classification; empty string must be opt-in | PGE04021 enforces |
| PGE11005 for final inheritance | `<<` finality must extend through `<~` inheritance chain | Prevents silent overrides |
| 0D = scalar container | Consistent with %##Depth.Max = 0; no indexing needed | PGE04017 on index attempt |
| #Dataframe remains TBD | User deferred decision | No changes to pglib |

## Deviations from Plan

None — reviewed all 18 edge cases as planned, applied fixes for the 4 that needed them.

## Edge Case Disposition

| Edge Case | Title | Status |
|-----------|-------|--------|
| EC-24.1 | #String `.re` default | Clean |
| EC-24.2 | #Int leading zeros | Clean |
| EC-24.3 | #Dimension 0D | **Fixed** — regex |
| EC-24.4 | #Eng exponent | Clean |
| EC-24.5 | #KeyString excluded chars | Clean |
| EC-24.6 | #NestedKeyString allows dot/colon | Clean |
| EC-24.7 | `<~` inheritance finality | **Fixed** — PGE11005 |
| EC-24.8 | #Boolean dual schema | Clean |
| EC-24.9 | Enum inheritance | Clean |
| EC-24.10 | #None minimal type | **Fixed** — ###None + PGE04021 |
| EC-24.11 | Parameterized inheritance | Clean |
| EC-24.12 | ##Contiguous vs ##Sparse | Clean |
| EC-24.13 | 0D array | **Fixed** — semantics documented |
| EC-24.14 | Empty collections | Clean |
| EC-24.15 | Invalid key type | Clean |
| EC-24.16 | #Serial no schema | Clean |
| EC-24.17 | #Dataframe status | TBD (deferred) |
| EC-24.18 | Stale notation | Already fixed by 92-01 |

## Next Phase Readiness

**Ready:**
- All actionable edge cases resolved
- Issue #92 ready for `/paul:merge`

**Concerns:**
- #Dataframe (EC-24.17) deferred — needs future decision

**Blockers:**
- None

---
*Phase: issue-92-datatype-edge-case-audit, Plan: 92-02*
*Completed: 2026-03-28*
