---
phase: issue-282-metadata-paths
plan: 01
subsystem: docs
tags: [metadata, frontmatter, definition-files, metadata-tree]

requires:
  - phase: none
    provides: n/a
provides:
  - metadata_definition frontmatter on 33 definition files
  - Metadata sections with FULL-TREE.md references
affects: []

tech-stack:
  added: []
  patterns: [metadata_definition/metadata_instance frontmatter keys on definition files]

key-files:
  created: []
  modified:
    - docs/user/pglib/errors/builtin/*.md (13 files)
    - docs/user/pglib/types/scalars/*.md (9 files)
    - docs/user/pglib/types/datetime/calendars/*.md (11 files)

key-decisions:
  - "Error files use metadata_definition only (no instances — fixed namespaces)"
  - "Calendar files with multiple {#} definitions list all in Metadata table, primary in frontmatter"

patterns-established:
  - "Error metadata: %!.Namespace pattern (no metadata_instance)"
  - "Multi-definition files: primary type in frontmatter, all types in body Metadata table"

duration: ~10min
started: 2026-04-14
completed: 2026-04-14
---

# Issue #282 Plan 01: Add full % metadata tree paths to definition files

**Added `metadata_definition`/`metadata_instance` frontmatter and Metadata sections to 33 definition files across errors, scalars, and calendars.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Tasks | 3 completed |
| Files modified | 33 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Error builtin files have metadata | Pass | 13/13 files have `metadata_definition` + Metadata section |
| AC-2: Scalar type files have metadata | Pass | 9/9 new files + 1 existing (int.md) = 10/10 |
| AC-3: Calendar type files have metadata | Pass | 11/11 files have `metadata_definition` + Metadata section |
| AC-4: Paths consistent with FULL-TREE.md | Pass | Spot-checked file.md, float.md, gregorian.md |

## Accomplishments

- Added `metadata_definition` frontmatter to all 13 error builtin files with `%!.Namespace` paths
- Added `metadata_definition` + `metadata_instance` frontmatter to 9 scalar files with `%definition.#:Name` / `%#:String:alias` paths
- Added `metadata_definition` + `metadata_instance` frontmatter to 11 calendar files with `%definition.#:Name` / `%#:Name:N` paths
- All files now reference FULL-TREE.md as authoritative source

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `errors/builtin/{alias,csv,env,field,file,math,no,permission,rt,storage,text,timeout,validation}.md` | Modified | Added `metadata_definition` frontmatter + `## Metadata` section |
| `types/scalars/{float,unsigned-int,sci,eng,dimension,comma-separated-list,data-type-string,key-string,nested-key-string}.md` | Modified | Added `metadata_definition` + `metadata_instance` frontmatter + FULL-TREE reference |
| `types/datetime/calendars/{buddhist,chinese,coptic,custom,ethiopian,gregorian,hebrew,hijri,hindu,japanese,persian}.md` | Modified | Added `metadata_definition` + `metadata_instance` frontmatter + `## Metadata` section |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Error files get `metadata_definition` only, no `metadata_instance` | Errors are fixed namespaces with no runtime instances per FULL-TREE.md | Consistent with `%!` branch semantics |
| Multi-definition calendar files list all types in body table | Files like hijri.md define 5 types; frontmatter holds primary only | All definitions discoverable via Metadata section |
| Scalar frontmatter uses `%definition.#:Name` (not `##:Name`) | Matches FULL-TREE.md struct definition paths | Consistent with authoritative tree |
| CSV, Env, Storage, Text errors treated as top-level `%!` namespaces | They use `{!} !Name` syntax identical to FULL-TREE.md built-ins | Extends the `%!` namespace pattern for pglib additions |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- All 33 definition files now have metadata frontmatter
- Issue #282 scope for these 3 categories is complete

**Concerns:**
- Overview/INDEX files (21 files) were excluded as out of scope — they reference definitions but don't define them. Issue may want these addressed separately.

**Blockers:**
- None

---
*Phase: issue-282-metadata-paths, Plan: 01*
*Completed: 2026-04-14*
