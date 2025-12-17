# Quick Start Guide

**Version:** 0.0.3
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
{@} @Local::MyFirstApp.HelloWorld:1.0.0.0
[A] @Hello
{x}




// Pipeline: Hello World in Multiple Languages
{|} |Pipeline.SayHello

[t] |T.Cli                              // Trigger: Command-line activation

[W] |W.Polyglot.Scope                   // Wrapper: No special setup needed


// Python Hello
[r] |W.Python"print_hello"
\<\ <code << "print('Hello from Python!')"

// Rust Hello
[r] |W.Rust"print_hello"
\<\ <code << "println!(\"Hello from Rust!\");"

[>} !No.Output                          // No outputs
{x}
```

### Step 2: Understanding the Code

Let's break down what each part does:

**Registry Declaration:**
```polyglot
{@} @Local::MyFirstApp.HelloWorld:1.0.0.0
[A] @Hello
{x}
```
- `{@}` - Registry block defining namespace and version
- `@Scope::Namespace.Name:Major.Minor.Patch.Build` - Four-part versioning
- `[A]` - Alias for this registry (short name)
- `{x}` - Close registry block

**Pipeline Definition:**
```polyglot
{|} |Pipeline.SayHello
```
- `{|}` - Pipeline registry marker
- `|Pipeline.SayHello` - Full pipeline path

**Trigger:**
```polyglot
[t] |T.Cli
```
- `[t]` - Trigger marker (execution marker)
- `|T.Cli` - Command-line trigger (manual invocation)

**Wrapper:**
```polyglot
[W] |W.Polyglot.Scope
```
- `[W]` - Wrapper marker (provides setup/cleanup context)
- `|W.Polyglot.Scope` - Default Polyglot scope (no special setup)

**Run Blocks:**
```polyglot
[r] |W.Python"print_hello"
\<\ <code << "print('Hello from Python!')"
```
- `[r]` - Run marker (execute operation)
- `\<\` - Nested input marker
- `<code` - Input port name
- `<<` - Assignment operator
- `"..."` - String literal value

**Output:**
```polyglot
[>} !No.Output
```
- `[>}` - Output declaration marker
- `!No.Output` - Reserved symbol for pipelines with no output

### Step 3: Compile and Register

Once the Polyglot CLI is available:

```bash
# Compile the .pg file to IR
polyglot compile hello.pg

# Register the package with the daemon
polyglot register @Local::MyFirstApp.HelloWorld:1.0.0.0

