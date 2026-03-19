---
rule: "9.10"
code: PGE-910
name: Multi-File Self-Reference
severity: error
---

### Rule 9.10 — Multi-File Self-Reference
`PGE-910`

**Statement:** A file must not reference itself in its `[@]` file list. Self-references serve no purpose and indicate a mistake. This applies to explicit file paths only — folder shorthand (`[@] << "{.}"`) implicitly excludes the current file.
**Rationale:** A file is always part of its own package. Listing itself as a sibling file is redundant and likely a copy-paste error from duplicating the `{@}` block across files.
**Detection:** After resolving the `[@]` path to an absolute file path, the compiler checks whether it matches the current file's path. If so, PGE-910 fires.

**See also:** PGE-909 (file not found), PGE-911 (asymmetric reference)

**VALID:**
```polyglot
{ } file-01.pg — references only other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"
   [@] << "{.}\file-03.pg"

[ ] ✓ no self-reference
```

```polyglot
{ } file-01.pg — folder shorthand auto-excludes self
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}"

[ ] ✓ folder shorthand implicitly excludes the current file
```

**INVALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"            [ ] ✗ PGE-910 — file references itself
   [@] << "{.}\file-02.pg"
```

**Open point:** None.
