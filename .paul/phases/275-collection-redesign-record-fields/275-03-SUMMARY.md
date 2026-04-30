---
phase: "275"
plan: "03"
subsystem: "type-system"
tags: [schema-composition, syntax-update, retirement]
dependency_graph:
  requires: [275-01, 275-02]
  provides: [updated-schema-files, updated-serial-definition, composition-syntax]
  affects: [type-system, collections, syntax-docs]
tech_stack:
  patterns: [direct-property-composition, schema-retirement]
key_files:
  modified:
    - docs/user/aj3lib/types/schemas/Nullable.md
    - docs/user/aj3lib/types/schemas/Inf.md
    - docs/user/aj3lib/types/schemas/Rectangular.md
    - docs/user/aj3lib/types/schemas/Result.md
    - docs/user/aj3lib/types/schemas/String.md
    - docs/user/aj3lib/types/schemas/Array.md
    - docs/user/aj3lib/types/string.md
    - docs/user/aj3lib/types/Serial.md
    - docs/user/concepts/collections/serial.md
    - docs/user/syntax/types/prefix-system.md
decisions:
  - "##Rectangular retired as composable schema; properties stated directly"
  - "#Serial uses %##Depth.Max << #Inf and %##Count << #Inf instead of ##Deep/##Sparse"
  - "##Array uses %##Fields << #Range instead of %##Key << #UnsignedInt"
  - "#Inf alias for #Bound.Inf documented in Inf.md"
metrics:
  duration: "267s"
  completed: "2026-04-09"
---

# Phase 275 Plan 03: Schema Files and Composition Syntax Update Summary

Drop `<<` before `##`/`###` schema composition, retire ##Rectangular/##Deep/##Sparse/##Contiguous, replace with direct %## properties.

## What Changed

### Task 1: Schema files (6 files)
- **Nullable.md, Result.md**: `[#] << ##X` -> `[#] ##X` in usage examples
- **String.md** (schema): `[#] << ##Scalar`, `[#] << ###ScalarValue`, `[#] << ##String` all dropped `<<`
- **Inf.md**: `[#] << ##Int/##Inf` dropped `<<`; added `#Inf` alias note for `#Bound.Inf`
- **Rectangular.md**: Rewrote as retirement notice with migration table
- **Array.md**: Replaced `##Contiguous`/`##Rectangular` composition with direct properties (`%##Gap`, `%##Ordered`, `%##Regular`, `%##Depth.Max`, `%##Propagate`); replaced `%##Key << #UnsignedInt` with `%##Fields << #Range`

### Task 2: aj3lib type and concept docs (3 files)
- **string.md** (type): `[#] << ##Scalar` -> `[#] ##Scalar`; `[#] << ##String` -> `[#] ##String`
- **Serial.md**: Replaced `##Deep`/`##Sparse` with direct `%##Depth.Max << #Inf`, `%##Gap << #True`; `.Inf` -> `#Inf`
- **serial.md** (concepts): Rewrote constraint table to use direct properties; removed `##Heterogeneous`
- **errors.md**: No changes needed (no retired type references found)

### Task 3: Syntax docs (1 file)
- **prefix-system.md**: Replaced `##Contiguous` with `##Array` in tier examples
- **blocks.md, io.md, comments.md**: No `[#] << ##` patterns found -- no changes needed
- **macro-types.md**: Already retired; no changes needed

## Decisions Made

1. `##Rectangular` retired as composable schema; its properties are now stated directly on each type definition
2. `#Serial` uses `%##Depth.Max << #Inf` and `%##Count << #Inf` instead of composing `##Deep`/`##Sparse`
3. `##Array` uses `%##Fields << #Range` to replace the retired `%##Key << #UnsignedInt`
4. `#Inf` alias for `#Bound.Inf` documented in Inf.md for convenient property use

## Verification Results

| Check | Expected | Result |
|-------|----------|--------|
| `[#] << ##` in modified files | 0 | 0 -- PASS |
| `[#] << ###` in modified files | 0 | 0 -- PASS |
| `(<) << ##` in modified files | 0 | 0 -- PASS |
| Active `##Contiguous`/`##Rectangular` refs | 0 | 0 -- PASS (retirement notes only) |
| `%##Key`, `%##Flexible`, `#FlexKind` | 0 active | 0 active -- PASS (retirement notes only) |
| `[#] %##` present (properties kept `<<`) | >0 | 26 instances -- PASS |

## Deviations from Plan

### Skipped Files (No Changes Needed)

- **errors.md**: Listed in plan but contained no retired type references
- **blocks.md, io.md, comments.md**: Listed in plan but contained no `[#] << ##` patterns
- **macro-types.md**: Listed in plan but already retired with redirect

These are not deviations -- the plan listed them as "update if needed" and they needed no updates.

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | 6e5d8dc | Schema files: drop << composition, retire ##Rectangular |
| 2 | 556a8b1 | aj3lib type/concept docs: drop << composition, retire ##Deep/##Sparse |
| 3 | 7f8f9d6 | Syntax docs: replace ##Contiguous example |

## Self-Check: PASSED

All 10 modified files exist. All 3 task commits verified. SUMMARY.md created.
