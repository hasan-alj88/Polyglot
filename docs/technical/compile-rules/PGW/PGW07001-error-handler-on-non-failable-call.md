---
audience: developer
rule: "7.1w"
code: PGW07001
name: Error Handler on Non-Failable Call
severity: warning
---

### Rule 7.1w — Error Handler on Non-Failable Call
`PGW07001`

**Statement:** An `[!]` error handler block scoped under an `[r]` call to a pipeline that declares no `[=] !ErrorName` error declarations is unreachable dead code. The compiler emits a warning because the called pipeline is non-failable — it cannot produce errors, so the handler will never execute.
**Rationale:** Error handlers on non-failable calls mislead the reader into thinking the call can fail. Flagging dead `[!]` blocks keeps error handling honest and helps developers avoid cargo-culting error handling patterns where none are needed.
**Detection:** The compiler checks each `[!]` block's parent `[r]` call. If the called pipeline has no `[=] !...` declarations (and is not a pglib failable pipeline), PGW07001 fires.

**See also:**
- [PGE07001 — Error Block Scoping](../PGE/PGE07001-error-block-scoping.md) — structural scoping rules for `[!]`
- [PGE07005 — Undeclared Error Raise](../PGE/PGE07005-undeclared-error-raise.md) — raising an error not declared by the pipeline

**VALID:**
```polyglot
[ ] ✓ =ValidateUser declares [=] !Validation.Empty — handler is valid
{=} =Process
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] >result#string
   [r] =ValidateUser
      [=] <name << $name
      [=] >validated >> $validated
      [!] !Validation.Empty
         [r] >result << "Error: empty name"
   [r] >result << "User {$validated} processed"
```

**WARNING:**
```polyglot
[ ] ⚠ PGW07001 — =DoNothing declares no errors — handler is dead code
{=} =ProcessBad
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] >result#string
   [r] =DoNothing
      [!] !SomeError                          [ ] ⚠ PGW07001 — =DoNothing is non-failable
         [r] >result << "this never fires"
   [r] >result << "done"
```

```polyglot
[ ] ⚠ PGW07001 — =Format declares no errors — handler is dead code
{=} =ProcessAlsoBad
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [r] =Format
      [=] <text << $input
      [=] >formatted >> $out
      [!] !Format.Error                       [ ] ⚠ PGW07001 — =Format has no [=] !... declarations
         [r] >result << "format failed"
   [r] >result << $out
```

**Open point:** None.
