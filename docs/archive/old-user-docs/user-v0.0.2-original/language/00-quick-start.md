# Quick Start Guide

**Version:** 0.0.2
**Status:** Active Development - Design Phase

---

## Overview

Get started with Polyglot in minutes! This guide walks you through writing your first multi-language pipeline.

---

## Prerequisites

Before starting with Polyglot, ensure you have:

1. **Polyglot Service Running** - The Polyglot daemon must be running
   - *Note: Service implementation is not yet available (TBA)*
   - The daemon manages triggers, queues, and execution

2. **Development Environment**
   - Text editor or IDE with Polyglot syntax support
   - Command-line access for running `polyglot` CLI commands

3. **Language Runtimes** (for the examples in this guide)
   - Python 3.11+ (managed via `uv`)
   - Rust 1.70+ (for Rust examples)

---

## Installation

**Status:** To Be Announced (TBA)

The Polyglot installation process is under development. Future installation will include:
- Polyglot daemon (background service)
- CLI tools (`polyglot compile`, `polyglot register`, `polyglot activate`)
- Language runtime integrations

Check the [project repository](#) for the latest installation instructions.

---

## Your First Pipeline: Hello World

Let's create a simple pipeline that says hello in both Python and Rust, triggered from the command line.

### Step 1: Create a `.pg` File

Create a file named `hello.pg`:

```polyglot
[@] Local@MyFirstApp.HelloWorld:1.0.0
[#] 1
[X]





// Pipeline: Hello World in Multiple Languages
[|] SayHello

// Trigger: Command-line activation
[t] |T.Cli

// Runtimes Wrappers
[W] |W.Python3.10
[W] |W.Rust1.8

// Python Hello
[r] |Run.Python
[<] .code: pg\string << "print('Hello from Python!')"

// Rust Hello
[r] |Run.Rust
[<] .code: pg\String << "println!(\"Hello from Rust!\");"

// No outputs
[o] #None
[X]
```

### Step 2: Understanding the Code

Let's break down what each part does:

**Package Declaration:**
```polyglot
[@] Local@MyFirstApp.HelloWorld:1.0.0
[#] 1
[X]
```
- `[@]` - Package block defining namespace and version
- `[#]` - File number (for multi-file packages)
- `[X]` - Terminator (ends the package block)

**Pipeline Definition:**
```polyglot
[|] SayHello
```
- `[|]` - Pipeline marker, followed by pipeline name

**Trigger:**
```polyglot
[t] |T.Cli
```
- `[t]` - Trigger marker
- `|T.Cli` - Command-line trigger (manual invocation)

**Run Blocks:**
```polyglot
[r] |Run.Python
[<] .code: py\str << "print('Hello from Python!')"
```
- `[r]` - Run marker (execute code)
- `[<]` - Input assignment
- `py\str` - Python string type
- `<<` - Assignment operator

**Output:**
```polyglot
[o] #None
```
- `[o]` - Output marker
- `#None` - No outputs from this pipeline

### Step 3: Compile and Register

Once the Polyglot CLI is available:

```bash
# Compile the .pg file to IR
polyglot compile hello.pg

# Register the package with the daemon
polyglot register Local@MyFirstApp.HelloWorld:1.0.0

# Activate triggers
polyglot activate Local@MyFirstApp.HelloWorld:1.0.0
```

### Step 4: Run Your Pipeline

```bash
# Trigger the pipeline via CLI
polyglot run SayHello
```

**Expected Output:**
```
Hello from Python!
Hello from Rust!
```

---

## Key Concepts

### 1. Block Markers

Polyglot uses uppercase markers in square brackets to define structure:

| Marker | Purpose | Example |
|--------|---------|---------|
| `[@]` | Package declaration | `[@] Local@MyApp:1.0.0` |
| `[#]` | File number | `[#] 1` |
| `[\|]` | Pipeline definition | `[\|] ProcessData` |
| `[i]` | Input parameters | `[i] .data: pg\string` |
| `[t]` | Trigger | `[t] \|T.Daily` |
| `[Q]` | Queue configuration | `[Q] \|Q.Priority` |
| `[W]` | Wrapper (runtime setup) | `[W] \|W.Python3.12` |
| `[s]` | Setup block | `[s] \|Setup.CreateTempDir` |
| `[r]` | Run block | `[r] \|MyPipeline` |
| `[p]` | Parallel execution | `[p] \|ProcessPartA` |
| `[Y]` | Join (collect parallel results) | `[Y] \|Y.JoinAll` |
| `[c]` | Cleanup block | `[c] \|Cleanup.DeleteTemp` |
| `[o]` | Output parameters | `[o] .result: pg\string` |
| `[X]` | Terminator | `[X]` |

### 2. Pipeline Components

**Inputs** - Define parameters passed into the pipeline:
```polyglot
[i] .data: pg\string
[i] .timeout: pg\int << 30
```

**Triggers** - Specify when the pipeline runs:
```polyglot
[t] |T.Cli          // Manual command-line trigger
[t] |T.Daily        // Daily schedule
[t] |T.FileWatch    // File system events
[t] |T.Webhook      // HTTP webhooks
[t] |T.Call         // Called by another pipeline
```

**Important:** Triggers are the **entry point** for pipeline execution. Without a trigger, your pipeline won't run automatically. Use `|T.Cli` for manual testing or `|T.Call` when invoking from other pipelines.

**Queue Configuration** - Control execution priority and resources:
```polyglot
[Q] |Q.Priority
[<] .level: pg\int << 8    // Priority 1-10 (10 = highest)

[Q] |Q.RequireResource
[<] .cpu_cores: pg\int << 2
[<] .memory_mb: pg\int << 4096
```

**Wrapper** - Set up language runtime environment:
```polyglot
[W] |W.Python3.12
[<] .dependencies: pg\path << \\FileDir\\requirements.txt
```

**Setup** - Initialize resources before execution:
```polyglot
[\] |Setup.CreateTempDir
[>] .temp_path: pg\path >> .workspace
```

**Sequential Execution** - Run blocks execute one after another:
```polyglot
[r] |Step1
[r] |Step2
[r] |Step3
```

**Parallel Execution** - Process multiple branches simultaneously:
```polyglot
[p] |ProcessPartA
[>] .result_a: pg\string >> .output_a

[p] |ProcessPartB
[>] .result_b: pg\string >> .output_b

// Join results
[Y] |Y.JoinAll
[<] ... .output_a
[<] ... .output_b
```

**Fire and Forget** - Run without waiting for results:
```polyglot
[f] |SendNotification
[<] .message: pg\string << "Processing started"
// Execution continues immediately
```

**Cleanup** - Release resources after execution:
```polyglot
[/] |Cleanup.DeleteTempFiles
[<] .path: pg\path << .workspace
```

### 3. Types and Variables

**Type Format:** `language\type`

Common types:
- `pg\string`, `pg\int`, `pg\bool` - Polyglot native types
- `py\str`, `py\int`, `py\dict` - Python types
- `rs\String`, `rs\i32`, `rs\HashMap` - Rust types
- `pg\dt` - Polyglot datetime
- `pg\path` - File system path

**Variable Naming:**
- All variables start with `.` (dot prefix)
- Examples: `.data`, `.count`, `.file_path`
- Cannot end with `.`

**Assignment:**
```polyglot
[<] .input_var: pg\int << 42           // Input assignment
[>] .output_var: pg\string >> .result  // Output assignment
```

### 4. Comments

Use `//` for single-line comments:
```polyglot
// This is a comment
[r] |ProcessData  // Inline comment
```

**Formatting Rule:** Use 4 blank lines before new pipeline definitions (after package block).

---

## Next Steps

Now that you've created your first pipeline, explore these topics:

1. **Complete Syntax Reference** - [Complete Syntax](01-syntax-complete.md)
   - Learn all block markers and syntax rules
   - Advanced patterns and techniques

2. **Type System** - [Type System](02-type-system.md)
   - Cross-language type conversion
   - Type safety and validation

3. **Examples** - [Hello World Examples](../examples/hello-world.md)
   - More beginner examples
   - Common patterns and use cases

4. **BNF Grammar** - [BNF Grammar](bnf/polyglot grammer.md)
   - Authoritative grammar specification
   - Parser implementation reference

5. **Error Handling** - [Error Handling](04-error-handling.md)
   - `!Error` types
   - Recovery patterns

6. **Parallel Execution** - [Parallel Execution](08-parallel-execution.md)
   - Fork/join patterns
   - Performance optimization

7. **Standard Library** - [Standard Library](../standard-library/00-overview.md)
   - Built-in utilities and wrappers
   - Trigger catalog
   - Queue control operations

---

## Common Patterns

### Pattern 1: Scheduled Data Processing

```polyglot
[|] DailyReport

[t] |T.Daily
[<] .at: pg\dt << DT"16:30:"

[r] |FetchData
[>] .data: py\dict >> .raw_data

[r] |ProcessData
[<] .input: py\dict << .raw_data
[>] .report: pg\string >> .result

[o] .result: pg\string
[X]
```

### Pattern 2: File Watch Automation

```polyglot
[|] ProcessNewFiles

[t] |T.FileWatch
[<] .path: pg\path << \\Data\\incoming\\
[<] .pattern: pg\string << "*.csv"

[r] |ValidateFile
[r] |ImportToDatabase
[r] |SendNotification

[o] #None
[X]
```

### Pattern 3: Parallel Processing

```polyglot
[|] ParallelWorkflow

[t] |T.Call

[p] |TaskA
[p] |TaskB
[p] |TaskC

[Y] |Y.JoinAll

[r] |CombineResults

[o] .final_result: pg\string
[X]
```

---

## Troubleshooting

**Common Issues:**

1. **Missing package block** - Every `.pg` file must start with `[@]`, `[#]`, `[X]`
2. **Variable naming** - Variables must start with `.` (e.g., `.data` not `data`)
3. **Type separator** - Use backslash `\` (e.g., `pg\int` not `pg/int`)
4. **DateTime syntax** - Use `DT"..."` prefix (e.g., `DT"16:30:"` not `T"16:30:"`)
5. **Assignment operator** - Use `<<` only (not `=`)

---

## Getting Help

- **Documentation Index** - [README](../README.md)
- **Syntax Questions** - [BNF Grammar](bnf/polyglot grammer.md)
- **Examples** - [Examples Index](../examples/00-index.md)
- **Issues & Bugs** - Check the project issue tracker

---

**Last Updated:** 2025-11-15