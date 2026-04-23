---
phase: 345-wildcard-auto-wire
plan: 01
subsystem: language-spec
tags: [ebnf, auto-wire, compile-rules, type-identity, io-parameters]

requires:
  - phase: 340-retire-chain-operator
    provides: labeled [-] calls with operation labels; chain auto-wire left recoverable via wildcard
provides:
  - EBNF wildcard_output / wildcard_input productions (§7.4)
  - call_io_line wildcard auto-wire alternative (§10.2)
  - PGE08001/08002/08003/PGW08001 rewritten for wildcard context
  - docs/user/syntax/io/auto-wire.md user-facing spec
  - 4 edge cases (EC-10.17 – EC-10.20)
  - Glossary entry for "Wildcard auto-wire"
affects: [future language-spec issues that introduce new IO wiring forms; compiler implementation of IO resolution]

tech-stack:
  added: []
  patterns:
    - Wildcard IO pair as bijective type-topology matching (compile-time, not runtime)
    - Completion-wait semantics inherited from Final-inputs trigger model

key-files:
  created:
    - docs/user/syntax/io/auto-wire.md
  modified:
    - docs/technical/ebnf/07-io-parameters.md
    - docs/technical/ebnf/10-execution.md
    - docs/technical/compile-rules/PGE/PGE08001-auto-wire-type-mismatch.md
    - docs/technical/compile-rules/PGE/PGE08002-auto-wire-ambiguous-type.md
    - docs/technical/compile-rules/PGE/PGE08003-auto-wire-unmatched-parameter.md
    - docs/technical/compile-rules/PGW/PGW08001-auto-wire-succeeded.md
    - docs/user/syntax/io.md
    - docs/technical/edge-cases/10-execution.md
    - docs/technical/edge-cases/INDEX.md
    - docs/audit/reference/glossary.md

key-decisions:
  - "PGE08003 reframed as 'Port Count Mismatch' — bijective-and-onto requires identical cardinality; count check runs before type pairing"
  - "Completion-wait semantics made explicit: <* << $A>* implicitly requires all of $A's outputs Final before target triggers — no separate (*) barrier needed"
  - "Wildcard symbols reuse 'collect-all' intuition: <* = all inputs, $Label>* = all outputs — consistent with *All / *First / (*) collectors"

patterns-established:
  - "Wildcard IO productions live in §7.4, callsite alternative in §10.2; prose rules cross-reference bidirectionally"
  - "Compile rules for wildcard auto-wire all reference docs/user/syntax/io/auto-wire.md as the user-facing anchor"

duration: ~40min
started: 2026-04-23T04:20:00Z
completed: 2026-04-23T04:55:00Z
---

# Phase 345 Plan 01: Wildcard Auto-Wire Summary

**Recovered chain-style auto-wire as a general `<* << $Label>*` feature: EBNF productions in §7.4 + §10.2, four compile rules rewritten from retired `->` chain context to labeled `[-]` call context, user doc, edge cases, and glossary entry.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~40min |
| Started | 2026-04-23T04:20:00Z |
| Completed | 2026-04-23T04:55:00Z |
| Tasks | 2/2 completed (both autonomous) |
| Files modified | 11 (1 created, 10 edited) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: EBNF defines wildcard IO productions | PASS | `wildcard_output`, `wildcard_input` in §7.4; `call_io_line` extended in §10.2; bidirectional prose cross-refs added |
| AC-2: Compile rules rewritten for wildcard auto-wire context | PASS | PGE08001/08002/08003/PGW08001 all use labeled `[-]` calls + `<* << $Label>*`; grep confirms zero `->` chain syntax and zero `>N.`/`<N.` step numbering remain across all 6 files |
| AC-3: User documentation explains wildcard auto-wire | PASS | `docs/user/syntax/io/auto-wire.md` created with `automation-builder` audience; covers `<*`, `$Label>*`, bijective matching, completion-wait, failure modes, valid/explicit alternatives; linked from `io.md` |
| AC-4: Edge cases cover valid and invalid wildcard auto-wire | PASS | EC-10.17 (valid), EC-10.18 (PGE08001), EC-10.19 (PGE08002), EC-10.20 (PGE08003); all use labeled `[-]` calls; INDEX.md range and total (137→141) updated |

## Accomplishments

