---
audience: developer
rule: "9.19"
code: PGE10005
name: Invalid Permission Block Marker
severity: error
---

# Rule 9.19 — Invalid Permission Block Marker
`PGE10005`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A `{_}` permission object block may only contain `[.]` field lines, `(_)` template input declarations, and `[ ]` comment lines. Any other block element marker (`[-]`, `(-)`, `[=]`, `[T]`, `[Q]`, `[W]`, `[b]`, etc.) inside a `{_}` block is a compile error. Permission objects are static declarations — they do not execute pipelines, declare pipeline IO, or contain computation.
**Rationale:** `{_}` blocks define compile-time permission policies using `[.]` field lines for `.intent`, `.category`, `.capability`, `.scope`, and resource locator fields (`.path`, `.host`, etc.). Template definitions additionally use `(_)` input declarations to parameterize fields. They have no runtime behavior. Allowing execution markers would confuse the permission model with pipeline execution and create declarations that the compiler cannot enforce as static policies.
**Detection:** The compiler checks all lines within `{_}` blocks. If any line uses a marker other than `[.]`, `(_)`, or `[ ]`, PGE10005 fires immediately on the offending line. Valid `{_}` content: `(_) <param#type` (template input), `[.] .intent << #Ceiling` or `#Grant`, `[.] .field value` (decomposed fields), and `[ ]` comment lines.

**See also:** PGE10003 (unknown permission category), PGE01024 (incompatible operation marker — general), PGE10009 (unresolved permission template), [[permissions#{_} Permission Objects]]

**VALID:**
```aljam3
[ ] ✓ {_} instance uses only [.] field lines
{_} _DataAccess
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/reports/*.csv"
   [.] .path "/data/reports/q1.csv"
   [ ] grants read access to reports
```

```aljam3
[ ] ✓ {_} template uses (_) inputs + [.] field lines
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML
```

```aljam3
[ ] ✓ ceiling uses glob patterns
{_} _AppCeiling
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"
```

**INVALID:**
```aljam3
[ ] ✗ PGE10005 — [-] execution marker inside {_} block
{_} _BadPermission
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [-] $data << -File.Text.Read >> "/data/test.csv"    [ ] ✗ PGE10005 — [-] not allowed in {_}
```

```aljam3
[ ] ✗ PGE10005 — (-) pipeline IO marker inside {_} block
{_} _BadIO
   [.] .intent << #Grant
   (-) <path#string                                     [ ] ✗ PGE10005 — (-) not allowed in {_}; use (_) for template inputs
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
```

```aljam3
[ ] ✗ PGE10005 — [T] trigger marker inside {_} block
{_} _BadTrigger
   [.] .intent << #Grant
   [T] -T.CLI                                        [ ] ✗ PGE10005 — [T] not allowed in {_}
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
```

**Open point:** None.
