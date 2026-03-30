---
rule: "9.13"
code: PGE09012
name: Import Alias Shadows Standard Library
severity: error
---

### Rule 9.13 — Import Alias Shadows Standard Library
`PGE09012`

**Statement:** An `[@]` import alias must not use a name that matches a standard library namespace prefix. Reserved namespace prefixes include: `File`, `T`, `Q`, `W`, `Sys`, and any other top-level stdlib namespace. If an import alias matches a reserved name, PGE09012 fires. In the serialized path tree, stdlib pipelines occupy paths like `.pipeline:T.Daily` — a user alias `@T` would create a conflicting reference at the same tree level.
**Rationale:** Standard library pipelines (`=File.*`, `=T.*`, `=Q.*`, `=W.*`, `=Sys.*`) are built-in and accessed without `[@]` import. If a user declares `[@] @T << ...`, then `@T=Something` becomes ambiguous — is it an imported pipeline call or a reference to the stdlib `=T.Something`? The serialized tree cannot hold two different objects at the same path.
**Detection:** The compiler maintains a list of reserved stdlib namespace prefixes. Each `[@]` alias is checked against this list. If the alias matches (case-sensitive), PGE09012 fires.

**See also:** PGE09001 (undefined import alias), PGE09011 (duplicate import alias), packages.md (stdlib pipelines don't require import)

**VALID:**
```polyglot
[ ] ✓ aliases do not shadow stdlib namespaces
{@} @Local:999.MyApp:v1.0.0
   [@] @triggers << @Local:999.CustomTriggers:v1.0.0
   [@] @fileUtils << @Local:999.FileHelpers:v1.0.0

{=} =Main
   [t] =T.Manual                                [ ] ✓ stdlib trigger — no conflict
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @triggers=OnSchedule                     [ ] ✓ unambiguous — @triggers is not a stdlib name
      [=] <cron#string << "0 * * * *"
   [r] @fileUtils=ReadConfig                    [ ] ✓ unambiguous — @fileUtils is not a stdlib name
      [=] <path#string << "/etc/app.conf"
```

**INVALID:**
```polyglot
[ ] ✗ PGE09012 — @T shadows stdlib namespace =T.*
{@} @Local:999.MyApp:v1.0.0
   [@] @T << @Local:999.CustomTriggers:v1.0.0   [ ] ✗ PGE09012 — T is a reserved stdlib namespace

{=} =Main
   [t] =T.Manual                                [ ] ambiguous — stdlib =T or imported @T?
   [Q] =Q.Default
   [W] =W.Polyglot
```

```polyglot
[ ] ✗ PGE09012 — @File shadows stdlib namespace =File.*
{@} @Local:999.MyApp:v1.0.0
   [@] @File << @Local:999.FileUtils:v1.0.0     [ ] ✗ PGE09012 — File is a reserved stdlib namespace
```

```polyglot
[ ] ✗ PGE09012 — @W shadows stdlib namespace =W.*
{@} @Local:999.MyApp:v1.0.0
   [@] @W << @Local:999.Wrappers:v1.0.0         [ ] ✗ PGE09012 — W is a reserved stdlib namespace
```

The reserved namespace list is maintained in [`docs/user/stdlib/INDEX.md`](../../../user/stdlib/INDEX.md). Any top-level namespace listed there is reserved. Current reserved prefixes: `File`, `Path`, `Sys`, `T`, `Q`, `W`.

### See Also

- [[user/syntax/packages|Packages]] — references PGE09012 in import rules
- [[user/stdlib/INDEX|Stdlib INDEX]] — authoritative reserved namespace list, references PGE09012
