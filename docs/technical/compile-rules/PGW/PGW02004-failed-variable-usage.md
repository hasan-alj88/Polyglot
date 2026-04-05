---
audience: developer
rule: "2.5"
code: PGW02004
name: Pipeline Terminates on Error
severity: warning
---

### Rule 2.5w — Pipeline Terminates on Error
`PGW02004`

**Statement:** When an `[!]` error handler provides neither a replacement value nor a `[*] *Continue >FallBack`, the pipeline terminates on error. The compiler emits PGW02004 to alert the developer that this error path will stop execution. This is the safe default — but the developer should be aware of it.
**Rationale:** Termination on error is correct behavior, but it may be unintentional. The warning ensures the developer has explicitly considered whether they want the pipeline to stop or whether they should provide a fallback value via direct replacement or `*Continue`.
**Detection:** At compile time: for each `[!]` block, check whether it contains a replacement push or a `[*] *Continue`. If neither is present, emit PGW02004.

**Suppression:** `[ ] Ignore PGW02004` comment above the `[!]` block.

**WARNING:**
```polyglot
[ ] ⚠ PGW02004 — pipeline will terminate on error
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [ ] ⚠ PGW02004 — no replacement and no *Continue; pipeline terminates here
[r] =Process
   [=] <input << >data            [ ] unreachable if >data is Failed (implicit gate)
```

```polyglot
[ ] ⚠ PGW02004 suppressed via comment
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [ ] Ignore PGW02004
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [ ] warning suppressed — developer intends termination
```

**VALID (no warning):**
```polyglot
[ ] ✓ [!] pushes replacement — no warning
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] >data << ""              [ ] replacement value → Final
[r] =Process
   [=] <input << >data             [ ] ✓ always Final
```

```polyglot
[ ] ✓ [!] with *Continue — no warning
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [*] *Continue >FallBack << ""
[r] =Process
   [=] <input << >data             [ ] ✓ Final (success or >FallBack)
```

**See also:**
- [PGE02005 — Failed Is Terminal](../PGE/PGE02005-failed-is-terminal.md) — core Failed state semantics
- [PGE02007 — Continue After Error](../PGE/PGE02007-continue-after-error.md) — `*Continue` fallback mechanism
