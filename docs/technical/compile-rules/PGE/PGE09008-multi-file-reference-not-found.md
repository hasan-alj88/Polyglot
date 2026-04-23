---
audience: developer
rule: "9.9"
code: PGE09008
name: Multi-File Reference Not Found
severity: error
---

# Rule 9.9 — Multi-File Reference Not Found
`PGE09008`

<!-- @u:syntax/operators -->

**Statement:** Every `[@] << "path"` file reference in a `{@}` block must resolve to an existing `.pg` file. If the path does not point to a file that exists at compile time, PGE09008 fires. For folder references (`[@] << "{.}"`), the directory must exist and contain at least one other `.pg` file.
**Rationale:** Referencing a non-existent file is always a bug — a typo, a deleted file, or a misconfigured path. Catching this at compile time prevents partial package loading.
**Detection:** The compiler resolves the path relative to the current file's directory (using `{.}` expansion). If the resolved path does not exist or is not a `.pg` file, PGE09008 fires. For folder references, the compiler checks the directory exists and contains at least one `.pg` file other than the current file.

**See also:** PGE09009 (self-reference), PGE09010 (asymmetric reference)

**VALID:**
```polyglot
{ } file-01.pg — file-02.pg exists in the same directory
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

[ ] ✓ file-02.pg exists at the resolved path
```

```polyglot
{ } file-01.pg — folder contains other .pg files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}"

[ ] ✓ directory contains at least one other .pg file
```

**INVALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"            [ ] ✗ PGE09008 — file-02.pg does not exist
```

```polyglot
{ } file-01.pg — typo in filename
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\flie-02.pg"            [ ] ✗ PGE09008 — flie-02.pg not found (did you mean file-02.pg?)
```

```polyglot
{ } file-01.pg — folder reference to empty directory
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\empty-dir"             [ ] ✗ PGE09008 — directory contains no .pg files
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09008 in multi-file package rules

**Open point:** None.
