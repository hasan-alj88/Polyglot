---
issue: "011"
title: PGE-104 macro element whitelist is incomplete
related: PGE-104 (Rule 1.4)
priority: enhancement
status: resolved
created: 2026-03-19
resolved: 2026-03-19
---

# 011 — PGE-104 macro element whitelist is incomplete

## Resolution

**Complete element whitelist added to PGE-104.**

- **Allowed:** `[{]`, `[}]`, `[\]`, `[/]`, `[r]`, `[W]`, `[?]`, `[!]`
- **Prohibited:** `[t]`, `[Q]`, `[=]`, `[p]`, `[b]`, `[*]`
- Added VALID examples for conditionals and error handlers in macro setup
- Added INVALID example for `[p]` parallel in macro

## Original Problem

PGE-104 (Macro Structural Constraints) explicitly prohibits `[t]`, `[Q]`, and pipeline-level `[=]` inside `{M}` macros. It lists valid elements as: `[{]`, `[}]`, `[\]`, `[/]`, execution markers, and composite `[W]` calls.

However, several elements are not addressed — the compiler behavior for these is undefined:

| Element | Status in PGE-104 |
|---------|-------------------|
| `[t]` trigger | Explicitly prohibited |
| `[Q]` queue | Explicitly prohibited |
| `[=]` pipeline IO | Explicitly prohibited |
| `[?]` conditionals | **Not addressed** |
| `[p]` parallel | **Not addressed** |
| `[b]` fire-and-forget | **Not addressed** |
| `[!]` error blocks | **Not addressed** |
| `[*]` collectors | **Not addressed** |
| Nested `[W]` | Partially addressed (composite wrapper shown in VALID) |

A developer writing a macro with `[?]` conditionals inside `[\]` setup has no guidance on whether this is valid.

## Affected Rules

- `compile-rules/PGE/PGE-104-macro-structural-constraints.md`

## Proposed Resolution

Expand PGE-104 to explicitly categorize every element:

**Allowed in `{M}` macros:**
- `[{]` / `[}]` — macro IO
- `[\]` / `[/]` — setup/cleanup scopes
- `[r]` — pipeline calls (within `[\]` or `[/]`)
- `[W]` — composite wrapper calls (within `[\]` or `[/]`)
- `[?]` — conditionals (within `[\]` or `[/]`, for branching setup logic)
- `[!]` — error handlers (scoped under `[r]` calls within `[\]` or `[/]`)

**Prohibited in `{M}` macros:**
- `[t]` — triggers (already documented)
- `[Q]` — queues (already documented)
- `[=]` — pipeline-level IO (already documented)
- `[p]` / `[b]` — parallel execution (macros are sequential setup/cleanup)
- `[*]` — collectors (no parallel to collect from)

Add VALID/INVALID examples for each newly addressed element.

## Investigation Needed

- Confirm whether `[?]` conditionals are valid inside macro `[\]`/`[/]` scopes (likely yes — setup may need to branch on input)
- Confirm whether `[p]`/`[b]` are prohibited (likely yes — macros should not spawn concurrent work)

## See also

- [PGE-104 — Macro Structural Constraints](../compile-rules/PGE/PGE-104-macro-structural-constraints.md)
