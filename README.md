# Polyglot

**Version:** 0.2.0-specification
**Status:** Active Development - Language Specification Phase
**License:** TBD (Apache 2.0 or MIT)

<img src="./Polyglot%20Logo/PNG/Logo.png" width="150" alt="">

> **Important**: Polyglot is currently in the specification phase. The language design is stable but the compiler is not yet built. We're building in public and welcome contributors!

## Overview

Polyglot is a **trigger-driven programming language and platform** — async-centric and parallel-by-design, not as an afterthought — built on two pillars:

1. **Cross-Language Integration** — Utilise well-tested legacy code across Python, Rust, JavaScript, C++, or any supported runtime — don't reinvent what already works.
2. **Trigger-Driven Orchestration** — First-class parallelism, concurrency, queuing, and resource management. Every pipeline is triggered, not called — concurrency is the starting point, not an add-on.

Think: *what if your API orchestration layer was a programming language?*

For the full project vision, philosophy, and design principles, see **[Project Vision](docs/vision.md)**.

## Core Philosophy

- **The Right Tool for the Right Job** — Use the best language for each task in a workflow
- **Don't Reinvent the Wheel — Utilise Legacy Code** — The safest code is the code that already works; reuse battle-tested codebases instead of rewriting
- **Trigger-Driven, Async-Centric, Parallel-by-Design** — Every pipeline is triggered by an event; task behaviors are intentional, not afterthoughts
- **Pipeline-Centric** — Compose workflows through chaining, parallelism, and branching
- **Everything Is a Tree** — All data, types, pipelines, and metadata are trees on a unified `%` metadata tree
- **Resource Governance** — Explicit resource management, queuing, and limits
- **Security First** — Permissions, concurrency, and pipeline interactions are handled intentionally from day one

## Quick Example

A pipeline that watches for new log files, summarizes them with an LLM, and writes reports:

```polyglot
{@} @Local:1000::LogSummarizer:v1.0.0
   [@] @llm << @Community:ai::LLMService:v1.0.0

{-} -SummarizeCompletedLogs
   (-) <NewFiles#array.path
   (-) >ReportCount#int ~> 0
   [T] -T.Folder.NewFiles"/var/logs/app/"
      (-) >NewFiles >> <NewFiles
   [Q] -Q.Default
   [W] -W.Polyglot
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

## Why Polyglot?

### The Problem
Modern automation often requires:
- Python for data science and scripting
- Rust for performance-critical operations
- JavaScript for web interfaces
- C++ for legacy system integration
- Shell scripts for system operations

Existing solutions force you to choose one language or write brittle glue code.

### The Solution
Polyglot provides:
- **Unified Syntax** — Single language for multi-language workflows
- **Three-Bracket System** — `{X}` definitions, `[X]` control flow, `(X)` IO
- **Event-Driven** — React to file changes, schedules, webhooks, or direct calls
- **Resource Management** — Built-in queuing, throttling, and permission policies

## Key Features

### 1. Pipeline Structure

Every pipeline follows a mandatory structure: trigger, IO, queue, wrapper, execution body.

```polyglot
{-} -ProcessData
   (-) <input#string
   (-) >result#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $result << -Transform
      (-) <data << $input
      (-) >output >> $result
```

### 2. Event-Driven Triggers

```polyglot
{ } Watch for new CSV files
[T] -T.Folder.NewFiles"/data/"
   (-) >NewFiles >> <FilesToProcess

{ } Triggered by external call
[T] -T.Call

{ } Webhook trigger
[T] -T.Webhook"/api/process"
```

### 3. Parallel Execution with Expand/Collect

```polyglot
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

```polyglot
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

```polyglot
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

```polyglot
{ } Execute Python code
[W] -W.RT.Python:3:14
[-] [C]
   import pandas as pd
   df = pd.read_csv(input_path)
   result = df.describe().to_dict()

{ } Execute a compiled Rust binary
[W] -W.Polyglot
[-] -RT.CLI"./target/release/my_tool --input {$path}"
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

## Minimal Example

**hello.pg**
```polyglot
{@} @Local:1000::HelloWorld:v1.0.0

{-} -SayHello
   (-) <name#string
   [T] -T.CLI"--name"
      (-) >name >> <name
   [Q] -Q.Default
   [W] -W.RT.Shell
   [-] [C]
      echo "Hello, {$name}!"
```

## Use Cases

- **Data Engineering** — ETL pipelines combining Python preprocessing with Rust analytics
- **DevOps** — CI/CD orchestration across multiple tools and languages
- **Machine Learning** — Train in Python, serve with Rust for performance
- **System Automation** — Scheduled maintenance across heterogeneous systems
- **Legacy Integration** — Bridge old C++ systems with modern services
- **IoT Processing** — Real-time sensor data processing with resource limits

## Documentation

- **[Project Vision & Philosophy](docs/vision.md)** — What Polyglot is, why it exists, and where it's going
- **[Language Specification](docs/user/SPEC-INDEX.md)** — Complete syntax and concept reference
- **[Technical Reference](docs/technical/INDEX.md)** — EBNF grammar, compile rules, edge cases

## Project Status

**Current Phase:** Documentation-first specification (v0.2 complete). The language design is stable with comprehensive specs covering syntax, type system, pipelines, concurrency, error handling, permissions, and the standard library (pglib).

The project previously had a Rust implementation prototype which was reset in favor of a specification-first approach. Next steps: compiler architecture design and implementation.

## Installation

> Polyglot is not yet installable — we're in the specification phase. The compiler will be built on the foundations of the completed language spec.

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
