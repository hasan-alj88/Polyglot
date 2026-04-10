---
phase: typed-cross-references
plan: 01
status: complete
completed: 2026-04-10
---

# Summary: Define Typed Reference Protocol

## What Was Done

Updated 3 audit infrastructure files to define the `@c:` (concept) and `@u:` (usage) typed cross-reference protocol.

### Files Modified
- `docs/audit/README.md` — "Dual Smart Referencing" → "Typed Smart Referencing" with @c:/@u:/untyped definitions, real examples, behavioral guidance
- `docs/audit/rules/conventions.md` — Added 3 cross-ref rules: typed-refs, usage-coverage, untyped-valid
- `docs/audit/rules/checklist.md` — Added Cross-Reference Check section with typed-ref-check and usage-coverage-check

## Decisions
- Both `@c:` and `@u:` are mandatory read-before-write (semantic tag, not behavioral difference)
- Untyped `@` remains valid for backward compatibility
- Wikilink `c:`/`u:` display prefixes defined but migration deferred to touch

## Deviations
None.
