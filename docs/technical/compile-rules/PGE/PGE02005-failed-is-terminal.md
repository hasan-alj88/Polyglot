> [!WARNING]
> **DEPRECATED:** This general rule has been deprecated and split into specific edge-case rules for stricter compiler enforcement.

---
audience: developer
rule: "2.5"
code: PGE02005
name: Failed Must Resolve
severity: error
---

# Rule 2.5 — Failed Must Resolve
`PGE02005`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/errors -->
<!-- @u:syntax/operators -->

**Statement:** A variable in Failed state is terminal — it will never resolve to Final. The exhaustiveness rules (PGE07007) ensure every failable call is handled at compile time, preventing variables from reaching Failed state. PGE02005 fires if static analysis discovers a code path where a variable could enter Failed without any handler — this is a secondary safety net behind PGE07007.
**Rationale:** Failed means the producing pipeline errored. The compiler performs static analysis to account for all possible states, including Failed. PGE07007 enforces exhaustive error handling at call sites; PGE02005 verifies the consequence: no Failed variable ever reaches downstream code unresolved.
**Detection:** At compile time: for each variable that receives its value from a failable call, verify that the call has exhaustive error handling per PGE07007. If PGE07007 is satisfied, PGE02005 is automatically satisfied. PGE02005 serves as a secondary check — if a variable can still reach Failed state through any code path (e.g., complex control flow), the compiler emits PGE02005.

## Error handling mechanisms

Two ways to handle Failed state:

1. **`[!]` block replacement** — push a value inside `[!]`: `[-] >var << "fallback"`. Variable becomes Final.
2. **`!<`/`!>` fallback operators** — declare fallback values on IO lines under `(>)`/`(<)` markers. `!> "value"` catches all errors; `!ErrorName> "value"` catches a specific error. Variable bypasses Failed and becomes Final with the fallback value.

Both produce a Final variable — downstream jobs trigger normally.

**VALID:**
```polyglot
[ ] ✓ [!] handles the failure — direct replacement value
(-) >result#string
[-] -Compute
   (-) >value >> >result
   [!] !ComputeError
      [-] >result << "fallback"    [ ] ✓ error handler provides replacement → Final
[-] -Log
   (-) <msg << >result             [ ] ✓ >result is Final (either from success or error handler)
```

```polyglot
[ ] ✓ !> fallback on IO line — compiler satisfied
(-) >data#string
[-] -Fetch
   (-) >payload >> >data
   (>) !> "default"                [ ] ✓ fallback value → Final on any error
[-] -Process
   (-) <input << >data             [ ] ✓ >data always Final
```

```polyglot
[ ] ✓ !ErrorName> — specific error fallback
(-) >data#string
[-] -Fetch
   (-) >payload >> >data
   (>) !FetchError> "unavailable"  [ ] ✓ fallback for FetchError
   (>) !> ""                       [ ] ✓ catch-all for other errors
[-] -Process
   (-) <input << >data             [ ] ✓ >data always Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE02005 — no error handling for failable call
(-) >result#string
[-] -Compute
   (-) >value >> >result
   [ ] ✗ PGE02005 — no [!] block and no !< / !> fallback; Failed state unresolved
[-] >result << "override"          [ ] ✗ unreachable — compiler rejects
```

**Failure propagation:** Failure propagates automatically up the call chain. An unhandled failure in a called pipeline causes the caller's output variable to enter Failed state, which propagates upward until an `[!]` handler or `!<`/`!>` fallback intercepts it.

## See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — defines Failed state and references PGE02005
- [[compile-rules/PGE/PGE07007-error-handling-must-be-exhaustive|PGE07007]] — the exhaustiveness algorithm that prevents variables from reaching Failed state
