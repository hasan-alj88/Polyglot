---
issue: "016"
title: PGE-605 rule file missing
related: PGE-605 (Rule 6.5), PGE-604 (Rule 6.4), PGE-601 (Rule 6.1)
priority: critical
status: resolved
created: 2026-03-19
---

# 016 — PGE-605 rule file missing

## Problem

COMPILE-RULES.md lists `PGE-605 | 6.5 | Compound Condition Overlap` in the error code reference table, but no corresponding rule file exists at `compile-rules/PGE/PGE-605-compound-condition-overlap.md`.

PGE-605 was created when Issue 006 (compound exhaustiveness) was resolved — the error code was registered in the table but the rule file was never written. The algorithm for compound condition overlap detection is documented in `docs/technical/plan/TODO/006-compound-exhaustiveness-algorithm.md` but has no formal rule file with VALID/INVALID examples.

PGE-604 (Numeric Range Overlap) exists as a parallel rule for numeric ranges. PGE-605 should follow the same structure but for compound conditions using `[&]`/`[+]`/`[^]` operators.

## Affected Rules

- `COMPILE-RULES.md` — lists PGE-605 (file missing)
- `compile-rules/PGE/PGE-604-numeric-range-overlap.md` — parallel rule (template)
- `compile-rules/PGE/PGE-601-conditional-must-be-exhaustive.md` — dispatch table should reference PGE-605

## Proposed Resolution

Create `compile-rules/PGE/PGE-605-compound-condition-overlap.md` with:
- Statement: compound conditions using `[&]`/`[+]`/`[^]` must not overlap — if two branches can match the same input, PGE-605 fires
- VALID examples: non-overlapping compound conditions
- INVALID examples: overlapping compound conditions
- Reference the algorithm from Issue 006

## See also

- [PGE-604 — Numeric Range Overlap](../compile-rules/PGE/PGE-604-numeric-range-overlap.md)
- [PGE-601 — Conditional Must Be Exhaustive](../compile-rules/PGE/PGE-601-conditional-must-be-exhaustive.md)
- [Issue 006 — Compound Condition Exhaustiveness](006-compound-condition-exhaustiveness.md)
