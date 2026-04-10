---
phase: typed-cross-references
plan: 03
status: complete
completed: 2026-04-10
---

# Summary: Migrate Refs + Add Coverage

## What Was Done

Migrated all classified `@` refs to `@c:`/`@u:` and added 435 new `@u:` coverage refs to compile-rules files.

### Migration Results

| Metric | Count |
|---|---|
| @c: refs (concept) | 195 |
| @u: refs (usage) | 533 |
| Untyped (intentional) | 82 |
| **Total** | **810** |
| Pre-migration baseline | 409 |
| Double-prefix errors | 0 |

### Breakdown
- **Existing ref migration**: 293 refs retyped (195 @c: + 98 @u:)
- **New coverage refs**: 435 @u: refs added across 159 compile-rules files
- **Files touched**: ~385 total

## Decisions
- Section-specific refs processed before whole-file refs to avoid partial matches
- Compile-rules refs deduplicated (one `@u:syntax/blocks` per file, not per-section)
- Files defining a construct don't get a self-referencing @u: for that construct

## Deviations
- Coverage gaps in brainstorming/ (medium priority) deferred — compile-rules was the high-priority target
- pglib/ coverage gaps (lower priority) deferred — most already had @c:types refs
