---
audience: developer
rule: "9.11"
code: PGE09010
name: Asymmetric Multi-File Reference
severity: error
---

# Rule 9.11 — Asymmetric Multi-File Reference
`PGE09010`

<!-- @u:syntax/operators -->

**Statement:** All files in a multi-file package must form a full mesh — every file must reference every other file. If file A includes file B in its `[@]` file list, then file B must also include file A. More generally, every file must list all other files in the package. Folder shorthand (`[@] << "{.}"`) satisfies this automatically since it expands to include all `.aj3` files in the directory.
**Rationale:** A partial reference graph means some files see definitions that others don't, creating inconsistent compilation views. The full mesh ensures every file in the package has the same complete view of all definitions.
**Detection:** After resolving all `[@]` file references across the package, the compiler builds the reference graph. For each pair of files (A, B) where A references B, it checks that B also references A. If any pair is asymmetric, PGE09010 fires on the file missing the reference.

**See also:** PGE09008 (file not found), PGE09009 (self-reference), PGE09005 (version mismatch)

**VALID:**
```aljam3
{ } file-01.aj3 — full mesh: references both other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"
   [@] << "{.}\file-03.aj3"

{ } file-02.aj3 — full mesh: references both other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.aj3"
   [@] << "{.}\file-03.aj3"

{ } file-03.aj3 — full mesh: references both other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.aj3"
   [@] << "{.}\file-02.aj3"

[ ] ✓ every file references every other file
```

```aljam3
{ } All files use folder shorthand — full mesh automatic
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}"

[ ] ✓ folder shorthand expands to all .aj3 files in directory
```

**INVALID:**
```aljam3
{ } file-01.aj3 — references file-02
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"

{ } file-02.aj3 — does NOT reference file-01 back
{@} @Local:1000.MyApp:v1.0.0
   [ ] ✗ PGE09010 — file-02.aj3 does not reference file-01.aj3 (asymmetric)
```

```aljam3
{ } file-01.aj3 — references both
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.aj3"
   [@] << "{.}\file-03.aj3"

{ } file-02.aj3 — references both
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.aj3"
   [@] << "{.}\file-03.aj3"

{ } file-03.aj3 — only references file-01, missing file-02
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.aj3"
   [ ] ✗ PGE09010 — file-03.aj3 does not reference file-02.aj3
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09010 in multi-file package rules

**Open point:** None.
