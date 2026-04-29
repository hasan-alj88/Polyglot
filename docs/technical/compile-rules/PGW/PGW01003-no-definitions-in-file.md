---
audience: developer
rule: "1.3w"
code: PGW01003
name: No Definitions in File
severity: warning
---

# Rule 1.3w — No Definitions in File
`PGW01003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/packages -->
<!-- @u:syntax/types -->
<!-- @u:concepts/permissions -->

**Statement:** A `.aj3` file that contains a `{@}` package declaration but no `{#}`, `{-}`, `{W}`, `{T}`, `{Q}`, `{N}`, `{!}`, or `{_}` definitions produces a warning. Comment blocks (`{ }`) do not count as definitions.
**Rationale:** A file with only a package declaration and comments defines nothing useful. It is likely a stub or placeholder that was never completed. This is a warning rather than an error because the file is structurally valid.
**Detection:** After parsing the package declaration, the compiler checks whether the file contains at least one definition block (`{#}`, `{-}`, `{W}`, `{T}`, `{Q}`, `{N}`, `{!}`, or `{_}`).

**VALID:**
```aljam3
[ ] ✓ file with package and definition
{@} @Local:001.App:v1.0.0

{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] -DoWork
```

**WARNING:**
```aljam3
[ ] ⚠ PGW01003 — no definitions
{@} @Local:999.EmptyPackage:v1.0.0
{ } This file defines nothing
```

```aljam3
[ ] ⚠ PGW01003 — package declaration only
{@} @Local:001.Stub:v0.1.0
```

**Diagnostic:** "File contains no definitions — only `{@}` package declaration"
