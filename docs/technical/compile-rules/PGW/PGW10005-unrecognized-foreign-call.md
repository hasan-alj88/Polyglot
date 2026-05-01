---
audience: developer
rule: "9.6"
code: PGW10005
name: Unrecognized Foreign Call
severity: warning
---

# Rule 9.6 — Unrecognized Foreign Call
`PGW10005`

<!-- @u:concepts/permissions/foreign-code -->
<!-- @c:technical/algorithms/foreign-code-analysis -->
<!-- @c:technical/compiler/io-registry -->

**Statement:** When the compiler's AST analysis encounters a function call in foreign code that is not found in the IO sink table and is not in the known-pure function list, PGW10005 fires. The compiler cannot classify the call as IO or pure computation.
**Rationale:** The IO sink table covers standard library and common third-party IO functions. Unknown calls may be pure computation (safe) or may perform IO through libraries the registry does not cover. The warning ensures developers are aware of gaps in the compiler's analysis. Bottom-up analysis helps: if an unknown function's body contains known IO calls, the warning includes the IO category. If the function is truly unknown, the warning is generic.
**Detection:** During the foreign code AST walk:
1. Each `CallExpression` is resolved to a canonical name via import alias resolution
2. If the canonical name is in the IO sink table → handled by PGE10013/PGW10002 (not this rule)
3. If the canonical name is in the known-pure function list (builtins like `len`, `str`, `sorted`, `enumerate`, `range`, `zip`, `isinstance`, `type`, string methods like `.upper()`, `.split()`, `.join()`) → silently skipped
4. If the canonical name is in neither → PGW10005 fires
5. Bottom-up enhancement: if the function body (when visible in the same file) contains calls to known IO functions, the warning includes the detected IO category

**See also:** PGW10002 (unverifiable foreign IO — known IO function, unresolvable argument), PGE10014 (AST-invisible code — banned dynamic constructs), [[permissions/foreign-code#IO Detection]]

**WARNING:**
```aljam3
[ ] ⚠ PGW10005 — unknown function, not in sink table or known-pure list
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -CustomLib
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import custom_lib
         [C] data = custom_lib.process(raw_data)                       [ ] ⚠ PGW10005 — custom_lib.process not in registry
         [C] result = len(data)                                        [ ] ✓ len() is known-pure — no warning
      (-) <Bind#Record
         [.] .raw_data#string << $input
      (-) >Bind#Record
         [.] .result#int >> >result
```

```aljam3
[ ] ⚠ PGW10005 — bottom-up: function body contains known IO
{-} -BottomUpDetect
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] def load_data(path):
         [C]     with open(path) as f:                                 [ ] open() is known File IO
         [C]         return f.read()
         [C]
         [C] result = load_data("/data/report.csv")                    [ ] ⚠ PGW10005 — load_data not in registry,
         [C]                                                           [ ]   but contains open() (#File IO detected)
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

**Open point:** None.
