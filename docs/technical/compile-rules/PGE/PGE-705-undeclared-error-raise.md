---
rule: "7.5"
code: PGE-705
name: Undeclared Error Raise
severity: error
---

### Rule 7.5 — Undeclared Error Raise
`PGE-705`

**Statement:** A `[!] >> !ErrorName` raise statement in a pipeline's execution body must reference an error that the pipeline declares via `[=] !ErrorName` in its IO section. Raising an undeclared error is a compile error — the pipeline's error tree must be complete and explicit.
**Rationale:** Error declarations form the pipeline's contract with callers. If a pipeline can raise an error, it must declare it so callers can handle it. Undeclared raises break the contract and prevent callers from writing correct error handling code. This also enables the compiler to validate that callers handle all possible errors.
**Detection:** The compiler collects all `[=] !ErrorName` declarations in the pipeline's IO section. For each `[!] >> !ErrorName` in the execution body, it checks whether the error is in the declared set. If not, PGE-705 fires.

**See also:**
- [PGE-706 — Unused Error Declaration](PGE-706-unused-error-declaration.md) — declared but never raised
- [PGW-701 — Error Handler on Non-Failable Call](../PGW/PGW-701-error-handler-on-non-failable-call.md) — caller-side check for non-failable pipelines

**VALID:**
```polyglot
[ ] ✓ All raised errors are declared
{=} =ValidateUser
   [=] <name;string
   [=] >validated;string
   [=] !Validation.Empty
   [=] !Validation.TooLong
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty                [ ] ✓ declared above
         [=] .Message << "Name is required"
   [?] $name.length >? 100
      [!] >> !Validation.TooLong              [ ] ✓ declared above
         [=] .Message << "Name exceeds limit"
   [?] *?
      [r] >validated << $name
```

**INVALID:**
```polyglot
[ ] ✗ PGE-705 — raises !Validation.InvalidEmail but pipeline doesn't declare it
{=} =ValidateUserBad
   [=] <name;string
   [=] >validated;string
   [=] !Validation.Empty
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty
         [=] .Message << "Name is required"
   [?] $name =!? *?"*@*"
      [!] >> !Validation.InvalidEmail         [ ] ✗ PGE-705 — not in [=] !... declarations
         [=] .Message << "Invalid email format"
   [?] *?
      [r] >validated << $name
```

**Open point:** None.
