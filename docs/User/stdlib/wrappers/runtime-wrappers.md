# Runtime Wrappers
**Category:** Standard Library - Wrappers
**Prefix:** `|W.Runtime.*`
**Since:** v0.0.4
**Status:** ✅ Verified in Training

---

## Overview

Runtime wrappers provide integration with external programming language runtimes (Python, Rust, JavaScript, etc.), allowing Polyglot pipelines to execute code in other languages and manage their execution environments.

**Key Features:**
- Version-specific runtime isolation
- Configuration file support (requirements.txt, Cargo.toml, package.json)
- Session management for stateful execution
- Code and file execution modes
- Multi-language integration in single pipeline

---

## Available Runtime Wrappers

### Python Runtime

#### |W.Runtime.Python{VERSION}

Execute Python code with specific Python version.

**Available Versions:**
- `|W.Runtime.Python3.11` - Python 3.11.x
- `|W.Runtime.Python3.10` - Python 3.10.x
- `|W.Runtime.Python3.12` - Python 3.12.x

**Inputs:**
- `<requirements:pg.path` (optional) - Path to requirements.txt
- `<venv:pg.path` (optional) - Path to virtual environment

**Outputs:**
- `>session:serial` - Runtime session handle

**Example:**
```polyglot
[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\FileDir\\python\requirements.txt
(|) >session:serial >> $py_session
```

---

### Rust Runtime

#### |W.Runtime.Rust{VERSION}

Execute Rust code with specific Rust toolchain version.

**Available Versions:**
- `|W.Runtime.Rust1.75` - Rust 1.75.x
- `|W.Runtime.Rust1.74` - Rust 1.74.x
- `|W.Runtime.Rust1.76` - Rust 1.76.x

**Inputs:**
- `<manifest:pg.path` (optional) - Path to Cargo.toml
- `<edition:string` (optional) - Rust edition (2018, 2021)

**Outputs:**
- `>session:serial` - Runtime session handle

**Example:**
```polyglot
[w] |W.Runtime.Rust1.75
(|) >session:serial >> $rust_session
```

---

### JavaScript/Node.js Runtime

#### |W.Runtime.JavaScript.Node.{VERSION}

Execute JavaScript code with specific Node.js version.

**Available Versions:**
- `|W.Runtime.JavaScript.Node.20` - Node.js 20.x LTS
- `|W.Runtime.JavaScript.Node.18` - Node.js 18.x LTS
- `|W.Runtime.JavaScript.Node.21` - Node.js 21.x Current

**Inputs:**
- `<package_json:pg.path` (optional) - Path to package.json
- `<node_modules:pg.path` (optional) - Path to node_modules

**Outputs:**
- `>session:serial` - Runtime session handle

**Example:**
```polyglot
[w] |W.Runtime.JavaScript.Node.20
(|) >session:serial >> $js_session
```

---

## Runtime Execution Pipelines

### Code Execution

Execute code strings in the runtime session.

#### |{Language}.Run.Code

**Python Example:**
```polyglot
[r] |RT.Python.Run.Code
(|) <session:serial << $py_session
(|) <code:string << |U.String.Python""
[+] +"import sys"
[+] -"print(f'Python version: {sys.version}')"
(|) >result:serial >> $py_result
```

**Rust Example:**
```polyglot
[r] |RT.Rust.Run.Code
(|) <session:serial << $rust_session
(|) <code:string << |U.String.Rust""
[+] +"fn main() {"
[+] -"    println!(\"Hello from Rust!\");"
[+] -"}"
(|) >result:serial >> $rust_result
```

**JavaScript Example:**
```polyglot
[r] |RT.JavaScript.Run.Code
(|) <session:serial << $js_session
(|) <code:string << |U.String.JS""
[+] +"console.log('Hello from Node.js!');"
(|) >result:serial >> $js_result
```

---

### File Execution

Execute code from files in the runtime session.

#### |{Language}.Run.File

**Python Example:**
```polyglot
[r] |Python.Run.File
(|) <session:serial << $py_session
(|) <file:pg.path << |Path"/scripts/process_data.py"
(|) >result:serial >> $py_result
```

**Rust Example:**
```polyglot
[r] |Rust.Run.File
(|) <session:serial << $rust_session
(|) <file:pg.path << |Path"/src/main.rs"
(|) >result:serial >> $rust_result
```

**JavaScript Example:**
```polyglot
[r] |JavaScript.Run.File
(|) <session:serial << $js_session
(|) <file:pg.path << |Path"/app/index.js"
(|) >result:serial >> $js_result
```

---

## Complete Multi-Language Example

```polyglot
{|} |MultiLanguageProcessor
[<] <input_file:pg.path
[<] <output_file:pg.path
[>] >results:serial <~ #Empty

[Q] |Q.Sequential


# Initialize Python runtime
[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << |Path"/config/requirements.txt"
(|) >session:serial >> $py_session

# Initialize Rust runtime
[w] |W.Runtime.Rust1.75
(|) >session:serial >> $rust_session

# Initialize JavaScript runtime
[w] |W.Runtime.JavaScript.Node.20
(|) >session:serial >> $js_session


# Process with Python
[r] |RT.Python.Run.Code
(|) <session:serial << $py_session
(|) <code:string << |U.String.Python""
[+] +"import json"
[+] -"with open('{{$input_file}}', 'r') as f:"
[+] -"    data = json.load(f)"
[+] -"print(f'Processed {len(data)} records')"
(|) >result:serial >> $py_result


# Process with Rust
[r] |RT.Rust.Run.Code
(|) <session:serial << $rust_session
(|) <code:string << |U.String.Rust""
[+] +"use std::fs;"
[+] -"fn main() {"
[+] -"    let contents = fs::read_to_string(\"{{$input_file}}\");"
[+] -"    println!(\"File size: {} bytes\", contents.unwrap().len());"
[+] -"}"
(|) >result:serial >> $rust_result


# Process with JavaScript
[r] |RT.JavaScript.Run.Code
(|) <session:serial << $js_session
(|) <code:string << |U.String.JS""
[+] +"const fs = require('fs');"
[+] -"const data = JSON.parse(fs.readFileSync('{{$input_file}}'));"
[+] -"fs.writeFileSync('{{$output_file}}', JSON.stringify(data, null, 2));"
(|) >result:serial >> $js_result


# Combine results
[r] |Serial.Create
(|) <python:serial << $py_result
(|) <rust:serial << $rust_result
(|) <javascript:serial << $js_result
(|) >combined:serial >> >results

{x}
```

