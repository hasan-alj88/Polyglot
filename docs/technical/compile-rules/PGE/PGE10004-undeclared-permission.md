---
audience: developer
rule: "9.18"
code: PGE10004
name: Undeclared Permission
severity: error
---

# Rule 9.18 — Undeclared Permission
`PGE10004`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** If a pipeline calls a pglib IO pipeline (e.g., `-File.Text.Read`) without declaring a `{_}` permission object via `(-) _PermName` IO that grants the needed capability, PGE10004 fires. The compiler traces the call graph to verify that every IO operation in a pipeline's execution — including transitive calls through other pipelines — is covered by a declared `_` permission object.
**Rationale:** Polyglot uses an implicit-deny permission model. Every pipeline starts with zero IO capabilities. This forces developers to explicitly declare their IO footprint via named `{_}` objects in their IO declarations, making each pipeline's external interactions auditable. Without this rule, a pipeline could silently perform IO that was never authorized.
**Detection:** The compiler builds a call graph from each pipeline's execution body. For every call to a known IO pipeline (pglib pipelines under `-File.*`, `-Web.*`, `-Database.*`, etc.), it checks that the calling pipeline has a `(-) _ObjectName` IO declaration whose `{_}` definition includes the matching `.category` and `.capability`. This includes transitive calls: if `-A` calls `-B` which calls `-File.Text.Read`, then `-A` must declare a `(-) _PermName` IO whose `{_}` grants `.category #File` and `.capability #Read` covering the path. If no matching permission is found, PGE10004 fires. The same rule applies to `{#}` definitions that use `[#]` file loads — the definition must declare `(#) _PermName` covering the file access.

**See also:** PGE10001 (pipeline exceeds package ceiling), PGE10003 (unknown permission category), PGW10001 (unused permission — the inverse), [[permissions#Compile-Time Enforcement]]

**VALID:**
```polyglot
[ ] ✓ permission object grants File.Read, pipeline declares it via (-)
{_} _LogReader
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app.log"
   [.] .path "/var/log/app.log"

{-} -GoodPipe
   (-) _LogReader
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _LogReader
      (-) >content >> $data
```

```polyglot
[ ] ✓ pure computation — no IO calls, no permission declarations needed
{-} -PureCompute
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <a#int
   (-) <b#int
   (-) >sum#int
   [-] >sum << -Math.Add $a $b
```

```polyglot
[ ] ✓ {#} definition with permission for file load
{_} _ConfigFile
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/app.json"
   [.] .path "/config/app.json"
   [.] .format #JSON

{#} #AppConfig
   (#) _ConfigFile
   [#] #data << -Json.LoadFile
      (-) <source << _ConfigFile
   [.] .dbHost#string <~ #data.db.host
```

**INVALID:**
```polyglot
[ ] ✗ PGE10004 — calls -File.Text.Read without any permission declaration granting File.Read
{-} -BadPipe
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read                                  [ ] ✗ PGE10004 — no {_} grants File.Read
      (-) <path << "/var/log/app.log"
      (-) >content >> $data
```

```polyglot
[ ] ✗ PGE10004 — transitive call: -Outer calls -Inner which calls IO
{_} _InnerRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{-} -Inner
   (-) _InnerRead
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _InnerRead
      (-) >content >> $data

{-} -Outer
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -Inner                                           [ ] ✗ PGE10004 — -Outer calls -Inner which uses File.Read, but -Outer has no permission granting File.Read
```

**Open point:** None.
