---
audience: developer
rule: "9.28"
code: PGE10014
name: AST-Invisible Foreign Code
severity: error
---

### Rule 9.28 — AST-Invisible Foreign Code
`PGE10014`

<!-- @u:concepts/permissions/foreign-code -->

**Statement:** Foreign code in `-Run.*` pipelines or `[C]` blocks that contains dynamic execution constructs — functions that generate or execute code at runtime, bypassing AST analysis — is a compile error. The compiler cannot verify permission compliance for code it cannot parse.
**Rationale:** Same security logic as SQL injection prevention. If the compiler cannot parse and verify what code does, that code must not run. Dynamic execution constructs are the foreign code equivalent of SQL string concatenation — they create an unanalyzable channel that could perform any IO operation regardless of declared permissions. Banning these constructs ensures the compiler's AST walk sees every possible code path.
**Detection:** During Phase 1 of the foreign code analysis algorithm, the compiler walks every node in the foreign AST and checks against the `BANNED_CONSTRUCTS` table for the target language. If any match is found, PGE10014 fires immediately — no further analysis is performed on the file.

**Banned constructs per language:**

| Language | Banned Construct | Why |
|----------|-----------------|-----|
| Python | `eval()` | Executes arbitrary expression from string |
| Python | `exec()` | Executes arbitrary statement from string |
| Python | `importlib.import_module()` | Dynamic module loading bypasses import analysis |
| Python | `__import__()` | Same as importlib — dynamic import |
| Python | `getattr(module, 'func')()` | Indirect function call — callee not in AST |
| Python | `ctypes.CDLL()` / `ctypes.cdll.LoadLibrary()` | Loads native library — arbitrary syscalls |
| Rust | `unsafe` block containing raw syscalls | Bypasses Rust's safety guarantees |
| Rust | `libloading::Library::new()` / `dlopen` | Dynamic library loading |
| C/C++ | `dlopen()` / `dlsym()` | Dynamic library loading and symbol resolution |
| C/C++ | `asm` / `__asm__` / `__asm` | Inline assembly — arbitrary CPU instructions |
| C/C++ | `system()` without matching `{_}` | Shell execution (redirects to PGE10011 if `{_}` present) |
| JavaScript | `eval()` | Executes arbitrary code from string |
| JavaScript | `new Function()` | Creates function from string — equivalent to eval |
| JavaScript | Dynamic `require(variable)` | Module path not statically known |
| JavaScript | Dynamic `import(variable)` | Same as require — dynamic module loading |
| Shell | `eval` builtin | Executes string as shell command |
| Shell | Backtick substitution with variables | `` `$cmd` `` — command not statically known |

**See also:** PGE10013 (foreign resource outside scope — for analyzable code), PGW10003 (Bind mode opacity — a different kind of unanalyzable code), [[permissions/foreign-code#No AST-Invisible Code]]

**VALID:**
```polyglot
[ ] ✓ standard library calls — fully visible to AST analysis
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -SafeScript
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import json
         [C] with open("/data/config.json") as f:                      [ ] ✓ open() is in sink table — analyzable
         [C]     config = json.loads(f.read())                         [ ] ✓ json.loads is known-pure
         [C] result = str(config.get("key", "default"))                [ ] ✓ str(), dict.get() are known-pure
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

**INVALID:**
```polyglot
[ ] ✗ PGE10014 — eval() generates code at runtime
{_} _DataRead
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -EvalScript
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] eval("open('/etc/passwd').read()")                        [ ] ✗ PGE10014 — eval()
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

```polyglot
[ ] ✗ PGE10014 — exec() executes arbitrary statements
{-} -ExecScript
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] user_code = "import os; os.system('rm -rf /')"
         [C] exec(user_code)                                           [ ] ✗ PGE10014 — exec()
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

```polyglot
[ ] ✗ PGE10014 — __import__() bypasses import analysis
{-} -DynamicImport
   (-) _DataRead
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] mod = __import__("os")                                    [ ] ✗ PGE10014 — __import__()
         [C] mod.system("whoami")
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

**Open point:** None.
