---
phase: 275-collection-redesign-record-fields
plan: 01
status: complete
completed: 2026-04-09
---

# Plan 275-01 Summary: Core Definitions + Property Tables

## What Was Done

### Task 1: schema-properties.md
- Replaced branch-level property table: `%##Fields` replaces `%##Key`, `%##Range`, `%##Flexible`, `%##Regular`
- Added `%##Fields` subsection with `#FieldsDescriptor` definition and usage
- Added `#Bound` and `#Inf` subsection
- Updated tree-level `.Inf` → `#Inf`
- Rewrote Approved `##` Schema Types: added `##Record`, `##Array`, `##Dataframe` collection schemas
- Added Retired Schemas table (##Map, ##Set, ##Contiguous, ##Rectangular, ##Sparse, ##Deep)
- Updated all schema composition syntax: `[#] << ##Schema` → `[#] ##Schema`
- Added syntax distinction note: `[#] ##Schema` (no <<) vs `[#] %##Prop << value` (with <<)

### Task 2: generic-types.md and basic-types.md
- generic-types.md: Replaced `#Map` example with `#Record`, removed `#Set`, updated parameterized schema section with `##Record` definition, updated `[#]` roles table, fixed bootstrap layers
- basic-types.md: Updated all `[#] <<` composition to `[#]`, updated `#Boolean`/`#String`/scalar examples, replaced `#Map`/`#Set` in Other Types with `##Record`/retirement, removed `%##Key` reference

### Task 3: schema INDEX.md and Fields.md
- INDEX.md: Reorganized into Static/Collection/Other sections, added `##Record` to collection schemas, expanded Retired Schemas table
- Fields.md: Complete rewrite from `##Fields` schema doc to `%##Fields` property doc with `#FieldsDescriptor`, `#Range`/enum ref usage, retired table

## Files Modified
- docs/user/syntax/types/schema-properties.md
- docs/user/syntax/types/generic-types.md
- docs/user/syntax/types/basic-types.md
- docs/user/pglib/types/schemas/INDEX.md
- docs/user/pglib/types/schemas/Fields.md

## Verification Results
- Retired properties (%##Key, %##Range, %##Flexible, %##Regular, #FlexKind): 0 in modified files
- Retired schemas (##Contiguous, ##Rectangular, ##Sparse, ##Deep): 0 in modified files (only in Retired tables)
- `[#] <<` composition syntax: 0 in syntax/types/ directory
- ##Record present in schema-properties.md: 6 occurrences
- #FieldsDescriptor present: 6 occurrences
- %##Fields present: confirmed

## Decisions
None — straightforward execution of issue #275 spec.
