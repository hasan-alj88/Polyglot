# Quick Reference: Runtime Orchestration - v0.0.5

**Version:** 0.0.5
**Last Updated:** 2026-01-04
**Status:** Official Reference

---

## TL;DR

Execute Python, Rust, or JavaScript code from Polyglot pipelines using:
1. Wrapper: `|W.RT.{Language}` → creates runtime environment
2. Pipeline: `|RT.{Language}.Code` → executes code in that environment
3. Environment: Pass via `<env.vars.*:string` (all string type)
4. Code: Multi-line using `[c]` marker

---

## Pattern Template

```polyglot
%% 1. Declare wrapper (before execution markers!)
[w] |W.RT.{Language}
 |  <version:string << "{version}"
 |  >environment-RTenv-{language} >> ${language}Env

%% 2. Execute code
[r] |RT.{Language}.Code
 |  <env.lang-RTenv-{language} << ${language}Env
 |  <env.vars.{var}:string << $value  %% All env.vars are :string
 |  <code:string << |{Language}""
[c] // Your code here
 |  >exit_code:uint >> $exitCode
```

---

## Supported Languages

### Python

**Wrapper:** `|W.RT.Python`
**Pipeline:** `|RT.Python.Code`
**Environment:** `-RTenv-python`

```polyglot
[w] |W.RT.Python
 |  <version:string << "3.11"
 |  >environment-RTenv-python >> $pyEnv

[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.log_file:string << $logPath
 |  <code:string << |Python""
[c] import os
[c] log_file = os.environ['log_file']
[c] print(f"Logging to {log_file}")
 |  >exit_code:uint >> $exitCode
```

### Rust

**Wrapper:** `|W.RT.Rust`
**Pipeline:** `|RT.Rust.Code`
**Environment:** `-RTenv-rust`

```polyglot
[w] |W.RT.Rust
 |  <version:string << "1.75"
 |  >environment-RTenv-rust >> $rustEnv

[r] |RT.Rust.Code
 |  <env.lang-RTenv-rust << $rustEnv
 |  <env.vars.output_path:string << $path
 |  <code:string << |Rust""
[c] use std::env;
[c] let path = env::var("output_path").unwrap();
[c] println!("Output: {}", path);
 |  >exit_code:uint >> $exitCode
```

### JavaScript/Node.js

**Wrapper:** `|W.RT.JavaScript`
**Pipeline:** `|RT.JavaScript.Code`
**Environment:** `-RTenv-javascript`

```polyglot
[w] |W.RT.JavaScript
 |  <version:string << "20"
 |  >environment-RTenv-javascript >> $jsEnv

[r] |RT.JavaScript.Code
 |  <env.lang-RTenv-javascript << $jsEnv
 |  <env.vars.config_path:string << $configPath
 |  <code:string << |JavaScript""
[c] const configPath = process.env.config_path;
[c] console.log(`Config: ${configPath}`);
 |  >exit_code:uint >> $exitCode
```

---

## Environment Variables

### All Variables are `:string`

**Pattern:** `<env.vars.{name}:string << $value`

```polyglot
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.db_host:string << "localhost"
 |  <env.vars.db_port:string << "5432"  %% Not :uint!
 |  <env.vars.debug:string << "true"    %% Not :bool!
 |  <code:string << |Python""
[c] import os
[c] host = os.environ['db_host']
[c] port = int(os.environ['db_port'])      %% Convert in code
[c] debug = os.environ['debug'] == 'true'  %% Convert in code
```

**Why all `:string`?**
- Environment variables stored in Shell/CMD as strings
- Runtime code performs type conversion as needed
- Consistent with OS environment variable behavior

---

## Keyword Arguments (Alternative)

For **single-function code blocks only**, use native types:

```polyglot
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <kwargs.user_id:py.int << $userId
 |  <kwargs.name:py.str << $userName
 |  <kwargs.active:py.bool << $isActive
 |  <code:string << |Python""
[c] def process_user(user_id, name, active):
[c]     print(f"User {user_id}: {name} (active={active})")
[c] process_user(user_id, name, active)  %% Function called with kwargs
```

**Native Type Mapping:**

| Polyglot | Python | Rust | JavaScript |
|----------|--------|------|------------|
| `:string` | `:py.str` | `:rust.String` | `:js.string` |
| `:uint` | `:py.int` | `:rust.u32` | `:js.number` |
| `:int` | `:py.int` | `:rust.i32` | `:js.number` |
| `:bool` | `:py.bool` | `:rust.bool` | `:js.boolean` |
| `:float` | `:py.float` | `:rust.f64` | `:js.number` |

**Recommendation:** Use `<env.vars.*` for general code, `<kwargs.*` only for single-function blocks.

