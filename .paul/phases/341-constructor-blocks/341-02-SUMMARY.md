---
phase: 341-constructor-blocks
plan: 02
subsystem: language-design
tags: [constructor, ebnf, metadata-tree, block-type, grammar]

requires:
  - phase: 341-constructor-blocks/01
    provides: "{$} constructor specification (constructors.md), blocks.md entries"
provides:
  - "EBNF §9.13 constructor grammar (09-13-constructor.md)"
  - "[$] and ($) dual-context in EBNF §5"
  - "%definition.$ metadata tree branch across 4 spec files"
  - "Constructor row in data-is-trees.md concept table"
affects: [342-jm3lib-constructors, 343-compile-rules, 344-doc-updates]

tech-stack:
  added: []
  patterns: ["overload-indexed definition templates (%definition.$:Name:N)"]

key-files:
  created: [docs/technical/ebnf/definition-blocks/09-13-constructor.md]
  modified: [docs/technical/ebnf/09-definition-blocks.md, docs/technical/ebnf/05-block-elements.md, docs/technical/ebnf/INDEX.md, docs/technical/spec/metadata-tree/branches.md, docs/technical/spec/metadata-tree/FULL-TREE.md, docs/technical/spec/metadata-tree/object-types.md, docs/technical/spec/metadata-tree/definition-templates.md, docs/user/concepts/data-is-trees.md]

key-decisions:
  - "Constructors live under %definition.$ not %$ — %$ is the variable branch"
  - "Overloads indexed sequentially: %definition.$:DT:0, :1, :2"
  - "[$] added to execution_elem production (not a new category)"

patterns-established:
  - "Overload-indexed definition templates for multi-definition block types"

duration: ~10min
completed: 2026-04-22
---

# Issue #341 Plan 02: {$} EBNF Grammar & Metadata Tree Summary

**Created EBNF §9.13 constructor grammar and updated %definition.$ metadata tree branch across 9 files — compiler infrastructure ready for #342 (jm3lib) and #343 (compile rules)**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Completed | 2026-04-22 |
| Tasks | 2 completed |
| Files modified | 9 (1 created, 8 modified) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF grammar for {$} | Pass | 09-13-constructor.md with 8 productions, rules, 3 examples |
| AC-2: §5 and §9 index updated | Pass | [$] in execution_elem, ($) dual-context, §9.13 row, INDEX.md updated |
| AC-3: Metadata tree updated | Pass | branches.md, FULL-TREE.md, object-types.md, definition-templates.md, data-is-trees.md |

## Accomplishments

- Created EBNF §9.13 with full grammar: constructor_def, constructor_header, constructor_io_line, constructor_action, constructor_field_line, constructor_native_body
- Updated §5 block-elements with [$] execution element and ($) dual-context documentation
- Added %definition.$ constructor branch to all 4 metadata tree spec files with overload-indexed template pattern
- Added Constructor row to data-is-trees.md concept mapping table
- Zero PGE/PGW codes introduced (boundary respected — #343 scope)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/ebnf/definition-blocks/09-13-constructor.md | Created | EBNF §9.13 — full constructor grammar, rules, examples |
| docs/technical/ebnf/09-definition-blocks.md | Modified | Added §9.13 row to section table and related user docs |
| docs/technical/ebnf/05-block-elements.md | Modified | [$] in execution_elem; ($) dual-context note |
| docs/technical/ebnf/INDEX.md | Modified | {$} added to §9 description |
| docs/technical/spec/metadata-tree/branches.md | Modified | Constructor Branch section with full structure |
| docs/technical/spec/metadata-tree/FULL-TREE.md | Modified | %definition.$ entries with DT examples |
| docs/technical/spec/metadata-tree/object-types.md | Modified | Constructor in %definition description |
| docs/technical/spec/metadata-tree/definition-templates.md | Modified | Constructor Definition Templates section |
| docs/user/concepts/data-is-trees.md | Modified | Constructor row in concept table |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| %definition.$ not %$ for constructors | %$ is already the variable branch; constructors are definitions, not instances | Clean separation; constructor invocations produce %$ variables |
| Overload index as second level | Each overload needs its own template entry; sequential indexing matches source order | %definition.$:DT:0 (keyword), :1 (parser) — extensible pattern |
| [$] as execution_elem not new category | [$] appears only inside {$} definitions, analogous to [#] dual-context | Minimal grammar surface; context disambiguates |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- EBNF grammar provides authoritative reference for #343 compile rules
- Metadata tree provides authoritative paths for #342 jm3lib constructors
- Both plans 341-01 and 341-02 are complete — issue #341 is fully specified

**Concerns:**
- inline-calls.md still contains execution-body examples (e.g., cross-language section) — #344 should clean these up

**Blockers:**
- None

---
*Phase: 341-constructor-blocks, Plan: 02*
*Completed: 2026-04-22*
