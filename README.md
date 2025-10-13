# Polyglot Automation Language

**Version:** 0.2.0-draft  
**Status:** Brainstorming Phase  
**License:** TBD (Apache 2.0 or MIT)

## Overview

Polyglot is an automation language designed to seamlessly integrate code from multiple programming languages into single coherent pipelines. Its core philosophy is pragmatic: **leverage existing legacy code** instead of reinventing the wheel.

**Note:** This language is currently in the brainstorming phase. Nothing concrete has been built yet.

## Core Philosophy

- **Don't Reinvent the Wheel, Embrace Legacy Code**—Bridge existing code from Python, JavaScript, Rust, C++, and more
- **Asynchronous by Default**—Every operation is inherently async, enabling efficient coordination
- **Pipeline-Centric Thinking**—Compositions of chained, parallel, and switching events
- **Use the Right Tool for the Job**—Choose the best language for each task in a workflow
- **Divide and Conquer**—Subdivide complex tasks into smaller tasks that can be tackled by any programming language.
- **Minimalist Footprint**—Orchestrate with minimal intervention and overhead from polyglot microservices.
- **Tasks partition**— Partition your task to smaller atomic subtasks that are Queued and dispatched in according to your computational pressures.

## Primary Goals

- **Simplify Event-Driven Automation**—Construct complex workflows with clear and simple syntax
- **Polyglot Integration**—Seamlessly integrates codes from different programming languages.
- **Minimize Integration Overhead**—Optimize time and memory footprint.
- **Concurrency Discipline**—Enforce data dependencies at compile-time to prevent race conditions.
- **Explicit Resource Management**—Mandatory explicit setup and cleanup phases.
- **Complete Resource Governance**—Transparent monitoring, graceful degradation, predictable behavior.
- **Performance Analysis**—Built-in metrics for debugging and optimization
- **Favoring Runtime instead of command-lines**—At the start, commandlines will be used as integration tool but will develop Runtimes where the function calls and import are native to the target programing language. Or even a docker images runs.

## Architecture

Polyglot consists of three main microservices:

### 1. Trigger Monitor
Watches for conditions that activate pipelines:
- File system changes
- Scheduled times
- REST API calls
- Message queue events
- CLI commands
- Resource thresholds

### 2. Queue Manager
Handles pipeline queueing and resource management:
- Priority-based scheduling
- Resource admission control
- Retry logic with backoff
- Dead letter queues
- Kill conditions for rouge processes

### 3. Executioner
Executes pipeline logic:
- Multi-language runtime management
- Type conversion between languages
- Error handling and propagation
- Performance metrics collection

## Quick Example

```polyglot
\\Define current module and import libraries
[@] com.example>DataPipeline>Analytics
\\ Defice short alias for rust's HashMap
[D] rust\HasMap = rust\HashMap<string, string>
[X]



\\ Pipline Defination
[|] ProcessUserData
[i] user_data: py\dict
[t] |T.Call

[Q] |Q.Priority
[<] .priority: pg\uint = 2
[Q] |Q.CpuAvailable
[<] .thrushold: pg\float = 75.0

[w] |W.Python3.10

[r] |ValidateData
[<] data.in: pg\ = user_data
[>] data.validated: py\dict = validated
[~][!] ?> py!/InvalidData \\ custom python error raised
[~][~][r] |U.Log.Error
[~][~][<] .message: pg\string = "Invalid data"
[~][~][x] |Exit \\ Exit whole pipline with code 400
[~][~][<] .code: pg\unit = 400


[f] |AnalyzePython
[<] .data: py\dict= validated
[>] .result:py\dict =  py_results 


[f] |AnalyzeRust
[<] .data: rust\HashMap = validated
[>] .result: rust\HashMap = rust_results 


[j] |JoinAll
[<] ... py_results
[<] ... rust_results


[r] |CombineResults 
[<] py_results: py\dict =  py_results
[<] rust_results: py\dict = rust_results \\ implict convertion rust\HasMap to py\dict
[>] final: py\dict

[o] .results :pg\json = final
[x]
```

## Documentation Structure

