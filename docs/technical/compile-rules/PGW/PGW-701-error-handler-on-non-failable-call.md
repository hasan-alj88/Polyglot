---
rule: "7.1w"
code: PGW-701
name: Error Handler on Non-Failable Call
severity: warning
---

### Rule 7.1w — Error Handler on Non-Failable Call
`PGW-701`

**Statement:** An `[!]` error handler block scoped under an `[r]` call to a pipeline that declares no `[=] !ErrorName` error declarations is unreachable dead code. The compiler emits a warning because the called pipeline is non-failable — it cannot produce errors, so the handler will never execute.
**Rationale:** Error handlers on non-failable calls mislead the reader into thinking the call can fail. Flagging dead `[!]` blocks keeps error handling honest and helps developers avoid cargo-culting error handling patterns where none are needed.
**Detection:** The compiler checks each `[!]` block's parent `[r]` call. If the called pipeline has no `[=] !...` declarations (and is not a stdlib failable pipeline), PGW-701 fires.

**See also:**
- [PGE-701 — Error Block Scoping](../PGE/PGE-701-error-block-scoping.md) — structural scoping rules for `[!]`
- [PGE-705 — Undeclared Error Raise](../PGE/PGE-705-undeclared-error-raise.md) — raising an error not declared by the pipeline

**VALID:**
```polyglot
[ ] ✓ =ValidateUser declares [=] !Validation.Empty — handler is valid
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name;string
   [=] >result;string
   [r] =ValidateUser
      [=] <name << $name
      [=] >validated >> $validated
      [!] !Validation.Empty
         [r] >result << "Error: empty name"
   [r] >result << "User {$validated} processed"
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-701 — =DoNothing declares no errors — handler is dead code
{=} =ProcessBad
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name;string
   [=] >result;string
   [r] =DoNothing
      [!] !SomeError                          [ ] ⚠ PGW-701 — =DoNothing is non-failable
         [r] >result << "this never fires"
   [r] >result << "done"
```

```polyglot
[ ] ⚠ PGW-701 — =Format declares no errors — handler is dead code
{=} =ProcessAlsoBad
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [r] =Format
      [=] <text << $input
      [=] >formatted >> $out
      [!] !Format.Error                       [ ] ⚠ PGW-701 — =Format has no [=] !... declarations
         [r] >result << "format failed"
   [r] >result << $out
```

**Open point:** None.