# Activate triggers
polyglot activate @Local::MyFirstApp.HelloWorld:1.0.0.0
```

### Step 4: Run Your Pipeline

```bash
# Trigger the pipeline via CLI
polyglot run |Pipeline.SayHello
```

**Expected Output:**
```
Hello from Python!
Hello from Rust!
```

---

## Key Concepts

### 1. Block Markers (v0.0.3)

Polyglot v0.0.3 uses **two marker types**:
- **Registry Markers** `{ }` - Define structure (packages, pipelines, types)
- **Execution Markers** `[ ]` - Control execution flow
- **Nested Markers** `\marker\` - Nested operations

| Marker | Type | Purpose | Example |
|--------|------|---------|---------|
| `{@}` | Registry | Package declaration | `{@} @Local::MyApp:1.0.0.0` |
| `[A]` | Execution | Alias declaration | `[A] @MyApp` |
| `{|}` | Registry | Pipeline definition | `{|} |Pipeline.ProcessData` |
| `{#}` | Registry | Enum/Serial definition | `{#} #MyType` |
| `[.]` | Execution | Field definition | `[.] .field:pg.string` |
| `[<}` | Execution | Input parameters | `[<} .data:pg.string` |
| `[t]` | Execution | Trigger | `[t] |T.Daily` |
| `[Q]` | Execution | Queue configuration | `[Q] |Q.Priority` |
| `[W]` | Execution | Wrapper (runtime setup) | `[W] |W.Python` |
| `[\][/]` | Execution | Setup/cleanup section | `[\][/]` (within wrapper) |
| `[r]` | Execution | Run block | `[r] |MyPipeline` |
| `[p]` | Execution | Parallel execution | `[p] ~ForEach` |
| `[v]` | Execution | Join (v-shaped merge) | `[v] ~V.JoinAll` |
| `[y]` | Execution | Switch (y-shaped fork) | `[y] .x >? 10` |
| `[>}` | Execution | Output parameters | `[>} .result:pg.string` |
| `{x}` | Registry | Close block | `{x}` |
| `\<\` | Nested | Pass input (nested) | `\<\ <value << .x` |
| `\>\` | Nested | Pass output (nested) | `\>\ >result >> .y` |
| `\~\` | Nested | Expand-above | `\~\[r] ...` |
| `\&\` | Nested | AND (nested boolean) | `\&\ .y >? 5` |
| `\|\` | Nested | OR (nested boolean) | `\|\ .y <? 10` |

### 2. Pipeline Components

**Inputs** - Define parameters passed into the pipeline:
```polyglot
[<} .data:pg.string                     // Required input
[<} .timeout:pg.int <~ 30               // Default input
[<} .api_key:pg.string << "secret"      // Constant input
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
\<\ <level << 8                         // Priority 1-10 (10 = highest)

[Q] |Q.RequireResource
\<\ <cpu_cores << 2
\<\ <memory_mb << 4096
```

**Wrapper** - Set up language runtime environment:
```polyglot
[W] |W.Python
\<\ <dependencies << \\FileDir\\requirements.txt
```

**Setup/Cleanup** - Initialize and release resources:
```polyglot
[W] |W.CustomSetup
[\] .workspace:pg.path << |CreateTempDir
[/] |DeleteTempFiles
```

**Sequential Execution** - Run blocks execute one after another:
```polyglot
[r] |Step1
[r] |Step2
[r] |Step3
```

**Parallel Execution** - Process multiple branches simultaneously:
```polyglot
[p] ~ForEach
\<\ <array << .items
\>\ >item >> .current

\~\[r] |ProcessItem
\~\\<\ <input << .current
\~\\>\ >output >> .result

\~\[v] ~V.JoinAll                       // Join results
\~\\<\ <append << .result
\~\\>\ >array >> .all_results
```

**Background Execution** - Run without waiting for results:
```polyglot
[b] |SendNotification
\<\ <message << "Processing started"
// Execution continues immediately
```

### 3. Types and Variables

**Type Format:** `pg.type` or `#CustomType`

Common types:
- `pg.string`, `pg.int`, `pg.bool` - Polyglot native types
- `pg.dt` - Polyglot datetime
- `pg.path` - File system path
- `pg.array{T}` - Array of type T
- `pg.serial` - Serialized structure
- `#CustomType` - User-defined enumeration/serial

**Variable Naming:**
- All variables start with `.` (dot prefix)
- Examples: `.data`, `.count`, `.file_path`
- Cannot end with `.`

**Assignment:**
```polyglot
[<} .input_var:pg.int                   // Declared input (required)
[<} .timeout:pg.int <~ 30               // DefaultReady input
[<} .version:pg.string << "1.0"         // Constant input
[r] .result:pg.string << .input_var     // Variable assignment
```

### 4. Comments

Use `//` for single-line comments:
```polyglot
// This is a comment
[r] |ProcessData                        // Inline comment
```

**Formatting Rule:** Use 4 blank lines before new pipeline definitions (after registry block).

---

## Next Steps

Now that you've created your first pipeline, explore these topics:

1. **Complete Syntax Reference** - [Complete Syntax](01-syntax-complete.md)
   - Learn all block markers and syntax rules
   - Advanced patterns and techniques

2. **Type System** - [Type System](02-type-system.md)
   - Cross-language type conversion
   - Type safety and validation

3. **Examples** - [Example Files](../../project/examples/)
   - Comprehensive .pg examples
   - Common patterns and use cases

4. **BNF Grammar** - [BNF Grammar](12-bnf-grammar.md)
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
{|} |Pipeline.DailyReport

[t] |T.Daily
\<\ <at << |DT"16:30:"

[W] |W.Polyglot.Scope


[r] |FetchData
\>\ >data >> .raw_data

[r] |ProcessData
\<\ <input << .raw_data
\>\ >report >> .result

[>} .result:pg.string
{x}
```

### Pattern 2: File Watch Automation

```polyglot
{|} |Pipeline.ProcessNewFiles

[t] |T.FileWatch
\<\ <path << \\Data\\incoming\\
\<\ <pattern << "*.csv"

[W] |W.Polyglot.Scope


[r] |ValidateFile
[r] |ImportToDatabase
[r] |SendNotification

[>} !No.Output
{x}
```

### Pattern 3: Parallel Processing

```polyglot
{|} |Pipeline.ParallelWorkflow

[t] |T.Call

[W] |W.Polyglot.Scope


[p] |TaskA
\>\ >result >> .result_a

[p] |TaskB
\>\ >result >> .result_b

[p] |TaskC
\>\ >result >> .result_c

[v] ~V.JoinAll
\<\ <append << .result_a
\<\ <append << .result_b
\<\ <append << .result_c
\>\ >array >> .all_results

[r] |CombineResults
\<\ <inputs << .all_results
\>\ >output >> .final_result

[>} .final_result:pg.string
{x}
```

---

## Troubleshooting

**Common Issues:**

1. **Missing registry block** - Every `.pg` file must start with `{@}`, `[A]`, `{x}`
2. **Variable naming** - Variables must start with `.` (e.g., `.data` not `data`)
3. **Type separator** - Use dot `.` (e.g., `pg.int` not `pg\int`)
4. **DateTime syntax** - Use `|DT"..."` prefix (e.g., `|DT"16:30:"`)
5. **Assignment operator** - Use `<<` for assignment, `<~` for default
6. **Marker type confusion** - Registry `{ }`, Execution `[ ]`, Nested `\marker\`
7. **Missing mandatory sections** - All pipelines need `[<}`, `[t]`, `[\][/]` or `[W]`, `[>}`

---

## Getting Help

- **Documentation Index** - [README](../README.md)
- **Syntax Questions** - [BNF Grammar](12-bnf-grammar.md)
- **Examples** - [Example Files](../../project/examples/)
- **Issues & Bugs** - Check the project issue tracker

---

**Last Updated:** 2025-12-10
