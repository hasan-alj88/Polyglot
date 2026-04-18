---
audience: developer
rule: "9.5"
code: PGW10003
name: Bind Mode Opacity
severity: warning
---

### Rule 9.5 — Bind Mode Opacity
`PGW10003`

<!-- @u:concepts/permissions/foreign-code -->

**Statement:** Any `{-}` pipeline using a `-Run.*.Bind` mode always emits PGW10003. In Bind mode, foreign code controls data flow via `pull()`/`push()` API calls — the compiler cannot see what data the foreign code accesses or produces, nor can it analyze the code's IO calls for permission compliance.
**Rationale:** `-Run.*.Bind` is fundamentally opaque. Unlike `.Script` mode (where the compiler injects variables via `<Bind` and can trace them through the foreign AST) or `.Function` mode (where the compiler validates function signatures), `.Bind` mode hands control to the foreign code entirely. The foreign code decides when to `pull()` input and `push()` output, making the data flow invisible to the compiler. Permission compliance for `.Bind` pipelines relies entirely on runtime sandbox enforcement.
**Detection:** This warning fires unconditionally on any `{-}` pipeline definition that uses a `-Run.*.Bind` call. No AST analysis is attempted — the mode itself is the trigger.

**See also:** PGW10002 (unverifiable foreign IO — partial opacity), PGE10013 (foreign resource outside scope — full verification possible), [[permissions/foreign-code#Bind Mode]]

**WARNING:**
```polyglot
[ ] ⚠ PGW10003 — Bind mode is fully opaque to compiler analysis
{_} _DataAccess
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -BindProcessor
   (-) _DataAccess
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Bind;PythonUV                                      [ ] ⚠ PGW10003 — Bind mode, compiler cannot verify
      (-) <code.file#path << _DataAccess "/scripts/processor.py"
      (-) >Bind#Code:Python.Output >> >result
```

```polyglot
[ ] ⚠ PGW10003 — even with matching permissions, Bind is opaque
{_} _WebAccess
   [.] .intent << #Grant
   [.] .category #Web
   [.] .capability #Request
   [.] .scope "https://api.internal.com/*"
   [.] .host "api.internal.com"

{_} _FileAccess
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -BindMultiIO
   (-) _WebAccess
   (-) _FileAccess
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Bind;PythonUV                                      [ ] ⚠ PGW10003 — foreign code calls pull()/push()
      (-) <code.file#path << _FileAccess "/scripts/multi_io.py"
      (-) >Bind#Code:Python.Output >> >result
   [ ] sandbox enforcement only — compiler trusts {_} declarations but cannot verify actual IO
```

**Open point:** None.
