---
issue: "028"
title: Retired PGE-204 still in main error code table
related: PGE-204 (retired), PGE-203 (Rule 2.3)
priority: cleanup
status: resolved
created: 2026-03-19
---

# 028 — Retired PGE-204 still in main error code table

## Problem

PGE-204 (Default Allows Exactly One More Push) was retired and merged into PGE-203 (Final Is Push-Once). The COMPILE-RULES.md table shows:

```
| PGE-204 | 2.4 | ~~Default Allows Exactly One More Push~~ *(retired → PGE-203)* |
```

The retired entry is in the main error code reference table alongside active codes. This can confuse developers scanning the table for valid error codes.

## Affected Rules

- `COMPILE-RULES.md` — error code table

## Proposed Resolution

**Option A — Move to retired section:**

Add a "Retired Codes" section at the bottom of COMPILE-RULES.md and move PGE-204 there. This preserves the historical record while cleaning up the active table.

**Option B — Remove entirely:**

Delete the PGE-204 row. The retirement is documented in Issue 008 and git history. Cleanest table but loses the redirect hint for anyone searching for PGE-204.

Option A preferred — keeps the redirect while decluttering the active table.

## See also

- [PGE-203 — Final Is Push-Once](../compile-rules/PGE/PGE-203-final-is-push-once.md)
- [Issue 008 — PGE-203/PGE-204 Overlap](008-pge203-pge204-overlap.md)
