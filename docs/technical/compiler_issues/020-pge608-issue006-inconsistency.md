---
issue: "020"
title: PGE-608 still requires *? despite Issue 006 resolution
related: PGE-608 (Rule 6.8), PGE-605 (Rule 6.5)
priority: cleanup
status: resolved
created: 2026-03-19
---

# 020 — PGE-608 still requires *? despite Issue 006 resolution

## Problem

PGE-608 (Compound Condition Exhaustiveness) has an open point stating:

> When compiler issue 006 is fully resolved, the compiler may be able to prove compound exhaustiveness statically, making `*?` optional in some cases.

Issue 006 IS resolved — the algorithm is documented in `docs/technical/plan/TODO/006-compound-exhaustiveness-algorithm.md`. However, PGE-608 still unconditionally requires `*?` catch-all for all compound conditions.

This creates an inconsistency: the algorithm to prove exhaustiveness exists, but the rule doesn't use it. Either:
1. The open point should be resolved (remove it, update the rule to allow proven-exhaustive compounds without `*?`)
2. Or the open point should be reworded to clarify that the algorithm exists but implementation is deferred

## Affected Rules

- `compile-rules/PGE/PGE-608-compound-condition-exhaustiveness.md`
- `docs/technical/plan/TODO/006-compound-exhaustiveness-algorithm.md` (algorithm spec)

## Proposed Resolution

**Option A — Keep `*?` mandatory, close the open point:**

Update PGE-608's open point to say: "The exhaustiveness algorithm (Issue 006) exists but compound condition analysis is complex enough that `*?` remains mandatory as a safety net. The algorithm may be used in future to emit a PGW warning when `*?` is provably unreachable."

**Option B — Allow proven exhaustiveness:**

If the algorithm can prove exhaustiveness, remove the `*?` requirement for those cases. Add VALID example showing compound conditions without `*?` when provably exhaustive. This requires PGE-605 (overlap detection) to also be implemented.

Option A is safer for now — compound exhaustiveness proofs are complex and `*?` is low cost.

## See also

- [PGE-608 — Compound Condition Exhaustiveness](../compile-rules/PGE/PGE-608-compound-condition-exhaustiveness.md)
- [Issue 006 — Compound Condition Exhaustiveness](006-compound-condition-exhaustiveness.md)
