---
issue: "008"
title: PGE-203 and PGE-204 have overlapping scope
related: PGE-203 (Rule 2.3), PGE-204 (Rule 2.4)
priority: cleanup
status: resolved
created: 2026-03-19
resolved: 2026-03-19
---

# 008 — PGE-203 and PGE-204 have overlapping scope

## Resolution

**Option B selected — PGE-204 folded into PGE-203.**

1. PGE-203's statement expanded to cover Default re-assignment (`<~ → <~`)
2. PGE-204's VALID examples (Default→Final, Default pull, per-branch promotion) moved to PGE-203
3. PGE-204's open point (per-execution-path semantics) preserved in PGE-203
4. PGE-204 file marked as retired with redirect to PGE-203
5. Error code PGE-204 retired in COMPILE-RULES.md

## Original Problem

PGE-204 (Default Allows Exactly One More Push) states: "A second push after the Default-to-Final promotion is rejected (fires PGE-203, same as Rule 2.3)." Its first INVALID example shows Default → Final → push, which fires **PGE-203**, not PGE-204.

This means PGE-204 only catches **one unique case**: double default assignment (`<~ → <~`). Every other violation PGE-204 could catch — including the most common scenario (pushing into a variable that was Default but has since been promoted to Final) — already fires PGE-203.

The overlap causes confusion:
1. PGE-204's INVALID examples include cases that fire PGE-203, not PGE-204
2. A developer reading PGE-204 may think it covers more than it does
3. The rule's statement is broader than its unique enforcement scope

## Affected Rules

- `compile-rules/PGE/PGE-203-final-is-push-once.md`
- `compile-rules/PGE/PGE-204-default-allows-one-more-push.md`

## Proposed Resolution

**Option A — Narrow PGE-204 (recommended):**
1. Rewrite PGE-204's statement to cover ONLY the double-default case: "A variable already in Default state cannot receive another default assignment (`<~` or `~>`)"
2. Remove the INVALID example that fires PGE-203 (Default → Final → push)
3. Add a cross-reference: "Once a Default variable is promoted to Final, further pushes are caught by PGE-203"

**Option B — Fold into PGE-203:**
1. Add the double-default case as a sub-section of PGE-203
2. Retire PGE-204 as a standalone rule
3. Redirect PGE-204 code to PGE-203 in the error code table

Option A is preferred because the double-default case has a distinct user-facing message ("cannot re-default a variable") that is clearer than PGE-203's generic "push into Final."

## See also

- [PGE-203 — Final Is Push-Once](../compile-rules/PGE/PGE-203-final-is-push-once.md)
- [PGE-204 — Default Allows Exactly One More Push](../compile-rules/PGE/PGE-204-default-allows-one-more-push.md)
