---
audience: developer
rule: "7.7"
code: PGE07007
name: Error Handling Must Be Exhaustive
severity: error
---

### Rule 7.7 — Error Handling Must Be Exhaustive
`PGE07007`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** When calling a failable pipeline (one that declares `(-) !ErrorName`), the caller must address every declared error. If any declared error has no handler and no fallback, PGE07007 fires. This mirrors PGE06001 (Conditional Must Be Exhaustive) — just as every conditional branch must route every value, every failable call must route every error.
**Rationale:** Unaddressed errors cause silent pipeline termination — the caller believes downstream code will execute, but the pipeline ends without explanation. Explicit handling ensures the developer has acknowledged every failure mode, even if the chosen response is termination. This eliminates a class of "it just stopped" bugs. This is Polyglot's exhaustive coverage principle applied to error paths — the compiler demands that every scenario is accounted for before the pipeline runs, rather than discovering unhandled failures in production.
**Detection:** The compiler collects the called pipeline's `(-) !ErrorName` declarations. It then checks the caller's handling mechanisms. An error is "addressed" if any of the following cover it:

| Mechanism | Scope |
|-----------|-------|
| `[!] !ErrorName` handler block | Addresses that specific error |
| `[!] !*` wildcard catch-all | Addresses all remaining errors |
| `(>) !>` generic fallback | Addresses all errors (on that output) |
| `(>) !Error.Name>` specific fallback | Addresses that specific error (on that output) |

If any declared error is not addressed by at least one mechanism, PGE07007 fires.

**Exemption:** `[b]` (fire-and-forget) calls are exempt from PGE07007. The `[b]` marker is an explicit acknowledgment that the caller does not participate in the called pipeline's error handling — the called pipeline handles its own errors internally. See [PGW03002](../PGW/PGW03002-error-handler-on-fire-and-forget.md).

**See also:**
- [PGE06001 — Conditional Must Be Exhaustive](PGE06001-conditional-must-be-exhaustive.md) — the analogous rule for conditionals
- [PGE07001 — Error Block Scoping](PGE07001-error-block-scoping.md) — `[!]` blocks must be under their producing `[-]`
- [PGE07005 — Undeclared Error Raise](PGE07005-undeclared-error-raise.md) — pipeline-side: can't raise undeclared errors
- [PGW07001 — Error Handler on Non-Failable Call](../PGW/PGW07001-error-handler-on-non-failable-call.md) — inverse: handler on non-failable call

**VALID:**
```polyglot
[ ] ✓ all declared errors handled with specific [!] blocks
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !File.NotFound
         [-] >content << "Error: file not found"
      [!] !File.ReadError
         [-] >content << "Error: could not read file"
   [-] >content << $content
```

```polyglot
[ ] ✓ [!] !* wildcard covers all errors — like [?] *? for conditionals
{-} -ProcessWildcard
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !*
         [-] >content << "Error: operation failed"
   [-] >content << $content
```

```polyglot
[ ] ✓ generic (>) !> fallback addresses all errors
{-} -ProcessFallback
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !> "unavailable"
   [-] >content << $content
```

```polyglot
[ ] ✓ mixed — specific [!] + [!] !* wildcard for the rest
{-} -ProcessMixed
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !File.NotFound
         [-] >content << "not found — using default"
      [!] !*
         [ ] all other errors (e.g., !File.ReadError) handled here
         [-] >content << "Error: read failed"
   [-] >content << $content
```

```polyglot
[ ] ✓ error-specific fallbacks cover each declared error
{-} -ProcessSpecificFallbacks
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !File.NotFound> "missing"
         (>) !File.ReadError> "unreadable"
   [-] >content << $content
```

**INVALID:**
```polyglot
[ ] ✗ PGE07007 — no error handling on failable call
{-} -ProcessNone
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read                              [ ] declares !File.NotFound, !File.ReadError
      (-) <path << $path
      (-) >content >> $content
                                                     [ ] ✗ PGE07007 — !File.NotFound unaddressed
                                                     [ ] ✗ PGE07007 — !File.ReadError unaddressed
   [-] >content << $content
```

```polyglot
[ ] ✗ PGE07007 — partial handling — !File.ReadError not addressed
{-} -ProcessPartial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !File.NotFound
         [-] >content << "not found"
                                                     [ ] ✗ PGE07007 — !File.ReadError unaddressed
   [-] >content << $content
```

```polyglot
[ ] ✗ PGE07007 — specific fallback covers one error but not the other
{-} -ProcessPartialFallback
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !File.NotFound> "missing"
                                                     [ ] ✗ PGE07007 — !File.ReadError unaddressed
   [-] >content << $content
```

**Diagnostic:** "Unaddressed error `!ErrorName` from failable call `-PipelineName` at line N — add `[!] !ErrorName` handler, `[!] !*` wildcard, or `(>) !>` fallback"

### See Also

- [[user/concepts/errors|Errors]] — references PGE07007 in declaring pipeline errors

**Open point:** None.
