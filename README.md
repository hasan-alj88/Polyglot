# Aljam3

**Version:** 0.2.0-specification
**Status:** Active Development — Language Specification Phase
**License:** TBD (Apache 2.0 or MIT)

<img src="./Aljam3%20Logo/PNG/Logo.png" width="150" alt="">

> **Important**: Aljam3 is in the specification phase — the language design is complete but the compiler is not yet built. We're building in public and welcome contributors!

## What Is Aljam3?

**الجامع** (Aljam3, `/ælˈdʒæːmɪʕ/`) means to collect, gather, and bring together. It is also the author's family name (Hasan Aljamea), representing a personal vision to create a language that *gathers* systems together. The abbreviation and file extension `.jm3` reflects the root verb *جمع* (jama'a), meaning "to gather" or "to bring together", emphasizing its role in unifying disparate systems. It is a **trigger-driven programming language and platform** designed to communicate with the computer based on *how to react* rather than just *what to execute*. It is built on two core pillars that reflect its name:

1. **Gathering Languages (Cross-Language Integration)** — The objective is to *use* legacy code, not reinvent it. Aljam3 seamlessly integrates Python, Rust, JavaScript, and C++ into a unified workflow, bringing programming languages together under dynamic, trigger-driven control.
2. **Gathering Programs (Parallel Orchestration)** — First-class parallelism with strict resource control. It introduces dynamic, priority-based resource allocation to prevent critical jobs from starving, combined with **exhaustive logic** ensuring there is never a scenario where the program doesn't know what to do.

Think: *what if your API orchestration layer was a programming language?*

For the full project vision, philosophy, and design principles, see **[Project Vision](docs/vision.md)**.

### There Is No Main Function

Aljam3 pipelines are not programs you run — they are reactions you define. Something happens, a trigger fires, work executes. You are not writing a sequence of instructions — you are defining how the system responds to events and conditions.

This is a fundamental shift: traditional approaches build synchronous code first, then retrofit async handling as an afterthought. Aljam3 inverts this — triggers and concurrency are the starting point.

## Hello World

**hello.jm3**
```aljam3
{@} @Local:1000::HelloWorld:v1.0.0

{-} -SayHello
   (-) <name#string
   [T] -T.CLI"--name"
      (-) >name >> <name
   [Q] -Q.Default
   [W] -W.Env.Shell
   [ ]
   [-] [C]
      echo "Hello, {$name}!"
```

**What this shows:**
- `{@}` — every file declares its package
- `{-}` — a pipeline definition, not a function
- `[T]` — this pipeline fires when a CLI argument arrives, not when someone calls it
- `[Q]` — every pipeline has a queue strategy (even "just run it")
- `[W]` — a wrapper connects to the Shell runtime
- `[-] [C]` — the execution body, running shell code
- No `main()`, no entry point — the trigger *is* the entry point

## Why Aljam3?

### The Problem
Modern automation often requires:
- Python for data science and scripting
- Rust for performance-critical operations
- JavaScript for web interfaces
- C++ for legacy system integration
- Shell scripts for system operations

Existing solutions force you to choose one language or write brittle, ad-hoc integrations.

### The Solution
Aljam3 provides:
- **Unified Syntax** — Single language for multi-language workflows
- **Three-Bracket System** — `{X}` definitions, `[X]` control flow, `(X)` IO
- **Trigger-Driven** — React to file changes, schedules, webhooks, or direct calls
- **Resource Management** — Built-in queuing, throttling, and permission policies

## Core Philosophy

- **On the Shoulders of Giants — Utilise Legacy Code** — The safest code is the code that already works; reuse battle-tested codebases instead of rewriting
- **Trigger-Driven, Async-Centric, Parallel-by-Design** — Every pipeline is triggered by an event; task behaviours are intentional, not afterthoughts
- **The Right Tool for the Right Job** — Use the best language for each task in a workflow
- **Everything Is a Tree** — All data, types, pipelines, and metadata are trees on a unified `%` metadata tree
- **Implicit Deny** — Nothing runs without explicit, intentional permission. Permissions, concurrency, and pipeline interactions are handled from day one
- **Resource Governance** — Explicit resource management, queuing, and limits

