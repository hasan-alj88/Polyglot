---
audience: developer
rule: "9.10"
code: PGE09009
name: Multi-File Self-Reference
severity: error
---

# Rule 9.10 — Multi-File Self-Reference
`PGE09009`

<!-- @u:syntax/operators -->

**Statement:** A file must not reference itself in its `[@]` file list. Self-references serve no purpose and indicate a mistake. This applies to explicit file paths only — folder shorthand (`[@] << "{.}"`) implicitly excludes the current file.
**Rationale:** A file is always part of its own package. Listing itself as a sibling file is redundant and likely a copy-paste error from duplicating the `{@}` block across files.
**Detection:** After resolving the `[@]` path to an absolute file path, the compiler checks whether it matches the current file's path. If so, PGE09009 fires.

**See also:** PGE09008 (file not found), PGE09010 (asymmetric reference)

**VALID:**
```aljam3
{ } file-01.aj3 — references only other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"
   [@] << "{.}\file-03.aj3"

[ ] ✓ no self-reference
```

```aljam3
{ } file-01.aj3 — folder shorthand auto-excludes self
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}"

[ ] ✓ folder shorthand implicitly excludes the current file
```

**INVALID:**
```aljam3
{ } file-01.aj3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.aj3"            [ ] ✗ PGE09009 — file references itself
   [@] << "{.}\file-02.aj3"
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09009 in multi-file package rules

**Open point:** None.
