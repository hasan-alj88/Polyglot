---
rule: "9.3"
code: PGW10001
name: Unused Permission
severity: warning
---

### Rule 9.3 — Unused Permission
`PGW10001`

**Statement:** A `[_]` permission declared in a pipeline or macro that is never exercised by any IO call in its call graph is flagged with a warning. The permission is dead — it grants a capability that is never used.
**Rationale:** Unused permissions indicate incomplete refactoring (an IO call was removed but the permission was left behind) or over-broad permission requests. While not a correctness error, unused permissions make a pipeline's declared IO footprint misleading — auditing permissions becomes harder when declarations do not match actual IO usage. This is analogous to PGW09002 (unused import).
**Detection:** The compiler traces the call graph from each `{=}` pipeline or `{M}` macro definition. For each `[_]` declaration, it checks whether any call in the graph exercises that permission category and subfield. If no call matches a declared permission, PGW10001 fires on that `[_]` line.

**See also:** PGW09002 (unused import — analogous pattern), PGE10004 (undeclared permission — the inverse: using IO without permission), [[permissions#Compile-Time Enforcement]]

**VALID:**
```polyglot
[ ] ✓ all declared permissions are exercised
{=} =LogAnalyzer
   [_] _File.read"/var/log/*"
   [_] _File.write"/tmp/reports/*"
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $content << =File.Text.Read >> "/var/log/app.log"       [ ] ✓ exercises _File.read
   [r] =File.Text.Write >> "/tmp/reports/summary.txt"          [ ] ✓ exercises _File.write
      [=] <content#string << $content
```

```polyglot
[ ] ✓ pure computation — no permissions declared, no IO calls
{=} =PureCompute
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <a#int
   [=] <b#int
   [=] >sum#int
   [r] >sum << =Math.Add $a $b
```

**WARNING:**
```polyglot
[ ] ⚠ PGW10001 — _Web.request declared but never exercised
{=} =PartialIO
   [_] _File.read"/var/log/*"
   [_] _Web.request                                            [ ] ⚠ PGW10001 — _Web.request never used
      [_] <url#string << "https://api.example.com/*"
      [_] <method#string << "GET"
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $content << =File.Text.Read >> "/var/log/app.log"       [ ] ✓ exercises _File.read
   [ ] no Web.Request call — _Web.request is unused
```

```polyglot
[ ] ⚠ PGW10001 — all permissions unused (pure computation despite declarations)
{=} =OverDeclared
   [_] _File.read"/var/log/*"                                  [ ] ⚠ PGW10001 — _File.read never used
   [_] _System.env"APP_MODE"                                   [ ] ⚠ PGW10001 — _System.env never used
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [r] >output << $input                                       [ ] no IO calls at all
```

**Open point:** None.
