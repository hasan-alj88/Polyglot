---
audience: developer
rule: "9.3"
code: PGW10001
name: Unused Permission
severity: warning
---

# Rule 9.3 — Unused Permission
`PGW10001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A `(-) _PermName` permission IO declaration in a pipeline that is never exercised by any IO call in its call graph is flagged with a warning. The declaration is dead — the `{_}` object grants capabilities that are never used.
**Rationale:** Unused permission declarations indicate incomplete refactoring (an IO call was removed but the `(-) _PermName` declaration was left behind) or over-broad permission requests. While not a correctness error, unused declarations make a pipeline's declared IO footprint misleading — auditing permissions becomes harder when declarations do not match actual IO usage. This is analogous to PGW09002 (unused import).
**Detection:** The compiler traces the call graph from each `{-}` pipeline definition. For each `(-) _ObjectName` permission IO declaration, it resolves the `{_}` definition and checks whether any call in the graph exercises at least one capability granted by that object. If no call matches any capability in the referenced `{_}` object, PGW10001 fires on that `(-)` line. The same applies to `(#) _PermName` on `{#}` definitions.

**See also:** PGW09002 (unused import — analogous pattern), PGE10004 (undeclared permission — the inverse: using IO without permission), [[permissions#Compile-Time Enforcement]]

**VALID:**
```polyglot
[ ] ✓ all referenced {_} objects have exercised capabilities
{_} _LogIO
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{-} -LogAnalyzer
   (-) _LogIO
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _LogIO
      (-) >content >> $content                                     [ ] ✓ exercises File.Read
```

```polyglot
[ ] ✓ pure computation — no permission IO, no IO calls
{-} -PureCompute
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <a#int
   (-) <b#int
   (-) >sum#int
   [-] >sum << -Math.Add $a $b
```

**WARNING:**
```polyglot
[ ] ⚠ PGW10001 — _WebGrant declared but never exercised
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{_} _WebGrant
   [.] .intent << #Grant
   [.] .category #Web
   [.] .capability #Request
   [.] .scope "https://api.example.com/*"
   [.] .host "api.example.com"

{-} -PartialIO
   (-) _FileGrant
   (-) _WebGrant                                                   [ ] ⚠ PGW10001 — _WebGrant never used
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _FileGrant
      (-) >content >> $content                                     [ ] ✓ exercises File.Read from _FileGrant
   [ ] no Web.Request call — _WebGrant is unused
```

```polyglot
[ ] ⚠ PGW10001 — all permission IO unused (pure computation despite declarations)
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{_} _SysGrant
   [.] .intent << #Grant
   [.] .category #System
   [.] .capability #Env
   [.] .scope "APP_MODE"

{-} -OverDeclared
   (-) _FileGrant                                                  [ ] ⚠ PGW10001 — _FileGrant never used
   (-) _SysGrant                                                   [ ] ⚠ PGW10001 — _SysGrant never used
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >output#string
   [-] >output << $input                                           [ ] no IO calls at all
```

**Open point:** None.
