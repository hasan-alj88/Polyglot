---
audience: developer
rule: "2.5"
code: PGW02004
name: Pipeline Terminates on Error
severity: warning
---

# Rule 2.5w — Pipeline Terminates on Error
`PGW02004`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/errors -->
<!-- @u:syntax/operators -->

**Statement:** When an `[!]` error handler provides a replacement value but does not cover all error types, the compiler emits PGW02004 to alert the developer that some error paths may terminate the pipeline. Under the compiler-enforced error handling model (PGE07007), all failable calls must have exhaustive handling — PGW02004 warns about partial coverage before the stricter PGE07007 fires.
**Rationale:** Termination on error may be unintentional. The warning ensures the developer has explicitly considered whether all error types are handled via `[!]` replacement or `!<`/`!>` fallback operators.
**Detection:** At compile time: for each `[!]` block, check whether it covers all possible error types from the called pipeline. If coverage is partial and no `!<`/`!>` catch-all fallback exists, emit PGW02004.

**Suppression:** `[ ] Ignore PGW02004` comment above the `[!]` block.

**WARNING:**
```polyglot
[ ] ⚠ PGW02004 — partial error coverage
(-) >data#string
[-] -Fetch
   (-) >payload >> >data
   [!] !FetchError
      [-] -LogError
         (-) <msg << "fetch failed"
      [-] >data << ""              [ ] ✓ handles FetchError
   [ ] ⚠ PGW02004 — other errors from -Fetch not handled
```

```polyglot
[ ] ⚠ PGW02004 suppressed via comment
(-) >data#string
[-] -Fetch
   (-) >payload >> >data
   [ ] Ignore PGW02004
   [!] !FetchError
      [-] -LogError
         (-) <msg << "fetch failed"
      [-] >data << ""
      [ ] warning suppressed — developer intends partial handling
   (>) !> ""                       [ ] catch-all fallback for remaining errors
```

**VALID (no warning):**
```polyglot
[ ] ✓ [!] pushes replacement — no warning
(-) >data#string
[-] -Fetch
   (-) >payload >> >data
   [!] !FetchError
      [-] >data << ""              [ ] replacement value → Final
[-] -Process
   (-) <input << >data             [ ] ✓ always Final
```

```polyglot
[ ] ✓ !> catch-all fallback — no warning
(-) >data#string
[-] -Fetch
   (-) >payload >> >data
   (>) !> ""                       [ ] catch-all fallback → Final on any error
[-] -Process
   (-) <input << >data             [ ] ✓ always Final
```

**See also:**
- [PGE02005 — Failed Must Resolve](../PGE/PGE02005-failed-is-terminal.md) — core Failed state semantics
