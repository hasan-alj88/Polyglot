---
audience: developer
rule: "7.6"
code: PGE07006
name: Unused Error Declaration
severity: error
---

# Rule 7.6 — Unused Error Declaration
`PGE07006`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** An `(-) !ErrorName` error declaration in a pipeline's IO section must have a corresponding `[!] >> !ErrorName` raise in the pipeline's execution body. Declaring an error that is never raised is a compile error — the pipeline's error tree must accurately reflect its behavior.
**Rationale:** Error declarations are a contract. If a pipeline declares it can raise `!Validation.Empty` but never does, callers write dead error handling code for an impossible case. This wastes effort and misleads readers about the pipeline's actual failure modes. Keeping declarations in sync with raises ensures the error tree is trustworthy. Aljam3's compiler enforces contract accuracy in both directions — undeclared raises (PGE07005) and phantom declarations alike are rejected to maintain the integrity of the exhaustive error handling chain.
**Detection:** The compiler collects all `(-) !ErrorName` declarations in the pipeline's IO section and all `[!] >> !ErrorName` raises in the execution body. Any declaration without a matching raise triggers PGE07006.

**See also:**
- [PGE07005 — Undeclared Error Raise](PGE07005-undeclared-error-raise.md) — raise without declaration (the inverse)

**VALID:**
```aljam3
[ ] ✓ All declared errors are raised
{-} -ValidateUser
   (-) <name#string
   (-) >validated#string
   (-) !Validation.Empty
   (-) !Validation.TooLong
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [?] $name =? ""
      [!] >> !Validation.Empty                [ ] ✓ matches declaration
         (!) .Message << "Name is required"
   [?] $name.length >? 100
      [!] >> !Validation.TooLong              [ ] ✓ matches declaration
         (!) .Message << "Name exceeds limit"
   [?] *?
      [-] >validated << $name
```

**INVALID:**
```aljam3
[ ] ✗ PGE07006 — declares !Validation.TooLong but never raises it
{-} -ValidateUserBad
   (-) <name#string
   (-) >validated#string
   (-) !Validation.Empty
   (-) !Validation.TooLong                   [ ] ✗ PGE07006 — never raised in body
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [?] $name =? ""
      [!] >> !Validation.Empty
         (!) .Message << "Name is required"
   [?] *?
      [-] >validated << $name
```

## See Also

- [[user/concepts/errors|Errors]] — references PGE07006 in declaring pipeline errors

**Open point:** None.
