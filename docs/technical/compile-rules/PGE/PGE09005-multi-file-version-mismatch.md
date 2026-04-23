---
audience: developer
rule: "9.5"
code: PGE09005
name: Multi-File Version Mismatch
severity: error
---

# Rule 9.5 — Multi-File Version Mismatch
`PGE09005`

<!-- @u:syntax/operators -->

**Statement:** All files in a multi-file package must declare the same version in their `{@}` block. If any file's version differs from the others, PGE09005 fires on the mismatching file.
**Rationale:** A multi-file package is one logical unit — all files must agree on what version they represent. A version mismatch means either a file was not updated during a version bump or belongs to a different package entirely.
**Detection:** After the compiler resolves the `[@]` file references and loads all files in the package, it compares the version segment of each file's `{@}` address. If any version differs, PGE09005 fires on the file whose version does not match the majority (or the first file processed, if no majority).

**See also:** PGE09006 (package name mismatch), PGE09010 (asymmetric multi-file reference)

**VALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

[ ] ✓ both files declare v1.0.0
```

**INVALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{ } file-02.pg — version mismatch
{@} @Local:1000.MyApp:v1.1.0              [ ] ✗ PGE09005 — v1.1.0 ≠ v1.0.0
   [@] << "{.}\file-01.pg"
```

```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v2.0.0
   [@] << "{.}\file-02.pg"
   [@] << "{.}\file-03.pg"

{ } file-02.pg
{@} @Local:1000.MyApp:v2.0.0
   [@] << "{.}\file-01.pg"
   [@] << "{.}\file-03.pg"

{ } file-03.pg — forgot to bump version
{@} @Local:1000.MyApp:v1.0.0              [ ] ✗ PGE09005 — v1.0.0 ≠ v2.0.0
   [@] << "{.}\file-01.pg"
   [@] << "{.}\file-02.pg"
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09005 in multi-file package rules

**Open point:** None.