- **[README.md](README.md)**—This overview (you are here)
- **[doc/01-getting-started.md](doc/01-getting-started.md)** - Installation and first pipeline
- **[doc/02-language-syntax.md](doc/02-language-syntax.md)** - Complete syntax reference
- **[doc/03-type-system.md](doc/03-type-system.md)** - Type system and conversions
- **[doc/04-package-management.md](doc/04-package-management.md)** - Namespaces and imports
- **[doc/05-standard-library.md](doc/05-standard-library.md)** - Built-in pipelines reference
- **[doc/06-error-handling.md](doc/06-error-handling.md)** - Error handling patterns
- **[doc/07-flow-control.md](doc/07-flow-control.md)** - Switch statements and conditionals
- **[doc/08-queue-system.md](doc/08-queue-system.md)** - Queue configuration and management
- **[doc/09-architecture.md](doc/09-architecture.md)** - System architecture deep dive
- **[doc/10-execution-model.md](doc/10-execution-model.md)** - Pipeline execution flow
- **[doc/11-language-integration.md](doc/11-language-integration.md)** - Multi-language integration details
- **[doc/12-development-roadmap.md](doc/12-development-roadmap.md)** - Implementation phases and timeline
- **[doc/13-contributing.md](doc/13-contributing.md)** - How to contribute

## Key Features

### Multi-Language Support
Call functions seamlessly across Python, Rust, JavaScript, C++, and more with automatic type conversion.

### Event-Driven Triggers
```polyglot
[t] |T.FileChanged
[<] .path: pg\path =  \\cwd\\data
[<] .pattern: pg\string = "*.csv"

[t] |T.Schedule.Cron
[<] .cron: pg\string = "0 2 * * *"

[t] |T.Cpu.LowerThan
[<] .thrushold: pg\float = 90.0
```

### Parallel Execution
```polyglot
[f] |ProcessBranchA >> result_a
[f] |ProcessBranchB >> result_b
[j] |JoinAll
```

### Sophisticated Error Handling
```polyglot
[r] |RiskyOperation
[~] << input >> output

[~][!] !> py\!ValueError
[~][~][r] |U.Error.RetryWithBackoff << max_attempts: 5

[~][!] !> pg\!NetworkError
[~][~][r] |FallbackOperation << input >> output
```

### Resource Management
```polyglot
[Q] |Q.CpuAvailable << 80.0
[Q] |Q.MemoryAvailable << 16384
[Q] |Q.Kill.CpuLimit << 95.0
[Q] |Q.Kill.MemoryLimit << 90.0
[Q] |Q.Kill.ExecutionTimeout << T"30:"
```

### Flow Control
```polyglot
[?] status ?> "success"
[~][r] |HandleSuccess

[?] |CheckResourceAvailable >> is_available: pg\bool
[~][r] |UseIntensiveAlgorithm
```

## Installation (Future)

```bash
# Download and install Polyglot
curl -sSL https://polyglot.io/install.sh | sh

# Verify installation
polyglot --version

# Run your first pipeline
polyglot run my_pipeline.pg
```

## Minimal Example

**hello.pg**
```polyglot
[@] com.example>HelloWorld
[X]

[|] SayHello
[i] name: pg\string
[t] |T.Call

[w] |W.Python3.10

[r] |U.Console.Print << "Hello, {name}!"

[o] >> name
[x]
```

Run it:
```bash
polyglot run hello.pg --input name="World"
```


## Why Polyglot?

### Problem
Modern data pipelines often require:
- Python for data science
- Rust for performance-critical parts
- JavaScript for web interfaces
- C++ for legacy system integration


### Solution
Polyglot provides:
- **Unified syntax** for multi-language workflows
- **Type-safe** conversions between languages
- **Resource management** built-in
- **Event-driven** by default
- **Production-ready** monitoring and error handling

## Use Cases

- **Data Engineering—**ETL pipelines combining Python preprocessing with Rust analytics
- **Machine Learning—**Train models in Python, serve with Rust for performance
- **System Automation—**Orchestrate maintenance tasks across different tools
- **Legacy Integration—**Bridge old C++ systems with modern Python/JS applications
- **Microservice Orchestration** - Coordinate services written in different languages

## Community

- **GitHub Discussions** - Design discussions and Q&A
- **Discord** - Real-time collaboration (coming soon)
- **RFC Process—**Propose new features
- **Contributing**—See [CONTRIBUTING.md](doc/13-contributing.md)

## License

To be determined (likely Apache 2.0 or MIT)

