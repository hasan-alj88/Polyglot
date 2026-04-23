---
audience: developer
type: specification
status: complete
updated: 2026-04-18
---

# AST-Invisible Functions Registry

<!-- @c:technical/algorithms/foreign-code-analysis -->
<!-- @u:concepts/permissions/foreign-code -->
<!-- @c:technical/compiler/io-registry -->

The AST-invisible registry maps foreign function names and constructs that evade static AST analysis to their categories and ban reasons. It is the data source for PGE10014 — the compiler loads this registry and rejects any foreign code containing a match.

## Overview

Unlike the IO registry (which maps *analyzable* IO calls to permission categories), this registry lists constructs that **cannot be analyzed at all**. These are functions that generate, load, or execute code at runtime — the compiler cannot see what they will do, so they must be banned outright.

For the analyzable side of the analysis surface, see [[io-registry|IO Registry]] — together they form the complete foreign-code analysis input.

The registry is a separate configuration file (`ast-invisible-registry.toml`) that ships with the compiler but is versioned independently. It contains:

- **Banned constructs** — per-language tables of functions/constructs that evade AST analysis
- **Categories** — dynamic execution, dynamic loading, inline assembly, dynamic code generation, serialization-based execution
- **Ban reasons** — why each construct is unanalyzable
- **Package extensions** — packages can declare their own AST-invisible wrappers

## Registry Structure

The registry uses TOML format, organized by language and category:

```toml
[meta]
version = "2026.04"
compiler_min = "0.1.0"

[python.dynamic_execution]
"builtins.eval" = { reason = "Executes arbitrary expression from string" }
"builtins.exec" = { reason = "Executes arbitrary statement from string" }
"builtins.compile" = { reason = "Compiles source string into code object for later execution" }
"code.InteractiveInterpreter.runsource" = { reason = "Interprets and executes code strings" }
"code.InteractiveInterpreter.runcode" = { reason = "Executes code objects created from strings" }

[python.dynamic_loading]
"importlib.import_module" = { reason = "Imports module by string name at runtime" }
"importlib.util.spec_from_file_location" = { reason = "Loads module from dynamic file path" }
"importlib.util.module_from_spec" = { reason = "Creates module object from dynamic specification" }
"builtins.__import__" = { reason = "Low-level dynamic import bypassing static import statements" }

[python.ffi]
"ctypes.CDLL" = { reason = "Loads native C library dynamically — arbitrary syscalls" }
"ctypes.WinDLL" = { reason = "Loads Windows DLL dynamically" }
"ctypes.cdll.LoadLibrary" = { reason = "Loads native library by name" }
"ctypes.util.find_library" = { reason = "Resolves and enables dynamic loading of system libraries" }

[python.serialization]
"pickle.loads" = { reason = "Deserializes Python objects — can execute arbitrary code via __reduce__" }
"pickle.load" = { reason = "Deserializes from file — can execute arbitrary code via __reduce__" }
"marshal.loads" = { reason = "Deserializes Python bytecode objects — executable code" }
"marshal.load" = { reason = "Loads marshaled bytecode from file — executable code" }

[python.reflection]
"builtins.getattr().call" = { pattern = "getattr(obj, name)()", reason = "Indirect function call — callee not visible in AST" }

# ...additional languages follow same structure
```

## Python

| Function | Category | Why Banned |
|----------|----------|------------|
| `eval()` | Dynamic execution | Executes arbitrary expression from string |
| `exec()` | Dynamic execution | Executes arbitrary statement from string |
| `compile()` | Dynamic execution | Compiles source string into code object for later execution |
| `code.InteractiveInterpreter.runsource()` | Dynamic execution | Interprets and executes code strings in interactive context |
| `code.InteractiveInterpreter.runcode()` | Dynamic execution | Executes code objects created from strings |
| `importlib.import_module()` | Dynamic loading | Imports module by string name at runtime |
| `importlib.util.spec_from_file_location()` | Dynamic loading | Loads module from dynamic file path |
| `importlib.util.module_from_spec()` | Dynamic loading | Creates module object from dynamic specification |
| `__import__()` | Dynamic loading | Low-level dynamic import bypassing static import statements |
| `getattr(obj, name)()` | Reflection | Indirect function call — callee not visible in AST |
| `ctypes.CDLL()` | FFI | Loads native C library dynamically — arbitrary syscalls |
| `ctypes.WinDLL()` | FFI | Loads Windows DLL dynamically |
| `ctypes.cdll.LoadLibrary()` | FFI | Loads native library by name |
| `ctypes.util.find_library()` | FFI | Resolves and enables dynamic loading of system libraries |
| `pickle.loads()` | Serialization | Deserializes Python objects — can execute arbitrary code via `__reduce__` |
| `pickle.load()` | Serialization | Deserializes from file — can execute arbitrary code via `__reduce__` |
| `marshal.loads()` | Serialization | Deserializes Python bytecode objects — executable code |
| `marshal.load()` | Serialization | Loads marshaled bytecode from file — executable code |

**Note:** `os.system()`, `subprocess.*`, and other process execution functions are **not** in this registry — they are analyzable IO calls handled by the [[compiler/io-registry]] sink tables and PGE10011.

## Rust

| Construct | Category | Why Banned |
|-----------|----------|------------|
| `asm!()` macro | Inline assembly | Embeds arbitrary machine code, bypasses all Rust safety guarantees |
| `unsafe` block containing raw syscalls | Unsafe operations | Direct syscalls bypass Rust's safety model and permission analysis |
| `libloading::Library::new()` | Dynamic loading | Loads shared library at runtime — arbitrary code execution |
| `dlopen()` via libc | Dynamic loading | C-level dynamic library loading |
| `dlsym()` via libc | Dynamic loading | Retrieves function address from loaded library for indirect call |