---

## Error Handling

All runtime pipelines can error:

```polyglot
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <code:string << |Python""
[c] raise Exception("Something went wrong")
 |  >exit_code:uint >> $exitCode
   [!] !RT.Python.Error
      [r] $status << "failed"
   [!] !*
      [r] $status << "success"
```

**Error Types:**
- `!RT.Python.Error` - Python execution failed
- `!RT.Rust.Error` - Rust compilation or execution failed
- `!RT.JavaScript.Error` - JavaScript execution failed

---

## Critical Rules

### 1. Wrapper Ordering

**RULE:** ALL `[w]` markers MUST come BEFORE ALL execution markers.

```polyglot
%% CORRECT
{|} |MyPipeline
[t] |T.Cli"run"
[<] <input:string
[>] >output:serial

[w] |W.RT.Python >> $pyEnv    %% Wrapper
[w] |W.RT.Rust >> $rustEnv    %% Wrapper

[r] |RT.Python.Code           %% Execution marker
[r] |RT.Rust.Code             %% Execution marker
{x}

%% WRONG - COMPILE ERROR
{|} |MyPipeline
[r] $value << 42              %% Execution marker
[w] |W.RT.Python              %% ERROR: Wrapper after execution!
{x}
```

### 2. Pipeline Structure Order

```
{|} → [t] → [<] → [>] → [w] → [r][p][*][b][f][s][>]
```

### 3. One Expression Per Line

```polyglot
%% CORRECT
[r] $a << 1
[r] $b << 2

%% WRONG
[r] $a << 1; [r] $b << 2  %% Multiple expressions!
```

---

## Complete Example

```polyglot
{@} @Local:DataProcessor:1.0.0
{x}

{#} #Config
[A] #AppConfig
[s] << |YAML.Load"\\FileDir\\config.yaml"
   [.] .python_version:string << .runtimes.python
   [.] .input_file:path << .paths.input
   [.] .output_file:path << .paths.output
[s][!] !*
{x}

{|} |ProcessData
[t] |T.Cli"process"

[<] <config#Config  %% Input shorthand - value implied

%% ALL WRAPPERS FIRST
[w] |W.RT.Python
 |  <version:string << $config.python_version
 |  >environment-RTenv-python >> $pyEnv

%% Process with Python
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.input_file:string << $config.input_file
 |  <env.vars.output_file:string << $config.output_file
 |  <code:string << |Python""
[c] import os
[c] import json
[c]
[c] input_file = os.environ['input_file']
[c] output_file = os.environ['output_file']
[c]
[c] with open(input_file, 'r') as f:
[c]     data = json.load(f)
[c]
[c] processed = {k: v.upper() for k, v in data.items()}
[c]
[c] with open(output_file, 'w') as f:
[c]     json.dump(processed, f, indent=2)
 |  >exit_code:uint >> $exitCode
   [!] !RT.Python.Error
      [>] >result
         [.] .success:bool << -False
         [.] .message:string << "Processing failed"
   [!] !*
      [>] >result
         [.] .success:bool << -True
         [.] .message:string << "Processing complete"
{x}
```

---

## Common Mistakes

### ❌ Mistake 1: Wrong Environment Variable Type

```polyglot
<env.vars.port:uint << 5432  %% WRONG!
```

**Fix:**
```polyglot
<env.vars.port:string << "5432"  %% Correct
```

### ❌ Mistake 2: Wrapper After Execution

```polyglot
[r] $value << 42
[w] |W.RT.Python  %% ERROR!
```

**Fix:**
```polyglot
[w] |W.RT.Python  %% Wrapper first
[r] $value << 42
```

### ❌ Mistake 3: Empty Init Then Fields

```polyglot
<env.vars:serial << {:}              %% Contradicts next line
<env.vars.log:string << $logPath     %% Negates previous
```

**Fix:**
```polyglot
<env.vars.log:string << $logPath  %% Just set what you need
```

### ❌ Mistake 4: Boolean Literals

```polyglot
<result.success:bool << true  %% WRONG: lowercase literal
```

**Fix:**
```polyglot
<result.success:bool << -True  %% Correct: reserved enum
```

---

## See Also

- [Hello World Multi-Runtime Example](../examples/hello-world-multi-runtime.pg)
- [Training Session 001](../training-sessions/session-001-2026-01-02.md)
- [Runtime Wrappers](../stdlib/standard-wrappers.yaml)
- [Runtime Pipelines](../stdlib/standard-pipelines.yaml)
- [Reserved Enums](../stdlib/reserved-enums.yaml)

---

**Status:** ✅ Official Reference
**Version:** 0.0.5
**Last Updated:** 2026-01-04
