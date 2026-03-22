---
rule: "7.3"
code: PGE-703
name: Duplicate Fallback Assignment
severity: error
---

### Rule 7.3 — Duplicate Fallback Assignment
`PGE-703`

**Statement:** Each output port may have at most one generic `<!` fallback and at most one `<!Error.Name` fallback per specific error. Duplicate generic fallbacks or duplicate error-specific fallbacks for the same error on the same output are compile errors.
**Rationale:** Multiple fallbacks for the same condition create ambiguity — which value should the runtime use? A single, deterministic fallback per error condition keeps the behavior predictable and explicit.
**Detection:** The compiler collects all `[>]`/`[<]` fallback lines scoped under each `[=]` output. If two or more generic `<!` or two or more `<!Error.Name` with the same error name are found, PGE-703 fires.

**VALID:**
```polyglot
[ ] ✓ one generic + distinct error-specific fallbacks
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "generic"
      [>] <!File.NotFound "missing"
      [>] <!File.ReadError "error"
```

```polyglot
[ ] ✓ error-specific only, no generic
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <!File.NotFound "missing"
      [>] <!File.ReadError "error"
```

```polyglot
[ ] ✓ generic only
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "fallback"
```

**INVALID:**
```polyglot
[ ] ✗ PGE-703 — duplicate generic fallback on same output
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "first"
      [>] <! "second"              [ ] ✗ PGE-703
```

```polyglot
[ ] ✗ PGE-703 — duplicate error-specific fallback for same error
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <!File.NotFound "a"
      [>] <!File.NotFound "b"      [ ] ✗ PGE-703
```

**See also:**
- [PGE-701 — Error Block Scoping](PGE-701-error-block-scoping.md) — `[!]` block scoping rules
- [PGE-702 — Chain Error Scoping](PGE-702-chain-error-scoping.md) — chain-specific error rules
