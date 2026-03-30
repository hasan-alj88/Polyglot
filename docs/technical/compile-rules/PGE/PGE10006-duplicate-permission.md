---
rule: "9.20"
code: PGE10006
name: Duplicate Permission
severity: error
---

### Rule 9.20 — Duplicate Permission
`PGE10006`

**Statement:** The same permission capability cannot be declared twice in one block scope. If the same `_Category.subfield` appears more than once within a single `{@}` ceiling or `{=}`/`{M}` definition, PGE10006 fires on the second (and subsequent) declaration(s).
**Rationale:** Duplicate permissions are ambiguous — if two `[_] _File.read` lines specify different glob patterns, which scope applies? Even if both are identical, duplicates indicate copy-paste errors or incomplete refactoring. Like PGE09011 (duplicate import alias), duplicate declarations at the same scope create resolution ambiguity.
**Detection:** The compiler collects all `[_]` declarations within each block scope (`{@}` or `{=}`/`{M}`). For each declaration, it extracts the `_Category.subfield` identifier. If the same identifier appears more than once in the same scope, PGE10006 fires on the second occurrence, reporting the duplicate capability and both declarations.

**See also:** PGE09011 (duplicate import alias — analogous pattern), PGE10001 (pipeline exceeds ceiling), PGE10003 (unknown permission category), [[permissions#Permission Categories]]

**VALID:**
```polyglot
[ ] ✓ different subfields within same category — not duplicates
{=} =FileHandler
   [_] _File.read"/var/log/*"
   [_] _File.write"/tmp/reports/*"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $content << =File.Text.Read >> "/var/log/app.log"
   [r] =File.Text.Write >> "/tmp/reports/summary.txt"
      [=] <content#string << $content
```

```polyglot
[ ] ✓ same capability in different scopes (ceiling vs pipeline) — not duplicates
{@} @Local:999.MyApp:v1.0.0
   [_] _File.read"/var/log/*"

{=} =LogReader
   [_] _File.read"/var/log/app/*"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $content << =File.Text.Read >> "/var/log/app/current.log"
```

**INVALID:**
```polyglot
[ ] ✗ PGE10006 — duplicate _File.read in same pipeline
{=} =DupReader
   [_] _File.read"/var/log/*"
   [_] _File.read"/tmp/*"                     [ ] ✗ PGE10006 — _File.read already declared in this scope
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $log << =File.Text.Read >> "/var/log/app.log"
   [r] $tmp << =File.Text.Read >> "/tmp/cache.txt"
```

```polyglot
[ ] ✗ PGE10006 — duplicate _File.read in package ceiling
{@} @Local:999.MyApp:v1.0.0
   [_] _File.read"/var/log/*"
   [_] _File.read"/var/log/*"                 [ ] ✗ PGE10006 — _File.read already declared (even identical)
```

```polyglot
[ ] ✗ PGE10006 — duplicate IO-form permission
{=} =DupWeb
   [_] _Web.request
      [_] <url#string << "https://api.example.com/*"
      [_] <method#string << "GET"
   [_] _Web.request                           [ ] ✗ PGE10006 — _Web.request already declared
      [_] <url#string << "https://other.example.com/*"
      [_] <method#string << "POST"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $resp << =Web.Request >> "https://api.example.com/data"
```

**Open point:** None.
