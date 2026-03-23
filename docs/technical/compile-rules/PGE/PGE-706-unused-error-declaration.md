---
rule: "7.6"
code: PGE-706
name: Unused Error Declaration
severity: error
---

### Rule 7.6 — Unused Error Declaration
`PGE-706`

**Statement:** An `[=] !ErrorName` error declaration in a pipeline's IO section must have a corresponding `[!] >> !ErrorName` raise in the pipeline's execution body. Declaring an error that is never raised is a compile error — the pipeline's error tree must accurately reflect its behavior.
**Rationale:** Error declarations are a contract. If a pipeline declares it can raise `!Validation.Empty` but never does, callers write dead error handling code for an impossible case. This wastes effort and misleads readers about the pipeline's actual failure modes. Keeping declarations in sync with raises ensures the error tree is trustworthy.
**Detection:** The compiler collects all `[=] !ErrorName` declarations in the pipeline's IO section and all `[!] >> !ErrorName` raises in the execution body. Any declaration without a matching raise triggers PGE-706.

**See also:**
- [PGE-705 — Undeclared Error Raise](PGE-705-undeclared-error-raise.md) — raise without declaration (the inverse)

**VALID:**
```polyglot
[ ] ✓ All declared errors are raised
{=} =ValidateUser
   [=] <name;string
   [=] >validated;string
   [=] !Validation.Empty
   [=] !Validation.TooLong
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty                [ ] ✓ matches declaration
         [=] .Message << "Name is required"
   [?] $name.length >? 100
      [!] >> !Validation.TooLong              [ ] ✓ matches declaration
         [=] .Message << "Name exceeds limit"
   [?] *?
      [r] >validated << $name
```

**INVALID:**
```polyglot
[ ] ✗ PGE-706 — declares !Validation.TooLong but never raises it
{=} =ValidateUserBad
   [=] <name;string
   [=] >validated;string
   [=] !Validation.Empty
   [=] !Validation.TooLong                   [ ] ✗ PGE-706 — never raised in body
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty
         [=] .Message << "Name is required"
   [?] *?
      [r] >validated << $name
```

**Open point:** None.
