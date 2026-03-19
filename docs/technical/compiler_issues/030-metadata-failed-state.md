---
issue: "030"
title: Metadata access on Failed-state variables undefined
related: PGE-205 (Rule 2.5), PGE-206 (Rule 2.6), PGE-303 (Rule 3.3)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 030 — Metadata access on Failed-state variables undefined

## Problem

PGE-206 defines that `live` metadata fields (like `%state`) are pull-only. PGE-303 (after Issue 003 narrowing) defines when `live` metadata is accessible during parallel execution. However, no rule addresses metadata access on variables in Failed state.

### Questions

1. **Can you pull `%state` from a Failed variable?**
   Logically yes — `%state` would return `"failed"`. But this isn't stated.

2. **Can you pull other metadata (`%description`, `%version`) from a Failed variable?**
   Static metadata should still be accessible. But is it?

3. **What about `live` metadata on Failed variables?**
   If a variable enters Failed state mid-pipeline, are its `live` metadata fields frozen at the point of failure, or do they become inaccessible?

This matters for error handling — a `[!]` handler may want to inspect the failed variable's metadata to decide recovery strategy.

## Affected Rules

- `compile-rules/PGE/PGE-205-failed-is-terminal.md` (Failed state definition)
- `compile-rules/PGE/PGE-206-live-metadata-fields-are-pull-only.md` (metadata access)
- `compile-rules/PGE/PGE-303-pull-isolation-until-collection.md` (metadata timing)

## Proposed Resolution

Add a note to PGE-206 clarifying:
1. Static metadata (`%description`, `%version`, etc.) is always accessible regardless of variable state
2. `%state` is always accessible and returns the current state (including `"failed"`)
3. `live` metadata on Failed variables is frozen at the point of failure — last known values remain readable

No new error code needed — this is a clarification of existing rules, not a new constraint.

## See also

- [PGE-205 — Failed Is Terminal](../compile-rules/PGE/PGE-205-failed-is-terminal.md)
- [PGE-206 — Live Metadata Fields Are Pull-Only](../compile-rules/PGE/PGE-206-live-metadata-fields-are-pull-only.md)
