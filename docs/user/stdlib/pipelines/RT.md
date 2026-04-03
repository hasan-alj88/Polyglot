---
audience: pg-coder
type: specification
updated: 2026-04-01
status: complete
---

# =RT — Runtime Execution

<!-- @pipelines -->
Runtime execution pipelines run foreign code (Python, Rust, etc.) within Polyglot pipelines. Each `=RT.<Lang>.*` pipeline takes a language-specific environment handle from `=W.RT` and executes code in that runtime.

No `[@]` import needed.

**PRIMITIVE** — Stdlib runtime pipelines are direct language runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

## Permissions

<!-- @permissions -->
All `=RT.*` pipelines require `[_] _System.process` permission in the `{@}` package block. See [[permissions]] for the permission system.

| Pipeline | Permission | Type |
|----------|-----------|------|
| `=RT.<Lang>.*` | `_System.process` | IO |

## Execution Modes

| Mode | Purpose | Input | Return value |
|------|---------|-------|-------------|
| `.Function` | Call a named function with args | `<func`, `<arg`, `<kwarg` | `>return#serial` |
| `.Script` | Run code with variable bindings | `<Bind` (opt) | `>Bind#serial` (opt) |
| `.CLI` | Invoke compiled binary | `<binary`, `<arg`, `<kwarg` | — |
| `.Bind` | Native code uses polyglot lib | — | — |

All modes except `.CLI` are split into `.Inline` (code via `[C]` blocks) and `.File` (source via `<file#path`). `.CLI` is inherently file-based (`<binary#path`).

### `.Script` vs `.Bind` — Binding Origin

Both `.Script` and `.Bind` execute code without a named function entry point. The difference is **who controls the data flow**:

- **`.Script`** — Polyglot-controlled binding. `<Bind#serial` injects Polyglot data as named variables before execution. `>Bind#serial` captures their final state after execution. The compiler can validate bound variable names exist in the code.
- **`.Bind`** — Foreign-code-controlled binding. Native code imports the polyglot lib and calls `pull("name")`/`push("name", value)` at arbitrary points during execution. The compiler cannot validate these — they are opaque runtime strings.

### Multiple `[W]` Wrappers

A pipeline may declare multiple `[W]` wrappers (e.g., one Python + one Rust). Setup runs in declaration order; cleanup runs in reverse order (bracket semantics).

## `.Function` — Call Named Function

Call a named function in foreign code, pass arguments, get a return value. The compiler validates that `<func` names a function that exists in the `[C]` block or source file.

### `.Function.Inline`

```
=RT.<Lang>.Function.Inline
   <env#<Lang>Env             [ ] Runtime environment from =W.RT
   <func#string               [ ] Function name (compiler-validated)
   <arg#array.string           [ ] Positional arguments
   <kwarg#map:string:string    [ ] Keyword arguments (optional)
   >output#Code:<Lang>.Output  [ ] .stdout, .stderr capture
   >return#serial              [ ] Function return value
   <code#string                [ ] Inline code via [C] blocks
```

```polyglot
{@} @Local:Example.PythonStats
   [_] _System.process

{=} =CalculateStats
   [=] <numbers#array.string
   [=] >result#Code:Python.Output
   [=] >stats#serial
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.RT:Python:3:14
      [=] >RTpy#PyEnv >> $pyenv

   [r] =RT.Python.Function.Inline
      [=] <env#PyEnv << $pyenv
      [=] <func#string << "calculate"
      [=] <arg#array.string << $numbers
      [=] >output#Code:Python.Output >> >result
      [=] >return#serial >> >stats
      [=] <code#string <<
         [C] import statistics
         [C] def calculate(args):
         [C]     nums = [float(x) for x in args]
         [C]     return {"mean": statistics.mean(nums), "stdev": statistics.stdev(nums)}
```

### `.Function.File`

```
=RT.<Lang>.Function.File
   <env#<Lang>Env             [ ] Runtime environment from =W.RT
   <func#string               [ ] Function name (compiler-validated against file)
   <arg#array.string           [ ] Positional arguments
   <kwarg#map:string:string    [ ] Keyword arguments (optional)
   >output#Code:<Lang>.Output  [ ] .stdout, .stderr capture
   >return#serial              [ ] Function return value
   <file#path                  [ ] Path to source file
```

```polyglot
[r] =RT.Python.Function.File
   [=] <env#PyEnv << $pyenv
   [=] <func#string << "calculate"
   [=] <arg#array.string << $numbers
   [=] <kwarg#map:string:string << {"precision": "4"}
   [=] >output#Code:Python.Output >> >result
   [=] >return#serial >> >stats
   [=] <file#path << =Path"/scripts/stats.py"
```

## `.Script` — Run Code with Variable Bindings

Run code without a named entry point. Optionally inject Polyglot data as named variables via `<Bind` and capture final variable state via `>Bind`. The compiler validates that bound variable names exist in the code or file.

### `.Script.Inline`

```
=RT.<Lang>.Script.Inline
   <env#<Lang>Env             [ ] Runtime environment from =W.RT
   <Bind#serial                [ ] Variable bindings injected as code variables (optional)
   >Bind#serial                [ ] Final state of bound variables (optional)
   >output#Code:<Lang>.Output  [ ] .stdout, .stderr capture
   <code#string                [ ] Inline code via [C] blocks
```

