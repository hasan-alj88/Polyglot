---
rule: "9.11"
code: PGE-911
name: Asymmetric Multi-File Reference
severity: error
---

### Rule 9.11 — Asymmetric Multi-File Reference
`PGE-911`

**Statement:** All files in a multi-file package must form a full mesh — every file must reference every other file. If file A includes file B in its `[@]` file list, then file B must also include file A. More generally, every file must list all other files in the package. Folder shorthand (`[@] << "{.}"`) satisfies this automatically since it expands to include all `.pg` files in the directory.
**Rationale:** A partial reference graph means some files see definitions that others don't, creating inconsistent compilation views. The full mesh ensures every file in the package has the same complete view of all definitions.
**Detection:** After resolving all `[@]` file references across the package, the compiler builds the reference graph. For each pair of files (A, B) where A references B, it checks that B also references A. If any pair is asymmetric, PGE-911 fires on the file missing the reference.

**See also:** PGE-909 (file not found), PGE-910 (self-reference), PGE-905 (version mismatch)

**VALID:**
```polyglot
{ } file-01.pg — full mesh: references both other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"
   [@] << "{.}\file-03.pg"

{ } file-02.pg — full mesh: references both other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"
   [@] << "{.}\file-03.pg"

{ } file-03.pg — full mesh: references both other files
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"
   [@] << "{.}\file-02.pg"

[ ] ✓ every file references every other file
```

```polyglot
{ } All files use folder shorthand — full mesh automatic
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}"

[ ] ✓ folder shorthand expands to all .pg files in directory
```

**INVALID:**
```polyglot
{ } file-01.pg — references file-02
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{ } file-02.pg — does NOT reference file-01 back
{@} @Local:1000.MyApp:v1.0.0
   [ ] ✗ PGE-911 — file-02.pg does not reference file-01.pg (asymmetric)
```

```polyglot
{ } file-01.pg — references both
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"
   [@] << "{.}\file-03.pg"

{ } file-02.pg — references both
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"
   [@] << "{.}\file-03.pg"

{ } file-03.pg — only references file-01, missing file-02
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"
   [ ] ✗ PGE-911 — file-03.pg does not reference file-02.pg
```

**Open point:** None.
