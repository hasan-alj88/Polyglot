---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
---

# Foreign Code Permissions

<!-- @u:syntax/blocks#Foreign Code -->
<!-- @u:pglib/pipelines/Run/INDEX -->
Pipelines using `[C]` foreign code blocks ([[blocks#Foreign Code|u:Foreign Code]]) and `-Run.*` pipelines ([[pglib/pipelines/Run/INDEX|u:Run pipelines]]) interact with permissions as follows:

- The pipeline must declare `{_}` permission objects via `(-)` IO for the IO the foreign code will perform
- The **compiler analyzes** the foreign code AST to verify permission compliance
- **AST-invisible constructs** (eval, exec, dynamic imports) are a **compile error** (PGE10014)
- **Permission violations** detected through AST analysis are a **compile error** (PGE10013)
- **Unverifiable IO calls** (variable indirection the compiler cannot resolve) are a **compile warning** (PGW10002)

## AST Analysis

The compiler parses foreign code into an AST and walks it to detect IO calls. Each detected call is cross-referenced against the pipeline's declared `{_}` permissions.

```polyglot
{_} _DataRead
   [.] .intent << #Grant
   [.] .category << #File
   [.] .capability << #Read
   [.] .scope << "/data/reports/*"
   [.] .path << "/data/reports/*"

{-} -AnalyzeData
   (-) _DataRead
   (-) _ProcessGrant
   (-) ;PythonUV
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      (-) <code.inline <<
         [C] import pandas as pd
         [C] df = pd.read_csv("/data/reports/q1.csv")
         [C] result = df.describe()
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >output
```

The compiler detects `pd.read_csv("/data/reports/q1.csv")` as a `#File.#Read` operation, checks the path `"/data/reports/q1.csv"` against `_DataRead.scope` (`"/data/reports/*"`), and confirms the call is within declared permissions.

## No AST-Invisible Code

<!-- @c:technical/compile-rules/PGE/PGE10014-ast-invisible-foreign-code -->
Foreign code that evades AST analysis is a compile error ([[technical/compile-rules/PGE/PGE10014-ast-invisible-foreign-code|PGE10014]]). This follows the same security logic as SQL injection prevention — if the system cannot parse and verify what code does, that code must not run.

**Banned constructs** (maintained in the AST-invisible registry — [[technical/compiler/ast-invisible-registry]]):

| Language | Example Banned Constructs |
|----------|--------------------------|
| Python | `eval()`, `exec()`, `compile()`, `importlib.import_module()`, `__import__()`, `getattr(obj, name)()`, `ctypes.CDLL()`, `pickle.loads()`, `marshal.loads()` |
| Rust | `asm!()` macro, `unsafe` blocks with raw syscalls, `libloading::Library::new()`, `dlopen`/`dlsym` |
| C/C++ | `dlopen()`/`dlsym()`, `GetProcAddress()`, `LoadLibrary()`, inline assembly (`asm`/`__asm__`), `system()` without matching `{_}` |
| JavaScript | `eval()`, `new Function()`, `Function.prototype.constructor(string)`, indirect eval, dynamic `require(variable)`, dynamic `import(variable)` |
| Shell | `eval` builtin, backtick substitution with variables, `${!variable}` indirect expansion |

The complete list is maintained in the AST-invisible registry and grows independently of compiler releases. Packages can declare additional banned wrappers via `ast-invisible-registry-ext.toml`.

## Permission Violation Detection

The compiler classifies detected IO calls by category and checks resource arguments against declared `{_}` scopes:

| Detection | Result | Rule |
|-----------|--------|------|
| Literal path/host outside declared `{_}` scope | **Compile error** | PGE10013 |
| IO call with no matching `{_}` category declared | **Compile error** | PGE10013 |
| AST-invisible construct (eval, exec, etc.) | **Compile error** | PGE10014 |
| IO call detected but resource value not resolvable | **Compile warning** | PGW10002 |
| Function call not in IO registry | **Compile warning** | PGW10005 |

## Per-Mode Analysis

<!-- @u:pglib/pipelines/Run/INDEX -->
Each `-Run.*` mode ([[pglib/pipelines/Run/INDEX|u:Run pipelines]]) has a different level of AST analysis:

| Mode | AST Analysis | Notes |
|------|-------------|-------|
| `-Run.<Lang>.Function` | Full (inline) | Arg types + function body analyzed |
| `-Run.<Lang>.Script` | Full (inline and file) | Full AST scan; `<code.file` parsed at compile time |
| `-Run.<Lang>.CLI` | Command path only | Compiled binary — sandbox-only enforcement |
| `-Run.<Lang>.Bind` | None | Fully opaque — PGW10003 warning; sandbox-only enforcement |
| `-Run.Shell` | Limited | Shell AST parsed; variable expansion triggers PGW10006 |

Both `<code.inline` (with `[C]` blocks) and `<code.file` are AST-analyzed at compile time. The content hash provides integrity verification and caching; the AST provides permission compliance checking.

## Sandbox Defense-in-Depth

<!-- @c:permissions/enforcement#Foreign Code Sandbox -->
In addition to compile-time AST analysis, the Polyglot Service applies OS-level restrictions (Landlock, seccomp) before spawning the job process. See [[permissions/enforcement#Foreign Code Sandbox|c:Foreign Code Sandbox]] for details.