```polyglot
[r] =RT.Python.Script.Inline
   [=] <env#PyEnv << $pyenv
   [=] <Bind#serial << {"target_dir": $targetDir, "deleted_count": 0}
   [=] >output#Code:Python.Output >> >log
   [=] >Bind#serial >> >state
   [=] <code#string <<
      [C] import os, glob
      [C] files = glob.glob(target_dir + "/stale_*.log")
      [C] for f in files:
      [C]     os.remove(f)
      [C] deleted_count = len(files)
```

### `.Script.File`

```
=RT.<Lang>.Script.File
   <env#<Lang>Env             [ ] Runtime environment from =W.RT
   <Bind#serial                [ ] Variable bindings (optional)
   >Bind#serial                [ ] Final state of bound variables (optional)
   >output#Code:<Lang>.Output  [ ] .stdout, .stderr capture
   <file#path                  [ ] Path to source file
```

```polyglot
[r] =RT.Python.Script.File
   [=] <env#PyEnv << $pyenv
   [=] <Bind#serial << {"target_dir": $targetDir}
   [=] >output#Code:Python.Output >> >log
   [=] >Bind#serial >> >state
   [=] <file#path << =Path"/scripts/cleanup.py"
```

## `.CLI` — Invoke Compiled Binary

Invoke a compiled binary with positional and keyword arguments. The OS runs the binary — no language runtime is needed. Use `[W] =W.Polyglot` (not `=W.RT`). No `<env` parameter.

```
=RT.<Lang>.CLI
   <binary#path               [ ] Path to executable
   <arg#array.string           [ ] Positional arguments (optional)
   <kwarg#map:string:string    [ ] CLI flags (optional)
   >output#Code:<Lang>.Output  [ ] Console capture (.stdout, .stderr)
```

```polyglot
{@} @Local:Example.RustBinary
   [_] _System.process

{=} =RunRustTool
   [=] <inputPath#path
   [=] >toolOutput#Code:Rust.Output
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [r] =RT.Rust.CLI
      [=] <binary#path << =Path"/usr/local/bin/mytool"
      [=] <arg#array.string << ["{$inputPath}"]
      [=] <kwarg#map:string:string << {"--format": "json", "--verbose": "true"}
      [=] >output#Code:Rust.Output >> >toolOutput
```

## `.Bind` — Native Code Uses Polyglot Lib

Native code imports the polyglot lib and calls `pull()`/`push()` to interact with Polyglot IO ports. No `<Bind`, `<arg`, or `<kwarg` — the foreign code controls all data flow.

### `.Bind.Inline`

```
=RT.<Lang>.Bind.Inline
   <env#<Lang>Env             [ ] Runtime environment from =W.RT
   >output#Code:<Lang>.Output  [ ] .stdout, .stderr capture
   <code#string                [ ] Inline code via [C] blocks
```

```polyglot
[r] =RT.Python.Bind.Inline
   [=] <env#PyEnv << $pyenv
   [=] >output#Code:Python.Output >> >inlineResult
   [=] <code#string <<
      [C] from polyglot import pull, push
      [C] data = pull("input_data")
      [C] push("result", data.upper())
```

### `.Bind.File`

```
=RT.<Lang>.Bind.File
   <env#<Lang>Env             [ ] Runtime environment from =W.RT
   >output#Code:<Lang>.Output  [ ] .stdout, .stderr capture
   <file#path                  [ ] Path to source file
```

```polyglot
[r] =RT.Python.Bind.File
   [=] <env#PyEnv << $pyenv
   [=] >output#Code:Python.Output >> >fileResult
   [=] <file#path << =Path"/scripts/transform.py"
```

## IO Summary

| Mode | `<env` | `<func` | `<arg` | `<kwarg` | `<Bind` | `>Bind` | `>output` | `>return` | `<code`/`<file` |
|------|--------|---------|--------|----------|---------|---------|-----------|-----------|-----------------|
| `.Function.Inline` | yes | yes | yes | opt | — | — | yes | yes | `<code` |
| `.Function.File` | yes | yes | yes | opt | — | — | yes | yes | `<file` |
| `.Script.Inline` | yes | — | — | — | opt | opt | yes | — | `<code` |
| `.Script.File` | yes | — | — | — | opt | opt | yes | — | `<file` |
| `.CLI` | — | — | opt | opt | — | — | yes | — | `<binary` |
| `.Bind.Inline` | yes | — | — | — | — | — | yes | — | `<code` |
| `.Bind.File` | yes | — | — | — | — | — | yes | — | `<file` |

## Compiler Validation

| Mode | What the compiler validates |
|------|---------------------------|
| `.Function` | `<func` name exists as a function definition in the `[C]` block or source file |
| `.Script` | `<Bind` variable names exist as identifiers in the code or file |
| `.CLI` | No code validation — binary is opaque |
| `.Bind` | No validation — `pull()`/`push()` calls are opaque runtime strings |

## Related

- [[stdlib/pipelines/W|=W]] — `=W.RT` wrapper that manages runtime environments
- [[stdlib/types/rt|Runtime types]] — `#Code`, `#PyEnv`, `#RsEnv`
- [[stdlib/errors/errors|errors]] — `!RT` error namespace
- [[syntax/blocks|blocks]] — `[C]` inline foreign code element

NOTE: Retry/timeout/rate-limiting are `[Q]` queue strategies, not runtime pipeline concerns.
