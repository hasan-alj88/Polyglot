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

**Statement:** A variable in Failed state cannot proceed to the next job or task. The compiler enforces exhaustive error handling: every failable call must have an `[!]` error block or `!<`/`!>` fallback operators on its IO lines. If neither is present, the compiler emits PGE02005. At runtime, the Failed state triggers the declared fallback, resolving the variable to Final before downstream code executes.
**Rationale:** Failed means the producing pipeline errored. The compiler performs static analysis to account for all possible states, including Failed. By enforcing exhaustive handling at compile time, no Failed variable ever reaches downstream code unresolved.
**Detection:** At compile time: for each `[-]` call that can produce an error, verify that either (1) an `[!]` block provides a replacement value, or (2) `!<`/`!>` fallback operators are declared on the IO lines. If neither exists, emit PGE02005.

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
