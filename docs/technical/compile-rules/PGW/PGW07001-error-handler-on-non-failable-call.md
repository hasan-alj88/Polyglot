---
audience: developer
rule: "7.1w"
code: PGW07001
name: Error Handler on Non-Failable Call
severity: warning
---

# Rule 7.1w — Error Handler on Non-Failable Call
`PGW07001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** An `[!]` error handler block scoped under an `[-]` call to a pipeline that declares no `(-) !ErrorName` error declarations is unreachable dead code. The compiler emits a warning because the called pipeline is non-failable — it cannot produce errors, so the handler will never execute.
**Rationale:** Error handlers on non-failable calls mislead the reader into thinking the call can fail. Flagging dead `[!]` blocks keeps error handling honest and helps developers avoid cargo-culting error handling patterns where none are needed.
**Detection:** The compiler checks each `[!]` block's parent `[-]` call. If the called pipeline has no `(-) !...` declarations (and is not a pglib failable pipeline), PGW07001 fires.

**See also:**
- [PGE07001 — Error Block Scoping](../PGE/PGE07001-error-block-scoping.md) — structural scoping rules for `[!]`
- [PGE07005 — Undeclared Error Raise](../PGE/PGE07005-undeclared-error-raise.md) — raising an error not declared by the pipeline

**VALID:**
```aljam3
[ ] ✓ -ValidateUser declares (-) !Validation.Empty — handler is valid
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <name#string
   (-) >result#string
   [ ]
   [-] -ValidateUser
      (-) <name << $name
      (-) >validated >> $validated
      [!] !Validation.Empty
         [-] >result << "Error: empty name"
   [-] >result << "User {$validated} processed"
```

**WARNING:**
```aljam3
[ ] ⚠ PGW07001 — -DoNothing declares no errors — handler is dead code
{-} -ProcessBad
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <name#string
   (-) >result#string
   [ ]
   [-] -DoNothing
      [!] !SomeError                          [ ] ⚠ PGW07001 — -DoNothing is non-failable
         [-] >result << "this never fires"
   [-] >result << "done"
```

```aljam3
[ ] ⚠ PGW07001 — -Format declares no errors — handler is dead code
{-} -ProcessAlsoBad
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >result#string
   [ ]
   [-] -Format
      (-) <text << $input
      (-) >formatted >> $out
      [!] !Format.Error                       [ ] ⚠ PGW07001 — -Format has no (-) !... declarations
         [-] >result << "format failed"
   [-] >result << $out
```

**Open point:** None.
