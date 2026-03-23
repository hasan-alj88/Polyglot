# Polyglot

**Version:** 0.1.0-development
**Status:** Active Development - Not Production Ready
**License:** TBD (Apache 2.0 or MIT)

<img src="./Polyglot%20Logo/PNG/Logo.png" width="150" alt="">

> **Important**: Polyglot is currently in early development. APIs, architecture, and features are subject to change. We're building in public and welcome contributors!

## Overview

Polyglot is an **async-centric programming language and platform** built on two pillars:

1. **Cross-Language Integration** — Write code in multiple programming languages and run them together seamlessly, leveraging the strengths of each.
2. **Async-Centric Automation** — First-class parallelism, concurrency, race condition handling, and resource management — designed in, not bolted on.

Think: *what if API was a programming language?*

For the full project vision, philosophy, and design principles, see **[Project Vision](docs/vision.md)**.

## Core Philosophy

- **The Right Tool for the Right Job** — Use the best language for each task in a workflow
- **Don't Reinvent the Wheel, Use Legacy Code** — Bridge existing Python, JavaScript, Rust, C++, and other codebases
- **Async-Centric by Design** — Every operation is inherently async; task behaviors are intentional, not afterthoughts
- **Divide and Conquer** — Break cross-language integration into smaller, solvable pieces and optimize each one
- **Pipeline-Centric** — Compose workflows through chaining, parallelism, and branching
- **Resource Governance** — Explicit resource management, queuing, and limits
- **Security First** — Concurrency, race conditions, and pipeline interactions are handled intentionally from day one

## Quick Example

```polyglot
// Module declaration
[@] com.example>DataPipeline>Analytics
[X]

[|] ProcessUserData
[i] user_data: py\dict
[t] |T.Call

// Queue configuration
[Q] |Q.Priority
[<] .level: pg\uint = 2
[<] .maxInstances: pg\uint = 1

// Runtime wrapper
[W] |W.Python3.10

// Validate data
[r] |ValidateData
[<] .data: py\dict = user_data
[>] .validated: py\dict = validated_data

[~][!] !> py\!ValueError
[~][~][r] |U.Log.Error
[~][~][<] .message: pg\string = "Invalid data"
[~][~]
[~][~][x] |Exit << .status = #Status.MainOperations.DataCorruption

// Parallel processing
[f] |AnalyzePython
[<] .data: py\dict = validated_data
[>] .result: py\dict = py_results

[f] |AnalyzeRust
[<] .data: rust\HashMap = validated_data
[>] .result: rust\HashMap = rust_results

// Wait for both
[j] |Y.JoinAll
[<] ... py_results
[<] ... rust_results

// Combine results
[r] |CombineResults
[<] .py_data: py\dict = py_results
[<] .rust_data: py\dict = rust_results
[>] .combined: py\dict = final_results

[o] .results: pg\json = final_results
[x]
```

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
- **Type-Safe Conversions** — Automatic conversion between language types
- **Event-Driven** — React to file changes, schedules, resource availability
- **Resource Management** — Built-in queuing, throttling, and limits

## Key Features

### 1. Multi-Language Integration
```polyglot
[f] |Python.ProcessData
[<] .csv_path: py\str = file_path
[>] .df: py\DataFrame = data

[f] |Rust.ComputeStats
[<] .data: rust\Vec<f64> = rust_data
[>] .stats: rust\HashMap = stats

[f] |Node.SendWebhook
[<] .payload: js\object = payload_object
[>] .status: js\number = status

[j] |Y.JoinAll
[<] ... data
[<] ... stats
[<] ... status
```

### 2. Event-Driven Triggers
```polyglot
[t] |T.File.Created
[<] .path: pg\path = //cwd//data
[<] .pattern: pg\string = "*.csv"

[t] |T.Schedule.Cron
[<] .cron: pg\string = "0 2 * * *"

[t] |T.HTTP.Webhook
[<] .endpoint: pg\string = "/process"
[<] .method: pg\Enum = #Rest.POST
```

### 3. Resource Management
```polyglot
// Pending queue: priority ordering with concurrency limits
[Q] |Q.Priority
[<] .level: pg\uint = 5
[<] .maxInstances: pg\uint = 2

// Active queue controls (called from execution body)
[r] |Q.KillIf.ExecutionTime.MoreThan
[<] .pipeline: pg\string = "=HeavyWork"
[<] .timeout: pg\string = "30m"

[r] |Q.PauseIf.CPU.MoreThan
[<] .pipeline: pg\string = "=HeavyWork"
[<] .threshold: pg\float = 95.0
```

### 4. Sophisticated Error Handling
```polyglot
[r] |RiskyOperation
[>] .result: pg\string = result

[~][!] !> py\!ValueError
[~][~][r] |U.Log.Error
[~][~][<] .message: pg\string = "Value error occurred"
[~][~]
[~][~][r] |RetryWithBackoff
[~][~][<] .max_attempts: pg\uint = 3

[~][!] !> pg\!NetworkError
[~][~][r] |FallbackOperation
[~][~][>] .result: pg\string = result
```

### 5. Parallel Execution with Synchronization
```polyglot
[f] |ProcessBatch1
[>] .results: pg\array<pg\json> = results1

[f] |ProcessBatch2
[>] .results: pg\array<pg\json> = results2

[f] |ProcessBatch3
[>] .results: pg\array<pg\json> = results3

[j] |Y.JoinFirst
[<] ... results1
[<] ... results2
[<] ... results3
[>] .first: pg\array<pg\json> = first_result

// Continue with whichever finishes first
```

## Minimal Example

**hello.pg**
```polyglot
[@] com.example>HelloWorld
[X]

[|] SayHello
[i] name: pg\string
[t] |T.Call

[W] |W.Python3.10

[r] |U.Console.Print << .message: f"Hello, {name}!"

[o] >> name
[x]
```

Run it:
```bash
polyglot run hello.pg --name="World"
>> Hello, World!
```

### The Cross-Language Bridge
One of the aims is the ability to integrate codebases from different programming languages easily. The Polyglot service needs to be running in the background for it.

```python
# Trigger from Python, get results from Rust
import polyglot as pg
import polars as pl
import os

rust_function = pg.register.rust(
    base_dir= os.environ['RustBaseDirectory'],
    function_name="rust_data_processing"
)

data = pl.readcsv('datafile.csv')
result = await pg.run(rust_function, data=data)
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

> Language specification, architecture, and ecosystem docs are being written as part of the documentation-first development approach. More coming soon.

## Project Status

**Current Phase:** Documentation-first specification — defining the language and architecture before coding.

The project previously had a Rust implementation prototype which was reset in favor of a specification-first approach. We're writing comprehensive specs, then building the implementation on solid foundations.

## Installation

> To be determined. Polyglot is not yet installable — we're in the specification phase.

## Getting Involved

We're looking for collaborators interested in:
- Language design and syntax refinement
- Architecture review and feedback
- Documentation and examples
- Compiler implementation (once specs are complete)
- Runtime system architecture
- Standard library development

**Contributing Guidelines:** Coming soon in `docs/contributing.md`

## License

To be determined (likely Apache 2.0 or MIT)

---

**Status:** The language is being specified. We're seeking feedback and collaborators to turn this vision into reality.
