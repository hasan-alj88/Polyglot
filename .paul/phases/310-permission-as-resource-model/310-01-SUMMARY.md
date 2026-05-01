---
phase: 310-permission-as-resource-model
plan: 01
subsystem: permissions
tags: [security, permissions, file-binding, EBNF, compile-rules]

provides:
  - Permission-as-Resource model ({_} carries grant + locator)
  - [_] marker retired, IO-based permission references
  - Decomposed permission fields (.category, .capability, .scope, .path)
  - {_} template inputs via (_)
  - Compile-Time File Binding as consequence of {_} model
  - PGE10009/PGE10010 new compile rules
affects: [compiler, jm3lib, all pipeline examples]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE10009-unresolved-permission-template.md
    - docs/technical/compile-rules/PGE/PGE10010-permission-resource-not-found.md
  modified:
    - docs/user/concepts/permissions/ (all 7 files)
    - docs/user/concepts/pipelines/permissions.md
    - docs/user/syntax/blocks.md
    - docs/user/syntax/packages.md
    - docs/technical/ebnf/definition-blocks/09-09-permission.md
    - docs/technical/ebnf/10-execution.md
    - docs/technical/ebnf/05-block-elements.md
    - docs/technical/compile-rules/PGE/ (PGE10003-10008, PGW10001)
    - docs/technical/edge-cases/ (10-execution.md, 06-operators.md)
    - docs/technical/spec/metadata-tree/ (branches.md, FULL-TREE.md)
    - ~55 jm3lib + pipeline files ([_] → (-) sweep)

key-decisions:
  - "{_} carries both grant AND resource locator — single gateway for all external access"
  - "[_] block element retired — permissions declared via IO markers ((#), (-), (@))"
  - ".File.Read shorthand deprecated — decomposed fields only"
  - "Templates use (_) inputs inside {_} definitions"
  - "Compile-Time File Binding is automatic consequence, not separate mechanism"

duration: ~90min
completed: 2026-04-17
---

# Issue #310 Plan 01: Permission-as-Resource Model Summary

**Redesigned {_} permission objects to carry both capability grant and resource locator; retired [_] marker; all external access mediated through IO-declared permissions with decomposed fields.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~90 min |
| Completed | 2026-04-17 |
| Tasks | 13 completed |
| Files modified | ~75 |
| Files created | 2 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Schema gains resource locator fields | Pass | __ResourceLocator with category-dependent fields |
| AC-2: Permission objects show instance vs template | Pass | (_) inputs, {<param} interpolation |
| AC-3: [_] marker retired across all docs | Pass | Only 2 intentional "RETIRED" mentions remain |
| AC-4: Enforcement rewritten as consequence | Pass | Compiler detection flow documented |
| AC-5: New compile rules PGE10009/10010 | Pass | Template resolution + resource not found |
| AC-6: EBNF grammar updated | Pass | permission_object_def allows (_) inputs |
| AC-7: Cross-reference sweep clean | Pass | grep confirms zero stale [_] or .File.Read shorthand |

## Accomplishments

- Unified permission + resource locator into single `{_}` model — pipelines receive whole object
- Retired `[_]` block element across ~75 files — replaced with IO markers `(#)`, `(-)`, `(@)`
- Deprecated `.File.Read "/path"` shorthand — all examples use decomposed `.category`, `.capability`, `.scope`, `.path`
- Added `{_}` template system with `(_)` inputs and `{<param}` interpolation
- Compile-Time File Binding reframed as automatic consequence of `{_}` model
- Created PGE10009 (unresolved permission template) and PGE10010 (permission resource not found)

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Scope expansion | 1 | Brainstorm evolved from cross-ref task to full redesign |

**Total impact:** Issue #310 as filed requested 5 cross-reference additions. Brainstorming with user led to a fundamental redesign of the permission system. The original issue's goals are fully subsumed — Compile-Time File Binding is now inherent in the `{_}` model rather than needing separate cross-references.

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| `{_}` = grant + locator | Single gateway for all external access; eliminates hardcoded paths | Every pipeline accessing external resources now uses `{_}` objects |
| `[_]` retired → IO markers | Permissions are IO of the definition, not a separate marker | Consistent with `(#)`, `(-)`, `(@)` pattern |
| Decomposed fields only | `.category #File` is more explicit than `.File.Read "/path"` | All permission examples updated |
| `(_)` inputs in `{_}` | Templates need parameterization for reusable permission shapes | PGE10005 updated to allow `(_)` |
| Content hashing as consequence | Every `{_}` with `.category #File` triggers hashing automatically | No separate file binding documentation needed |

## Next Phase Readiness

**Ready:**
- Permission-as-Resource model fully documented
- All cross-references updated
- Ready for merge to main

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 310-permission-as-resource-model, Plan: 01*
*Completed: 2026-04-17*
