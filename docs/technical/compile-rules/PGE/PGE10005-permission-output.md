---
audience: developer
rule: "9.19"
code: PGE10005
name: Invalid Permission Block Marker
severity: error
---

### Rule 9.19 — Invalid Permission Block Marker
`PGE10005`

**Statement:** A `{_}` permission object block may only contain `[.]` field lines. Any other block element marker (`[r]`, `[=]`, `[p]`, `[T]`, `[Q]`, `[W]`, `[b]`, etc.) inside a `{_}` block is a compile error. Permission objects are static declarations — they do not execute pipelines, declare IO, or contain computation.
**Rationale:** `{_}` blocks define compile-time permission policies using fixed `[.]` field lines: `.intent`, `.Category.Capability`, and schema fields. They have no runtime behavior. Allowing execution markers would confuse the permission model with pipeline execution and create declarations that the compiler cannot enforce as static policies.
**Detection:** The compiler checks all lines within `{_}` blocks. If any line uses a marker other than `[.]`, PGE10005 fires immediately on the offending line. Valid `{_}` content: `[.] .intent << #Ceiling` or `#Grant`, `[.] .Category.Capability "scope"`, and `[ ]` comment lines.

**See also:** PGE10003 (unknown permission category), PGE01024 (incompatible operation marker — general), [[permissions#{_} Permission Objects]]

**VALID:**
```polyglot
[ ] ✓ {_} block uses only [.] field lines
{_} _DataAccess
   [.] .intent << #Grant
   [.] .File.Read "/data/reports/*.csv"
   [.] .Database.Read "analytics.postgres"
   [ ] grants read access to reports and analytics DB
```

```polyglot
[ ] ✓ ceiling uses only [.] field lines with glob patterns
{_} _AppCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "/data/*"
   [.] .File.Write "/tmp/*"
   [.] .Web.Request "https://api.example.com/*"
```

**INVALID:**
```polyglot
[ ] ✗ PGE10005 — [r] execution marker inside {_} block
{_} _BadPermission
   [.] .intent << #Grant
   [.] .File.Read "/data/*"
   [r] $data << =File.Text.Read >> "/data/test.csv"    [ ] ✗ PGE10005 — [r] not allowed in {_}
```

```polyglot
[ ] ✗ PGE10005 — [=] IO marker inside {_} block
{_} _BadIO
   [.] .intent << #Grant
   [=] <path#string                                     [ ] ✗ PGE10005 — [=] not allowed in {_}
   [.] .File.Read "/data/*"
```

```polyglot
[ ] ✗ PGE10005 — [T] trigger marker inside {_} block
{_} _BadTrigger
   [.] .intent << #Grant
   [T] =T.Manual                                        [ ] ✗ PGE10005 — [T] not allowed in {_}
   [.] .File.Read "/data/*"
```

**Open point:** None.
