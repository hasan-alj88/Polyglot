---
rule: "7.4w"
code: PGW07004
name: Fallback on Non-Failable IO
severity: warning
---

### Rule 7.4w — Fallback on Non-Failable IO
`PGW07004`

**Statement:** A `<!` fallback on an IO line whose source is provably non-failable is dead code. The compiler emits a warning. A source is non-failable when:
1. The parent `[r]` call targets a pipeline that declares no `[=] !ErrorName` error declarations (non-failable call).
2. The input source is a literal value (e.g., `<data << "hello"` with `<! "fallback"`).
3. The input source is a variable that is already Final via `<<` constant assignment and cannot enter Failed state.

**Rationale:** Fallback values on non-failable sources give a false impression that the source can fail. This misleads developers into thinking error recovery is needed where none exists. Unlike PGW07001 (which flags dead `[!]` handler blocks), this rule targets dead `<!` fallback values on IO lines — a subtler form of dead error-handling code.
**Detection:** The compiler checks each `<!` fallback line. It determines whether the source can fail:
- For output IO (`>output >> $var` with `<! value`): resolve the parent `[r]` call. If the called pipeline has no `[=] !...` error declarations (and is not a stdlib failable pipeline), the source is non-failable.
- For input IO (`<input << source` with `<! value`): if the source is a literal or a variable that is already Final via `<<` assignment, the source is non-failable.
If the source is non-failable, PGW07004 fires.

**See also:**
- [PGW07001 — Error Handler on Non-Failable Call](PGW07001-error-handler-on-non-failable-call.md) — dead `[!]` handler on non-failable call
- [PGE07003 — Duplicate Fallback Assignment](../PGE/PGE07003-duplicate-fallback-assignment.md) — duplicate `<!` on same output
- [PGE07005 — Undeclared Error Raise](../PGE/PGE07005-undeclared-error-raise.md) — raising an error not declared by the pipeline
- [PGE02005 — Failed Is Terminal](../PGE/PGE02005-failed-is-terminal.md) — Failed state semantics

**VALID:**
```polyglot
[ ] ✓ =File.Text.Read declares errors — fallback on output is valid
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path#string
   [=] >content#string
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
   [=] <path#string
   [=] >content#string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
         [>] <!File.NotFound "missing"
         [>] <!File.ReadError "unreadable"
   [r] >content << $content
```

```polyglot
[ ] ✓ fallback on variable input from pipeline output — source may fail
{=} =ProcessChain
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path#string
   [=] >result#string
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $text
   [r] =Format
      [=] <text << $text
         [>] <! "default text"                  [ ] ✓ $text came from failable =File.Text.Read
      [=] >formatted >> $result
   [r] >result << $result
```

**WARNING:**
```polyglot
[ ] ⚠ PGW07004 — =Format declares no errors — fallback on output is dead code
{=} =ProcessBad
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [r] =Format
      [=] <text << $input
      [=] >formatted >> $out
         [>] <! "format failed"              [ ] ⚠ PGW07004 — =Format is non-failable
   [r] >result << $out
```

```polyglot
[ ] ⚠ PGW07004 — fallback on literal input source
{=} =ProcessLiteral
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >result#string
   [r] =Process
      [=] <data << "hello"
         [>] <! "fallback"                   [ ] ⚠ PGW07004 — source is a literal, cannot fail
      [=] >out >> $result
   [r] >result << $result
```

```polyglot
[ ] ⚠ PGW07004 — fallback on Final constant input source
{=} =ProcessConstant
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >greeting#string
   [r] $name << "Alice"
   [r] =Greet
      [=] <name << $name
         [>] <! "Unknown"                    [ ] ⚠ PGW07004 — $name is Final via <<, cannot fail
      [=] >msg >> $greeting
   [r] >greeting << $greeting
```

**Diagnostic:**
- Non-failable call: `"Fallback <!  at line {N} on output from non-failable call ={PipelineName} — pipeline declares no errors; fallback will never activate"`
- Literal source: `"Fallback <! at line {N} on input <{name} — source is a literal value; fallback will never activate"`
- Final constant source: `"Fallback <! at line {N} on input <{name} — source ${variable} is Final; fallback will never activate"`

**Open point:** None.
