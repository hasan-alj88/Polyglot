---
phase: issue-75-dict-kv
plan: 03
subsystem: docs
tags: [type-system, ebnf, metadata-tree, compile-rules, pglib, generics, schema-properties]

requires:
  - phase: issue-75-dict-kv/75-02
    provides: Rewritten types.md with # annotations, updated collections.md with #Dict/#Dataframe
provides:
  - EBNF §4 rewritten for # type annotations with generic params, schema properties, constraints
  - metadata-tree updated for # annotations and schema property documentation
  - 3 new compile rules (PGE04018/419/420) for type parameter constraints, duplicate keys, key gaps
  - pglib types.md with all 11 core type definitions
affects: [75-04 spec-wide migration, COMPILE-RULES rule-by-rule writing]

tech-stack:
  added: []
  patterns:
    - "# type annotation in EBNF grammar productions"
    - "Schema property documentation in metadata tree definition templates"
    - "Collection type EBNF productions (dict_type, dataframe_type)"

key-files:
  modified:
    - docs/technical/EBNF.md
    - docs/technical/spec/metadata-tree.md
    - docs/technical/COMPILE-RULES.md
    - docs/user/pglib/types/types.md

key-decisions:
  - "New PGE codes in 4.x range (418-420) not 3.x — type constraints are type-system rules"
  - "Schema properties table in metadata-tree rather than just tree diagram"

patterns-established:
  - "EBNF generic_param/schema_property/type_constraint productions"
  - "Schema property table format in metadata-tree definition templates"

completed: 2026-03-27
---

# Issue #75 Plan 03: Technical Spec + pglib Type Definitions

**Updated EBNF grammar, metadata tree, and compile rules for # type annotations; added all 11 core type definitions to pglib types.md.**

## Performance

| Metric | Value |
|--------|-------|
| Tasks | 3 completed |
| Files modified | 4 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF Type Annotation Grammar Uses # | Pass | §4 rewritten; dict_type, dataframe_type, generic_param, schema_property, type_constraint, wildcard_type all present; basic_type includes uint/sci/eng/dim; array uses flex_sep |
| AC-2: Metadata Tree Updated for # Annotations | Pass | Alias resolution uses #; definition templates include schema property table + #Array example; field expansion uses # |
| AC-3: New Compile Rules for Type System | Pass | PGE04018, PGE04019, PGE04020 added to error code reference table |
| AC-4: Core Type Definitions in pglib | Pass | #String through #Dimension (7 scalar types) + #Array/#Dict/#Serial/#Dataframe (4 collections) = 11 definitions; existing entries updated ; → # |

## Accomplishments

- EBNF §4 fully rewritten with # type annotations, collection types (dict, dataframe), generic type parameters, schema properties, type constraints, and wildcard type
- EBNF §4.3 added for generic type parameters in {#} definitions (schema_inheritance, schema_property, type_constraint productions)
- metadata-tree.md: String Subtype Nesting updated with all 7 subtypes and # alias resolution; Definition Templates gains schema property table and #Array example tree; Field Expansion uses #
- COMPILE-RULES.md: 3 new error codes (PGE04018 Type Parameter Constraint Violation, PGE04019 Duplicate Dictionary Key, PGE04020 Key Gap Violation)
- pglib/types/types.md: type hierarchy section, 7 scalar definitions, 4 collection definitions, all existing entries migrated ; → #, status promoted to complete

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/EBNF.md` | Modified | §4 rewritten for # annotations; §4.3 added for generics/schema/constraints; §5 [<] note updated |
| `docs/technical/spec/metadata-tree.md` | Modified | String Subtype Nesting, Definition Templates, Field Expansion updated for # |
| `docs/technical/COMPILE-RULES.md` | Modified | PGE04018, PGE04019, PGE04020 added to error code table |
| `docs/user/pglib/types/types.md` | Modified | 11 core type defs added, hierarchy section, ; → # migration, status → complete |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| PGE codes 418-420 in 4.x range | Type parameter constraints, duplicate keys, and key gaps are type-system rules, not parallel (3.x) | Consistent with existing 4.x type-system grouping |
| Schema property table in metadata-tree | Table format more scannable than prose for 5 properties | Establishes documentation pattern for future schema properties |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 4 technical spec files aligned with # type annotation system
- pglib types.md has complete core type definitions — downstream files can reference
- EBNF grammar is authoritative for # annotations, generics, schema properties

**Concerns:**
- ~120 remaining files still use ; annotations — Plan 75-04 needed for spec-wide migration
- COMPILE-RULES rule bodies for PGE04018/419/420 still need writing (deferred to rule-by-rule effort)
- metadata-tree.md IO Port Nesting and Permission Branch sections still use ; (Plan 75-04 scope)

**Blockers:**
None.

---
*Phase: issue-75-dict-kv, Plan: 03*
*Completed: 2026-03-27*
