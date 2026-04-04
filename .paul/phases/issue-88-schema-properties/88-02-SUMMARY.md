---
phase: issue-88-schema-properties
plan: 02
type: summary
status: complete
completed: 2026-03-28
---

# Plan 02 Summary — Collections Rewrite

## What Changed

**File modified:** `docs/user/concepts/collections.md`

### New sections added

1. **Collection Hierarchy** — overview table showing #Map as base, #Array as variant, #Serial as schema-free
2. **#Map — Base Collection** — full `{#}` definition from draft.md with ##Flat, ##Homogeneous, ##Sparse schema composition; `<` accessor documentation
3. **#Array — Map Variant** — full `{#}` definition with `<~` inheritance from #Map, ##Contiguous + ##Rectangular schemas, Cartesian product key structure, #Dimension parameter with default syntax, multidimensional access examples
4. **#Serial — Schema-Free Tree** — minimal `{#}` definition (no ## constraints), unlimited depth access
5. **User-Defined Struct** — `{#} #Person` example, compiler inference of ##, `.` vs `<` accessor distinction
6. **Idiomatic Dataframe Pattern** — #SalesRow + #SalesTable definitions, mixed `<` and `.` access (`$sales<0.product`), *Into.Array collector reference
7. **Nested Collection Safety** — PGE11002 (unbounded nesting requires explicit %##Depth.Max), PGW11003 (unlimited depth warning on user types), code example showing correct vs incorrect nesting

### Renamed

- All references to `#Dict` replaced with `#Map` (zero remaining #Dict references)

### Preserved

- Both Mermaid flowcharts (expand/collect and sync/race) retained unchanged
- All expand/collect operator documentation preserved
- All sync & race collector documentation preserved
- All parallel boundaries and compile rules preserved
- PGE11002 and PGW11003 added to the compile rules table

## Acceptance Criteria

- [x] AC-1: #Map as base collection with full {#} definition
- [x] AC-2: #Array as #Map variant with Cartesian keys
- [x] AC-3: #Serial as schema-free tree
- [x] AC-4: User-defined struct pattern
- [x] AC-5: Idiomatic dataframe pattern
- [x] AC-6: Nested collection depth safety

## Verification

- [x] `#Map<KeyType` present in collections.md
- [x] `#Array<ValueType` present in collections.md
- [x] `#Serial` present in collections.md
- [x] `#SalesTable` present in collections.md
- [x] `PGE11002` present in collections.md
- [x] `%##Depth.Max` present in collections.md
- [x] Zero `#Dict` references remaining
- [x] Mermaid diagrams preserved
- [x] No changes to types.md, data-is-trees.md, metadata-tree.md, EBNF.md, or COMPILE-RULES.md
