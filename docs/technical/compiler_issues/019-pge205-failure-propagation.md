---
issue: "019"
title: PGE-205 failure propagation semantics undefined
related: PGE-205 (Rule 2.5), PGE-701 (Rule 7.1), PGE-207 (Rule 2.7)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 019 — PGE-205 failure propagation semantics undefined

## Problem

PGE-205 (Failed Is Terminal) has an unresolved open point:

> Does failure propagate up the call chain automatically, or must the caller handle it via `[!]`?

Two possible semantics:

**Option A — Automatic propagation:** If a called pipeline enters Failed state and the caller has no `[!]` handler, the failure propagates to the caller (caller also enters Failed). This is similar to uncaught exceptions.

**Option B — Explicit handling required:** Failure does not propagate. If the caller has no `[!]` handler, the pipeline continues with the Failed variable (PGW-205 warning emitted). The caller must explicitly check state or handle errors.

This interacts with:
- PGE-701 — error blocks must be scoped under `[r]`, implying explicit handling
- PGE-207 — `*Continue` recovery mechanism, which only makes sense if failure would otherwise halt execution

## Affected Rules

- `compile-rules/PGE/PGE-205-failed-is-terminal.md`
- `compile-rules/PGE/PGE-701-error-block-scoping.md` (interaction)
- `compile-rules/PGE/PGE-207-continue-after-error.md` (interaction)

## Proposed Resolution

**Recommend Option A — Automatic propagation** with `[!]` as the catch mechanism:
- Unhandled failure in a called `[r]` propagates to the caller
- `[!]` handlers catch and recover (already defined by PGE-701)
- `*Continue` allows proceeding despite failure (already defined by PGE-207)
- PGW-205 warns when a pipeline terminates due to unhandled failure

This is consistent with Polyglot's design principle that error handling must be explicit and local.

## See also

- [PGE-205 — Failed Is Terminal](../compile-rules/PGE/PGE-205-failed-is-terminal.md)
- [PGE-701 — Error Block Scoping](../compile-rules/PGE/PGE-701-error-block-scoping.md)
- [PGE-207 — Continue After Error](../compile-rules/PGE/PGE-207-continue-after-error.md)
