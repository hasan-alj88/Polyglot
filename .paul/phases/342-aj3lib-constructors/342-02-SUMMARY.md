---
phase: 342-aj3lib-constructors
plan: 02
subsystem: aj3lib
tags: [constructors, regex, mime, duration, aj3lib, compile-time]

requires:
  - phase: 342-aj3lib-constructors-01
    provides: "Constructor directory, doc patterns, INDEX.md registry"
provides:
  - "#Re type definition with PgRegex native validation"
  - "#MIME type definition with .type/.subtype fields"
  - "$Re constructor (1 native-validation overload)"
  - "$MIME constructor (1 string-parsing overload)"
  - "$Dur constructor (6 duration-format overloads with -Dur.Convert)"
  - "-Re.Parse, -MIME.Parse, -Dur.Parse runtime pipelines"
affects: [342-03 Tier 2b constructors, 343 compile error codes, 344 docs updates]

tech-stack:
  added: []
  patterns: ["Native validation overload pattern ($Re)", "Native pipeline conversion pattern ($Dur via -Dur.Convert)"]

key-files:
  created:
    - docs/user/aj3lib/types/Re.md
    - docs/user/aj3lib/types/MIME.md
    - docs/user/aj3lib/constructors/Re.md
    - docs/user/aj3lib/constructors/MIME.md
    - docs/user/aj3lib/constructors/Dur.md
    - docs/user/aj3lib/pipelines/Re.Parse.md
    - docs/user/aj3lib/pipelines/MIME.Parse.md
    - docs/user/aj3lib/pipelines/Dur.Parse.md
  modified:
    - docs/user/aj3lib/constructors/INDEX.md
    - docs/user/aj3lib/pipelines/DT/INDEX.md

key-decisions:
  - "$Re uses native validation pattern — ($) regex is permissive (.+), PgRegex native class validates at compile time"
  - "$Dur uses [-] -Dur.Convert native pipeline for hours/minutes/seconds→Duration conversion"
  - "#Re is a struct with .pattern#RawString, not a ##String subtype — regex cannot validate regex"
  - "#MIME has .type and .subtype fields only — no .parameters (RFC 6838 extensions deferred)"

patterns-established:
  - "Native validation overload: ($) regex permissive, native class validates domain-specific syntax at compile time"
  - "Native pipeline conversion: [-] call inside {$} converts captures to target type when field mapping requires computation"

duration: ~10min
started: 2026-04-22
completed: 2026-04-22
---

# Plan 342-02: Tier 2a {$} Constructors ($Re, $MIME, $Dur) Summary

**Defined $Re (1 overload), $MIME (1 overload), and $Dur (6 overloads) constructors with companion parse pipelines, establishing two new constructor patterns: native validation and native pipeline conversion**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Started | 2026-04-22 |
| Completed | 2026-04-22 |
| Tasks | 3 completed |
| Files modified | 10 (8 created + 2 edited) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: #Re type definition | Pass | {#} #Re with .pattern#RawString, PgRegex native class, ##Scalar schema |
| AC-2: #MIME type definition | Pass | {#} #MIME with .type#string + .subtype#string, well-known values table |
| AC-3: $Re constructor defined | Pass | 1 overload with native validation note, usage examples |
| AC-4: $MIME constructor defined | Pass | 1 overload with / separator structural integrity note |
| AC-5: $Dur constructor defined | Pass | 6 overloads (HMS, HM, MS, H, M, S) with [-] -Dur.Convert |
| AC-6: Parse pipelines defined | Pass | -Re.Parse, -MIME.Parse, -Dur.Parse with {N} defs and error types |
| AC-7: Registry updated | Pass | INDEX.md lists 5 constructors; DT/INDEX.md references -Dur.Parse |

## Accomplishments

- Created `#Re` aj3lib type with native validation pattern — PgRegex validates regex syntax at compile time (not regex-on-regex)
- Created `#MIME` aj3lib type as a two-field struct (.type + .subtype) for type-safe media type handling
- Defined `$Dur` constructor with 6 duration-format overloads using the native pipeline conversion pattern via `-Dur.Convert`
- Established two new constructor patterns beyond the basic string-parsing pattern from Tier 1

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/aj3lib/types/Re.md` | Created | #Re type — regex pattern with PgRegex native class |
| `docs/user/aj3lib/types/MIME.md` | Created | #MIME type — .type + .subtype media type struct |
| `docs/user/aj3lib/constructors/Re.md` | Created | $Re constructor — 1 native-validation overload |
| `docs/user/aj3lib/constructors/MIME.md` | Created | $MIME constructor — 1 string-parsing overload |
| `docs/user/aj3lib/constructors/Dur.md` | Created | $Dur constructor — 6 overloads with -Dur.Convert |
| `docs/user/aj3lib/pipelines/Re.Parse.md` | Created | -Re.Parse runtime pipeline |
| `docs/user/aj3lib/pipelines/MIME.Parse.md` | Created | -MIME.Parse runtime pipeline |
| `docs/user/aj3lib/pipelines/Dur.Parse.md` | Created | -Dur.Parse runtime pipeline |
| `docs/user/aj3lib/constructors/INDEX.md` | Modified | Added $Re, $MIME, $Dur to registry (5 total) |
| `docs/user/aj3lib/pipelines/DT/INDEX.md` | Modified | Added -Dur.Parse cross-reference in Construction section |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| #Re as struct, not ##String subtype | Regex cannot be validated by regex; needs native parser | PgRegex native class validates at compile time |
| $Re ($) regex is permissive ".+" | Actual validation by native class, not capture regex | Establishes native validation overload pattern |
| $Dur uses [-] -Dur.Convert | Hours/minutes→seconds requires arithmetic; can't do field-by-field mapping | Same native pipeline pattern as $DT"Now" |
| #MIME omits .parameters | Keep simple for constructor use; RFC 6838 parameters are runtime concern | Future parse pipeline could handle full MIME with params |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Tier 2b constructors ($Ver, $URL, $IP, $Color) can proceed in plan 342-03
- All three constructor patterns now established: basic string-parsing (Tier 1), native validation ($Re), native pipeline conversion ($Dur)
- Parse pipeline doc pattern well-established for remaining types

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 342-aj3lib-constructors, Plan: 02*
*Completed: 2026-04-22*
