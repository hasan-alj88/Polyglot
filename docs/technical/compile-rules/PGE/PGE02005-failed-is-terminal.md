---
rule: "2.5"
code: PGE02005
name: Failed Is Terminal
severity: error
---

# Rule 2.5 — Failed Is Terminal
`PGE02005`

**Statement:** A variable in Failed state will never resolve. It cannot transition to any other stage — no push can target a Failed variable. Any downstream pipeline waiting on a Failed variable will not fire (IO implicit trigger gate). Attempting to push into a Failed variable is a compile error when statically detectable; otherwise a runtime error.
**Rationale:** Failed means the producing pipeline errored. Allowing recovery pushes into a Failed variable would mask the error and create inconsistent state. The caller must handle the failure explicitly via `[!]` error blocks, not by overwriting the variable.
**Detection:** At any push statement that targets a variable whose producing `[r]` call has an `[!]` error path that does not itself provide a replacement value or `*Continue` fallback. At runtime, if the producing pipeline fails and a downstream pipeline attempts to pull, the runtime fires PGE02005.

## Default behavior: `[!]` ends the pipeline

When an `[!]` error handler does NOT push a replacement value into the output variable and does NOT use `[*] *Continue`, the pipeline **terminates on error**. No downstream code runs. This is the safe default — Failed state never reaches downstream code. The compiler emits PGW02004 to alert the developer about this termination behavior.

## Recovery options

Two ways to prevent termination:

1. **Direct replacement** — push a value inside `[!]`: `[r] >var << "fallback"`. Variable becomes Final.
2. **`*Continue` fallback** — use `[*] *Continue >FallBack << "value"` inside `[!]`. See [PGE02007 — Continue After Error](PGE02007-continue-after-error.md).

Both produce a Final variable — downstream pipelines trigger normally.

**VALID:**
```polyglot
[ ] ✓ [!] handles the failure — direct replacement value
[=] >result#string
[r] =Compute
   [=] >value >> >result
   [!] !ComputeError
      [r] >result << "fallback"    [ ] ✓ error handler provides replacement → Final
[r] =Log
   [=] <msg << >result             [ ] ✓ >result is Final (either from success or error handler)
```

```polyglot
[ ] ✓ [!] without replacement — pipeline ends on error (default)
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [ ] pipeline ends here on error — default behavior (PGW02004 warns)
[r] =Process
   [=] <input << >data             [ ] ✓ safe — only reachable if >data is Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE02005 — pushing into a variable that may be Failed
[=] >result#string
[r] =Compute
   [=] >value >> >result
   [!] !ComputeError
      [ ] no replacement, no *Continue — >result is Failed
[r] >result << "override"          [ ] ✗ PGE02005 — >result may be Failed, cannot push
```

**Resolved — Failure propagation:** Failure propagates automatically up the call chain. An unhandled failure in a called pipeline causes the caller's output variable to enter Failed state, which propagates upward until an `[!]` handler intercepts it or the top-level pipeline terminates. `[!]` is the catch mechanism; `*Continue` (PGE02007) allows proceeding despite failure. PGW02004 warns when a pipeline terminates due to unhandled failure.

## See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — defines Failed state and references PGE02005
- [[stdlib/collectors/Continue|*Continue Collector]] — recovery mechanism that prevents pipeline termination on error
