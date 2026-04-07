---
phase: 145-wait-collect-markers
plan: 01
subsystem: docs
tags: [block-elements, io, collectors, disambiguation]

requires:
  - phase: none
    provides: existing documentation
provides:
  - "[>]/[<] renamed to IO parameter handling markers"
  - "Positional implicit IO documented for collectors"
  - "Disambiguation between [>]/[<] and [*] <</>>"
affects: []

key-files:
  modified: [docs/user/syntax/blocks.md, docs/user/syntax/io.md, docs/user/concepts/collections/collect.md, docs/technical/ebnf/12-collections.md, docs/user/concepts/errors.md, docs/technical/ebnf/05-block-elements.md]

key-decisions:
  - "[>]/[<] are IO parameter handling markers, not just fallback markers"
  - "Positional implicit IO (<args.0, <args.1...) for [*] << lines"

duration: 15min
started: 2026-04-07
completed: 2026-04-07
---

# Issue #145 Plan 01: Wait/collect markers disambiguation Summary

**Renamed [>]/[<] from "fallback" to "IO parameter handling," added disambiguation from [*] <</>>, documented positional implicit IO for collectors**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Completed | 2026-04-07 |
| Tasks | 6 completed |
| Files modified | 6 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: [>]/[<] as "IO parameter handling" | Pass | blocks.md and EBNF §5 updated |
| AC-2: Disambiguation note in io.md | Pass | Blockquote after "Wait and Collect IO" section |
| AC-3: Section renamed to "IO Parameter Handling" | Pass | Was "Fallback IO" |
| AC-4: Positional implicit IO in collect.md | Pass | *All and *First/*Nth sections |
| AC-5: EBNF §12 positional IO rule | Pass | New rule note after collect_io_line production |
| AC-6: errors.md cross-reference updated | Pass | Uses "IO parameter handling markers" |

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/syntax/blocks.md | Modified | Renamed [>]/[<] from "Output/Input fallback" to "Output/Input parameter handling" |
| docs/user/syntax/io.md | Modified | Disambiguation note + "Fallback IO" → "IO Parameter Handling" section rename |
| docs/user/concepts/collections/collect.md | Modified | Positional implicit IO docs for *All and *First/*Nth |
| docs/technical/ebnf/12-collections.md | Modified | EBNF rule for positional implicit IO + §5 cross-ref |
| docs/user/concepts/errors.md | Modified | Updated to "IO parameter handling markers" terminology |
| docs/technical/ebnf/05-block-elements.md | Modified | Rule text updated + disambiguation cross-reference to §12 |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| [>]/[<] are IO parameter handling markers | Fallback is one use case, not the full purpose | Correct terminology going forward |
| Positional implicit IO for collectors | *All/*First use <args.0, <args.1... inferred by compiler | Documents previously undocumented behavior |

## Deviations from Plan

None — plan executed exactly as written.

## Next Phase Readiness

**Ready:**
- Issue #145 changes complete, ready for merge
- Group 2 (Syntax Gaps) will be 5/5 after merge

**Blockers:** None

---
*Phase: 145-wait-collect-markers, Plan: 01*
*Completed: 2026-04-07*
