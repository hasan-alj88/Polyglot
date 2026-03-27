---
rule: "7.1"
code: PGE-701
name: Error Block Scoping
severity: error
---

### Rule 7.1 — `[!]` Error Block Scoping
`PGE-701`

**Statement:** `[!]` error handler blocks must be scoped directly under the specific `[r]` call that can produce the error. Each `[r]` call owns its own `[!]` blocks, indented under the call after any `[=]` IO lines. A `[!]` block that is not under a specific `[r]` call — such as at pipeline level or under a `[?]` conditional — is a compile error. Errors do not bubble up automatically to parent pipelines; each `[r]` must handle its own errors explicitly.
**Rationale:** Scoping errors to their producing call makes error handling explicit and local. The caller always knows which operation failed and can respond specifically. Ambient or pipeline-level catch-all blocks would obscure the error source and encourage imprecise handling.
**Detection:** The compiler checks the parent of every `[!]` block. If the immediate parent is not an `[r]` call (or a chain `[r]`), PGE-701 fires.

**VALID:**
```polyglot
[ ] ✓ [!] scoped under the [r] that produces the error
[r] =File.Text.Read
   [=] <path << $filepath
   [=] >content >> $content
   [!] !File.NotFound
      [r] $content << "Error: file not found"
   [!] !File.ReadError
      [r] $content << "Error: could not read file"
```

```polyglot
[ ] ✓ chain errors scoped under the chain [r] — see PGE-702 for chain syntax
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path
   [=] <1.rows#string >> >content
   [!] .0!File.NotFound
      [r] >content << "Error: file not found"
   [!] .1!Parse.InvalidFormat
      [r] >content << "Error: invalid CSV"
```

**See also:**
- [PGE-702 — Chain Error Scoping](PGE-702-chain-error-scoping.md) — chain-specific error addressing and scope rules

**INVALID:**
```polyglot
[ ] ✗ PGE-701 — [!] at pipeline level, not under [r]
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path << $filepath
      [=] >content >> $content
   [!] !File.NotFound                  [ ] ✗ PGE-701 — not under [r]
      [r] $content << "fallback"
```

```polyglot
[ ] ✗ PGE-701 — [!] under [?] conditional, not under [r]
[?] $mode
   [?] "read"
      [r] =File.Text.Read
         [=] <path << $filepath
         [=] >content >> $content
   [!] !File.NotFound                  [ ] ✗ PGE-701 — parent is [?], not [r]
      [r] $content << "fallback"
   [?] *?
      [r] =NoOp
```
