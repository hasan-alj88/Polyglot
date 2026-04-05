---
phase: issue-131-permission-path-grammar
plan: 01
subsystem: docs
tags: [metadata-tree, path-grammar, permissions]

requires:
  - phase: none
    provides: existing metadata-tree docs
provides:
  - Disambiguated type_prefix table separating instance_path vs exception prefixes
  - Cross-references from branches.md and object-types.md to specific grammar rules
affects: [issue-140-permission-object-redesign]

tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - docs/technical/spec/metadata-tree/path-grammar.md
    - docs/technical/spec/metadata-tree/branches.md
    - docs/technical/spec/metadata-tree/object-types.md

key-decisions:
  - "Split type_prefix into standard (instance_path) and exception (own grammar) groups"
  - "Applied same treatment to %! and %@ alongside %_ — all three have own grammar rules"
  - "Created #140 for {_} permission object redesign (brainstormed during discussion)"

patterns-established: []

duration: ~15min
started: 2026-04-05
completed: 2026-04-05
---

# Issue #131 Plan 01: Permission branch %_ path grammar fix — Summary

**Disambiguated type_prefix table to separate instance_path prefixes from exception branches (_/!/@ with own grammar rules), and added cross-references across metadata-tree docs.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-05 |
| Completed | 2026-04-05 |
| Tasks | 4 completed |
| Files modified | 3 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: type_prefix table no longer implies _ follows instance_path | Pass | Split into two rows: standard vs exception |
| AC-2: Exception branches explicitly named with their grammar rules | Pass | `_` → `permission_path`, `!` → `error_path`, `@` → `package_path` |
| AC-3: Cross-references added in related files | Pass | branches.md and object-types.md updated |
| AC-4: No other files reference old flat type_prefix list | Pass | Grep confirmed only path-grammar.md uses `type_prefix` |

## Accomplishments

- Split `type_prefix` into standard prefixes (11 that follow `instance_path`) and exception prefixes (3 with own grammar rules)
- Added `(uses permission_path)` / `(uses error_path)` / `(uses package_path)` annotations to object-types.md
- Added `permission_path` cross-reference to branches.md Permission Branch section
- Created GitHub issue #140 for `{_}` permission object redesign based on brainstorm discussion

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/spec/metadata-tree/path-grammar.md` | Modified | Split type_prefix table, clarify ref/instance apply to instance_path only, rename "Branch-specific rules" to "Exception branch rules" |
| `docs/technical/spec/metadata-tree/branches.md` | Modified | Added `permission_path` cross-reference to %_ section |
| `docs/technical/spec/metadata-tree/object-types.md` | Modified | Added grammar rule annotations to %_, %!, %@ rows |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Treat `!` and `@` same as `_` | All three have own grammar rules in the EBNF; same contradiction applies | Consistent documentation |
| Create separate issue #140 for {_} redesign | Scope too large for #131; ~15-20 files affected | Keeps #131 focused on docs fix |

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Branch ready for commit and merge
- Issue #140 created for follow-up {_} permission object redesign

**Concerns:**
- None

**Blockers:**
- None

---
*Issue: #131, Plan: 01*
*Completed: 2026-04-05*
