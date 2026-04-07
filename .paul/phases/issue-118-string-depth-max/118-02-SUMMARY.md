---
phase: issue-118-string-depth-max
plan: 02
status: complete
started: 2026-04-04
completed: 2026-04-04
---

# Plan 118-02 Summary — Propagation of ##Scalar/##Leaf Changes

## Result

All 4 tasks completed. All 6 acceptance criteria met. Zero deviations.

## Tasks Completed

### Task 1: pglib scalar enum types — ###Enum → ###ScalarEnum
- Updated 7 files: boolean.md, enums.md, BaseCode.md, supporting-enums.md, calendar-infrastructure.md, cultural-types.md, non-standard-time.md
- Also updated calendar-date-types.md (14 enum types) — overlapped with Task 2

### Task 2: pglib scalar value types — ###Value → ###ScalarValue
- Updated 5 files: string.md, scalars.md, structs.md, core-components.md, calendar-date-types.md
- Also updated supporting-enums.md (#BusinessWeek), cultural-types.md (#Holiday, #Observance, #Season), non-standard-time.md (#ChineseTime, #HinduTime, #DecimalTime, #CustomTimeUnit), calendar-infrastructure.md (#CalendarProjection)
- string.md: "Depth.Max << 0" → "Depth.Max << 1" in explanatory text

### Task 3: Type hierarchy, prefix, and reference files
- hierarchy.md: Added [##Leaf] to RawString, ###ScalarValue/###ScalarEnum to #String/#Boolean
- types.md: Same hierarchy updates
- prefix-system.md: Added ##Leaf to ## examples, ###ScalarValue/###ScalarEnum/###None to ### examples
- macro-types.md: Depth.Max = 0 → 1 in ##Scalar constraint text
- INDEX.md: Date updated
- collections.md: ###Enum → ###ScalarEnum in #SalesColumns example
- dataframe.md: ###Enum → ###ScalarEnum in #SalesColumns example

### Task 4: Technical files
- EBNF 04-type-system.md: Updated field_type_composition example, dataframe enum_type_param comments to ###ScalarEnum
- EBNF 03-identifiers.md: Added ###ScalarValue/###ScalarEnum/###None to field_type_id comment
- edge-cases/24-datatype-defs.md: EC-24.8 depth 0→1 + ###ScalarEnum, EC-24.17 ###ScalarEnum
- COMPILE-RULES.md: #Boolean example updated to ###ScalarEnum
- definition-templates.md: Label text updated to ###ScalarEnum

## Verification Results

- `[#] << ###Enum` in docs/: Only 4 remaining — all in non-scalar contexts (general inference text, PGE11003 example, EC-24.9 inheritance example)
- `[#] << ###Value` in docs/: Only 3 remaining — all in non-scalar contexts (general inference text, #DateTime main-type.md)
- `###ScalarEnum`: 60 occurrences across 20 files
- `###ScalarValue`: 39 occurrences across 12 files
- `##Leaf`: Present in hierarchy, types, prefix-system, schema-properties, enums (via ##Leafs), collections
- `Depth.Max...0`: Only in ##Leaf context (schema-properties.md), EBNF grammar example, and 0D array edge case

## Files Modified (27 total)

1. docs/user/pglib/types/boolean.md
2. docs/user/pglib/types/enums.md
3. docs/user/pglib/types/BaseCode.md
4. docs/user/pglib/types/string.md
5. docs/user/pglib/types/scalars.md
6. docs/user/pglib/types/structs.md
7. docs/user/pglib/types/types.md
8. docs/user/pglib/types/collections.md
9. docs/user/pglib/types/datetime/main-type.md (not modified — ###Value retained, non-scalar)
10. docs/user/pglib/types/datetime/core-components.md
11. docs/user/pglib/types/datetime/calendar-date-types.md
12. docs/user/pglib/types/datetime/calendar-infrastructure.md
13. docs/user/pglib/types/datetime/cultural-types.md
14. docs/user/pglib/types/datetime/non-standard-time.md
15. docs/user/pglib/types/datetime/supporting-enums.md
16. docs/user/syntax/types/hierarchy.md
17. docs/user/syntax/types/prefix-system.md
18. docs/user/syntax/types/macro-types.md
19. docs/user/syntax/types/INDEX.md
20. docs/user/concepts/collections/dataframe.md
21. docs/technical/COMPILE-RULES.md
22. docs/technical/ebnf/04-type-system.md
23. docs/technical/ebnf/03-identifiers.md
24. docs/technical/edge-cases/24-datatype-defs.md
25. docs/technical/spec/metadata-tree/definition-templates.md

## Acceptance Criteria

- [x] AC-1: All scalar enum types use ###ScalarEnum
- [x] AC-2: All scalar value types use ###ScalarValue
- [x] AC-3: Non-scalar types retain ###Value/###Enum
- [x] AC-4: Type hierarchy and prefix docs updated
- [x] AC-5: Technical docs consistent
- [x] AC-6: No stale Depth.Max=0 for scalars in any modified file