**Note:** General `unsafe` blocks that do not contain raw syscalls are **not** banned — they are flagged by PGW10003 (opacity warning) instead. Only `unsafe` blocks containing direct syscall invocations (e.g., `libc::syscall()`, raw `asm!`) trigger PGE10014.

## C/C++

| Construct | Category | Why Banned |
|-----------|----------|------------|
| `dlopen()` | Dynamic loading | Loads shared library at runtime — arbitrary code execution |
| `dlsym()` | Dynamic loading | Retrieves function address from loaded library for indirect call |
| `GetProcAddress()` | Dynamic loading | Windows API for retrieving function addresses from DLLs |
| `LoadLibrary()` / `LoadLibraryEx()` | Dynamic loading | Windows API for dynamic DLL loading |
| `asm` / `__asm__` / `__asm` | Inline assembly | Inline assembly — arbitrary CPU instructions |
| `asm volatile` | Inline assembly | Volatile inline assembly — cannot be optimized away |
| `system()` | Process execution | Executes shell command string (redirects to PGE10011 if `{_}` #System.#Shell present) |

**Note:** `system()` appears in both registries. In the IO registry, it is an analyzable sink when a matching `{_}` #System.#Shell permission exists. In this registry, it triggers PGE10014 when **no** matching `{_}` is declared — the compiler cannot verify what the shell command will do without a permission scope.

## JavaScript

| Function | Category | Why Banned |
|----------|----------|------------|
| `eval()` | Dynamic execution | Evaluates string as JavaScript code with access to local scope |
| `new Function()` | Dynamic execution | Creates function object from string arguments |
| `Function.prototype.constructor(string)` | Dynamic execution | Constructor that creates functions from code strings |
| Indirect eval `(0, eval)(code)` | Dynamic execution | Indirect eval call evaluated in global scope |
| Dynamic `import(variable)` | Dynamic loading | Module path not statically known — runtime module loading |
| Dynamic `require(variable)` | Dynamic loading | Module path not statically known — runtime module loading |

**Note:** Static `import 'module'` and `require('literal')` are **not** banned — only dynamic forms where the module path is a variable or expression.

## Shell/Bash

| Construct | Category | Why Banned |
|-----------|----------|------------|
| `eval` builtin | Dynamic execution | Re-parses and executes command strings with variable expansion |
| Backtick substitution with variables `` `$cmd` `` | Dynamic execution | Command not statically known — executes computed string |
| `${!variable}` indirect expansion | Reflection | Accesses variable whose name is stored in another variable |

**Note:** Static command substitution `` `literal` `` and `$(literal)` are **not** banned — only forms where the command is derived from a variable, making the executed code invisible to AST analysis.

## Categories

| Category | Description | Risk |
|----------|------------|------|
| Dynamic execution | Functions that execute code from strings (`eval`, `exec`, `compile`) | Critical — arbitrary code runs outside AST visibility |
| Dynamic loading | Functions that load modules/libraries by computed name or path | High — loaded code is not visible to compiler |
| Inline assembly | Constructs that embed raw CPU instructions | Critical — bypasses all language safety guarantees |
| FFI | Foreign function interface constructs that load native code | High — loaded native code performs arbitrary operations |
| Reflection | Indirect function calls where the callee is not statically known | High — compiler cannot determine what function executes |
| Serialization | Deserialization functions that can execute code during object reconstruction | High — `pickle.__reduce__` and `marshal` can run arbitrary code |

## Package-Level Extension

Developers can declare additional AST-invisible wrappers for libraries that wrap banned functions. Extensions are package-scoped and travel with the code:

```toml
# In package's ast-invisible-registry-ext.toml
[python.dynamic_execution]
"custom_lib.run_code" = { reason = "Wraps exec() — dynamic code execution" }
"custom_lib.dynamic_eval" = { reason = "Wraps eval() — dynamic expression evaluation" }

[python.serialization]
"custom_lib.deserialize" = { reason = "Wraps pickle.loads — arbitrary code execution on deserialize" }
```

Extensions follow the same format as the built-in registry and are merged at compile time. This ensures that third-party wrappers around banned functions cannot bypass PGE10014 detection.

## Versioning and Updates

- **Ships with compiler** — each compiler release includes a registry version
- **Registry patches** — can be updated independently of compiler releases (same model as `io-registry.toml`)
- **Community submissions** — community can submit registry entries for newly discovered AST-invisible patterns
- **Version format** — `YYYY.MM` (e.g., `2026.04`)

## Compiler Integration

The compiler loads the registry at startup and uses it during Phase 2.1 of the foreign code analysis algorithm:

```text
LOAD ast-invisible-registry.toml → BANNED_CONSTRUCTS[language]

FOR each node IN ast.walk():
  IF node matches BANNED_CONSTRUCTS[language]:
    EMIT PGE10014(node, construct_name, category, reason)
    ABORT compilation
```

The `is_banned_construct(node, language)` method on the `ForeignParser` trait ([[compiler/foreign-code-parsers]]) consumes this registry to perform the check.

## Related

- [[compiler/io-registry]] — companion registry for *analyzable* IO calls (sink tables)
- [[algorithms/foreign-code-analysis]] — detection algorithm that consumes this registry (Phase 2.1)
- PGE10014 — compile error fired when a match is found
- [[compiler/foreign-code-parsers]] — parser trait with `is_banned_construct()` method
