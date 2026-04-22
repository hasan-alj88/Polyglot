---
audience: developer
rule: "7.5"
code: PGE07005
name: Undeclared Error Raise
severity: error
---

### Rule 7.5 — Undeclared Error Raise
`PGE07005`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** A `[!] >> !ErrorName` raise statement in a pipeline's execution body must reference an error that the pipeline declares via `(-) !ErrorName` in its IO section. Raising an undeclared error is a compile error — the pipeline's error tree must be complete and explicit.
**Rationale:** Error declarations form the pipeline's contract with callers. If a pipeline can raise an error, it must declare it so callers can handle it. Undeclared raises break the contract and prevent callers from writing correct error handling code. This also enables the compiler to validate that callers handle all possible errors. This is core to Polyglot's "compiles intent" model — the compiler can only enforce exhaustive error handling if every pipeline's error contract is complete and honest.
**Detection:** The compiler collects all `(-) !ErrorName` declarations in the pipeline's IO section. For each `[!] >> !ErrorName` in the execution body, it checks whether the error is in the declared set. If not, PGE07005 fires.

**See also:**
- [PGE07006 — Unused Error Declaration](PGE07006-unused-error-declaration.md) — declared but never raised
- [PGW07001 — Error Handler on Non-Failable Call](../PGW/PGW07001-error-handler-on-non-failable-call.md) — caller-side check for non-failable pipelines

**VALID:**
```polyglot
[ ] ✓ All raised errors are declared
{-} -ValidateUser
   (-) <name#string
   (-) >validated#string
   (-) !Validation.Empty
   (-) !Validation.TooLong
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty                [ ] ✓ declared above
         (!) .Message << "Name is required"
   [?] $name.length >? 100
      [!] >> !Validation.TooLong              [ ] ✓ declared above
         (!) .Message << "Name exceeds limit"
   [?] *?
      [-] >validated << $name
```

**INVALID:**
```polyglot
[ ] ✗ PGE07005 — raises !Validation.InvalidEmail but pipeline doesn't declare it
{-} -ValidateUserBad
   (-) <name#string
   (-) >validated#string
   (-) !Validation.Empty
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty
         (!) .Message << "Name is required"
   [?] $name =!? *?"*@*"
      [!] >> !Validation.InvalidEmail         [ ] ✗ PGE07005 — not in (-) !... declarations
         (!) .Message << "Invalid email format"
   [?] *?
      [-] >validated << $name
```

### See Also

- [[user/concepts/errors|Errors]] — references PGE07005 in raising errors section

**Open point:** None.
