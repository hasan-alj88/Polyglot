---
phase: 312-ast-invisible-functions-registry
plan: "01"
completed: 2026-04-18
duration: ~15min
key-files:
  created:
    - docs/technical/compiler/ast-invisible-registry.md
  modified:
    - docs/technical/compile-rules/PGE/PGE10014-ast-invisible-foreign-code.md
    - docs/technical/algorithms/foreign-code-analysis.md
    - docs/technical/compiler/io-registry.md
    - docs/user/concepts/permissions/foreign-code.md
key-decisions:
  - "Process execution functions (os.system, subprocess) stay in io-registry — they are analyzable IO, not AST-invisible"
  - "General reflection (hasattr, type()) excluded — only getattr(obj,name)() indirect call pattern is banned"
  - "system() appears in both registries with different semantics — banned without {_}, analyzable with {_}"
---

# Issue #312 Plan 01: AST-Invisible Functions Registry

**Externalized PGE10014's banned constructs into a versioned, extensible registry with 32 entries across 5 languages — up from 18 hardcoded entries.**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Registry with per-language sections | Pass | 5 languages, 6 categories, 32 entries, TOML format |
| AC-2: PGE10014 references registry | Pass | Inline table removed, @c: cross-reference added |
| AC-3: Algorithm references registry | Pass | Phase 2.1 updated with LOAD statement and registry reference |
| AC-4: Package extension documented | Pass | ast-invisible-registry-ext.toml format with examples |

## Accomplishments

- Created `ast-invisible-registry.md` with 32 entries (vs 18 previously hardcoded in PGE10014)
- New entries: `compile()`, `pickle.loads/load`, `marshal.loads/load`, `ctypes.WinDLL`, `ctypes.util.find_library`, `importlib.util.*`, `code.InteractiveInterpreter.*`, `Function.prototype.constructor`, indirect eval, `GetProcAddress`, `LoadLibrary`, `asm volatile`, `${!variable}`
- Established clear boundary: IO registry = analyzable calls, AST-invisible registry = unanalyzable constructs
- Package extension format (`ast-invisible-registry-ext.toml`) enables third-party wrapper detection

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/compiler/ast-invisible-registry.md` | Created | Registry specification with 5 language sections, TOML format, extension docs |
| `docs/technical/compile-rules/PGE/PGE10014-ast-invisible-foreign-code.md` | Modified | Replaced inline table with registry reference |
| `docs/technical/algorithms/foreign-code-analysis.md` | Modified | Phase 2.1 references registry as BANNED_CONSTRUCTS source |
| `docs/technical/compiler/io-registry.md` | Modified | Related section links companion registry |
| `docs/user/concepts/permissions/foreign-code.md` | Modified | Expanded banned constructs table + registry reference |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- All plans complete for issue #312 — ready for MERGE

**Blockers:**
- None

---
*Phase: 312-ast-invisible-functions-registry, Plan: 01*
*Completed: 2026-04-18*
