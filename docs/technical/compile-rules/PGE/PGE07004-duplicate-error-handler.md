---
audience: developer
rule: "7.4"
code: PGE07004
name: Duplicate Error Handler
severity: error
---

### Rule 7.4 — Duplicate Error Handler
`PGE07004`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Under a single `[-]` call, no two `[!]` blocks may handle the same `!Error.Name`. Multiple `[!]` blocks handling different errors under the same `[-]` call are valid and encouraged — each error gets its own handler. The wildcard `[!] !*` is also limited to one per `[-]` call.
**Rationale:** When two `[!]` blocks catch the same error, the runtime cannot determine which handler to invoke. Handling different errors separately is the intended pattern — it gives each error a distinct recovery path. Duplicate same-name handlers are always ambiguous.
**Detection:** The compiler collects all `[!]` blocks scoped under each `[-]` call and groups them by error name. If any error name (including `!*`) appears more than once, PGE07004 fires on the duplicate.

**See also:** PGE07001 (error block scoping), PGE07003 (duplicate fallback assignment — `<!` lines, not `[!]` blocks)

**VALID:**
```polyglot
[ ] ✓ different errors under same [-] — encouraged
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
   [!] !File.NotFound
      [-] >content << "not found"
   [!] !File.ReadError
      [-] >content << "read error"
   [!] !*
      [-] >content << "unknown error"
```

```polyglot
[ ] ✓ single error handler — no duplication
[-] -Fetch.Data
   (-) <url << $endpoint
   (-) >data >> $result
   [!] !Network.Timeout
      [-] >data << $cachedResult
```

**INVALID:**
```polyglot
[ ] ✗ PGE07004 — same error handled twice
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
   [!] !File.NotFound
      [-] >content << "handler 1"
   [!] !File.NotFound                        [ ] ✗ PGE07004 — !File.NotFound already handled
      [-] >content << "handler 2"
```

```polyglot
[ ] ✗ PGE07004 — duplicate wildcard handler
[-] -Fetch.Data
   (-) <url << $endpoint
   (-) >data >> $result
   [!] !*
      [-] >data << "fallback 1"
   [!] !*                                    [ ] ✗ PGE07004 — !* already handled
      [-] >data << "fallback 2"
```

### See Also

- [[user/concepts/errors|Errors]] — references PGE07004 in error scoping rules

**Open point:** None.
