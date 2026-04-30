---
phase: 321-run-bridge
plan: 01
subsystem: aj3lib
tags: [bridge, cross-language, native-type, variable, conversion]

requires:
  - phase: issue-319
    provides: Polyglot SDK specification
  - phase: issue-281
    provides: -Run.* input binding and compile rules
provides:
  - "#NativeType enum and marshalling table"
  - "#Variable language-tagged variable type"
  - "-Variable.Convert pipeline"
  - "-Run.Bridge.Function pipeline definition"
  - "-Run.Bridge.Script pipeline definition"
  - "Bridge Conversion Algorithm"
affects: [321-02, implementation-issues]

tech-stack:
  added: []
  patterns: ["dual-wrapper ;Caller;Callee syntax", "language-tagged #Variable"]

key-files:
  created:
    - docs/user/aj3lib/types/NativeType.md
    - docs/user/aj3lib/types/Variable.md
    - docs/user/aj3lib/pipelines/Variable/Convert.md
    - docs/user/aj3lib/pipelines/Run/Bridge.Function.md
    - docs/user/aj3lib/pipelines/Run/Bridge.Script.md
    - docs/technical/algorithms/bridge-conversion.md
  modified:
    - docs/user/aj3lib/pipelines/Run/INDEX.md

key-decisions:
  - "#NativeType uses per-language enum variants (NativeType.Python.list, NativeType.Rust.Vec)"
  - "#Variable carries .name, .value, .type — language-tagged wrapper for cross-boundary data"
  - "Dual-wrapper syntax: ;Caller;Callee on [-] call line"
  - "PGE01041-01043 compile rules for Bridge validation"

patterns-established:
  - "Dual-env syntax for cross-language pipelines"
  - "#Variable as universal language-tagged data container"

duration: ~30min
started: 2026-04-19
completed: 2026-04-19
---

# Issue #321 Plan 01: Bridge Types, Pipelines, and Algorithm

**Core Bridge infrastructure: #NativeType, #Variable, -Variable.Convert, -Run.Bridge.Function/Script, and the bridge-conversion algorithm.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~30min |
| Started | 2026-04-19 |
| Completed | 2026-04-19 |
| Tasks | 3 completed |
| Files modified | 7 (6 created, 1 updated) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: #NativeType and #Variable types | Pass | NativeType.md and Variable.md created with full definitions |
| AC-2: -Variable.Convert pipeline | Pass | Convert.md with marshalling table and conversion algorithm |
| AC-3: Bridge pipeline definitions | Pass | Bridge.Function.md and Bridge.Script.md with {N} definitions, IO, errors |
| AC-4: Bridge conversion algorithm | Pass | bridge-conversion.md with dual-wrapper lifecycle and conversion flow |
| AC-5: Run INDEX updated | Pass | Bridge entries added to INDEX.md |

## Accomplishments

- Created #NativeType enum with per-language type variants for Python, Rust, Go, JavaScript
- Created #Variable as the universal language-tagged data container for cross-boundary exchange
- Defined -Run.Bridge.Function and -Run.Bridge.Script with dual-wrapper ;Caller;Callee syntax
- Documented the full bridge conversion algorithm including dual-wrapper lifecycle

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/aj3lib/types/NativeType.md | Created | Per-language native type enum for marshalling |
| docs/user/aj3lib/types/Variable.md | Created | Language-tagged variable type (.name, .value, .type) |
| docs/user/aj3lib/pipelines/Variable/Convert.md | Created | Cross-language variable conversion pipeline |
| docs/user/aj3lib/pipelines/Run/Bridge.Function.md | Created | Cross-language function call pipeline |
| docs/user/aj3lib/pipelines/Run/Bridge.Script.md | Created | Cross-language variable binding pipeline |
| docs/technical/algorithms/bridge-conversion.md | Created | Bridge conversion algorithm and dual-wrapper lifecycle |
| docs/user/aj3lib/pipelines/Run/INDEX.md | Modified | Added Bridge entries to pipeline index |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| #NativeType per-language variants | Each language has unique type names; per-language enum enables compile-time validation | Type mapping is extensible per language |
| #Variable carries source language type | Enables -Variable.Convert to determine conversion path | Conversion is explicit, not inferred |
| Dual-wrapper ;Caller;Callee syntax | Both language environments must be active for Bridge to work | PGE01041 validates different languages |
| PGE01041-01043 compile rules | Bridge-specific validation beyond single-language rules | Compile-time safety for cross-language calls |

## Deviations from Plan

None -- plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- All Bridge infrastructure in place for plan 02 (examples, cross-refs, GitHub issues)
- Type definitions and pipeline specs stable

**Concerns:** None
**Blockers:** None

---
*Phase: 321-run-bridge, Plan: 01*
*Completed: 2026-04-19*
