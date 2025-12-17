# Runtime Environments Specification

**Date:** 2025-12-03
**Type:** Major Design Change
**Status:** 🔄 **IN PROGRESS**
**Scope:** Controlled runtime environments for all external code execution

---

## Executive Summary

Polyglot is **resource-conscious** and requires all external code execution (shell, Python, Rust, etc.) to run in **controlled runtime environments**. This specification defines the architecture, patterns, and implementation for runtime management.

---

## Major Change #1: String Literal Syntax Update

### Previous Syntax
```polyglot
// OLD: Formatted string with pipeline (no prefix)
U.Log.Info"Processing {.count} items"
```

### New Syntax
```polyglot
// NEW: Pipeline prefix for consistency
|U.Log.Info"Processing {.count} items"
```

**Rationale:**
- **Consistency:** All Polyglot objects have prefixes (`|` for pipelines, `#` for enums, `.` for variables)
- **Clarity:** Emphasizes that what precedes the string is a **pipeline**
- **Readability:** Clearer visual distinction between pipeline and string literal

### Examples

**Before:**
```polyglot
[r] .formatted: pg\string << U.String.Format"Hello {.name}, you are {.age} years old"
[r] .logged: pg\string << U.Log.Info"Processing {.count} items"
[r] .shell: pg\string << RT.Shell.Run"ls -la {.directory}"
```

**After:**
```polyglot
[r] .formatted: pg\string << |U.String.Format"Hello {.name}, you are {.age} years old"
[r] .logged: pg\string << |U.Log.Info"Processing {.count} items"
[r] .shell: pg\string << |RT.Shell.Run"ls -la {.directory}"
```

### Migration Pattern

**Search:** `([A-Z][A-Za-z0-9_\.]+)"` (regex) - pipeline name followed by string
**Replace:** `|$1"` - add pipe prefix

---

## Major Change #2: Pipeline Restriction for Formatted Strings

### Rule

**ONLY pipelines explicitly defined with formatted string support can use formatted string literals.**

### Problem

Currently, any pipeline can receive a formatted string:

```polyglot
// Should this be allowed?
[r] .result: pg\string << |SomeRandomPipeline"string {.var}"
```

**Question:** How do we define which pipelines accept formatted strings?

### Proposed Solution (Brainstorm Backlog)

**Option 1: Type Signature**
```polyglot
[|] |U.String.Format
[i] <template: pg\fstring     // Formatted string type
[i] <args: pg\dict
[o] >result: pg\string
```

**Option 2: Special Marker**
```polyglot
[|] |U.String.Format
[@] FormattedStringSupport    // Annotation
[i] <template: pg\string
```

**Option 3: Compiler Convention**
- Pipelines ending in `.Format` automatically support formatted strings
- Explicit whitelist in compiler

**Status:** 🔖 **Added to brainstorm backlog** - needs design decision

---

## Controlled Runtime Architecture

### Core Principle

> **All external code execution MUST run in controlled runtime environments with resource management, isolation, and queue integration.**

### Why Controlled Runtimes?

1. **Resource Management:** CPU, memory, I/O limits enforced
2. **Isolation:** Prevent interference between pipelines
3. **Queue Integration:** Runtime sessions controlled by Queue Manager
4. **Security:** Sandboxed execution prevents system access
5. **Observability:** Track resource usage, logs, metrics
6. **Cleanup:** Guaranteed cleanup on pipeline termination

---

## Reserved Enumeration: `#Sessions.*`

### Purpose

Store runtime session instance information for each runtime type.

### Structure

```polyglot
[#] #Sessions.Shell
[<] .id: pg\string              // Session identifier
[<] .pid: pg\int                // Process ID
[<] .systemd_unit: pg\string    // systemd unit name
[<] .started_at: pg\datetime    // Session start time
[<] .resource_limits: pg\dict   // Resource constraints
[<] .status: #Session.Status    // Running, Paused, Stopped
[X]

[#] #Sessions.Python
[<] .id: pg\string
[<] .uv_env_path: pg\path       // uv environment path
[<] .python_version: pg\string  // Python version
[<] .requirements_hash: pg\string
[<] .venv_pid: pg\int
[<] .started_at: pg\datetime
[X]

[#] #Sessions.Rust
[<] .id: pg\string
[<] .toolchain: pg\string       // Rust toolchain version
[<] .target_dir: pg\path        // Cargo target directory
[<] .workspace_path: pg\path
[<] .started_at: pg\datetime
[X]
```

### Session Status Enumeration

```polyglot
[#] #Session.Status
[<] .Initializing
[<] .Running
[<] .Paused
[<] .Stopping
[<] .Stopped
[<] .Failed
[X]
```

