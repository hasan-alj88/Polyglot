---
audience: developer
rule: "9.6"
code: PGE09006
name: Multi-File Package Name Mismatch
severity: error
---

# Rule 9.6 — Multi-File Package Name Mismatch
`PGE09006`

<!-- @u:syntax/operators -->

**Statement:** All files in a multi-file package must declare the same package name (registry, ID, and name segments) in their `{@}` block. If any file's package name differs, PGE09006 fires on the mismatching file. Version is checked separately by PGE09005.
**Rationale:** A multi-file package is one logical unit. If a file declares a different package name, it either belongs to a different package or contains a typo in the `{@}` declaration.
**Detection:** After the compiler resolves the `[@]` file references, it compares the non-version segments of each file's `{@}` address. If any file's package name differs, PGE09006 fires.

**See also:** PGE09005 (version mismatch), PGE09010 (asymmetric multi-file reference)

**VALID:**
```aljam3
{ } file-01.aj3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"

{ } file-02.aj3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.aj3"

[ ] ✓ both files declare @Local:1000.MyApp
```

**INVALID:**
```aljam3
{ } file-01.aj3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"

{ } file-02.aj3 — different package name
{@} @Local:1000.MyOtherApp:v1.0.0         [ ] ✗ PGE09006 — MyOtherApp ≠ MyApp
   [@] << "{.}\file-01.aj3"
```

```aljam3
{ } file-01.aj3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"

{ } file-02.aj3 — typo in package name
{@} @Local:1000.MyAp:v1.0.0               [ ] ✗ PGE09006 — MyAp ≠ MyApp (typo?)
   [@] << "{.}\file-01.aj3"
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09006 in multi-file package rules

**Open point:** None.
