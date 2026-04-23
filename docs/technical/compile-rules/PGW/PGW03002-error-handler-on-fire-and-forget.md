---
audience: developer
rule: "3.5w"
code: PGW03002
name: Error Handler on Fire-and-Forget
severity: warning
---

# Rule 3.5w — Error Handler on Fire-and-Forget
`PGW03002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** An `[!]` error handler block scoped under a `[b]` (fire-and-forget) call is unreachable dead code. `[b]` calls run detached from the prime pipeline — the caller does not wait for completion and cannot receive errors. Any `[!]` handler under `[b]` will never execute.
**Rationale:** `[b]` is explicitly fire-and-forget: no output, no error propagation. Adding `[!]` handlers suggests the developer expects to catch errors, which is impossible with `[b]`. Either the call should use `[-]` (synchronous, can handle errors) or `[=]` (parallel, can collect and handle errors), or the error handling should be removed. Note that `[b]` on a failable pipeline is valid — the called pipeline handles its own errors internally, and `[b]` exempts the caller from PGE07007 exhaustive handling.
**Detection:** The compiler checks each `[!]` block's parent call marker. If the parent is `[b]`, PGW03002 fires.

**See also:**
- [PGE03005 — `[b]` Has No Collectible Output](../PGE/PGE03005-b-no-collectible-output.md) — `[b]` cannot produce output
- [PGW03001 — `[b]` Called Pipeline Has Discarded Outputs](PGW03001-b-discarded-outputs.md) — warns about discarded outputs on `[b]`
- [PGW07001 — Error Handler on Non-Failable Call](PGW07001-error-handler-on-non-failable-call.md) — dead handler on non-failable call (different cause, same symptom)
- [PGE07007 — Error Handling Must Be Exhaustive](../PGE/PGE07007-error-handling-must-be-exhaustive.md) — `[b]` calls are exempt from this rule

**VALID:**
```polyglot
[ ] ✓ [b] with no error handler — correct fire-and-forget
[b] -Audit.Log
   (-) <event << $event
```

```polyglot
[ ] ✓ [b] on failable pipeline — valid, errors handled internally by called pipeline
[b] -File.Text.Write
   (-) <path << $logPath
   (-) <content << $event
```

```polyglot
[ ] ✓ [-] with error handler — synchronous call can handle errors
[-] -File.Text.Write
   (-) <path << $logPath
   (-) <content << $event
   [!] !File.NotFound
      [-] -Fallback.Log
         (-) <msg << "log path missing"
   [!] !File.WriteError
      [-] -Fallback.Log
         (-) <msg << "log write failed"
```

**WARNING:**
```polyglot
[ ] ⚠ PGW03002 — [!] handler under [b] is dead code
[b] -File.Text.Write
   (-) <path << $logPath
   (-) <content << $event
   [!] !File.NotFound                    [ ] ⚠ PGW03002 — [b] cannot propagate errors
      [-] -Fallback.Log
         (-) <msg << "log path missing"
   [!] !File.WriteError                  [ ] ⚠ PGW03002 — [b] cannot propagate errors
      [-] -Fallback.Log
         (-) <msg << "log write failed"
```

```polyglot
[ ] ⚠ PGW03002 — wildcard [!] !* under [b] is also dead code
[b] -Notify.Admin
   (-) <msg << "alert: system overload"
   [!] !*                                [ ] ⚠ PGW03002 — [b] cannot propagate errors
      [-] -Fallback.Log
         (-) <msg << "notification failed"
```

**Fix:** Remove the `[!]` handlers if the call is truly fire-and-forget. If error handling is needed, change `[b]` to `[-]` (synchronous) or `[=]` (parallel with collection).

**Diagnostic:** "Error handler `[!]` at line N under fire-and-forget `[b]` call — `[b]` cannot propagate errors to the caller. Remove handler or use `[-]`/`[=]`"

**Open point:** None.
