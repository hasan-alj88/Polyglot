---
rule: "7.7"
code: PGE-707
name: Error Handling Must Be Exhaustive
severity: error
---

### Rule 7.7 — Error Handling Must Be Exhaustive
`PGE-707`

**Statement:** When calling a failable pipeline (one that declares `[=] !ErrorName`), the caller must address every declared error. If any declared error has no handler and no fallback, PGE-707 fires. This mirrors PGE-601 (Conditional Must Be Exhaustive) — just as every conditional branch must route every value, every failable call must route every error.
**Rationale:** Unaddressed errors cause silent pipeline termination — the caller believes downstream code will execute, but the pipeline ends without explanation. Explicit handling ensures the developer has acknowledged every failure mode, even if the chosen response is termination. This eliminates a class of "it just stopped" bugs.
**Detection:** The compiler collects the called pipeline's `[=] !ErrorName` declarations. It then checks the caller's handling mechanisms. An error is "addressed" if any of the following cover it:

| Mechanism | Scope |
|-----------|-------|
| `[!] !ErrorName` handler block | Addresses that specific error |
| `[!] !*` wildcard catch-all | Addresses all remaining errors |
| `[>] <!` generic fallback | Addresses all errors (on that output) |
| `[>] <!Error.Name` specific fallback | Addresses that specific error (on that output) |

If any declared error is not addressed by at least one mechanism, PGE-707 fires.

**Exemption:** `[b]` (fire-and-forget) calls are exempt from PGE-707. The `[b]` marker is an explicit acknowledgment that the caller does not participate in the called pipeline's error handling — the called pipeline handles its own errors internally. See [PGW-302](../PGW/PGW-302-error-handler-on-fire-and-forget.md).

**See also:**
- [PGE-601 — Conditional Must Be Exhaustive](PGE-601-conditional-must-be-exhaustive.md) — the analogous rule for conditionals
- [PGE-701 — Error Block Scoping](PGE-701-error-block-scoping.md) — `[!]` blocks must be under their producing `[r]`
- [PGE-705 — Undeclared Error Raise](PGE-705-undeclared-error-raise.md) — pipeline-side: can't raise undeclared errors
- [PGW-701 — Error Handler on Non-Failable Call](../PGW/PGW-701-error-handler-on-non-failable-call.md) — inverse: handler on non-failable call

**VALID:**
```polyglot
[ ] ✓ all declared errors handled with specific [!] blocks
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
      [!] !File.NotFound
         [r] >content << "Error: file not found"
      [!] !File.ReadError
         [r] >content << "Error: could not read file"
   [r] >content << $content
```

```polyglot
[ ] ✓ [!] !* wildcard covers all errors — like [?] *? for conditionals
{=} =ProcessWildcard
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
      [!] !*
         [r] >content << "Error: operation failed"
   [r] >content << $content
```

```polyglot
[ ] ✓ generic [>] <! fallback addresses all errors
{=} =ProcessFallback
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
         [>] <! "unavailable"
   [r] >content << $content
```

```polyglot
[ ] ✓ mixed — specific [!] + [!] !* wildcard for the rest
{=} =ProcessMixed
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
      [!] !File.NotFound
         [r] >content << "not found — using default"
      [!] !*
         [ ] all other errors (e.g., !File.ReadError) handled here
         [r] >content << "Error: read failed"
   [r] >content << $content
```

```polyglot
[ ] ✓ error-specific fallbacks cover each declared error
{=} =ProcessSpecificFallbacks
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
         [>] <!File.NotFound "missing"
         [>] <!File.ReadError "unreadable"
   [r] >content << $content
```

**INVALID:**
```polyglot
[ ] ✗ PGE-707 — no error handling on failable call
{=} =ProcessNone
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read                              [ ] declares !File.NotFound, !File.ReadError
      [=] <path << $path
      [=] >content >> $content
                                                     [ ] ✗ PGE-707 — !File.NotFound unaddressed
                                                     [ ] ✗ PGE-707 — !File.ReadError unaddressed
   [r] >content << $content
```

```polyglot
[ ] ✗ PGE-707 — partial handling — !File.ReadError not addressed
{=} =ProcessPartial
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
      [!] !File.NotFound
         [r] >content << "not found"
                                                     [ ] ✗ PGE-707 — !File.ReadError unaddressed
   [r] >content << $content
```

```polyglot
[ ] ✗ PGE-707 — specific fallback covers one error but not the other
{=} =ProcessPartialFallback
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
         [>] <!File.NotFound "missing"
                                                     [ ] ✗ PGE-707 — !File.ReadError unaddressed
   [r] >content << $content
```

**Diagnostic:** "Unaddressed error `!ErrorName` from failable call `=PipelineName` at line N — add `[!] !ErrorName` handler, `[!] !*` wildcard, or `[>] <!` fallback"

**Open point:** None.
