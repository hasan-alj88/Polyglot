---
audience: developer
rule: "7.4"
code: PGE07004
name: Duplicate Error Handler
severity: error
---

### Rule 7.4 — Duplicate Error Handler
`PGE07004`

**Statement:** Under a single `[r]` call, no two `[!]` blocks may handle the same `!Error.Name`. Multiple `[!]` blocks handling different errors under the same `[r]` call are valid and encouraged — each error gets its own handler. The wildcard `[!] !*` is also limited to one per `[r]` call.
**Rationale:** When two `[!]` blocks catch the same error, the runtime cannot determine which handler to invoke. Handling different errors separately is the intended pattern — it gives each error a distinct recovery path. Duplicate same-name handlers are always ambiguous.
**Detection:** The compiler collects all `[!]` blocks scoped under each `[r]` call and groups them by error name. If any error name (including `!*`) appears more than once, PGE07004 fires on the duplicate.

**See also:** PGE07001 (error block scoping), PGE07003 (duplicate fallback assignment — `<!` lines, not `[!]` blocks)

**VALID:**
```polyglot
[ ] ✓ different errors under same [r] — encouraged
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
   [!] !File.NotFound
      [r] >content << "not found"
   [!] !File.ReadError
      [r] >content << "read error"
   [!] !*
      [r] >content << "unknown error"
```

```polyglot
[ ] ✓ single error handler — no duplication
[r] =Fetch.Data
   [=] <url << $endpoint
   [=] >data >> $result
   [!] !Network.Timeout
      [r] >data << $cachedResult
```

**INVALID:**
```polyglot
[ ] ✗ PGE07004 — same error handled twice
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
   [!] !File.NotFound
      [r] >content << "handler 1"
   [!] !File.NotFound                        [ ] ✗ PGE07004 — !File.NotFound already handled
      [r] >content << "handler 2"
```

```polyglot
[ ] ✗ PGE07004 — duplicate wildcard handler
[r] =Fetch.Data
   [=] <url << $endpoint
   [=] >data >> $result
   [!] !*
      [r] >data << "fallback 1"
   [!] !*                                    [ ] ✗ PGE07004 — !* already handled
      [r] >data << "fallback 2"
```

### See Also

- [[user/concepts/errors|Errors]] — references PGE07004 in error scoping rules

**Open point:** None.
