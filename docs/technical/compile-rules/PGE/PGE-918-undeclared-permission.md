---
rule: "9.18"
code: PGE-918
name: Undeclared Permission
severity: error
---

### Rule 9.18 — Undeclared Permission
`PGE-918`

**Statement:** If a pipeline calls a stdlib IO pipeline (e.g., `=File.Text.Read`) without a matching `[_]` permission declaration, PGE-918 fires. The compiler traces the call graph to verify that every IO operation in a pipeline's execution — including transitive calls through other pipelines — is covered by a `[_]` declaration.
**Rationale:** Polyglot uses an implicit-deny permission model. Every pipeline starts with zero IO capabilities. This forces developers to explicitly declare their IO footprint, making each pipeline's external interactions auditable. Without this rule, a pipeline could silently perform IO that was never authorized.
**Detection:** The compiler builds a call graph from each pipeline's execution body. For every call to a known IO pipeline (stdlib pipelines under `=File.*`, `=Web.*`, `=Database.*`, etc.), it checks that the calling pipeline has a matching `[_] _Category.subfield` declaration. This includes transitive calls: if `=A` calls `=B` which calls `=File.Text.Read`, then `=A` must have `[_] _File.read` covering the path. If no matching permission is found, PGE-918 fires.

**See also:** PGE-915 (pipeline exceeds package ceiling), PGE-917 (unknown permission category), PGW-903 (unused permission — the inverse), [[permissions#Compile-Time Enforcement]]

**VALID:**
```polyglot
[ ] ✓ permission declared matches IO call
{=} =GoodPipe
   [_] _File.read"/var/log/*"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $data << =File.Text.Read >> "/var/log/app.log"
```

```polyglot
[ ] ✓ pure computation — no IO calls, no permissions needed
{=} =PureCompute
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <a#int
   [=] <b#int
   [=] >sum#int
   [r] >sum << =Math.Add $a $b
```

**INVALID:**
```polyglot
[ ] ✗ PGE-918 — calls =File.Text.Read without _File.read permission
{=} =BadPipe
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $data << =File.Text.Read >> "/var/log/app.log"  [ ] ✗ PGE-918 — undeclared _File.read
```

```polyglot
[ ] ✗ PGE-918 — transitive call: =Outer calls =Inner which calls IO
{=} =Inner
   [_] _File.read"/var/log/*"
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $data << =File.Text.Read >> "/var/log/app.log"

{=} =Outer
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $result << =Inner                               [ ] ✗ PGE-918 — =Outer calls =Inner which uses _File.read, but =Outer has no [_] _File.read
```

**Open point:** None.
