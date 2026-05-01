---
phase: issue-86-audit-fixed-vs-flexible-field-usage
plan: 01
subsystem: spec
tags: [metadata-tree, permissions, errors, packages, field-separators]

provides:
  - Consistent . vs : field usage across all % tree branches
  - New !Error user-extensible namespace
  - New :: registry separator for package addresses
  - Company registry type (replaces Registry)
affects: [EDGE-CASES, EBNF, compile-rules (old package format remains)]

key-files:
  modified:
    - docs/user/concepts/metadata.md
    - docs/technical/spec/metadata-tree.md
    - docs/user/concepts/data-is-trees.md
    - docs/user/syntax/packages.md
    - docs/user/jm3lib/errors/errors.md
    - docs/user/concepts/errors.md
    - docs/user/syntax/identifiers.md

key-decisions:
  - "Permissions: all . fixed — no user-extensible levels in %_"
  - "Error namespaces: . for Polyglot-defined, : under .Error for user-extensible"
  - "Packages: :: separator, : for all flexible levels, Registry → Company"
  - "Sibling homogeneity: children of a parent must all use same separator"
  - "Migrate on touch: technical/ files (EDGE-CASES, compile-rules) deferred to next touch"

duration: ~25min
completed: 2026-03-25
---

# Issue #86 Plan 01: Audit Fixed vs Flexible Field Usage — Summary

**Consistent `.` (fixed) vs `:` (flexible) field separators across all `%` tree branches, plus new `!Error` namespace and `::` package separator.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~25min |
| Completed | 2026-03-25 |
| Tasks | 5 completed |
| Files modified | 7 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| %_ permission tree uses only `.` | Pass | All `:Category` and `:subcategory` → `.` in metadata.md, metadata-tree.md, data-is-trees.md |
| %! error namespaces use `.` | Pass | `%!:File.NotFound` → `%!.File.NotFound` across all files |
| !Error namespace added | Pass | User-extensible namespace with `[:]`/`[.]` pattern documented in errors.md |
| %@ package format uses `::` | Pass | New format in packages.md, identifiers.md, data-is-trees.md, metadata.md, metadata-tree.md |
| Registry → Company rename | Pass | Updated in packages.md registry table |
| Path grammar updated | Pass | Branch-specific EBNF patterns added to metadata-tree.md |
| No `%_:` patterns remain in docs | Pass | Grep verified zero matches |
| No `%!:` patterns remain in docs | Pass | Grep verified zero matches |

## Accomplishments

- Changed all `%_` permission tree paths from `:` to `.` — 3 files updated with consistent all-fixed structure
- Changed `%!` error namespace level from `:` to `.` and added `!Error` as the 7th namespace — the only one with user-extensible `:` children
- Introduced `::` registry separator for package addresses, renamed `Registry` → `Company`, made package names flexible (`:`)
- Updated path grammar in metadata-tree.md with branch-specific EBNF patterns
- Skill audit: No required skills configured ✓

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/concepts/metadata.md` | Modified | %_ all `.`, %! with `.` namespaces + `.Error:*`, %@ with `::` |
| `docs/technical/spec/metadata-tree.md` | Modified | %_ all `.`, %! + .Error, %@ `::`, branch-specific path grammar |
| `docs/user/concepts/data-is-trees.md` | Modified | Updated instance examples for %_, %!, %@ |
| `docs/user/syntax/packages.md` | Modified | New address format with `::`, Company rename, all examples updated |
| `docs/user/jm3lib/errors/errors.md` | Modified | Tree path ref to `.`, added `!Error` namespace section |
| `docs/user/concepts/errors.md` | Modified | Added !Error to namespace list, count (six→seven), table |
| `docs/user/syntax/identifiers.md` | Modified | Updated package address example and description |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Permissions all `.` fixed | Categories and capabilities are Polyglot-defined, not user-extensible | Consistent with `_File.read` identifier syntax |
| Error namespaces `.` fixed | jm3lib namespaces are Polyglot-defined; only `!Error` is user-extensible | `.Error` is fixed name, children are `:` flexible |
| `!Error` added as 7th namespace | Users need extensibility point for custom errors via `{!}` blocks | `[:]` for branches, `[.]` for terminal leaves |
| Sibling homogeneity rule documented | All children of a parent must use same separator | Prevents mixed `.`/`:` at same level |
| Packages use `::` separator | Clearly separates registry+ID from package name | Eliminates ambiguity at `.` → `:` transition |
| `Registry` → `Company` | More descriptive name for registered company domains | Matches Local/Community naming convention |
| Migrate on touch for technical/ files | EDGE-CASES, compile-rules have many old-format addresses | Deferred per existing "migrate on touch, not batch" decision |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Scope additions | 1 | Essential — identifiers.md defines the format |
| Deferred | 1 | Technical files use old package format |

**Total impact:** Minimal scope addition, consistent with plan intent.

### Scope Addition

**1. identifiers.md update**
- **Found during:** Task 5 (verification grep)
- **Issue:** identifiers.md defines the package address format and had old examples
- **Fix:** Updated package address example and description text
- **Files:** `docs/user/syntax/identifiers.md`

### Deferred Items

- Old-format package addresses remain in `docs/technical/` files (EDGE-CASES.md, EBNF.md, ~30 compile-rules files). Per "migrate docs on touch, not batch" decision, these update when next modified.

## Next Phase Readiness

**Ready:**
- All core spec files (metadata.md, metadata-tree.md, data-is-trees.md, packages.md) have consistent `.`/`:` usage
- `!Error` namespace documented with user code example
- Path grammar formalized with branch-specific patterns

**Concerns:**
- ~30 technical/ files still use old package address format (deferred to touch)

**Blockers:**
- None

---
*Phase: issue-86-audit-fixed-vs-flexible-field-usage, Plan: 01*
*Completed: 2026-03-25*
