---
issue: "010"
title: PGE-101 covers 5 distinct failure modes under one code
related: PGE-101 (Rule 1.1)
priority: enhancement
status: resolved
created: 2026-03-19
resolved: 2026-03-19
---

# 010 — PGE-101 covers 5 distinct failure modes under one code

## Resolution

**PGE-101 split into 5 focused codes:**

| Code | File | Fires when |
|------|------|-----------|
| PGE-101 | `PGE-101-pipeline-execution-order.md` | Sections present but misordered |
| PGE-105 | `PGE-105-missing-trigger.md` | No `[t]` |
| PGE-106 | `PGE-106-missing-queue.md` | No `[Q]` |
| PGE-107 | `PGE-107-missing-setup-cleanup.md` | No `[W]` and no `[\]/[/]` |
| PGW-101 | `PGW-101-empty-execution-body.md` | No `[r]` calls in body (warning) |

PGE-101 renamed from "Pipeline Execution Order" to "Pipeline Section Misordering" to reflect its narrowed scope.

## Original Problem

PGE-101 (Pipeline Execution Order) fires for five different failure modes, each with a different root cause and fix:

1. **Missing `[t]`** — pipeline has no trigger
2. **Missing `[Q]`** — pipeline has no queue
3. **Missing setup/cleanup** — no `[W]` and no `[\]/[/]`
4. **Section misordering** — all sections present but in wrong order
5. **Empty execution body** — mentioned in the rule as producing a warning, but no PGW code is assigned

A developer receiving "PGE-101" must read the entire rule to determine which sub-scenario triggered the error. The fix for "missing trigger" is fundamentally different from "sections misordered."

## Affected Rules

- `compile-rules/PGE/PGE-101-pipeline-execution-order.md`

## Proposed Resolution

Split PGE-101 into distinct error codes with specific messages:

| Code | Name | Fires when |
|------|------|------------|
| PGE-101 | Pipeline Section Misordering | Sections present but in wrong order |
| PGE-105 | Missing Pipeline Trigger | No `[t]` in `{=}` pipeline |
| PGE-106 | Missing Pipeline Queue | No `[Q]` in `{=}` pipeline |
| PGE-107 | Missing Pipeline Setup/Cleanup | No `[W]` and no `[\]/[/]` pair |
| PGW-101 | Empty Execution Body | Pipeline has setup/cleanup but no `[r]` calls in body |

Each code gets a focused error message:
- PGE-105: "Pipeline `=Name` has no trigger — add `[t]` before `[Q]`"
- PGE-106: "Pipeline `=Name` has no queue — add `[Q]` after `[t]`/`[=]` section"
- PGE-107: "Pipeline `=Name` has no setup/cleanup — add `[W]` or explicit `[\]/[/]`"
- PGE-101: "Pipeline `=Name` sections are misordered — expected `[t],[=]` → `[Q]` → setup → body → cleanup"
- PGW-101: "Pipeline `=Name` has an empty execution body"

## Implementation Notes

- PGE-101's existing INVALID examples already demonstrate each sub-scenario separately — they map cleanly to the proposed split
- The VALID examples stay with PGE-101 (they demonstrate correct ordering)
- Update `COMPILE-RULES.md` error code table with new codes
- New rule files: `PGE-105-missing-trigger.md`, `PGE-106-missing-queue.md`, `PGE-107-missing-setup-cleanup.md`, `PGW-101-empty-execution-body.md`

## See also

- [PGE-101 — Pipeline Execution Order](../compile-rules/PGE/PGE-101-pipeline-execution-order.md)
