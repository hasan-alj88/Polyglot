---
issue: "024"
title: No IO signature validation for expand/collect operators
related: PGE-302 (Rule 3.2), PGE-305 (Rule 3.5), PGE-306 (Rule 3.6)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 024 — No IO signature validation for expand/collect operators

## Problem

Expand operators (`~ForEach.Array`, `~ForEach.Serial`, `~ForEach.Level`) and collect operators (`*Into.*`, `*Agg.*`, `*All`, `*First`, `*Nth`, `*Continue`) have specific IO requirements, but no compiler rule validates that the correct IO signatures are provided.

### Missing validations

1. **Expand operator inputs:** `~ForEach.Array` requires `<Array` input, `~ForEach.Serial` requires `<Serial`/`<key`/`<value`, `~ForEach.Level` requires `<level`. No rule checks these.

2. **Collect operator outputs:** `*Agg.Sum` expects numeric input, `*Into.Serial` expects `<key`/`<value`. No rule validates the operator receives the right IO.

3. **Operator placement:** No rule prevents using a collect operator outside a `~ForEach` loop or parallel sync block.

4. **`[*] <<` and `[*] >>` forms:** `*All` uses `[*] <<` only (sync). Race collectors (`*First`, `*Nth`) use `[*] <<` + `[*] >>`. No rule validates correct form usage per collector type.

## Affected Rules

- `compile-rules/PGE/PGE-302-parallel-output-must-be-collected.md` (related but doesn't cover IO signatures)
- `compile-rules/PGE/PGE-305-b-has-no-collectible-output.md` (related)
- `compile-rules/PGE/PGE-306-race-collector-type-homogeneity.md` (related)

## Proposed Resolution

Create new rules in the 3xx category:

| Code | Rule | Name |
|------|------|------|
| PGE-307 | 3.7 | Expand Operator Input Mismatch |
| PGE-308 | 3.8 | Collect Operator IO Mismatch |

Defer placement rules and `[*] <<`/`[*] >>` form validation to separate issues if needed.

## See also

- [PGE-302 — Parallel Output Must Be Collected](../compile-rules/PGE/PGE-302-parallel-output-must-be-collected.md)
- [PGE-306 — Race Collector Type Homogeneity](../compile-rules/PGE/PGE-306-race-collector-type-homogeneity.md)
