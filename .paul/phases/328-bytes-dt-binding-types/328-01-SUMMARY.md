---
phase: 328-bytes-dt-binding-types
plan: 01
subsystem: docs
tags: [sdk, native-dispatch, marshalling, bytes, datetime]

requires:
  - phase: 319-polyglot-sdk
    provides: polyglot-sdk.md primitive type mapping table
  - phase: 321-run-bridge
    provides: NativeType.md marshalling table and enum
provides:
  - "#bytes binding type in all three marshalling tables"
  - "#dt (epoch seconds) binding type in all three marshalling tables"
  - "NativeType enum byte fields for Rust, Go, JavaScript"
affects: ["322-327 bridge implementations"]

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/user/aj3lib/types/NativeType.md
    - docs/technical/spec/polyglot-sdk.md
    - docs/technical/spec/native-dispatch.md

key-decisions:
  - "#dt uses epoch seconds (integer string), not ISO 8601"
  - "#bytes uses Base64 encoding in JSON envelope"

patterns-established: []

duration: 5min
started: 2026-04-19
completed: 2026-04-19
---

# Issue #328 Plan 01: Add #bytes and #dt binding types Summary

**Added #bytes (Base64 binary) and #dt (epoch seconds) to all three marshalling/encoding tables, plus NativeType enum byte fields for all four languages.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Started | 2026-04-19 |
| Completed | 2026-04-19 |
| Tasks | 3 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: #bytes row in all three tables | Pass | NativeType, SDK, native-dispatch all have consistent #bytes row |
| AC-2: #bytes enum branches in NativeType | Pass | .bytes (Python), .Vec_u8 (Rust), .byte_slice (Go), .Uint8Array (JS) |
| AC-3: #dt row in NativeType and SDK | Pass | Both tables have #dt: Python int, Rust i64, Go int64, JS number |
| AC-4: #dt format updated to epoch seconds | Pass | native-dispatch.md changed from ISO 8601 to epoch seconds |
| AC-5: #dt enum branches in NativeType | Pass | Maps to existing numeric fields (.int, .i64, .int64, .number) |

## Accomplishments

- Added `#bytes` as a first-class binding type across all three marshalling documents
- Updated `#dt` wire format from ISO 8601 to epoch seconds for universal support
- Extended `#NativeType` enum with byte-equivalent fields for Rust (`.Vec_u8`), Go (`.byte_slice`), and JavaScript (`.Uint8Array`)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/aj3lib/types/NativeType.md` | Modified | Added #bytes/#dt marshalling rows; added .Vec_u8, .byte_slice, .Uint8Array enum fields |
| `docs/technical/spec/polyglot-sdk.md` | Modified | Added #bytes/#dt to primitive type mapping table + notes |
| `docs/technical/spec/native-dispatch.md` | Modified | Added #bytes row (Base64); updated #dt from ISO 8601 to epoch seconds |

## Decisions Made

None beyond what the issue specified. Epoch seconds and Base64 encoding were pre-decided in the brainstorm.

## Deviations from Plan

None -- plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All three marshalling tables now agree on the complete set of binding-compatible types
- Bridge implementation issues (#322-#327) can reference these tables

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 328-bytes-dt-binding-types, Plan: 01*
*Completed: 2026-04-19*