## A Richer Example

A pipeline that watches for new log files, summarises them with an LLM, and writes reports:

```aljam3
{@} @Local:1000::LogSummarizer:v1.0.0
   [@] @llm << @Community:ai::LLMService:v1.0.0

{-} -SummarizeCompletedLogs
   (-) <NewFiles#array.path
   (-) >ReportCount#int ~> 0
   [T] -T.Folder.NewFiles"/var/logs/app/"
      (-) >NewFiles >> <NewFiles
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [=] =ForEach.Array.Enumerate
      (=) <Array << $NewFiles
      (=) >item >> $logFile
      (=) >index >> $index
      [?] -File.Access"{$logFile}" =? #FileAccess.Available
         [-] $logContent#string << -File.Text.Read"{$logFile}"
            [!] !*
               [-] $logContent#string << ""
         [-] @llm-Summarize
            (-) <content << $logContent
            (-) <prompt << "Summarize this log file concisely."
            (-) >response >> $summary
         [-] -File.Text.Write
            (-) <path << -Path"/var/logs/reports/log_report_{$index}.txt"
            (-) <content << $summary
      [?] *?
         [-] -DoNothing
   [-] *Agg.Count
      (*) <item << $summary
      (*) >count >> >ReportCount
```

**What this shows:**
- `{@}` package declaration with `[@]` import
- `{-}` pipeline with mandatory `[T]` trigger, `[Q]` queue, `[W]` wrapper
- `(-)` IO parameters with `<<`/`>>` assignment and `#type` annotations
- `[=]` parallel expand over an array with `(=)` IO
- `[?]` conditionals with mandatory comparison operators and `*?` wildcard catch-all
- `[!]` error handling with `!*` wildcard catch and recovery value
- `[-]` sequential calls with inline string args (`"{$logFile}"`)
- `*Agg.Count` collector with `(*)` IO writing to output port

## Key Features

### 1. Pipeline Structure

Every pipeline follows a mandatory structure: trigger, IO, queue, wrapper, execution body.

```aljam3
{-} -ProcessData
   (-) <input#string
   (-) >result#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] $result << -Transform
      (-) <data << $input
      (-) >output >> $result
```

### 2. Trigger-Driven Pipelines

```aljam3
{ } Watch for new CSV files
[T] -T.Folder.NewFiles"/data/"
   (-) >NewFiles >> <FilesToProcess

{ } Triggered by external call
[T] -T.Call

{ } Webhook trigger
[T] -T.Webhook"/api/process"
```

### 3. Parallel Execution with Expand/Collect

```aljam3
{ } Process items in parallel, collect results
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $current
   [-] -ProcessItem
      (-) <data << $current
      (-) >output >> $processed
[-] *Into.Array
   (*) <item << $processed
   (*) >Array >> >results
```

### 4. Error Handling

All errors must be handled — no silent failures. Errors use the `!` prefix and `[!]` blocks:

```aljam3
[-] -RiskyOperation
   (-) <input << $data
   (-) >output >> $result
   [!] !File.NotFound
      [-] $result << "fallback"
   [!] !*
      [-] $result << ""
```

### 5. Type System

Everything is a tree. Types use `{#}` definitions with `#` prefix:

```aljam3
{#} #UserRecord
   .name#string
   .email#string
   .age#int
   .role#UserRole

{#} #UserRole
   .Admin
   .Editor
   .Viewer
```

### 6. Cross-Language Execution

Runtime wrappers (`[W]`) connect to foreign language runtimes:

