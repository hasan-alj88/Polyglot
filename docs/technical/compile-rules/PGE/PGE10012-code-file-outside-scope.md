---
audience: developer
rule: "9.26"
code: PGE10012
name: Code File Outside Scope
severity: error
---

### Rule 9.26 — Code File Outside Scope
`PGE10012`

<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:concepts/permissions/enforcement -->

**Statement:** If a `-Run.*` pipeline's `<code.file` path literal falls outside every declared `{_}` permission object's `.scope` where `.category` is `#File`, PGE10012 fires. The compiler resolves the `<code.file` path at compile time and glob-matches it against all applicable `.scope` patterns.
**Rationale:** The `<code.file` path determines which foreign script the pipeline executes. If the path is outside declared file scopes, the pipeline could load and execute arbitrary code from unexpected locations. This rule ensures that the source code itself — not just the data it accesses — is within the permission boundary.
**Detection:** For each `-Run.*` pipeline call with a `<code.file` parameter:
1. Resolve the `<code.file` path literal (must be a string literal or compile-time constant)
2. Collect all `{_}` permission objects referenced by the pipeline's `(-) _PermName` declarations
3. Filter to those with `.category #File`
4. Normalize the `<code.file` path (resolve `../`, `~/`, remove double separators)
5. Glob-match against each matching `{_}` object's `.scope`
6. If no scope matches, PGE10012 fires on the `<code.file` line

**See also:** PGE10010 (permission resource not found — the file must also exist), PGE10013 (foreign resource outside scope — IO calls within the code), [[permissions/enforcement#Compile-Time File Binding]]

**VALID:**
```polyglot
[ ] ✓ <code.file within declared {_} .scope
{_} _ScriptGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Execute
   [.] .scope "/scripts/etl/*"
   [.] .path "/scripts/etl/*"

{-} -RunETL
   (-) _ScriptGrant
   (-) ;PythonUV
   [T] -T.Schedule.Cron "0 6 * * *"
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.file#path << _ScriptGrant "/scripts/etl/transform.py"  [ ] ✓ within /scripts/etl/*
      (-) <Bind#Record
      (-) >Bind#Code:Python.Output >> >result
```

**INVALID:**
```polyglot
[ ] ✗ PGE10012 — <code.file path outside declared scope
{_} _ScriptGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Execute
   [.] .scope "/scripts/etl/*"
   [.] .path "/scripts/etl/*"

{-} -RunRogue
   (-) _ScriptGrant
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.file#path << _ScriptGrant "/tmp/rogue.py"              [ ] ✗ PGE10012 — /tmp/rogue.py outside /scripts/etl/*
      (-) <Bind#Record
      (-) >Bind#Code:Python.Output >> >result
```

```polyglot
[ ] ✗ PGE10012 — path traversal attempt
{_} _ScriptGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Execute
   [.] .scope "/scripts/etl/*"
   [.] .path "/scripts/etl/*"

{-} -RunTraversal
   (-) _ScriptGrant
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.file#path << _ScriptGrant "/scripts/etl/../../etc/exploit.py"  [ ] ✗ PGE10012 — normalizes to /etc/exploit.py
      (-) <Bind#Record
      (-) >Bind#Code:Python.Output >> >result
```

**Open point:** None.
