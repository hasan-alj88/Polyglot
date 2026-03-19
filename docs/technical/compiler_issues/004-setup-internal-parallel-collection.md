---
issue: "004"
title: Can [\] setup have both [p] and [*] internally?
related: PGE-304 (Rule 3.4)
priority: design-decision
status: resolved
created: 2026-03-18
resolved: 2026-03-19
---

# 004 — Can `[\]` setup have both `[p]` and `[*]` internally?

## Question

Rule 3.4 allows same-section pairing. If `[p]` starts in `[\]` and `[*]` is also in `[\]` (before the body starts), is this valid?

## Resolution

**Yes.** `[\]` setup and `[/]` cleanup form one scope with a mandatory sequential body execution between them. Parallel branches launched in either section must be collected somewhere within that scope before it ends — otherwise it's a compile error.

Valid collection points for parallels launched in `[\]` or `[/]`:
- Same-section internal (`[p]` and `[*]` both in `[\]`)
- Cross-section (`[p]` in `[\]`, `[*]` in `[/]`)
- Same-section internal in `[/]` (`[p]` and `[*]` both in `[/]`)

No rule changes needed — same-section pairing already covers this. Rule 3.4's existing VALID example demonstrates `[\]`-internal pairing.

## See also

- [PGE-304 — Section-Boundary Pairing](../compile-rules/PGE/PGE-304-section-boundary-pairing.md)
