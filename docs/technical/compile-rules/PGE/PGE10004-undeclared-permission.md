---
audience: developer
rule: "9.18"
code: PGE10004
name: Undeclared Permission
severity: error
---

### Rule 9.18 — Undeclared Permission
`PGE10004`

**Statement:** If a pipeline calls a pglib IO pipeline (e.g., `=File.Text.Read`) without a `[_]` reference to a `{_}` permission object that grants the needed capability, PGE10004 fires. The compiler traces the call graph to verify that every IO operation in a pipeline's execution — including transitive calls through other pipelines — is covered by a referenced `{_}` object.
**Rationale:** Polyglot uses an implicit-deny permission model. Every pipeline starts with zero IO capabilities. This forces developers to explicitly declare their IO footprint via named `{_}` objects, making each pipeline's external interactions auditable. Without this rule, a pipeline could silently perform IO that was never authorized.
**Detection:** The compiler builds a call graph from each pipeline's execution body. For every call to a known IO pipeline (pglib pipelines under `=File.*`, `=Web.*`, `=Database.*`, etc.), it checks that the calling pipeline has a `[_] _ObjectName` reference whose `{_}` definition includes the matching `Category.Capability`. This includes transitive calls: if `=A` calls `=B` which calls `=File.Text.Read`, then `=A` must reference a `{_}` object granting `File.Read` covering the path. If no matching permission is found, PGE10004 fires.

**See also:** PGE10001 (pipeline exceeds package ceiling), PGE10003 (unknown permission category), PGW10001 (unused permission — the inverse), [[permissions#Compile-Time Enforcement]]

**VALID:**
```polyglot
[ ] ✓ permission object grants File.Read, pipeline references it
{_} _LogReader
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app.log"

{=} =GoodPipe
   [_] _LogReader
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $data << =File.Text.Read >> "/var/log/app.log"
```

```polyglot
[ ] ✓ pure computation — no IO calls, no [_] references needed
{=} =PureCompute
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <a#int
   [=] <b#int
   [=] >sum#int
   [r] >sum << =Math.Add $a $b
```

**INVALID:**
```polyglot
[ ] ✗ PGE10004 — calls =File.Text.Read without any [_] reference granting File.Read
{=} =BadPipe
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $data << =File.Text.Read >> "/var/log/app.log"  [ ] ✗ PGE10004 — no {_} object grants File.Read
```

```polyglot
[ ] ✗ PGE10004 — transitive call: =Outer calls =Inner which calls IO
{_} _InnerRead
   [.] .intent << #Grant
   [.] .File.Read "/var/log/*"

{=} =Inner
   [_] _InnerRead
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $data << =File.Text.Read >> "/var/log/app.log"

{=} =Outer
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $result << =Inner                               [ ] ✗ PGE10004 — =Outer calls =Inner which uses File.Read, but =Outer has no [_] granting File.Read
```

**Open point:** None.
