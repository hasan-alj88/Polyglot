---
rule: "9.6"
code: PGE-906
name: Multi-File Package Name Mismatch
severity: error
---

### Rule 9.6 — Multi-File Package Name Mismatch
`PGE-906`

**Statement:** All files in a multi-file package must declare the same package name (registry, ID, and name segments) in their `{@}` block. If any file's package name differs, PGE-906 fires on the mismatching file. Version is checked separately by PGE-905.
**Rationale:** A multi-file package is one logical unit. If a file declares a different package name, it either belongs to a different package or contains a typo in the `{@}` declaration.
**Detection:** After the compiler resolves the `[@]` file references, it compares the non-version segments of each file's `{@}` address. If any file's package name differs, PGE-906 fires.

**See also:** PGE-905 (version mismatch), PGE-911 (asymmetric multi-file reference)

**VALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

[ ] ✓ both files declare @Local:1000.MyApp
```

**INVALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{ } file-02.pg — different package name
{@} @Local:1000.MyOtherApp:v1.0.0         [ ] ✗ PGE-906 — MyOtherApp ≠ MyApp
   [@] << "{.}\file-01.pg"
```

```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{ } file-02.pg — typo in package name
{@} @Local:1000.MyAp:v1.0.0               [ ] ✗ PGE-906 — MyAp ≠ MyApp (typo?)
   [@] << "{.}\file-01.pg"
```

**Open point:** None.
