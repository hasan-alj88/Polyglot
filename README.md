# Polyglot Automation System

**Version:** 0.1.0-development  
**Status:** Active Development - Not Production Ready  
**License:** TBD (Apache 2.0 or MIT)

<img src="./Polyglot%20Logo/PNG/Logo.png" width="150" alt="">

## 🚧 Under Active Development

> **Important**: Polyglot is currently in early development. APIs, architecture, and features are subject to change. We're building in public and welcome contributors!

## Overview

Polyglot is an **automation microservice** that enables **cross-language workflow orchestration**. Trigger pipelines from any supported language and consume results from codebases written in completely different ecosystems.

## Core Philosophy

- **Don't Reinvent the Wheel, Use legacy code**—Bridge existing Python, JavaScript, Rust, C++, and other codebases
- **Asynchronous by Default**—Being an automation language, every operation is inherently async for efficient coordination.
- **Pipeline-Centric**—Compose workflows through chaining, parallelism, and branching
- **Right Tool for the Job**—Use the best language for each task in a workflow
- **Divide and Conquer**—Automation allows you to break complex tasks into language-specific smaller atomic tasks.
- **Minimalist Orchestration**—Coordinate with minimal overhead
- **Resource Governance**—Explicit resource management, queuing, and limits


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

[Q] |Q.DispatchIf.CPU.Available.MoreThan
[<] .threshold: pg\float = 75.0
[Q] |Q.ReplaceReTriggeredIf.QueueTime.MoreThan
[<] .timeout: pg\time = T"1::"

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
- **Unified Syntax**—Single language for multi-language workflows
- **Type-Safe Conversions**—Automatic conversion between language types
- **Event-Driven**—React to file changes, schedules, resource availability
- **Resource Management**—Built-in queuing, throttling, and limits
- **Production Ready**—Monitoring, error handling, graceful degradation

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
[Q] |Q.Priority
[<] .level: pg\uint = 5

[Q] |Q.DispatchIf.CPU.Available.MoreThan
[<] .threshold: pg\float = 80.0

[Q] |Q.DispatchIf.RAM.Available.MB.MoreThan
[<] .threshold: pg\float = 2048

[Q] |Q.KillIf.ExecutionTime.MoreThan
[<] .timeout: pg\time = T"30:"

[Q] |Q.KillIf.CPU.Usage.MoreThan
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

## Use Cases

- **Data Engineering**—ETL pipelines combining Python preprocessing with Rust analytics
- **DevOps**—CI/CD orchestration across multiple tools and languages
- **Machine Learning**—Train in Python, serve with Rust for performance
- **System Automation**—Scheduled maintenance across heterogeneous systems
- **Legacy Integration**—Bridge old C++ systems with modern services
- **IoT Processing**—Real-time sensor data processing with resource limits

## Documentation

### 📖 Language Specification (v0.0.2)
Complete reference for Polyglot syntax, semantics, and standard library:

- **[v0.0.2 Documentation](docs/v0.0.2/)** - Complete language specification
  - [Complete Syntax Reference](docs/v0.0.2/language/01-syntax-complete.md)
  - [Operators](docs/v0.0.2/language/05-operators.md) - `|`, `~`, `@`, `#`, `!`, `<<`, `>>`
  - [Block Markers](docs/v0.0.2/language/06-block-markers.md) - `[|]`, `[X]`, `[r]`, `[p]`, `[Q]`, etc.
  - [Type System](docs/v0.0.2/language/02-type-system.md)
  - [Error Handling](docs/v0.0.2/language/04-error-handling.md)
  - [BNF Grammar](docs/v0.0.2/language/12-bnf-grammar.md)
  - [Examples](docs/v0.0.2/examples/)

### 📋 Implementation Planning (BMAD Project)
Development roadmap and architecture decisions:

- **[Product Requirements (PRD)](docs/prd.md)** - Vision, requirements, success criteria
- **[Architecture](docs/architecture.md)** - Technical design, ADRs, patterns
- **[Epic Breakdown](docs/epics.md)** - Story sequencing and dependencies
- **[Development Stories](docs/stories/)** - Current implementation status

### 🔗 Understanding the Documentation
**Important:** Polyglot has two complementary documentation sets:

- **v0.0.2** defines the complete language (syntax, operators, type system)
- **BMAD docs** define what gets implemented and when (MVP scope, architecture)

👉 See **[v0.0.2 and BMAD Alignment](docs/v0.0.2-bmad-alignment.md)** to understand how they work together

### 📚 Quick Links by Role

**Writing `.pg` files?** → [v0.0.2 Language Specification](docs/v0.0.2/)

**Contributing to implementation?** → [Architecture](docs/architecture.md) + [Epic Breakdown](docs/epics.md)

**Understanding operators vs block markers?** → [Alignment Document](docs/v0.0.2-bmad-alignment.md)

