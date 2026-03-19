---
issue: "003"
title: Can live metadata be inspected before parallel collection?
related: PGE-303 (Rule 3.3)
priority: design-decision
status: resolved
created: 2026-03-18
resolved: 2026-03-19
---

# 003 — Can `live` metadata be inspected before parallel collection?

## Question

Rule 3.3 (PGE-303) isolates parallel output variables until collection. Should `$var%state` be accessible before the `[*]` collector executes?

## Resolution

**Allowed.** `live` metadata fields are always readable regardless of isolation boundaries.

PGE-303 isolates *user-assignable* variables to prevent data races on push/pull. `live` fields are exempt because:
- Users can only pull `live` fields, never push — the Polyglot runtime owns writes (PGE-206)
- No user push means no write-side race condition
- Blocking `live` reads in parallel scope would be inconsistent with `live` semantics everywhere else

No carve-out in Rule 3.3 is needed — the rule already applies to push/pull operations. `live` metadata pulls are not user push/pull operations; they are runtime-managed read-only accessors.

## See also

- [PGE-303 — Variable Isolation Until Collection](../compile-rules/PGE/PGE-303-variable-isolation-until-collection.md)
- [metadata.md](../../user/concepts/metadata.md)
