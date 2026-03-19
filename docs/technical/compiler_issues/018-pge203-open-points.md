---
issue: "018"
title: PGE-203 has unresolved open points
related: PGE-203 (Rule 2.3)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 018 — PGE-203 has unresolved open points

## Problem

PGE-203 (Final Is Push-Once) has two unresolved open points that affect compiler behavior:

### Open Point 1 — Data fields

> Does this apply to `{#}` data fields the same way as `$` variables?

If a `{#}` data definition has a field marked Final, can it be pushed to more than once during construction? The current rule only discusses `$` pipeline variables. Data field semantics may differ.

### Open Point 2 — Per-execution-path semantics

> If one conditional branch promotes to Final and another does not, what is the variable's state after the conditional?

Example:
```polyglot
[?] $mode
   [?] "fast"
      [r] $result << "done"    [ ] promotes $result to Final
   [?] "slow"
      [r] $result <~ "pending" [ ] default push, $result stays Default
   [?] *?
      [r] =NoOp
```

After the `[?]` block, is `$result` Final or Default? This affects whether subsequent pushes are allowed.

## Affected Rules

- `compile-rules/PGE/PGE-203-final-is-push-once.md`

## Proposed Resolution

1. **Data fields:** Clarify that Final semantics apply uniformly — `{#}` fields follow the same push-once rule as `$` variables
2. **Conditional paths:** Define that Final promotion is per-path — the compiler must prove ALL paths promote to Final before treating the variable as Final after the conditional. If any path leaves it Default, it remains Default after the block.

## See also

- [PGE-203 — Final Is Push-Once](../compile-rules/PGE/PGE-203-final-is-push-once.md)