---

## Version-in-Name Convention

Runtime wrappers use **version-in-name** convention instead of version parameters.

### ✅ Correct
```polyglot
[w] |W.Runtime.Python3.11
[w] |W.Runtime.Rust1.75
[w] |W.Runtime.JavaScript.Node.20
```

### ❌ Incorrect
```polyglot
[w] |W.Runtime.Python
(|) <version:string << "3.11"  # Wrong - version is part of pipeline name
```

**Rationale:**
- Different versions are different pipelines with potentially different behaviors
- Allows multiple runtime versions in same pipeline
- Explicit version selection at pipeline design time
- Better tooling support and autocomplete

---

## Configuration File Support

### Python Requirements

```polyglot
[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\FileDir\\python\requirements.txt
(|) >session:serial >> $py_session
```

**requirements.txt:**
```
requests>=2.28.0
pandas>=1.5.0
numpy>=1.23.0
```

### Rust Dependencies

```polyglot
[w] |W.Runtime.Rust1.75
(|) <manifest:pg.path << \\FileDir\\rust\Cargo.toml
(|) >session:serial >> $rust_session
```

**Cargo.toml:**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

### JavaScript Packages

```polyglot
[w] |W.Runtime.JavaScript.Node.20
(|) <package_json:pg.path << \\FileDir\\node\package.json
(|) >session:serial >> $js_session
```

**package.json:**
```json
{
  "dependencies": {
    "express": "^4.18.0",
    "axios": "^1.3.0"
  }
}
```

---

## Session Management

### Session Lifecycle

1. **Initialize** - Wrapper creates session
2. **Configure** - Load dependencies from config files
3. **Execute** - Run code/files in session
4. **Cleanup** - Session automatically cleaned up at pipeline end

### Session Isolation

Each session is isolated:
- Separate process/environment
- Independent package installations
- No state sharing between sessions
- Concurrent sessions possible (with `[Q] |Q.Parallel`)

### Session Reuse

Sessions can be reused across multiple executions:

```polyglot
[w] |W.Runtime.Python3.11
(|) >session:serial >> $py_session

# First execution
[r] |RT.Python.Run.Code
(|) <session:serial << $py_session
(|) <code:string << "x = 10"
(|) >result >> $result1

# Second execution (reuses session, x is still defined)
[r] |RT.Python.Run.Code
(|) <session:serial << $py_session
(|) <code:string << "print(x + 5)"  # x from previous execution
(|) >result >> $result2
```

---

## Result Handling

### Result Structure

Runtime execution results are returned as `:serial` type containing:

```polyglot
{
  "stdout": "...",      # Standard output
  "stderr": "...",      # Standard error
  "exit_code": 0,       # Exit code (0 = success)
  "duration_ms": 142,   # Execution time
  "output_file": "..."  # Path to output file (if specified)
}
```

### Accessing Results

```polyglot
[r] |RT.Python.Run.Code
(|) <session:serial << $py_session
(|) <code:string << "print('Hello')"
(|) >result:serial >> $py_result

# Access stdout
[r] $output << $py_result.stdout

# Check success
[f] $py_result.exit_code =? 0
   [r] |Log.Info"Python execution succeeded"
```

---

## Best Practices

### 1. Use Specific Versions

Always specify the exact runtime version you need:

```polyglot
✅ Good: |W.Runtime.Python3.11
❌ Avoid: |W.Runtime.Python  # Version required
```

### 2. Provide Configuration Files

Include dependencies in config files rather than inline installation:

```polyglot
✅ Good:
[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\FileDir\\requirements.txt

❌ Avoid: Installing packages in code strings
```

### 3. Handle Errors

Always check exit codes and handle errors:

```polyglot
[r] |RT.Python.Run.Code
(|) <session:serial << $py_session
(|) <code:string << $code
(|) >result >> $result

[f] $result.exit_code =? 0
   # Success path
[f] $result.exit_code !? 0
   [r] |Log.Error"Python execution failed: {{$result.stderr}}"
```

### 4. Use Multi-Line Strings

Use language-specific string utilities for readability:

```polyglot
✅ Good:
(|) <code:string << |U.String.Python""
[+] +"def process():"
[+] -"    return True"

❌ Avoid: Single-line string with \n
(|) <code:string << "def process():\n    return True"
```

---

## See Also

- [Multi-Line Strings](../../../User/language/advanced/multi-line-strings.md)
- [String Utilities](../../../User/stdlib/utilities/string-utilities.md)
- [Serial Data Type](../../../User/language/types/serial-type.md)
- [Wrapper System](../../../User/stdlib/wrappers/README.md)

---

**Added:** 2025-12-21 (Training Session 2)
**Verified:** Human review in training session
**Source:** L-2025-12-21-006
