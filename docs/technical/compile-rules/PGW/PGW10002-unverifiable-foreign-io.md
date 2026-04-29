---
audience: developer
rule: "9.4"
code: PGW10002
name: Unverifiable Foreign IO
severity: warning
---

# Rule 9.4 — Unverifiable Foreign IO
`PGW10002`

<!-- @u:concepts/permissions/foreign-code -->
<!-- @c:technical/algorithms/foreign-code-analysis -->

**Statement:** When the compiler's AST analysis detects an IO call in foreign code but cannot fully resolve the resource argument to a verifiable value, PGW10002 fires. The call may or may not comply with declared `{_}` permissions — the compiler cannot determine.
**Rationale:** The compiler performs intraprocedural constant propagation to trace variable assignments back to string literals. When the resource argument traces to a function return value, external input, cross-function value, or dynamically constructed string that cannot be fully resolved, the compiler cannot verify scope compliance. The warning alerts the developer that this call will rely on runtime sandbox enforcement rather than compile-time verification.
**Detection:** During the foreign code AST walk, when a `CallExpression` matches the IO sink table:
1. Extract the resource argument at the known parameter position
2. If the argument is a `Variable`, run `trace_assignment` (intraprocedural constant propagation within the same function)
3. If `trace_assignment` returns `Unresolvable` (function call return, external input, cross-module value), emit PGW10002
4. If the argument is a string concatenation where not all parts are resolvable, emit PGW10002
5. If the category has no `{_}` permission declared at all, also emit PGW10002 (no scope to check against)

**See also:** PGE10013 (foreign resource outside scope — fires when the resource IS resolvable and IS outside scope), PGW10005 (unrecognized foreign call — fires when the function itself is unknown), [[permissions/foreign-code#Confidence Levels]]

**VALID (no warning):**
```aljam3
[ ] ✓ string literal — fully verifiable, no warning needed
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/reports/*"
   [.] .path "/data/reports/*"

{-} -DirectRead
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] df = pd.read_csv("/data/reports/q1.csv")                  [ ] ✓ literal — no warning
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .count#int >> >count
```

**WARNING:**
```aljam3
[ ] ⚠ PGW10002 — resource traces to function call, unresolvable
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/reports/*"
   [.] .path "/data/reports/*"

{-} -IndirectRead
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] path = get_report_path()                                  [ ] traces to function call
         [C] df = pd.read_csv(path)                                    [ ] ⚠ PGW10002 — cannot verify path value
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .count#int >> >count
```

```aljam3
[ ] ⚠ PGW10002 — resource from <Bind input (medium confidence)
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/reports/*"
   [.] .path "/data/reports/*"

{-} -BindRead
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] df = pd.read_csv(data_path)                              [ ] ⚠ PGW10002 — data_path from <Bind, type #path
      (-) <Bind#Record
         [.] .data_path#path << $reportPath
      (-) >Bind#Code:Python.Output >> >result
```

```aljam3
[ ] ⚠ PGW10002 — dynamic string construction
{-} -DynamicPath
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] base = "/data/reports/"
         [C] name = input("Enter report name: ")
         [C] df = pd.read_csv(base + name)                            [ ] ⚠ PGW10002 — prefix "/data/reports/" known, suffix unresolvable
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .count#int >> >count
```

**Open point:** None.
