---
phase: issue-76-78-rt-runtime-execution
plan: 01
subsystem: aj3lib
tags: [rt, runtime, python, rust, wrapper, errors, pipelines, foreign-code]

requires:
  - phase: issue-112
    provides: "[C] uppercase foreign code element"
  - phase: issue-113
    provides: "{Q} dual-purpose documentation"
provides:
  - "=W.RT wrapper replacing =W.Python"
  - "=RT.* execution pipelines (7 modes)"
  - "!RT error namespace (4 leaves)"
  - "#Code, #PyEnv, #RsEnv runtime types"
affects: [EBNF grammar updates, compile-rules]

tech-stack:
  added: []
  patterns: [".Inline/.File mode split", "binding-origin distinction (Script vs Bind)", "code-last stylistic convention for [C] blocks"]

key-files:
  created:
    - docs/user/aj3lib/pipelines/RT.md
    - docs/user/aj3lib/types/rt.md
  modified:
    - docs/user/aj3lib/pipelines/W.md
    - docs/user/aj3lib/types/types.md
    - docs/user/aj3lib/errors/errors.md
    - docs/user/aj3lib/INDEX.md
    - docs/user/AJ3LIB.md
    - docs/user/syntax/blocks.md

key-decisions:
  - "Script vs Bind: distinguished by binding origin (Polyglot-controlled vs foreign-code-controlled)"
  - "All modes split into .Inline/.File variants; .CLI is inherently file-based"
  - ".CLI uses =W.Polyglot, not =W.RT (no language runtime needed)"
  - "Compiler validates <func in .Function and <Bind names in .Script; .CLI and .Bind are opaque"
  - "Multiple [W] wrappers: setup in declaration order, cleanup in reverse (bracket semantics)"
  - "<Bind/#serial replaces <Bind#Map (serial is the universal exchange format)"
  - ">return#serial is a separate output port from >output (function return value)"
  - "Retry/timeout are [Q] queue strategies, not RT pipeline concerns"
  - "Code-last stylistic convention: <code and [C] blocks appear last in IO"
  - "=W.RT uses flexible fields for language+version (:Python:3:14)"
  - "#Code uses flexible :Lang, fixed .Output (;Code:Python.Output)"
  - "!RT has 4 leaves: .CompileError, .RuntimeError, .Timeout, .EnvironmentError"
  - ".CLI gets !RT.RuntimeError + !RT.Timeout only (no compile/env errors)"

patterns-established:
  - "Inline/File mode split pattern for code-accepting pipelines"
  - "Binding-origin naming: Script = Polyglot-controlled, Bind = foreign-code-controlled"

duration: ~90min
started: 2026-04-01T10:00:00Z
completed: 2026-04-01T11:30:00Z
---

# Phase issue-76-78 Plan 01: RT Runtime Execution Summary

**Complete =RT runtime execution subsystem: =W.RT wrapper, !RT errors, 7 execution pipelines, and #Code/#Env types**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~90min |
| Started | 2026-04-01 |
| Completed | 2026-04-01 |
| Tasks | 5 completed |
| Files modified | 9 (2 new + 7 edited) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: =W.RT wrapper replaces =W.Python | Pass | Flexible fields :Python/:Rust + version; permission table updated |
| AC-2: #Code output struct and env types added | Pass | rt.md created (72 lines); types.md hierarchy + category updated |
| AC-3: !RT error namespace added | Pass | 4 leaves + 7 pipeline error associations in errors.md |
| AC-4: =RT.md created with all 7 execution pipelines | Pass | 261 lines; IO signatures, examples, IO summary table, compiler validation |
| AC-5: INDEX.md and AJ3LIB.md updated | Pass | =RT row, =W description, !RT listing, #Code/#PyEnv/#RsEnv types |
| AC-6: blocks.md [C] example updated | Pass | <script→<code, >stdout→>output#Code:Python.Output, .Script.Inline suffix |

## Accomplishments

- Created RT.md documenting all 7 `=RT.*` execution pipeline modes with IO signatures, examples, and compiler validation rules
- Defined #Code output struct, #PyEnv/#RsEnv environment handles in new rt.md
- Replaced flat =W.Python with flexible =W.RT supporting multi-language + versioned runtimes
- Added !RT error namespace with 4 leaves and per-pipeline error associations

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/aj3lib/pipelines/RT.md` | Created | All 7 =RT.* execution pipelines (261 lines) |
| `docs/user/aj3lib/types/rt.md` | Created | #Code, #PyEnv, #RsEnv type definitions (72 lines) |
| `docs/user/aj3lib/pipelines/W.md` | Modified | =W.Python → =W.RT with flexible fields |
| `docs/user/aj3lib/types/types.md` | Modified | Added Runtime category + types to hierarchy |
| `docs/user/aj3lib/errors/errors.md` | Modified | Added {!} !RT block + pipeline associations |
| `docs/user/aj3lib/INDEX.md` | Modified | Added =RT, !RT, #Code/#PyEnv/#RsEnv rows |
| `docs/user/AJ3LIB.md` | Modified | Added =RT link in pipeline section |
| `docs/user/syntax/blocks.md` | Modified | Updated [C] example to use =RT.Python.Script.Inline |
| `.paul/STATE.md` | Modified | Active issue + loop position tracking |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Script vs Bind by binding origin | Script: Polyglot injects vars; Bind: foreign code calls pull/push | Clear mental model for users choosing between modes |
| .Inline/.File split for all modes | Consistent pattern; .CLI inherently file-based | 7 total variants instead of 4 |
| .CLI uses =W.Polyglot, not =W.RT | No language runtime needed for compiled binaries | Simpler setup for CLI mode |
| Compiler validates Function/Script only | CLI binary and Bind pull/push are opaque | Safety where possible; freedom where needed |
| <Bind#serial not #Map | Serial is Polyglot's universal exchange format | Consistent with rest of aj3lib |
| Code-last in IO declarations | Stylistic: [C] blocks at end reads better | Convention, not enforcement |
| Multiple [W] bracket semantics | Setup forward, cleanup reverse — standard resource pattern | Enables multi-runtime pipelines |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 0 | — |
| Scope additions | 0 | — |
| Deferred | 0 | — |

**Total impact:** None — plan executed exactly as written

## Issues Encountered

None

## Skill Audit

Skill audit: No required skills configured ✓

## Next Phase Readiness

**Ready:**
- Complete =RT subsystem documented and cross-referenced
- All index files updated; no broken links
- draft.md contains approved examples for reference

**Concerns:**
- =W.Python remains in `docs/technical/brainstorming/marker-declarations.md` (migrate on touch)
- EBNF grammar may need =RT production rules (separate issue if needed)

**Blockers:**
- None

---
*Phase: issue-76-78-rt-runtime-execution, Plan: 01*
*Completed: 2026-04-01*
