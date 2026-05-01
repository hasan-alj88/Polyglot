---
audience: developer
rule: "9.13"
code: PGE09012
name: Import Alias Shadows Standard Library
severity: error
---

# Rule 9.13 — Import Alias Shadows Standard Library
`PGE09012`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** An `[@]` import alias must not use a name that matches a standard library namespace prefix. Reserved namespace prefixes include: `File`, `T`, `Q`, `W`, `Sys`, and any other top-level jm3lib namespace. If an import alias matches a reserved name, PGE09012 fires. In the serialized path tree, jm3lib pipelines occupy paths like `.pipeline:T.Daily` — a user alias `@T` would create a conflicting reference at the same tree level.
**Rationale:** Standard library pipelines (`-File.*`, `-T.*`, `-Q.*`, `-W.*`, `-Sys.*`) are built-in and accessed without `[@]` import. If a user declares `[@] @T << ...`, then `@T-Something` becomes ambiguous — is it an imported pipeline call or a reference to the jm3lib `-T.Something`? The serialized tree cannot hold two different objects at the same path.
**Detection:** The compiler maintains a list of reserved jm3lib namespace prefixes. Each `[@]` alias is checked against this list. If the alias matches (case-sensitive), PGE09012 fires.

**See also:** PGE09001 (undefined import alias), PGE09011 (duplicate import alias), packages.md (jm3lib pipelines don't require import)

**VALID:**
```aljam3
[ ] ✓ aliases do not shadow jm3lib namespaces
{@} @Local:999.MyApp:v1.0.0
   [@] @triggers << @Local:999.CustomTriggers:v1.0.0
   [@] @fileUtils << @Local:999.FileHelpers:v1.0.0

{-} -Main
   [T] -T.CLI                                [ ] ✓ jm3lib trigger — no conflict
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] @triggers-OnSchedule                     [ ] ✓ unambiguous — @triggers is not a jm3lib name
      (-) <cron#string << "0 * * * *"
   [-] @fileUtils-ReadConfig                    [ ] ✓ unambiguous — @fileUtils is not a jm3lib name
      (-) <path#string << "/etc/app.conf"
```

**INVALID:**
```aljam3
[ ] ✗ PGE09012 — @T shadows jm3lib namespace -T.*
{@} @Local:999.MyApp:v1.0.0
   [@] @T << @Local:999.CustomTriggers:v1.0.0   [ ] ✗ PGE09012 — T is a reserved jm3lib namespace

{-} -Main
   [T] -T.CLI                                [ ] ambiguous — jm3lib -T or imported @T?
   [Q] -Q.Default
   [W] -W.Aljam3
```

```aljam3
[ ] ✗ PGE09012 — @File shadows jm3lib namespace -File.*
{@} @Local:999.MyApp:v1.0.0
   [@] @File << @Local:999.FileUtils:v1.0.0     [ ] ✗ PGE09012 — File is a reserved jm3lib namespace
```

```aljam3
[ ] ✗ PGE09012 — @W shadows jm3lib namespace -W.*
{@} @Local:999.MyApp:v1.0.0
   [@] @W << @Local:999.Wrappers:v1.0.0         [ ] ✗ PGE09012 — W is a reserved jm3lib namespace
```

The reserved namespace list is maintained in [`docs/user/jm3lib/INDEX.md`](../../../user/jm3lib/INDEX.md). Any top-level namespace listed there is reserved. Current reserved prefixes: `File`, `Path`, `Sys`, `T`, `Q`, `W`.

## See Also

- [[user/syntax/packages|Packages]] — references PGE09012 in import rules
- [[user/jm3lib/INDEX|jm3lib INDEX]] — authoritative reserved namespace list, references PGE09012
