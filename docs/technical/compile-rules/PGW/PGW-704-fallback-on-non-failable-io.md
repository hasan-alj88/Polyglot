---
rule: "7.4w"
code: PGW-704
name: Fallback on Non-Failable IO
severity: warning
---

### Rule 7.4w — Fallback on Non-Failable IO
`PGW-704`

**Statement:** A `[>] <!` fallback line on an output wired to a pipeline call that declares no `[=] !ErrorName` error declarations is dead code. The called pipeline is non-failable — it cannot produce errors, so the fallback will never activate. The compiler emits a warning.
**Rationale:** Fallback values on non-failable calls give a false impression that the call can fail. This misleads developers into thinking error recovery is needed where none exists. Unlike PGW-701 (which flags dead `[!]` handler blocks), this rule targets dead `<!` fallback values on IO lines — a subtler form of dead error-handling code.
**Detection:** The compiler checks each `[>] <!` fallback line. It resolves the parent `[r]` call and inspects the called pipeline's declarations. If the called pipeline has no `[=] !...` error declarations (and is not a stdlib failable pipeline), PGW-704 fires.

**See also:**
- [PGW-701 — Error Handler on Non-Failable Call](PGW-701-error-handler-on-non-failable-call.md) — dead `[!]` handler on non-failable call
- [PGE-703 — Duplicate Fallback Assignment](../PGE/PGE-703-duplicate-fallback-assignment.md) — duplicate `<!` on same output
- [PGE-705 — Undeclared Error Raise](../PGE/PGE-705-undeclared-error-raise.md) — raising an error not declared by the pipeline

**VALID:**
```polyglot
[ ] ✓ =File.Text.Read declares [=] !File.NotFound and [=] !File.ReadError — fallback is valid
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
         [>] <! "file not available"
   [r] >content << $content
```

```polyglot
[ ] ✓ error-specific fallback on failable call — valid
{=} =ProcessSpecific
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

**WARNING:**
```polyglot
[ ] ⚠ PGW-704 — =Format declares no errors — fallback is dead code
{=} =ProcessBad
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [r] =Format
      [=] <text << $input
      [=] >formatted >> $out
         [>] <! "format failed"              [ ] ⚠ PGW-704 — =Format is non-failable
   [r] >result << $out
```

```polyglot
[ ] ⚠ PGW-704 — =DoNothing declares no errors — fallback is dead code
{=} =ProcessAlsoBad
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [r] =DoNothing
      [=] <data << $input
      [=] >data >> $out
         [>] <! "this never activates"       [ ] ⚠ PGW-704 — =DoNothing is non-failable
         [>] <!File.NotFound "also dead"     [ ] ⚠ PGW-704 — =DoNothing can't raise !File.NotFound
   [r] >result << $out
```

**Diagnostic:** "Fallback `<!` at line N on output from non-failable call `=PipelineName` — pipeline declares no errors; fallback will never activate"

**Open point:** None.