- **Wildcard auto-wire formally defined in EBNF** — `wildcard_output ::= variable_id ">*"` and `wildcard_input ::= "<*"` as §7.4 productions; `call_io_line` extended with the `(-) <* << $Label>*` alternative in §10.2, with bidirectional prose cross-refs.
- **All four auto-wire compile rules rewritten** — PGE08001 (type mismatch), PGE08002 (ambiguous type), PGE08003 (renamed from "Unmatched Parameter" to "Port Count Mismatch"), PGW08001 (auto-wire succeeded). Every example updated from the retired `->` chain form to labeled `[-]` calls with `(-) $Label` / `<* << $Label>*`.
- **User-facing spec published** — `docs/user/syntax/io/auto-wire.md` with the canonical form, bijective-matching table, completion-wait explanation, and six worked examples (valid, multi-port unique, explicit alternative, PGE08001, PGE08002, PGE08003).
- **Completion-wait semantics called out explicitly** — wildcard form naturally waits for all of `$Label`'s outputs to be Final before the target triggers; no `(*)` barrier, `[/]`, or sync marker needed. Fall-out of the existing trigger model, but load-bearing for users deciding between wildcard vs per-port.

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/syntax/io/auto-wire.md` | Created | User-facing spec — form, bijective matching, completion-wait, 6 examples |
| `docs/technical/ebnf/07-io-parameters.md` | Modified | Added §7.4 Wildcard IO with two productions + prose rules |
| `docs/technical/ebnf/10-execution.md` | Modified | Extended `call_io_line` with wildcard alternative + prose rule |
| `docs/technical/compile-rules/PGE/PGE08001-auto-wire-type-mismatch.md` | Modified | Fully rewritten for wildcard auto-wire context |
| `docs/technical/compile-rules/PGE/PGE08002-auto-wire-ambiguous-type.md` | Modified | Fully rewritten for wildcard auto-wire context |
| `docs/technical/compile-rules/PGE/PGE08003-auto-wire-unmatched-parameter.md` | Modified | Rewritten; renamed to "Port Count Mismatch" (code unchanged) |
| `docs/technical/compile-rules/PGW/PGW08001-auto-wire-succeeded.md` | Modified | Rewritten; valid/warning examples use labeled calls |
| `docs/user/syntax/io.md` | Modified | Sections-table row linking to auto-wire.md |
| `docs/technical/edge-cases/10-execution.md` | Modified | Added EC-10.17 (valid bijection) + EC-10.18/19/20 (invalid cases) |
| `docs/technical/edge-cases/INDEX.md` | Modified | S10 range + coverage matrix + total (137→141) |
| `docs/audit/reference/glossary.md` | Modified | Added "Wildcard auto-wire" entry with definition + failure modes |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| PGE08003 reframed from "Unmatched Parameter" to "Port Count Mismatch" | Bijective-and-onto mapping requires identical cardinality — a count mismatch is a structural failure checked before type pairing, not a leftover after pairing | Rule code unchanged (PGE08003); frontmatter `name:` updated; detection order clearer |
| Completion-wait is implicit, not a separate marker | Trigger model already requires Final inputs to fire; wildcard's "all outputs Final" condition falls out naturally. Adding a barrier would be redundant and invent new syntax | Documented in both §7.4 and §10.2 prose; called out in auto-wire.md |
| Symbol reuse: `*` = "collect all" | `*All`, `*First`, `(*) <<`, `(*) >>` already establish the intuition. `<*` and `$Label>*` stay on-brand as "all inputs" / "all outputs" | Noted once in auto-wire.md symbology note to pre-empt confusion with runtime collectors |

**No decision record created** — per plan boundaries, `docs/audit/decisions/2026-04-22-retire-chain-operator.md` (#340) already documents #345 as the follow-up. Decision recorded in SUMMARY + STATE only.

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 0 | — |
| Scope additions | 0 | — |
| Deferred | 0 | — |

**Total impact:** None. Plan executed exactly as written; PGE08003 rename was anticipated in the plan's action description ("PGE08003: Rewrite as port count mismatch").

### Deferred Items

None.

## Issues Encountered

None.

## Verification

All 8 automated checks passed:

| Check | Result |
|-------|--------|
| V1: EBNF §7.4 productions present | ✓ |
| V2: EBNF §10.2 `call_io_line` wildcard alternative | ✓ |
| V3: `auto-wire.md` exists with `automation-builder` frontmatter | ✓ |
| V4: `io.md` links to auto-wire | ✓ |
| V5: 4 wildcard edge cases (EC-10.17-20) | ✓ |
| V6: Glossary entry present | ✓ |
| V7: Zero retired `->` chain / step-numbering across 6 Task-1 files | ✓ |
| V8: INDEX.md range + total updated (137→141) | ✓ |

## Next Phase Readiness

**Ready:**
- `<* << $Label>*` fully specified end-to-end: EBNF → compile rules → user docs → edge cases → glossary
- Chain retirement loop (#340 + #345) is now complete — sequential composition has a clear story (labeled `[-]` calls) with optional wildcard shorthand

**Concerns:**
- `docs/user/concepts/pipelines/chains.md` and `docs/user/syntax/io/chain-io.md` still reference chain patterns in their "retired" status — intentional per plan boundaries, but should be periodically re-checked for any remaining stale references in other docs
- `docs/user/syntax/io/operation-labels.md` example (line 26-34) still shows `(.)` step labels with `->` — outside this plan's scope, but a candidate for a follow-up audit sweep

**Blockers:**
- None

## Next Step

Run `/paul:merge` to merge `design/issue-345-wildcard-auto-wire` into `main` and close GitHub issue #345.

---
*Phase: 345-wildcard-auto-wire, Plan: 01*
*Completed: 2026-04-23*
