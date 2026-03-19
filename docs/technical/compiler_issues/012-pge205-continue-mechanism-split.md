---
issue: "012"
title: PGE-205 *Continue mechanism warrants its own rule
related: PGE-205 (Rule 2.5), PGW-205
priority: enhancement
status: resolved
created: 2026-03-19
resolved: 2026-03-19
---

# 012 — PGE-205 *Continue mechanism warrants its own rule

## Resolution

**PGE-205 split + `*Continue` semantics corrected.**

1. **PGE-205** narrowed to core Failed state semantics — no `*Continue` content
2. **PGE-207** created for `*Continue` with corrected semantics:
   - `>FallBack` provides alternative value in Final state (not `>IsFailed` boolean)
   - Variable state IS flow control — Final triggers downstream, Failed blocks
3. **PGW-205** rewritten: warns about pipeline termination (no replacement, no `*Continue`), not about unhandled booleans
4. Removed incorrect `>IsFailed` boolean pattern from all rule files

## Original Problem

PGE-205 (Failed Is Terminal) covers three distinct concerns in a single rule:

1. **Core constraint** — a Failed variable cannot receive further pushes
2. **`*Continue` opt-out mechanism** — a collector that allows the pipeline to continue after error handling, producing a `>IsFailed` boolean
3. **Failure propagation** — open point about whether failure propagates up the call chain automatically

The `*Continue` mechanism is substantial: it introduces a new collector type, a new output variable (`>IsFailed`), and a new flow control pattern (conditional branching on failure status). This is not a refinement of "Failed is terminal" — it's a separate feature with its own syntax, semantics, and warning (PGW-205).

Additionally, PGE-205 has an unresolved open point about failure propagation that affects the entire error handling model.

## Affected Rules

- `compile-rules/PGE/PGE-205-failed-is-terminal.md`
- `compile-rules/PGW/PGW-205-failed-variable-usage.md`

## Proposed Resolution

**Split into focused rules:**

| Current | Proposed | Scope |
|---------|----------|-------|
| PGE-205 (all) | PGE-205 | Core: Failed variable cannot receive pushes; default `[!]` behavior ends pipeline |
| PGE-205 (*Continue section) | PGE-207 | `*Continue` collector: syntax, `>IsFailed` output, flow control semantics |
| PGW-205 | PGW-205 | Warning when `*Continue` output not handled (remains, but references PGE-207 instead of PGE-205) |
| PGE-205 (open point) | New compiler issue | Failure propagation across call chains |

**PGE-205 after split:**
- Statement: Failed is terminal, no further pushes
- Default behavior: `[!]` without replacement ends the pipeline
- `[!]` with replacement value: variable becomes Final
- Cross-reference to PGE-207 for `*Continue`

**PGE-207 (new):**
- Statement: `*Continue` is a collector inside `[!]` that opts into continuing after error
- Syntax: `[*] *Continue >IsFailed >> $var`
- `>IsFailed` must be wired (PGW-205 if not)
- Downstream behavior: IO implicit gates prevent triggering on Failed inputs

## See also

- [PGE-205 — Failed Is Terminal](../compile-rules/PGE/PGE-205-failed-is-terminal.md)
- [PGW-205 — Failed Variable Usage](../compile-rules/PGW/PGW-205-failed-variable-usage.md)
- [001 — Static detection of unhandled Failed variables](001-static-failed-detection.md) — resolved design that introduced `*Continue`
