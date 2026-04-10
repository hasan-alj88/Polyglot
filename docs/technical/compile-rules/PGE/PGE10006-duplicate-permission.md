---
audience: developer
rule: "9.20"
code: PGE10006
name: Duplicate Permission
severity: error
---

### Rule 9.20 — Duplicate Permission
`PGE10006`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/packages -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Duplicate permission declarations are a compile error in two scopes:
1. **Duplicate `[_]` reference** — the same `{_}` object name referenced more than once within a single `{@}` ceiling or `{-}` definition.
2. **Duplicate capability in `{_}`** — the same `Category.Capability` declared more than once within a single `{_}` permission object block.

PGE10006 fires on the second (and subsequent) declaration(s).
**Rationale:** Duplicate `[_]` references are meaningless — referencing the same object twice grants no additional capability. Duplicate capabilities within a `{_}` block are ambiguous — if two `.File.Read` lines specify different scope patterns, which applies? Even if identical, duplicates indicate copy-paste errors or incomplete refactoring. Like PGE09011 (duplicate import alias), duplicate declarations create resolution ambiguity.
**Detection:** The compiler collects all `[_]` references within each block scope (`{@}` or `{-}`). If the same `_ObjectName` appears more than once, PGE10006 fires. Separately, within each `{_}` block, the compiler checks all `[.] .Category.Capability` field lines. If the same Category.Capability pair appears more than once, PGE10006 fires on the second occurrence.

**See also:** PGE09011 (duplicate import alias — analogous pattern), PGE10001 (pipeline exceeds ceiling), PGE10003 (unknown permission category), [[permissions]]

**VALID:**
```polyglot
[ ] ✓ different capabilities within same {_} block — not duplicates
{_} _FileHandler
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"
   [.] .File.Write "/tmp/reports/*"

{-} -FileProcessor
   [_] _FileHandler
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $content << -File.Text.Read >> "/var/log/app.log"
   [-] -File.Text.Write >> "/tmp/reports/summary.txt"
      (-) <content#string << $content
```

```polyglot
[ ] ✓ same {_} object referenced in different scopes (ceiling vs pipeline) — not duplicates
{_} _LogRead
   [.] .intent << #Ceiling
   [.] .File.Read "/var/log/*"

{_} _AppLogRead
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*"

{@} @Local:999.MyApp:v1.0.0
   [_] _LogRead

{-} -LogReader
   [_] _AppLogRead
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $content << -File.Text.Read >> "/var/log/app/current.log"
```

```polyglot
[ ] ✓ different {_} objects referenced in same pipeline — not duplicates
{_} _ReadGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*"

{_} _WriteGrant
   [.] .intent << #Grant
   [.] .File.Write "/tmp/reports/*"

{-} -ReadAndWrite
   [_] _ReadGrant
   [_] _WriteGrant
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $content << -File.Text.Read >> "/var/log/app/log.txt"
   [-] -File.Text.Write >> "/tmp/reports/summary.txt"
      (-) <content#string << $content
```

**INVALID:**
```polyglot
[ ] ✗ PGE10006 — same [_] reference twice in same pipeline
{_} _FileGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"

{-} -DupRef
   [_] _FileGrant
   [_] _FileGrant                              [ ] ✗ PGE10006 — _FileGrant already referenced in this scope
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $data << -File.Text.Read >> "/var/log/app.log"
```

```polyglot
[ ] ✗ PGE10006 — duplicate Category.Capability within {_} block
{_} _DupCapability
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"
   [.] .File.Read "/tmp/*"                     [ ] ✗ PGE10006 — File.Read already declared in this {_}
```

```polyglot
[ ] ✗ PGE10006 — same [_] reference twice in package ceiling
{_} _AppCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "/var/log/*"

{@} @Local:999.MyApp:v1.0.0
   [_] _AppCeiling
   [_] _AppCeiling                             [ ] ✗ PGE10006 — _AppCeiling already referenced (even identical)
```

**Open point:** None.