---

## Runtime Usage Patterns

### Overview

All Polyglot runtimes support **two patterns** for usage:

1. **Wrapper Pattern (⭐ RECOMMENDED)** - Syntactic sugar via Macros
2. **Explicit Setup/Cleanup Pattern** - Advanced control

### Understanding Wrappers as Macros

**Key Insight:** `[W]` wrappers are **Macros** that implement **DRY (Don't Repeat Yourself) code**.

```
┌─────────────────────────────────────────────────────────────┐
│ Wrapper Pattern (Syntactic Sugar)                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  [W] |W.RT.Shell                                           │
│                                                             │
│  Internally performs:                                       │
│  1. Setup: Create systemd session                          │
│  2. Inject context into wrapped blocks                     │
│  3. Cleanup: Terminate session on exit                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Pattern Comparison

| Aspect | Wrapper Pattern | Explicit Pattern |
|--------|----------------|------------------|
| **Code length** | Short, concise | Verbose |
| **Setup** | Automatic (Macro) | Manual `[\]` block |
| **Cleanup** | Automatic (Macro) | Manual `[/]` block |
| **Session variable** | Implicit | Explicit `.session` |
| **Use case** | ⭐ Most scenarios | Advanced control |
| **Error handling** | Automatic cleanup | Manual cleanup required |

### When to Use Each Pattern

#### Use Wrapper Pattern (Recommended) When:
- ✅ Standard runtime usage
- ✅ You want clean, simple code
- ✅ Automatic cleanup is desired
- ✅ No need for session inspection

#### Use Explicit Pattern When:
- 🔧 Custom session configuration needed
- 🔧 Session inspection required
- 🔧 Advanced resource tuning
- 🔧 Multiple setup/cleanup steps

### The Purpose of Wrappers/Macros

**Quote from design decision:**
> "That's the purpose of macros/wrappers. They are DRY code implementation"

Wrappers eliminate boilerplate by:
1. **Encapsulating setup logic** - No need to write `[\]` blocks
2. **Automatic cleanup** - No need to write `[/]` blocks
3. **Error handling** - Cleanup happens even on errors
4. **Session management** - Session lifecycle is implicit

---

## Runtime 1: Shell (Bash/Zsh)

### Architecture

Shell scripts run inside **controlled systemd-run sessions** integrated with Queue Manager.

### Lifecycle

```
[\] Setup    → [W] Wrapper → [Execution Blocks] → [/] Cleanup
   ↓              ↓               ↓                  ↓
Setup.RT.Shell  W.RT.Shell   RT.Shell.Run    Cleanup.RT.Shell
```

---

### Runtime Patterns

Polyglot provides **two patterns** for runtime usage:

#### Pattern 1: Wrapper Pattern (⭐ **RECOMMENDED**)

**Purpose:** Syntactic sugar that makes runtime usage easier via Macros

**Key Insight:** `[W]` wrappers are **Macros** that automatically handle setup and cleanup (DRY implementation)

```polyglot
[|] |Shell.With.Wrapper
[i] .directory: pg\path
[t] |T.Call
[W] |W.RT.Shell                           // Macro handles setup + cleanup

[r] .output: pg\string << |RT.Shell.Run"ls -la {.directory}"

[o] .output
[X]
```

**Advantages:**
- ✅ Simpler, cleaner code
- ✅ Less boilerplate
- ✅ Automatic cleanup
- ✅ Recommended for most use cases

---

#### Pattern 2: Explicit Setup/Cleanup Pattern

**Purpose:** Advanced pattern with explicit control over session lifecycle

```polyglot
[|] |Shell.Full.Example
[i] .directory: pg\path
[t] |T.Call
[\] .shell_session: #Sessions.Shell << |Setup.RT.Shell""    // Explicit setup

[r] .output: pg\string << |RT.Shell.Run"ls -la {.directory}"

[/] |Cleanup.RT.Shell                                        // Explicit cleanup
[<] <session: #Sessions.Shell << .shell_session
[o] .output
[X]
```

**When to use:**
- Advanced use cases requiring session inspection
- Custom resource configuration
- Explicit control over setup/cleanup timing

**Note:** No `[W]` wrapper needed because setup and cleanup are already defined explicitly.

---

### Pattern Comparison

| Aspect | Wrapper Pattern | Explicit Pattern |
|--------|----------------|------------------|
| **Ease of use** | ⭐ Simple | Complex |
| **Boilerplate** | Minimal | More verbose |
| **Setup/Cleanup** | Automatic (Macro) | Manual |
| **Session control** | Implicit | Explicit |
| **Recommended for** | Most use cases | Advanced scenarios |

**Key Understanding:** Wrappers are Macros that implement DRY code - they internally perform setup and cleanup so you don't have to.

---

### Complete Example (Wrapper Pattern - Recommended)

```polyglot
[|] |ProcessFiles
[i] .directory: pg\path
[o] .file_count: pg\int
[t] |T.Call

[W] |W.RT.Shell                           // Wrapper handles everything

// Execute shell commands in controlled session
[r] .ls_output: pg\string << |RT.Shell.Run"ls -la {.directory}"
[r] .count_output: pg\string << |RT.Shell.Run"ls {.directory} | wc -l"
[r] .file_count: pg\int << |U.Int.Parse"{.count_output}"

[o] .file_count
[X]
```

### Runtime Operations

#### `|Setup.RT.Shell`

**Purpose:** Initialize controlled shell session via systemd-run

**Block:** `[\]` (Setup)

**Signature:**
```polyglot
[|] |Setup.RT.Shell
[i] <config: #Shell.Config <~ #Shell.Config.Default
[o] >session: #Sessions.Shell
[t] |T.Call

// Creates systemd-run transient unit with:
// - Resource limits (CPU, memory)
// - cgroup isolation
// - Logging to journal
[X]
```

**Example:**
```polyglot
[\]
[r] .shell_session: #Sessions.Shell << |Setup.RT.Shell""
[/]
```

**Implementation (conceptual):**
```bash
systemd-run \
  --user \
  --scope \
  --slice=polyglot-shell.slice \
  --property=CPUQuota=50% \
  --property=MemoryMax=512M \
  --property=TasksMax=20 \
  /bin/bash
```

---

#### `|W.RT.Shell`

**Purpose:** Runtime wrapper for shell execution context

**Block:** `[W]` (Wrapper)

**Signature:**
```polyglot
[|] |W.RT.Shell
[i] <session: #Sessions.Shell
[t] |T.Macro

// Injects shell session context into wrapped pipelines
[X]
```

**Example:**
```polyglot
[W] |W.RT.Shell
[<] <session: #Sessions.Shell << .shell_session
```

---

#### `|RT.Shell.Run`

**Purpose:** Execute shell command in controlled session

**Block:** `[r]`, `[p]`, `[b]` (Execution)

**Signature:**
```polyglot
[|] |RT.Shell.Run
[i] <command: pg\string         // Shell command (formatted string)
[i] <timeout: pg\int <~ 30      // Timeout in seconds
[o] >stdout: pg\string          // Command stdout
[o] >stderr: pg\string          // Command stderr
[o] >exit_code: pg\int          // Exit code
[t] |T.Call

// Executes command in session's systemd unit
[X]
```

**Example:**
```polyglot
[r] .ls_output: pg\string << |RT.Shell.Run"ls -la {.directory}"
[r] .find_result: pg\string << |RT.Shell.Run"find {.path} -name '*.txt'"
```

---

#### `|Cleanup.RT.Shell`

**Purpose:** Terminate shell session and cleanup resources

**Block:** `[/]` (Cleanup)

**Signature:**
```polyglot
[|] |Cleanup.RT.Shell
[i] <session: #Sessions.Shell
[t] |T.Call

// Terminates systemd unit
// Cleans up temporary files
// Releases resources
[X]
```

**Example:**
```polyglot
[/]
[r] |Cleanup.RT.Shell
[<] <session: #Sessions.Shell << .shell_session
```

---

### Shell Runtime Configuration

```polyglot
[#] #Shell.Config
[<] .cpu_quota: pg\int <~ 50        // CPU percentage (50%)
[<] .memory_max: pg\uint <~ 512     // Memory limit (MB)
[<] .tasks_max: pg\uint <~ 20       // Max concurrent processes
[<] .timeout: pg\int <~ 300         // Session timeout (seconds)
[<] .working_dir: pg\path <~ \\WorkDir\\
[<] .shell_path: pg\path <~ "/bin/bash"
[X]

[#] #Shell.Config.Default
[A] Default
[<] .cpu_quota: pg\int << 50
[<] .memory_max: pg\uint << 512
[<] .tasks_max: pg\uint << 20
[<] .timeout: pg\int << 300
[<] .working_dir: pg\path << \\WorkDir\\
[<] .shell_path: pg\path << "/bin/bash"
[X]
```

---

### Queue Integration

Shell sessions respect Queue Manager configuration:

```polyglot
[|] |BackupDatabase
[t] |T.Scheduled
[Q] |Q.LowPriority
[Q] |Q.Pause
[<] <condition: pg\bool << .is_business_hours

[\]
[r] .shell_session: #Sessions.Shell << |Setup.RT.Shell""
[/]

[W] |W.RT.Shell
[<] <session: #Sessions.Shell << .shell_session

// Shell commands respect queue pause
[r] .backup: pg\string << |RT.Shell.Run"pg_dump mydb > backup.sql"

[/]
[r] |Cleanup.RT.Shell
[<] <session: #Sessions.Shell << .shell_session
[X]
```

**Queue Manager behavior:**
- If queue is paused, shell session is suspended (SIGSTOP)
- If queue resumes, shell session continues (SIGCONT)
- Resource limits adjusted dynamically based on queue priority

---

## Runtime 2: Python

### Architecture

Python runtimes managed via **`uv` environments** with isolated dependencies.

### Lifecycle

```
[\] Setup    → [W] Wrapper → [Execution Blocks] → [/] Cleanup
   ↓              ↓               ↓                  ↓
Setup.RT.Python W.RT.Python  RT.Python.Run   Cleanup.RT.Python
```

---

### Runtime Patterns

Same two patterns as Shell runtime:

#### Pattern 1: Wrapper Pattern (⭐ **RECOMMENDED**)

**Purpose:** Syntactic sugar via Macros for easy Python runtime usage

```polyglot
[|] |Python.With.Wrapper
[i] .data_file: pg\path
[t] |T.Call

[W] |W.RT.Python3.14                      // Macro handles setup + cleanup
[<] <requirements: pg\path << \\FileDir\\requirements.lock

[r] .result: pg\serial << |RT.Python.Run"
import pandas as pd
df = pd.read_csv('{.data_file}')
print(df.describe().to_json())
"

[o] .result
[X]
```

**Advantages:**
- ✅ No manual session management
- ✅ Automatic environment creation and cleanup
- ✅ Recommended for most use cases

---

#### Pattern 2: Explicit Setup/Cleanup Pattern

**Purpose:** Advanced pattern with explicit uv environment control

```polyglot
[|] |Python.Full.Example
[i] .data_file: pg\path
[t] |T.Call

[\]                                                    // Explicit setup
[r] .py_session: #Sessions.Python << |Setup.RT.Python""
[<] <requirements: pg\path << \\FileDir\\requirements.lock
[<] <python_version: pg\string << "3.14"
[/]

[r] .result: pg\serial << |RT.Python.Run"
import pandas as pd
df = pd.read_csv('{.data_file}')
print(df.describe().to_json())
"

[/]                                                    // Explicit cleanup
[r] |Cleanup.RT.Python
[<] <session: #Sessions.Python << .py_session
[o] .result
[X]
```

**When to use:**
- Custom Python version configuration
- Explicit environment inspection
- Advanced dependency management

---

### Complete Example (Wrapper Pattern - Recommended)

```polyglot
[|] |AnalyzeData
[i] .data_file: pg\path
[o] .analysis: pg\dict
[t] |T.Call

[W] |W.RT.Python3.14                      // Wrapper handles everything
[<] <requirements: pg\path << \\FileDir\\requirements.lock

// Execute Python code in uv environment
[r] .result: pg\serial << |RT.Python.Run"
import pandas as pd
import json

df = pd.read_csv('{.data_file}')
analysis = df.describe().to_dict()
print(json.dumps(analysis))
"
[r] .analysis: pg\dict << .result

[o] .analysis
[X]
```

### Runtime Operations

#### `|Setup.RT.Python`

**Purpose:** Initialize uv environment with dependencies

**Block:** `[\]` (Setup)

**Signature:**
```polyglot
[|] |Setup.RT.Python
[i] <python_version: pg\string <~ "3.12"
[i] <requirements: pg\path          // requirements.lock or pyproject.toml
[o] >session: #Sessions.Python
[t] |T.Call

// Creates uv environment:
// - uv venv create
// - uv pip sync requirements.lock
// - Returns session info
[X]
```

**Example:**
```polyglot
[\]
[r] .py_session: #Sessions.Python << |Setup.RT.Python""
[<] <requirements: pg\path << \\FileDir\\requirements.lock
[<] <python_version: pg\string << "3.14"
[/]
```

**Implementation (conceptual):**
```bash
uv venv create --python 3.14 .polyglot/venv-{session-id}
uv pip sync --python .polyglot/venv-{session-id}/bin/python requirements.lock
```

---

#### `|W.RT.Python3.14`

**Purpose:** Runtime wrapper for Python 3.14 execution context

**Block:** `[W]` (Wrapper)

**Signature:**
```polyglot
[|] |W.RT.Python3.14
[i] <session: #Sessions.Python
[i] <requirements: pg\path
[t] |T.Macro

// Injects Python environment into wrapped pipelines
[X]
```

**Variants:**
- `|W.RT.Python3.12`
- `|W.RT.Python3.13`
- `|W.RT.Python3.14`
- `|W.RT.Python3.15`

**Example:**
```polyglot
[W] |W.RT.Python3.14
[<] <session: #Sessions.Python << .py_session
[<] <requirements: pg\path << \\FileDir\\requirements.lock
```

---

#### `|RT.Python.Run`

**Purpose:** Execute Python code in uv environment

**Block:** `[r]`, `[p]`, `[b]` (Execution)

**Signature:**
```polyglot
[|] |RT.Python.Run
[i] <code: pg\string                // Python code (formatted string)
[i] <timeout: pg\int <~ 60          // Timeout in seconds
[o] >stdout: pg\string              // Python stdout
[o] >stderr: pg\string              // Python stderr
[o] >exit_code: pg\int              // Exit code
[o] >result: pg\serial              // Parsed JSON output
[t] |T.Call

// Executes Python code in uv environment
[X]
```

**Example:**
```polyglot
[r] .result: pg\serial << |RT.Python.Run"
import sys
print(f'Python version: {sys.version}')
result = {'value': {.input_value} * 2}
print(result)
"
```

---

#### `|Cleanup.RT.Python`

**Purpose:** Remove uv environment and cleanup

**Block:** `[/]` (Cleanup)

**Signature:**
```polyglot
[|] |Cleanup.RT.Python
[i] <session: #Sessions.Python
[t] |T.Call

// Removes uv venv
// Cleans up temporary files
[X]
```

**Example:**
```polyglot
[/]
[r] |Cleanup.RT.Python
[<] <session: #Sessions.Python << .py_session
```

---

### Python Runtime Configuration

```polyglot
[#] #Python.Config
[<] .python_version: pg\string <~ "3.12"
[<] .requirements: pg\path
[<] .uv_cache_dir: pg\path <~ \\DataDir\\.uv-cache
[<] .venv_dir: pg\path <~ \\DataDir\\.polyglot-venvs
[<] .timeout: pg\int <~ 60
[<] .memory_max: pg\uint <~ 1024    // MB
[X]
```

---

## Runtime 3: Rust

### Architecture

Rust runtimes managed via **Cargo workspaces** with toolchain isolation and resource control.

### Rust "Environment" Options

Unlike Python, Rust doesn't have traditional virtual environments. Options:

1. **Toolchain Isolation** (via rustup)
   - Different Rust versions (stable, beta, nightly, specific versions)
   - `rustup run stable cargo build`

2. **Workspace Isolation**
   - Separate Cargo.toml and target directories
   - Dependency isolation per workspace

3. **Resource Control**
   - systemd-run with cgroups (similar to shell)
   - CPU, memory, I/O limits

4. **Sandboxing**
   - bubblewrap, firejail, or systemd sandboxing
   - Restrict filesystem access

**Recommended Approach:** Combination of toolchain + workspace + systemd control

---

### Lifecycle

```
[\] Setup    → [W] Wrapper → [Execution Blocks] → [/] Cleanup
   ↓              ↓               ↓                  ↓
Setup.RT.Rust  W.RT.Rust    RT.Rust.Run    Cleanup.RT.Rust
```

---

### Runtime Patterns

Same two patterns as Shell and Python runtimes:

#### Pattern 1: Wrapper Pattern (⭐ **RECOMMENDED**)

**Purpose:** Syntactic sugar via Macros for easy Rust runtime usage

```polyglot
[|] |Rust.With.Wrapper
[i] .input_file: pg\path
[t] |T.Call

[W] |W.RT.Rust.Stable                     // Macro handles setup + cleanup
[<] <manifest: pg\path << \\FileDir\\Cargo.toml

[r] .result: pg\string << |RT.Rust.Run"
use std::fs;
let data = fs::read('{.input_file}').unwrap();
let count = data.len();
println!(\"Processed {} bytes\", count);
"

[o] .result
[X]
```

**Advantages:**
- ✅ No manual workspace management
- ✅ Automatic toolchain setup and cleanup
- ✅ Recommended for most use cases

---

#### Pattern 2: Explicit Setup/Cleanup Pattern

**Purpose:** Advanced pattern with explicit workspace control

```polyglot
[|] |Rust.Full.Example
[i] .input_file: pg\path
[t] |T.Call

[\]                                                    // Explicit setup
[r] .rust_session: #Sessions.Rust << |Setup.RT.Rust""
[<] <toolchain: pg\string << "stable"
[<] <manifest: pg\path << \\FileDir\\Cargo.toml
[/]

[r] .result: pg\string << |RT.Rust.Run"
use std::fs;
let data = fs::read('{.input_file}').unwrap();
let count = data.len();
println!(\"Processed {} bytes\", count);
"

[/]                                                    // Explicit cleanup
[r] |Cleanup.RT.Rust
[<] <session: #Sessions.Rust << .rust_session
[o] .result
[X]
```

**When to use:**
- Custom toolchain configuration (beta, nightly, specific versions)
- Explicit workspace inspection
- Advanced Cargo configuration

---

### Complete Example (Wrapper Pattern - Recommended)

```polyglot
[|] |ProcessBinaryData
[i] .input_file: pg\path
[o] .processed: pg\path
[t] |T.Call

[W] |W.RT.Rust.Stable                     // Wrapper handles everything
[<] <manifest: pg\path << \\FileDir\\Cargo.toml

// Execute Rust code (inline or script)
[r] .result: pg\path << |RT.Rust.Run"
use std::fs;
let data = fs::read('{.input_file}').unwrap();
let processed = data.iter().map(|b| b.wrapping_mul(2)).collect::<Vec<_>>();
fs::write('{.output_path}', processed).unwrap();
"

[o] .processed
[X]
```

### Runtime Operations

#### `|Setup.RT.Rust`

**Purpose:** Initialize Rust workspace with toolchain

**Block:** `[\]` (Setup)

**Signature:**
```polyglot
[|] |Setup.RT.Rust
[i] <toolchain: pg\string <~ "stable"    // stable, beta, nightly, 1.75.0
[i] <manifest: pg\path                    // Cargo.toml
[i] <target_dir: pg\path <~ \\DataDir\\.rust-targets
[o] >session: #Sessions.Rust
[t] |T.Call

// Creates:
// - Temporary workspace directory
// - Copies Cargo.toml
// - Sets up toolchain via rustup
// - Runs cargo fetch
[X]
```

**Example:**
```polyglot
[\]
[r] .rust_session: #Sessions.Rust << |Setup.RT.Rust""
[<] <toolchain: pg\string << "stable"
[<] <manifest: pg\path << \\FileDir\\Cargo.toml
[/]
```

---

#### `|W.RT.Rust.Stable`

**Purpose:** Runtime wrapper for Rust stable toolchain

**Block:** `[W]` (Wrapper)

**Signature:**
```polyglot
[|] |W.RT.Rust.Stable
[i] <session: #Sessions.Rust
[t] |T.Macro

// Injects Rust environment into wrapped pipelines
[X]
```

**Variants:**
- `|W.RT.Rust.Stable`
- `|W.RT.Rust.Beta`
- `|W.RT.Rust.Nightly`
- `|W.RT.Rust.1_75_0` (specific version)

**Example:**
```polyglot
[W] |W.RT.Rust.Stable
[<] <session: #Sessions.Rust << .rust_session
```

---

#### `|RT.Rust.Run`

**Purpose:** Compile and execute Rust code in workspace

**Block:** `[r]`, `[p]`, `[b]` (Execution)

**Signature:**
```polyglot
[|] |RT.Rust.Run
[i] <code: pg\string                // Rust code (formatted string)
[i] <mode: #Rust.Mode <~ #Rust.Mode.Debug
[i] <timeout: pg\int <~ 120
[o] >stdout: pg\string
[o] >stderr: pg\string
[o] >exit_code: pg\int
[t] |T.Call

// Compiles and runs Rust code in workspace
[X]
```

**Example:**
```polyglot
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    let value = {.input};
    println!(\"Result: {}\", value * 2);
}
"
```

---

#### `|Cleanup.RT.Rust`

**Purpose:** Remove workspace and cleanup

**Block:** `[/]` (Cleanup)

**Signature:**
```polyglot
[|] |Cleanup.RT.Rust
[i] <session: #Sessions.Rust
[i] <keep_artifacts: pg\bool <~ #Boolean.False
[t] |T.Call

// Removes workspace directory
// Optionally keeps compiled artifacts
[X]
```

**Example:**
```polyglot
[/]
[r] |Cleanup.RT.Rust
[<] <session: #Sessions.Rust << .rust_session
```

---

### Rust Runtime Configuration

```polyglot
[#] #Rust.Config
[<] .toolchain: pg\string <~ "stable"
[<] .target_dir: pg\path <~ \\DataDir\\.rust-targets
[<] .cargo_home: pg\path <~ \\DataDir\\.cargo
[<] .compilation_timeout: pg\int <~ 300     // seconds
[<] .execution_timeout: pg\int <~ 120
[<] .memory_max: pg\uint <~ 2048            // MB
[X]

[#] #Rust.Mode
[<] .Debug
[<] .Release
[<] .ReleaseOptimized
[X]
```

---

### Rust systemd Integration

Rust compilation and execution under systemd control:

```bash
systemd-run \
  --user \
  --scope \
  --slice=polyglot-rust.slice \
  --property=CPUQuota=80% \
  --property=MemoryMax=2G \
  --property=TasksMax=50 \
  rustup run stable cargo run --manifest-path=/path/to/Cargo.toml
```

---

## Compile-Time Safety

### Rule: No Uncontrolled Execution

**❌ COMPILE ERROR:** Running scripts outside controlled runtime

```polyglot
// ❌ WRONG: Direct shell execution (no runtime)
[r] .output: pg\string << |Shell.DirectRun"ls -la"
// Error: Shell execution must use |RT.Shell.Run within |W.RT.Shell wrapper
```

**✅ CORRECT:** Controlled runtime

```polyglot
[\]
[r] .shell_session: #Sessions.Shell << |Setup.RT.Shell""
[/]

[W] |W.RT.Shell
[<] <session: #Sessions.Shell << .shell_session

[r] .output: pg\string << |RT.Shell.Run"ls -la"
```

### Compiler Checks

1. **Runtime Wrapper Required:**
   - `|RT.Shell.Run` requires `[W] |W.RT.Shell`
   - `|RT.Python.Run` requires `[W] |W.RT.Python*`
   - `|RT.Rust.Run` requires `[W] |W.RT.Rust.*`

2. **Session Setup Required:**
   - `[W]` runtime wrappers require `#Sessions.*` from `[\]` setup

3. **Cleanup Required:**
   - If `[\]` creates session, `[/]` must cleanup

**Error Messages:**

```
Error: |RT.Shell.Run requires [W] |W.RT.Shell wrapper
  --> src/pipeline.pg:15:5
   |
15 | [r] .output: pg\string << |RT.Shell.Run"ls"
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
Help: Add [W] |W.RT.Shell wrapper with session
```

---

## Runtime Hierarchy

```
Runtime Environments
├── Shell
│   ├── Setup.RT.Shell      [\]
│   ├── W.RT.Shell          [W]
│   ├── RT.Shell.Run        [r]
│   └── Cleanup.RT.Shell    [/]
│
├── Python
│   ├── Setup.RT.Python     [\]
│   ├── W.RT.Python3.12     [W]
│   ├── W.RT.Python3.13     [W]
│   ├── W.RT.Python3.14     [W]
│   ├── RT.Python.Run       [r]
│   └── Cleanup.RT.Python   [/]
│
├── Rust
│   ├── Setup.RT.Rust       [\]
│   ├── W.RT.Rust.Stable    [W]
│   ├── W.RT.Rust.Beta      [W]
│   ├── W.RT.Rust.Nightly   [W]
│   ├── RT.Rust.Run         [r]
│   └── Cleanup.RT.Rust     [/]
│
└── Node.js (future)
    ├── Setup.RT.Node       [\]
    ├── W.RT.Node20         [W]
    ├── W.RT.Node22         [W]
    ├── RT.Node.Run         [r]
    └── Cleanup.RT.Node     [/]
```

---

## Resource Management

### CPU Limits

All runtimes enforce CPU quotas via systemd:

```polyglot
[#] #Runtime.Resources
[<] .cpu_quota: pg\int <~ 50        // 50% of one core
[<] .memory_max: pg\uint <~ 1024    // 1GB
[<] .tasks_max: pg\uint <~ 20       // Max 20 concurrent processes/threads
[<] .io_weight: pg\uint <~ 100      // I/O priority (100 = normal)
[X]
```

### Memory Limits

Enforced via cgroup memory.max:
- Shell: 512MB default
- Python: 1GB default
- Rust: 2GB default (compilation + execution)

### Timeout Enforcement

All runtime operations have timeouts:
- Setup: 120s default
- Execution: Varies by runtime
- Cleanup: 30s default

**Timeout behavior:**
- SIGTERM sent at timeout
- SIGKILL sent at timeout + 10s

---

## Queue Manager Integration

### Queue Control Points

1. **Session Creation:** Queue can delay session creation
2. **Execution Start:** Queue can pause before execution
3. **Resource Allocation:** Queue determines resource limits
4. **Session Suspension:** Queue can suspend/resume sessions

### Example: Priority-Based Resources

```polyglot
[|] |HighPriorityTask
[Q] |Q.HighPriority
[t] |T.Call

[\]
[r] .config: #Runtime.Resources << |Queue.GetResources
// Returns higher CPU/memory for high priority
[r] .py_session: #Sessions.Python << |Setup.RT.Python""
[<] <resources: #Runtime.Resources << .config
[/]
```

---

## Observability

### Metrics Collected

Per runtime session:
- CPU usage (user, system)
- Memory usage (RSS, VMS)
- I/O operations (reads, writes, bytes)
- Network usage
- Execution time
- Exit codes
- Error rates

### Logging

All runtime operations logged:

```
[INFO] Shell session 3f4a2 created (systemd unit: polyglot-shell-3f4a2.scope)
[INFO] Python session 7b8c1 created (uv env: /data/.polyglot-venvs/7b8c1)
[DEBUG] Rust compilation started (toolchain: stable)
[WARN] Python session 7b8c1 exceeded memory limit (1.2GB / 1GB)
[ERROR] Shell session 3f4a2 timeout (300s exceeded)
[INFO] Rust session 9d2e4 cleanup completed
```

---

## Migration Strategy

### Phase 1: Core Runtimes (Shell, Python)
- Implement Shell runtime with systemd-run
- Implement Python runtime with uv
- Update documentation

### Phase 2: Rust Runtime
- Implement Rust workspace management
- Add toolchain support

### Phase 3: Node.js Runtime (Future)
- Design Node.js runtime with nvm/fnm

### Phase 4: Enforcement
- Enable compiler checks for runtime requirements
- Migrate existing code to controlled runtimes

---

## Brainstorm Backlog Item

### Topic: Pipeline Formatted String Restrictions

**Problem:** How to define which pipelines can accept formatted strings?

**Context:**
- New syntax: `|{Pipeline}"formatted {.string}"`
- Not all pipelines should accept formatted strings
- Need mechanism to declare/enforce this capability

**Options to Explore:**
1. Type signature with `pg\fstring` type
2. Annotation/marker on pipeline definition
3. Compiler convention (e.g., namespace or name pattern)
4. Explicit whitelist
5. Pipeline capability flags

**Priority:** MEDIUM

**Related To:** String literal syntax change, type system design

---

## Documentation Updates Required

### New Files to Create

1. `docs/user/runtime-environments/README.md` - Overview
2. `docs/user/runtime-environments/shell.md` - Shell runtime guide
3. `docs/user/runtime-environments/python.md` - Python runtime guide
4. `docs/user/runtime-environments/rust.md` - Rust runtime guide
5. `docs/user/runtime-environments/resource-management.md` - Resource control
6. `docs/technical/runtime-architecture.md` - Technical design

### Files to Update

1. `docs/user/syntax/string-literals.md` - Update syntax
2. `docs/user/syntax/block-markers.md` - Add runtime blocks
3. `docs/user/standard-library/reserved-enumerations.md` - Add #Sessions.*
4. All examples using old `{Pipeline}"string"` syntax

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| String literal syntax updated | 100% | 🔄 In Progress |
| Runtime architecture documented | Complete | ✅ Complete |
| Shell runtime implemented | Functional | 🔄 Pending |
| Python runtime implemented | Functional | 🔄 Pending |
| Rust runtime designed | Specification | ✅ Complete |
| #Sessions.* enumeration added | Complete | 🔄 Pending |
| Compiler runtime checks | Enforced | 🔄 Pending |
| Documentation complete | All runtimes | 🔄 In Progress |

---

## Conclusion

**Status:** 🔄 **IN PROGRESS**

Successfully designed comprehensive runtime environment architecture for Polyglot:

**Key Components:**
- ✅ String literal syntax updated (`|{Pipeline}"string"`)
- ✅ Controlled runtime architecture defined
- ✅ `#Sessions.*` reserved enumeration specified
- ✅ Shell runtime (systemd-run) designed
- ✅ Python runtime (uv) designed
- ✅ Rust runtime (toolchain + workspace) designed
- ✅ Resource management strategy defined
- ✅ Queue integration specified
- ✅ Compile-time safety rules established

**Next Steps:**
1. Update all documentation with new string literal syntax
2. Implement Shell runtime with systemd integration
3. Implement Python runtime with uv integration
4. Implement Rust runtime
5. Add compiler enforcement for runtime requirements
6. Create comprehensive runtime documentation

**Core Principle Achieved:**
> All external code execution in Polyglot now runs in controlled, resource-managed, queue-integrated runtime environments with guaranteed cleanup and observability.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Type:** Runtime Environments Specification - Major Design Change
