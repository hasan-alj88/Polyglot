---
audience: developer
rule: "9.20"
code: PGE10006
name: Duplicate Permission
severity: error
---

# Rule 9.20 — Duplicate Permission
`PGE10006`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/packages -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Duplicate permission declarations are a compile error in two scopes:
1. **Duplicate permission IO reference** — the same `{_}` object name referenced more than once within a single `{@}` ceiling or `{-}` definition (via `(@) _PermName` or `(-) _PermName`).
2. **Duplicate capability in `{_}`** — the same `.category` + `.capability` pair declared more than once within a single `{_}` permission object block.

PGE10006 fires on the second (and subsequent) declaration(s).
**Rationale:** Duplicate permission IO references are meaningless — referencing the same object twice grants no additional capability. Duplicate capabilities within a `{_}` block are ambiguous — if two `.capability #Read` lines under the same `.category #File` specify different scope patterns, which applies? Even if identical, duplicates indicate copy-paste errors or incomplete refactoring. Like PGE09011 (duplicate import alias), duplicate declarations create resolution ambiguity. Under Polyglot's implicit-deny permission model, every permission grant is a conscious design decision — duplicates suggest the developer has lost track of what they have authorized, which undermines the intentionality the permission system demands.
**Detection:** The compiler collects all permission IO references within each block scope (`{@}` or `{-}`). If the same `_ObjectName` appears more than once, PGE10006 fires. Separately, within each `{_}` block, the compiler checks all `.category` + `.capability` field pairs. If the same pair appears more than once, PGE10006 fires on the second occurrence.

**See also:** PGE09011 (duplicate import alias — analogous pattern), PGE10001 (pipeline exceeds ceiling), PGE10003 (unknown permission category), [[permissions]]

**VALID:**
```polyglot
[ ] ✓ different capabilities within same {_} block — not duplicates
{_} _FileHandler
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{_} _FileWriter
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Write
   [.] .scope "/tmp/reports/*"
   [.] .path "/tmp/reports/summary.txt"

{-} -FileProcessor
   (-) _FileHandler
   (-) _FileWriter
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _FileHandler
      (-) >content >> $content
   [-] -File.Text.Write
      (-) <path << _FileWriter
      (-) <content << $content
```

```polyglot
[ ] ✓ same {_} object referenced in different scopes (ceiling vs pipeline) — not duplicates
{_} _LogRead
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/*"

{_} _AppLogRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app/*"
   [.] .path "/var/log/app/current.log"

{@} @Local:999.MyApp:v1.0.0
   (@) _LogRead

{-} -LogReader
   (-) _AppLogRead
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _AppLogRead
      (-) >content >> $content
```

```polyglot
[ ] ✓ different {_} objects referenced in same pipeline — not duplicates
{_} _ReadGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app/*"
   [.] .path "/var/log/app/log.txt"

{_} _WriteGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Write
   [.] .scope "/tmp/reports/*"
   [.] .path "/tmp/reports/summary.txt"

{-} -ReadAndWrite
   (-) _ReadGrant
   (-) _WriteGrant
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _ReadGrant
      (-) >content >> $content
   [-] -File.Text.Write
      (-) <path << _WriteGrant
      (-) <content << $content
```

**INVALID:**
```polyglot
[ ] ✗ PGE10006 — same permission IO reference twice in same pipeline
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{-} -DupRef
   (-) _FileGrant
   (-) _FileGrant                              [ ] ✗ PGE10006 — _FileGrant already referenced in this scope
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $data << -File.Text.Read >> "/var/log/app.log"
```

```polyglot
[ ] ✗ PGE10006 — duplicate category+capability within {_} block
{_} _DupCapability
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/*"
   [.] .category #File                         [ ] ✗ PGE10006 — File+Read already declared in this {_}
   [.] .capability #Read
   [.] .scope "/tmp/*"
   [.] .path "/tmp/*"
```

```polyglot
[ ] ✗ PGE10006 — same permission IO reference twice in package ceiling
{_} _AppCeiling
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/*"

{@} @Local:999.MyApp:v1.0.0
   (@) _AppCeiling
   (@) _AppCeiling                             [ ] ✗ PGE10006 — _AppCeiling already referenced (even identical)
```

**Open point:** None.
