---
phase: 281-rt-input-binding
plan: 01
subsystem: spec
tags: [run-pipelines, record-binding, compile-rules, foreign-code, native-dispatch]

requires:
  - phase: issue-76-78-rt-runtime-execution
    provides: original -RT.* pipeline specification
  - phase: 275-collection-redesign
    provides: "#Record type replacing #Map"
provides:
  - "-Run.* pipeline documentation (Function, Script, CLI, Bind)"
  - "Seven binding compile rules PGE01033-PGE01039"
  - "Record-based native variable binding specification"
affects: [RT-deprecation-pass, EBNF-updates, error-namespace-rename]

tech-stack:
  added: []
  patterns:
    - "#Record field names = native variable names"
    - "#Code:Source enum with %##Active one for inline/file"

key-files:
  created:
    - docs/user/pglib/pipelines/Run/Function.md
    - docs/user/pglib/pipelines/Run/Script.md
    - docs/user/pglib/pipelines/Run/CLI.md
    - docs/user/pglib/pipelines/Run/Bind.md
    - docs/technical/compile-rules/PGE/PGE01033-unbound-script-variable.md
    - docs/technical/compile-rules/PGE/PGE01034-unbound-script-output.md
    - docs/technical/compile-rules/PGE/PGE01035-unbound-function-argument.md
    - docs/technical/compile-rules/PGE/PGE01036-unbound-function-kwarg.md
    - docs/technical/compile-rules/PGE/PGE01037-bind-schema-mismatch.md
    - docs/technical/compile-rules/PGE/PGE01038-code-source-conflict.md
    - docs/technical/compile-rules/PGE/PGE01039-cli-non-string-argument.md
  modified:
    - docs/user/pglib/pipelines/Run/INDEX.md
    - docs/technical/COMPILE-RULES.md

key-decisions:
  - "PGE01033-01039 numbering (01029-01032 already taken by existing rules)"
  - "Record field names map exactly to native variable names (primary #281 answer)"
  - "#Code:Source enum unifies .Inline/.File into single pipeline variant"

patterns-established:
  - "Record binding pattern: field names = native variables, compiler-validated"
  - "Code source as active-field-one enum, not separate pipeline variants"

duration: 15min
started: 2026-04-13T09:00:00Z
completed: 2026-04-13T09:15:00Z
---

# Issue #281 Plan 01: -Run.* Pipeline Docs + Binding Compile Rules Summary

**Four -Run.* pipeline files and seven binding compile rules completing the Record-based native variable binding specification.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Tasks | 2 completed |
| Files created | 11 |
| Files modified | 2 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Four -Run.* pipeline files with complete IO spec | Pass | Function.md, Script.md, CLI.md, Bind.md — all use #Record, #Code:Source |
| AC-2: Seven binding compile rules documented | Pass | PGE01033-01039 created with examples; COMPILE-RULES.md index updated |
| AC-3: Run/INDEX.md PGE numbering corrected | Pass | All references updated from 01029-01035 to 01033-01039 |

## Accomplishments

- Created 4 pipeline documentation files answering issue #281's core question: Record field names become native variable names, compiler-validated
- Created 7 compile rules enforcing binding correctness at compile time (inline code only; file-based deferred to runtime)
- Fixed PGE numbering conflict (01029-01032 already assigned to existing rules)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/pipelines/Run/Function.md` | Created | Structured function call with `<arg`/`<kwarg` Record binding |
| `docs/user/pglib/pipelines/Run/Script.md` | Created | Polyglot-controlled binding — `<Bind` field names = native vars |
| `docs/user/pglib/pipelines/Run/CLI.md` | Created | Binary invocation with string-only Record args |
| `docs/user/pglib/pipelines/Run/Bind.md` | Created | Foreign-code-controlled `pull()`/`push()` binding |
| `docs/technical/compile-rules/PGE/PGE01033-*.md` | Created | Unbound Script Variable |
| `docs/technical/compile-rules/PGE/PGE01034-*.md` | Created | Unbound Script Output |
| `docs/technical/compile-rules/PGE/PGE01035-*.md` | Created | Unbound Function Argument |
| `docs/technical/compile-rules/PGE/PGE01036-*.md` | Created | Unbound Function Kwarg |
| `docs/technical/compile-rules/PGE/PGE01037-*.md` | Created | Bind Schema Mismatch |
| `docs/technical/compile-rules/PGE/PGE01038-*.md` | Created | Code Source Conflict |
| `docs/technical/compile-rules/PGE/PGE01039-*.md` | Created | CLI Non-String Argument |
| `docs/user/pglib/pipelines/Run/INDEX.md` | Modified | PGE renumbering 01029→01033 etc. |
| `docs/technical/COMPILE-RULES.md` | Modified | Added 7 new rules to index |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| PGE01033-01039 numbering | PGE01029-01032 already assigned to existing rules | New rules start at next available number |
| Scope limited to new -Run.* docs only | RT deprecation is a separate cross-cutting concern | Follow-up plan needed for RT→Run migration across ~40 files |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- -Run.* specification is self-consistent (INDEX + 4 pipelines + 7 compile rules)
- Ready for follow-up plan: RT deprecation markers + cross-reference migration

**Concerns:**
- ~40 files still reference -RT.* / -W.RT / !RT — needs a dedicated migration pass
- PGE01027 (empty foreign code) still references -RT.* in its examples

**Blockers:**
- None

---
*Phase: 281-rt-input-binding, Plan: 01*
*Completed: 2026-04-13*
