---
issue: "001"
title: Static detection of unhandled Failed variables
related: PGE-205 (Rule 2.5), PGW-205
priority: enhancement
status: resolved
created: 2026-03-18
resolved: 2026-03-18
---

# 001 — Static detection of unhandled Failed variables

## Problem

When an `[r]` call has an `[!]` error handler that does **not** push a replacement value into the output variable, the variable enters Failed state on error. Any downstream pipeline that pulls from this variable will hit PGE-205 at runtime.

Currently PGE-205 is only detectable at runtime. The compiler could catch this statically.

## Resolution

Three-part design based on two foundations:
- **Pipelines won't trigger if input is Failed** — IO implicit triggers already enforce this. Failed = no value = input never reaches Final = downstream pipeline doesn't fire.
- **`[*] *Continue` is a collector** — it collects the variable state and produces `>IsFailed` as a boolean output.

### 1. `[!]` ends the pipeline by default

When an `[!]` error handler does NOT push a replacement value and does NOT use `[*] *Continue`, the pipeline **terminates on error**. No downstream code runs. Failed state never reaches downstream code.

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [ ] pipeline ends here on error — default behavior
[r] =Process
   [=] <input << >data           [ ] safe — only reachable if >data is Final
```

### 2. `[*] *Continue` collects the variable state

`*Continue` is a collector scoped inside `[!]`. Its output `>IsFailed` is wired directly on the `[*]` line to a `$` variable. This boolean indicates whether the variable entered Failed state.

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [*] *Continue >IsFailed >> $fetchFailed
[?] $fetchFailed =? true
   [r] =HandleMissing
[?] *?
   [r] =Process
      [=] <input << >data        [ ] safe — >data is Final on this path
```

### 3. PGW-205 if `*Continue` output is not handled

If `[*] *Continue` is used but its `>IsFailed` output is not wired or checked, the compiler emits PGW-205. The warning can be suppressed with `[ ] Ignore PGW-205`.

```polyglot
[ ] ⚠ PGW-205 — *Continue output not handled
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
      [*] *Continue               [ ] ⚠ PGW-205 — >IsFailed not wired
[r] =Process
   [=] <input << >data            [ ] won't trigger if >data is Failed (implicit gate)
```

### Summary

| `[!]` behavior | Pipeline continues? | Warning? | Variable state |
|----------------|-------------------|----------|---------------|
| No replacement, no `*Continue` | No — pipeline ends | None | Never Failed (unreachable) |
| `*Continue` with `>IsFailed` handled | Yes | None | May be Failed — handled via boolean |
| `*Continue` without `>IsFailed` handled | Yes | PGW-205 | May be Failed — unhandled |
| Replacement pushed (`<<`/`>>`) | Yes | None | Always Final |

## See also

- [PGE-205 — Failed Is Terminal](../compile-rules/PGE/PGE-205-failed-is-terminal.md)
- [PGW-205 — Failed Variable Usage](../compile-rules/PGW/PGW-205-failed-variable-usage.md)
