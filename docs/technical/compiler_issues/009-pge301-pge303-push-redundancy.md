---
issue: "009"
title: PGE-303 push case is redundant with PGE-301
related: PGE-301 (Rule 3.1), PGE-303 (Rule 3.3)
priority: cleanup
status: resolved
created: 2026-03-19
resolved: 2026-03-19
---

# 009 — PGE-303 push case is redundant with PGE-301

## Resolution

**PGE-303 narrowed to pull isolation only.**

1. Statement rewritten to cover pulls only — push violations deferred to PGE-301
2. Rule renamed from "Variable Isolation Until Collection" to "Pull Isolation Until Collection"
3. Push INVALID example relabeled as PGE-301 cross-reference
4. Detection description updated to mention only pulls

## Original Problem

PGE-301 (No Push Across Parallel Boundaries) prohibits pushing into variables that originate inside a `[p]` parallel scope from outside that scope. PGE-303 (Variable Isolation Until Collection) prohibits both pushing to AND pulling from parallel-scoped variables before the `[*]` collector.

The push prohibition in PGE-303 is entirely subsumed by PGE-301:
- PGE-301 catches any push from outside the parallel scope — period
- PGE-303's push case is a subset (push before collection specifically)

PGE-303's INVALID example #2 shows a push into a parallel variable and notes in a comment "(also PGE-301)" — acknowledging the redundancy but not resolving it in the rule statement.

PGE-303's unique contribution is **pull isolation** — you cannot pull from a parallel-scoped variable before the collector synchronizes it. This is not covered by PGE-301.

## Affected Rules

- `compile-rules/PGE/PGE-301-no-push-across-parallel.md`
- `compile-rules/PGE/PGE-303-variable-isolation-until-collection.md`

## Proposed Resolution

1. Narrow PGE-303's statement to focus on **pull isolation only**: "A variable produced by a `[p]` parallel pipeline cannot be pulled from in the prime pipeline before the `[*]` collector for that parallel has executed."
2. Remove or relabel the push INVALID example in PGE-303 — it should cross-reference PGE-301 instead of claiming PGE-303 jurisdiction
3. Add a note to PGE-303: "Push violations are caught by PGE-301 regardless of collection status"

## See also

- [PGE-301 — No Push Across Parallel Boundaries](../compile-rules/PGE/PGE-301-no-push-across-parallel.md)
- [PGE-303 — Variable Isolation Until Collection](../compile-rules/PGE/PGE-303-variable-isolation-until-collection.md)
