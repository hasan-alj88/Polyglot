---
audience: developer
rule: "9.3"
code: PGW10001
name: Unused Permission
severity: warning
---

### Rule 9.3 — Unused Permission
`PGW10001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A `[_]` reference to a `{_}` permission object in a pipeline that is never exercised by any IO call in its call graph is flagged with a warning. The reference is dead — the `{_}` object grants capabilities that are never used.
**Rationale:** Unused permission references indicate incomplete refactoring (an IO call was removed but the `[_]` reference was left behind) or over-broad permission requests. While not a correctness error, unused references make a pipeline's declared IO footprint misleading — auditing permissions becomes harder when references do not match actual IO usage. This is analogous to PGW09002 (unused import).
**Detection:** The compiler traces the call graph from each `{-}` pipeline definition. For each `[_] _ObjectName` reference, it resolves the `{_}` definition and checks whether any call in the graph exercises at least one capability granted by that object. If no call matches any capability in the referenced `{_}` object, PGW10001 fires on that `[_]` line.

**See also:** PGW09002 (unused import — analogous pattern), PGE10004 (undeclared permission — the inverse: using IO without permission), [[permissions#Compile-Time Enforcement]]

**VALID:**
```polyglot
[ ] ✓ all referenced {_} objects have exercised capabilities
{_} _LogIO
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"
   [.] .File.Write "/tmp/reports/*"

{-} -LogAnalyzer
   [_] _LogIO
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $content << -File.Text.Read >> "/var/log/app.log"       [ ] ✓ exercises File.Read
   [-] -File.Text.Write >> "/tmp/reports/summary.txt"          [ ] ✓ exercises File.Write
      (-) <content#string << $content
```

```polyglot
[ ] ✓ pure computation — no [_] references, no IO calls
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
[ ] ⚠ PGW10001 — _WebGrant referenced but never exercised
{_} _FileGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"

{_} _WebGrant
   [.] .intent << #Grant
   [.] .Web.Request "https://api.example.com/*"

{-} -PartialIO
   [_] _FileGrant
   [_] _WebGrant                                               [ ] ⚠ PGW10001 — _WebGrant never used
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $content << -File.Text.Read >> "/var/log/app.log"       [ ] ✓ exercises File.Read from _FileGrant
   [ ] no Web.Request call — _WebGrant is unused
```

```polyglot
[ ] ⚠ PGW10001 — all [_] references unused (pure computation despite references)
{_} _FileGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"

{_} _SysGrant
   [.] .intent << #Grant
   [.] .System.Env "APP_MODE"

{-} -OverDeclared
   [_] _FileGrant                                              [ ] ⚠ PGW10001 — _FileGrant never used
   [_] _SysGrant                                               [ ] ⚠ PGW10001 — _SysGrant never used
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >output#string
   [-] >output << $input                                       [ ] no IO calls at all
```

**Open point:** None.