```aljam3
{ } Execute Python code
[W] -W.Env.Python:3:14
[ ]
[-] [C]
   import pandas as pd
   df = pd.read_csv(input_path)
   result = df.describe().to_dict()

 { } Execute a compiled Rust binary
 [W] -W.Aljam3
 [ ]
 [-] -Run.Rust.CLI
    (-) <binary#path << -Path"./target/release/my_tool"
    (-) <arg#Record
       [.] .input#string << "{$path}"
```

## Syntax At a Glance

### Three Bracket Types

| Shape | Role | Examples |
|-------|------|---------|
| `{X}` | Define | `{@}` package, `{#}` struct, `{-}` pipeline, `{!}` errors, `{_}` permissions |
| `[X]` | Control | `[T]` trigger, `[Q]` queue, `[W]` wrapper, `[-]` run, `[=]` parallel, `[?]` conditional |
| `(X)` | IO | `(-)` pipeline IO, `(=)` expand IO, `(*)` collect IO |

### Identifier Prefixes

| Prefix | Meaning | Example |
|--------|---------|---------|
| `@` | Package | `@Local:1000::MyPkg:v1` |
| `#` | Data type | `#UserRecord`, `#string` |
| `-` | Pipeline | `-ProcessData`, `-File.Text.Read` |
| `$` | Variable | `$input`, `$result` |
| `!` | Error | `!File.NotFound`, `!Error:Validation.Empty` |
| `%` | Metadata | `%#`, `%-`, `%!` |
| `_` | Permission | `_ReadOnly`, `__NetworkAccess` |

### Assignment Operators

| Operator | Name | Direction |
|----------|------|-----------|
| `<<` | PushLeft (Final) | Right to left |
| `>>` | PushRight (Final) | Left to right |
| `<~` | DefaultPushLeft | Right to left (one reassignment allowed) |
| `~>` | DefaultPushRight | Left to right (one reassignment allowed) |

## The Aljam3 Ecosystem

The **Aljam3 Service** is the runtime backbone, consisting of three components:

- **Trigger Monitor** — Monitors events and evaluates conditions that initiate automated tasks
- **Queue Handler** — Manages queue state and dispatches jobs to Runners
- **Runner** — Executes pipelines, managing the lifecycle of each task from dispatch to completion

The Aljam3 Service must be running in the background to handle execution of automated tasks and manage their interactions.

## Use Cases

- **Data Engineering** — ETL pipelines combining Python preprocessing with Rust analytics
- **DevOps** — CI/CD orchestration across multiple tools and languages
- **Machine Learning** — Train in Python, serve with Rust for performance
- **System Automation** — Scheduled maintenance across heterogeneous systems
- **Legacy Integration** — Bridge old C++ systems with modern services
- **IoT Processing** — Real-time sensor data processing with resource limits

## Documentation

- **[Project Vision & Philosophy](docs/vision.md)** — What Aljam3 is, why it exists, and where it's going
- **[Language Specification](docs/user/SPEC-INDEX.md)** — Complete syntax and concept reference
- **[Technical Reference](docs/technical/INDEX.md)** — EBNF grammar, compile rules, edge cases

## Project Status

**Current Phase:** Documentation-first specification (v0.2 complete). The language design is stable with comprehensive specs covering syntax, type system, pipelines, concurrency, error handling, permissions, and the standard library (jm3lib).

The project previously had a Rust implementation prototype which was reset in favour of a specification-first approach. Next steps: compiler architecture design and implementation.

## Installation

> Aljam3 is not yet installable — we're in the specification phase. The compiler will be built on the foundations of the completed language spec.

## Getting Involved

We're looking for collaborators interested in:
- Language design and syntax refinement
- Architecture review and feedback
- Documentation and examples
- Compiler implementation (Rust)
- Runtime system architecture
- Standard library development

## License

To be determined (likely Apache 2.0 or MIT)

---

**Status:** Language specification is complete. Seeking feedback and collaborators to build the compiler.
