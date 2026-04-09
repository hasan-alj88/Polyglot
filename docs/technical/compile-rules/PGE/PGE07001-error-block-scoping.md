---
audience: developer
rule: "7.1"
code: PGE07001
name: Error Block Scoping
severity: error
---

### Rule 7.1 — `[!]` Error Block Scoping
`PGE07001`

**Statement:** `[!]` error handler blocks must be scoped directly under the specific `[-]` call that can produce the error. Each `[-]` call owns its own `[!]` blocks, indented under the call after any `(-)` IO lines. A `[!]` block that is not under a specific `[-]` call — such as at pipeline level or under a `[?]` conditional — is a compile error. Errors do not bubble up automatically to parent pipelines; each `[-]` must handle its own errors explicitly.
**Rationale:** Scoping errors to their producing call makes error handling explicit and local. The caller always knows which operation failed and can respond specifically. Ambient or pipeline-level catch-all blocks would obscure the error source and encourage imprecise handling.
**Detection:** The compiler checks the parent of every `[!]` block. If the immediate parent is not an `[-]` call (or a chain `[-]`), PGE07001 fires.

**VALID:**
```polyglot
[ ] ✓ [!] scoped under the [-] that produces the error
[-] -File.Text.Read
   (-) <path << $filepath
   (-) >content >> $content
   [!] !File.NotFound
      [-] $content << "Error: file not found"
   [!] !File.ReadError
      [-] $content << "Error: could not read file"
```

```polyglot
[ ] ✓ chain errors scoped under the chain [-] — see PGE07002 for chain syntax
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path
   (-) <1.rows#string >> >content
   [!] .0!File.NotFound
      [-] >content << "Error: file not found"
   [!] .1!Parse.InvalidFormat
      [-] >content << "Error: invalid CSV"
```

**See also:**
- [PGE07002 — Chain Error Scoping](PGE07002-chain-error-scoping.md) — chain-specific error addressing and scope rules

**INVALID:**
```polyglot
[ ] ✗ PGE07001 — [!] at pipeline level, not under [-]
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << $filepath
      (-) >content >> $content
   [!] !File.NotFound                  [ ] ✗ PGE07001 — not under [-]
      [-] $content << "fallback"
```

```polyglot
[ ] ✗ PGE07001 — [!] under [?] conditional, not under [-]
[?] $mode
   [?] "read"
      [-] -File.Text.Read
         (-) <path << $filepath
         (-) >content >> $content
   [!] !File.NotFound                  [ ] ✗ PGE07001 — parent is [?], not [-]
      [-] $content << "fallback"
   [?] *?
      [-] -NoOp
```

### See Also

- [[user/concepts/errors|Errors]] — references PGE07001 in error scoping rules