## Installation (Future)

>To Be Determined

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
One of the aims is the ability to integrate code bases from different programming languages easily.
The polyglot service needs to be running in the background for it.
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

## Project Status

**Current Phase:** MVP Implementation (Epic 1 - Lexer & Parser)

**Recently Completed:**
- ✅ **Story 1.1:** Project workspace and build system setup complete
  - 9-crate Cargo workspace configured
  - CI/CD pipeline with test, clippy, fmt
  - All dependencies configured

**In Progress:**
- 🚧 **Story 1.2:** Lexer token definitions (drafted, ready for implementation)

**Implementation Roadmap:**
1. **Epic 1: Lexer & Parser** - Tokenization and AST generation *(in progress)*
2. **Epic 2: IR Generation** - Transform AST to 3-IR structure
3. **Epic 3: Database & Registry** - PostgreSQL schema and pipeline registry
4. **Epic 4: Trigger Monitor** - Event detection service
5. **Epic 5: Queue Manager** - Dispatch queue management
6. **Epic 6: Runner Service** - Pipeline execution engine
7. **Epic 7: Python Runtime Wrapper** - MVP proof-of-concept

**Methodology:** Using BMAD (BMad Methodology) for systematic development

See **[Epic Breakdown](docs/epics.md)** for detailed implementation timeline

## Development Setup

### Prerequisites

Before building Polyglot, ensure you have the following installed:

- **Rust Toolchain** (1.70+)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **PostgreSQL** (14+) - For pipeline and IR storage
  ```bash
  # Ubuntu/Debian
  sudo apt install postgresql postgresql-contrib

  # macOS
  brew install postgresql@14
  ```

- **Redis** (6+) - For queue management
  ```bash
  # Ubuntu/Debian
  sudo apt install redis-server

  # macOS
  brew install redis
  ```

- **InfluxDB** (2.x) - For metrics and monitoring (optional for core development)
  ```bash
  # See https://docs.influxdata.com/influxdb/v2/install/
  ```

### Building from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/polyglot-lang/polyglot.git
   cd polyglot
   ```

2. **Build the workspace**
   ```bash
   cargo build
   ```

   This builds all 9 crates:
   - **Libraries**: `polyglot-lexer`, `polyglot-parser`, `polyglot-ir`, `polyglot-db`, `polyglot-runtime-wrappers`
   - **Binaries**: `polyglot-cli`, `trigger-monitor`, `queue-manager`, `runner`

3. **Run tests**
   ```bash
   cargo test --workspace
   ```

4. **Check code quality**
   ```bash
   # Lint with Clippy
   cargo clippy --workspace --all-targets -- -D warnings

   # Format check
   cargo fmt --all -- --check
   ```

### Quick Start Development

```bash
# Build in debug mode (faster compilation)
cargo build

# Build in release mode (optimized)
cargo build --release

# Run the CLI
cargo run --bin polyglot -- --help

# Run specific tests
cargo test --package polyglot-lexer
cargo test --package polyglot-parser

# Watch mode (requires cargo-watch)
cargo install cargo-watch
cargo watch -x "test --workspace"
```

### Project Structure

```
polyglot/
├── polyglot-cli/              # CLI interface
├── polyglot-lexer/            # Tokenization
├── polyglot-parser/           # AST generation
├── polyglot-ir/               # IR generation (Trigger, Queue, Runner)
├── polyglot-db/               # Database operations
├── polyglot-runtime-wrappers/ # Python, Node, Rust execution
├── trigger-monitor/           # Trigger monitoring service
├── queue-manager/             # Queue management service
├── runner/                    # Pipeline execution service
└── Cargo.toml                 # Workspace configuration
```

### Running Services Locally

Start the required services for local development:

```bash
# Start PostgreSQL
sudo systemctl start postgresql

# Start Redis
sudo systemctl start redis

# (Optional) Start InfluxDB
sudo systemctl start influxdb
```

Then run the Polyglot services:

```bash
# Terminal 1: Trigger Monitor
cargo run --bin trigger-monitor

# Terminal 2: Queue Manager
cargo run --bin queue-manager

# Terminal 3: Runner
cargo run --bin runner

# Terminal 4: Use CLI
cargo run --bin polyglot -- run examples/hello.pg
```

## Getting Involved

We're looking for collaborators interested in:
- Language design and syntax refinement
- Compiler implementation
- Runtime system architecture
- Standard library development
- Documentation and examples
- Testing and tooling

**Contributing Guidelines:** Coming soon in `docs/contributing.md`

**Current Contribution Opportunities:**
- Lexer implementation (Story 1.2 - ready for development)
- Documentation improvements
- Example pipeline development
- Architecture review and feedback


## License

To be determined (likely Apache 2.0 or MIT)

---

**Status:** This is a design document. The language is not yet implemented. We're seeking feedback and collaborators to turn this vision into reality.