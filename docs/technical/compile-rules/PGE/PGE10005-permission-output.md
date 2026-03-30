---
rule: "9.19"
code: PGE10005
name: Permission Output
severity: error
---

### Rule 9.19 — Permission Output
`PGE10005`

**Statement:** Permission IO declarations (`[_]`) are input-only. Any `[_] >param` using output direction is a compile error. Permissions declare what IO capabilities are allowed, not what is returned.
**Rationale:** Permissions model access grants — "this pipeline may read files matching this pattern." Output direction (`>`) is meaningless in this context because permissions do not produce values. Allowing `>` in `[_]` would confuse the permission model with pipeline IO and create declarations that the compiler cannot enforce.
**Detection:** The compiler checks all `[_]` IO lines within permission blocks. If any line uses `>` (output direction) instead of `<` (input direction), PGE10005 fires immediately on the offending line. See [[permissions#IO Form]] for the correct input-only syntax.

**See also:** PGE08010 (IO direction mismatch — general), PGE10003 (unknown permission category), [[permissions#IO Form]]

**VALID:**
```polyglot
[ ] ✓ permission IO uses input direction only
{=} =WebCaller
   [_] _Web.request
      [_] <url#string << "https://api.example.com/*"    [ ] ✓ input direction
      [_] <method#string << "GET"                        [ ] ✓ input direction
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $response << =Web.Request >> "https://api.example.com/data"
```

```polyglot
[ ] ✓ inline form — no direction marker needed
{=} =FileReader
   [_] _File.read"/var/log/*"                            [ ] ✓ inline form
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $content << =File.Text.Read >> "/var/log/app.log"
```

**INVALID:**
```polyglot
[ ] ✗ PGE10005 — output direction in permission IO
{=} =BadWebCaller
   [_] _Web.request
      [_] <url#string << "https://api.example.com/*"
      [_] >result#string                                 [ ] ✗ PGE10005 — output direction in permission block
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $response << =Web.Request >> "https://api.example.com/data"
```

```polyglot
[ ] ✗ PGE10005 — output direction in database permission
{=} =BadDbQuery
   [_] _Database.connect
      [_] <host#string << "localhost"
      [_] <port#int << "5432"
      [_] >connection#string                             [ ] ✗ PGE10005 — permissions are input-only
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $conn << =Database.Connect >> "localhost:5432"
```

**Open point:** None.
