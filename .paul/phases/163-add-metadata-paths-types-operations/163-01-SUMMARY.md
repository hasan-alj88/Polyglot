---
phase: 163-add-metadata-paths-types-operations
plan: 01
subsystem: docs
tags: [metadata-tree, types, pglib, frontmatter]

requires:
  - phase: none
    provides: existing bundled type files (enums.md, structs.md, collections.md, rt.md)
provides:
  - 20 individual per-type files with metadata paths
  - 4 index pages linking to individual files
  - metadata_definition and metadata_instance on all 23 type files
affects: [163-02 (permission enums + schema tiers), 163-03 (pipeline metadata)]

tech-stack:
  added: []
  patterns:
    - "metadata_definition/metadata_instance YAML frontmatter on type files"
    - "## Metadata section with definition + instance path table"
    - "bundled files → index pages with summary tables"

key-files:
  created:
    - docs/user/pglib/types/OS.md
    - docs/user/pglib/types/PipelineStatus.md
    - docs/user/pglib/types/QueueStrategy.md
    - docs/user/pglib/types/RetriggerStrategy.md
    - docs/user/pglib/types/QueueState.md
    - docs/user/pglib/types/KillPropagation.md
    - docs/user/pglib/types/ResourceTag.md
    - docs/user/pglib/types/FileAccess.md
    - docs/user/pglib/types/VarState.md
    - docs/user/pglib/types/FieldKind.md
    - docs/user/pglib/types/path.md
    - docs/user/pglib/types/Queue.md
    - docs/user/pglib/types/Job.md
    - docs/user/pglib/types/Map.md
    - docs/user/pglib/types/Array.md
    - docs/user/pglib/types/Dataframe.md
    - docs/user/pglib/types/Serial.md
    - docs/user/pglib/types/Code.md
    - docs/user/pglib/types/PyEnv.md
    - docs/user/pglib/types/RsEnv.md
  modified:
    - docs/user/pglib/types/boolean.md
    - docs/user/pglib/types/string.md
    - docs/user/pglib/types/NativeKind.md
    - docs/user/pglib/types/enums.md
    - docs/user/pglib/types/structs.md
    - docs/user/pglib/types/collections.md
    - docs/user/pglib/types/rt.md

key-decisions:
  - "Enum instance paths use :0 pattern (single active field per instance)"
  - "Index pages keep category intro text and cross-cutting notes (e.g. collections Usage section)"

patterns-established:
  - "Per-type file pattern: frontmatter with metadata paths → H1 → <!-- @types --> → ## Definition → supporting content → ## Metadata table → ## Related"
  - "Index page pattern: intro → summary table with [[wikilinks]] → ## Related"

duration: ~5min
completed: 2026-04-07
---

# Issue #163 Plan 01: Split Type Definitions Summary

**20 individual type files created with metadata paths; 4 bundled files converted to index pages; 3 existing files updated with metadata**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~5min |
| Completed | 2026-04-07 |
| Tasks | 3 completed |
| Files created | 20 |
| Files modified | 7 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Individual type files with full content | Pass | 20 files created with definitions, tables, examples extracted from bundled files |
| AC-2: Metadata paths on all type files | Pass | 23 files have metadata_definition + metadata_instance frontmatter and ## Metadata section |
| AC-3: Bundled files converted to indexes | Pass | enums.md, structs.md, collections.md, rt.md rewritten as index pages; 0 {#} blocks remain |

## Accomplishments

- Split 20 types from 4 bundled files into individual per-type .md files, each with full Polyglot definitions, field tables, usage examples, and error codes
- Established metadata path pattern: YAML frontmatter (`metadata_definition`, `metadata_instance`) + `## Metadata` section with definition/instance path table
- Converted 4 bundled files to lightweight index pages with summary tables linking to individual files via [[wikilinks]]

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `types/OS.md` | Created | #OS enum — operating system |
| `types/PipelineStatus.md` | Created | #PipelineStatus enum — pipeline lifecycle |
| `types/QueueStrategy.md` | Created | #QueueStrategy enum — FIFO/LIFO/Priority |
| `types/RetriggerStrategy.md` | Created | #RetriggerStrategy enum — duplicate trigger handling |
| `types/QueueState.md` | Created | #QueueState enum — queue lifecycle |
| `types/KillPropagation.md` | Created | #KillPropagation enum — kill signal propagation |
| `types/ResourceTag.md` | Created | #ResourceTag enum — dispatch constraints |
| `types/FileAccess.md` | Created | #FileAccess enum — file access state |
| `types/VarState.md` | Created | #VarState enum — variable lifecycle |
| `types/FieldKind.md` | Created | #FieldKind enum — leaf content classifier |
| `types/path.md` | Created | #path struct — cross-platform file path |
| `types/Queue.md` | Created | #Queue struct — queue configuration |
| `types/Job.md` | Created | #Job struct — runtime job state |
| `types/Map.md` | Created | #Map collection — sparse key-value (macro) |
| `types/Array.md` | Created | #Array collection — contiguous rectangular (macro) |
| `types/Dataframe.md` | Created | #Dataframe collection — row-oriented table (macro) |
| `types/Serial.md` | Created | #Serial collection — unconstrained (plain {#}) |
| `types/Code.md` | Created | #Code struct — RT execution output |
| `types/PyEnv.md` | Created | #PyEnv struct — Python environment handle |
| `types/RsEnv.md` | Created | #RsEnv struct — Rust environment handle |
| `types/boolean.md` | Modified | Added metadata frontmatter + ## Metadata section |
| `types/string.md` | Modified | Added metadata frontmatter + ## Metadata section |
| `types/NativeKind.md` | Modified | Added metadata frontmatter + ## Metadata section |
| `types/enums.md` | Rewritten | Converted to index page — 10-row summary table |
| `types/structs.md` | Rewritten | Converted to index page — 3-row summary table |
| `types/collections.md` | Rewritten | Converted to index page — 4-row summary table + Usage section |
| `types/rt.md` | Rewritten | Converted to index page — 3-row summary table |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Enum instances use `:0` in path | Enums have single active field — no multi-instance numbering | Consistent with FULL-TREE.md pattern |
| Keep Usage section in collections index | Cross-cutting `:` separator syntax applies to all collection types | Users still see annotation examples without visiting individual files |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Pattern established for metadata paths — Plan 02 (permission enums + ## schemas + ### field types) and Plan 03 (pipeline files) can follow same pattern
- All # tier types now have individual files and metadata paths

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 163-add-metadata-paths-types-operations, Plan: 01*
*Completed: 2026-04-07*
