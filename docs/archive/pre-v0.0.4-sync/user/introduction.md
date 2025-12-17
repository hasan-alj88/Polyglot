---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/introduction.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Introduction to Polyglot

## The Problem

It's 2 AM. Your Python script is crashing—again. The Rust binary subprocess returned malformed JSON. You've spent three hours debugging FFI calls, marshaling data between languages, and fighting with `asyncio` when you should be solving actual business problems.

Sound familiar?

You need to:
- Fetch data with Python (because it has the best API libraries)
- Process it with Rust (because Python's too slow)
- Generate a PDF with Node.js (because you need that npm package)
- Email it with Go (because its networking is rock-solid)

But connecting them is a nightmare of:
- Subprocess management and pipe wrangling
- Manual JSON serialization between every language boundary
- Custom async coordination (`async/await`, promises, futures)
- Cron jobs you manually set up and monitor
- 200+ lines of glue code that's harder to maintain than the actual logic

**There must be a better way.**

---

## The Mental Shift Required

Polyglot is **not another scripting language**. If you're coming from Python, JavaScript, Rust, or Java, you need to **unlearn** traditional programming patterns:

| ❌ Traditional | ✅ Polyglot |
|----------------|-------------|
| Scripts you run | **Jobs you automate** |
| Variables have values now | **Variables transition through states** |
| Assignment (`=`) | **Push and pull via `>>`, `<<`** |
| `await` for async | **Automatic waiting via `>>`** |
| `if/else/for/while` | **Triggers, switches, unpack** |
| Call functions | **Register pipelines, triggers activate them** |

This isn't Python with different syntax. This isn't "async JavaScript without the pain." This is a **fundamentally different paradigm** designed for one thing: orchestrating multi-language automation workflows.

---

## What is Polyglot?

Polyglot is an **asynchronous orchestration language** designed for automation workflows that need to coordinate code across multiple programming languages. It's not a general-purpose programming language—it's a specialized tool for building pipelines, scheduled tasks, and event-driven systems.

**Three unique characteristics:**

1. **Multi-Language Integration:** Call Python, Rust, Go, Node.js, and Julia functions seamlessly
2. **Async-Centric Design:** All operations are asynchronous by default with automatic waiting
3. **Automation-First:** Built-in triggers, queues, and lifecycle management

**A Simple Example:**

```polyglot
[|Pipeline] .daily_report
[t] .trigger: #T.Daily(.hour: 9, .minute: 0
[i] .recipients: Array
[o] .sent: Boolean

// Python fetches data
.data << py\analytics.get_yesterday_stats(

// Rust processes it (faster than Python
.processed << rs\processor::compute_metrics(.data

// Node.js generates PDF (rich npm ecosystem
.pdf << js\report_generator.create(.processed

// Go sends email (excellent email libraries
.sent << go\mailer.send(.pdf, .recipients
```

This 14-line pipeline:
- Runs every day at 9 AM
- Fetches data with Python
- Processes with Rust (for performance
- Generates PDF with Node.js (for npm packages
- Sends email with Go (for robust networking
- Handles async operations automatically
- Manages errors and retries

In traditional code, this would require:
- Complex FFI or microservices architecture
- Manual async handling (promises, async/await
- Custom job scheduling system
- Error handling and retry logic
- Inter-process communication
- 200+ lines of glue code

## Automation-First Paradigm

Traditional scripting languages (Python, Bash, JavaScript were designed for general programming and adapted for automation. Polyglot is **designed for automation from the ground up**.

### Traditional Scripting Approach

```python
# Python cron job - manual everything
import schedule
import requests
import subprocess
import json

def daily_job(:
    try:
        # Fetch data
        response = requests.get("https://api.example.com/data"
        data = response.json(

        # Call Rust binary (awkward subprocess
        result = subprocess.run(
            ["./rust_processor"],
            input=json.dumps(data,
            capture_output=True
        
        processed = json.loads(result.stdout

        # Call Node script (another subprocess
        result = subprocess.run(
            ["node", "generate_pdf.js"],
            input=json.dumps(processed,
            capture_output=True
        

        # Error handling, logging, retries...
    except Exception as e:
        # Manual error handling
        log_error(e
        retry_with_backoff(

schedule.every(.day.at("09:00".do(daily_job

while True:
    schedule.run_pending(
    time.sleep(60
```

### Polyglot Approach

```polyglot
[|Pipeline] .daily_report
[t] .trigger: #T.Daily(.hour: 9, .minute: 0
[o] .result: String

.data << py\requests.get("https://api.example.com/data" >> json(
.processed << rs\processor::transform(.data
.pdf << js\generator.create(.processed
.result << go\mailer.send(.pdf
```

**What Polyglot Handles Automatically:**

- ✅ Scheduling (trigger system
- ✅ Multi-language calls (no subprocess juggling
- ✅ Async coordination (no manual promises
- ✅ Error propagation (state system
- ✅ Retries and queuing (built-in queue manager
- ✅ Service management (daemon/systemd integration

## Core Concepts

### 1. Pipelines

A **pipeline** is Polyglot's fundamental unit—a workflow that transforms inputs into outputs.

```polyglot
[|Pipeline] .example
[i] .input_data: String      // Input
[o] .output_result: Integer  // Output

// Pipeline body: transform input to output
.parsed << py\json.loads(.input_data
.count << .parsed >> length
.output_result << .count
```

Pipelines are:
- **Composable:** Pipelines call other pipelines
- **Reusable:** Define once, call many times
- **Stateless:** No hidden state between invocations

### 2. Triggers

**Triggers** are events that start pipeline execution automatically.

```polyglot
// Time-based trigger
[t] .daily: #T.Daily(.hour: 6, .minute: 30

// File system trigger
[t] .on_upload: #T.FileCreated(.path: "/uploads/*.csv"

// HTTP trigger
[t] .webhook: #T.HTTP(.port: 8080, .path: "/webhook"

// Message queue trigger
[t] .queue: #T.Queue(.topic: "data.processing"
```

### 3. State-Aware Variables

Variables in Polyglot have **states**, not just values:

```polyglot
.result: Integer  // Declared - exists but no value

.result << expensive_computation(  // Pending - computing

.value << .result + 10  // Ready - value available (waits if needed

// If computation fails:
// .result is Faulted - carries error information
```

States enable automatic async orchestration without explicit await keywords.

### 4. Three-Service Architecture

Polyglot runs as three coordinated services:

```
┌─────────────────┐
│ Trigger Monitor │ Watches for events (time, files, HTTP, etc.
└────────┬────────┘
         │ Enqueues pipeline execution
         ▼
┌─────────────────┐
│ Queue Manager   │ Manages execution queue, retries, priorities
└────────┬────────┘
         │ Dispatches to workers
         ▼
┌─────────────────┐
│ Runner Service  │ Executes pipeline code, manages workers
└─────────────────┘
```

**Why Three Services:**

- **Separation of Concerns:** Each service has one job
- **Resilience:** Services restart independently
- **Scalability:** Add more runners for parallel execution
- **Observability:** Monitor each service separately

**Learn More:** See [Polyglot Service Guide](polyglot-service.md) for installation, configuration, deployment (Docker/systemd), and troubleshooting.

## What Polyglot IS

✅ **An Orchestration Language**
- Coordinates existing code in multiple languages
- Defines workflows and data flow
- Manages async operations automatically

✅ **An Automation Platform**
- Built-in scheduling and event triggers
- Queue-based execution with retries
- Service lifecycle management

✅ **A Multi-Language Bridge**
- Seamless FFI without manual glue code
- Type-safe cross-language calls
- Unified error handling

✅ **An Async-First System**
- Everything is async by default
- Automatic parallelism where possible
- State-based synchronization

## What Polyglot IS NOT

❌ **Not a General-Purpose Language**
- Don't implement algorithms in Polyglot
- Use Python/Rust/Go for business logic
- Polyglot orchestrates, doesn't replace

❌ **Not a Replacement for Other Languages**
- Complements Python, Rust, Go, etc.
- Works *with* them, not instead of them
- Small orchestration layer, not application code

❌ **Not for Real-Time Systems**
- Queue-based execution has overhead
- Not suitable for <1ms latency requirements
- Use Rust/C++ for hard real-time

❌ **Not a UI Framework**
- No GUI primitives
- No web framework (but can call Express/Flask
- Focus on backend automation

❌ **Not for Simple Single-Language Scripts**
- If Python alone works, use Python
- Polyglot shines with multi-language workflows
- Don't add complexity unnecessarily

## Key Differences from Other Tools

### vs. Apache Airflow

| Aspect | Polyglot | Airflow |
|--------|----------|---------|
| **Language** | DSL with multi-language calls | Python DAGs |
| **Async Model** | Built-in state system | Manual task dependencies |
| **FFI** | Native multi-language | Subprocess/operators |
| **Syntax** | Declarative pipeline format | Python code |
| **Learning Curve** | New language to learn | Use existing Python skills |
| **Use Case** | General automation + ETL | Primarily data pipelines |

### vs. Bash/Shell Scripts

| Aspect | Polyglot | Bash |
|--------|----------|------|
| **Multi-Language** | First-class Python/Rust/Go/etc. | Subprocess only |
| **Type System** | Strongly typed with inference | Strings everywhere |
| **Async** | Automatic state-based waiting | Manual background jobs |
| **Error Handling** | Built-in error propagation | Exit codes and `$?` |
| **Scheduling** | Built-in triggers | External cron |

### vs. Microservices

| Aspect | Polyglot | Microservices |
|--------|----------|---------------|
| **Deployment** | Single service with workers | Many independent services |
| **Communication** | In-process FFI | HTTP/gRPC/message queues |
| **Overhead** | Low (function calls | High (network calls |
| **Complexity** | Centralized orchestration | Distributed coordination |
| **Observability** | Single service to monitor | Many services to monitor |

### vs. Workflow Engines (Temporal, Cadence

| Aspect | Polyglot | Temporal |
|--------|----------|----------|
| **Language** | Multi-language DSL | Go/Java/Python SDKs |
| **State Management** | Variable state system | Workflow history |
| **Syntax** | Declarative pipelines | Imperative code |
| **Use Case** | Automation + orchestration | Long-running workflows |
| **Complexity** | Simpler model | More powerful but complex |

## When to Use Polyglot

### You Need Polyglot If You're Fighting:

**🔥 "Subprocess Hell"**
- Spawning Python scripts from Go, piping output through temp files
- Manually serializing/deserializing JSON at every language boundary
- Fighting encoding issues between language ecosystems
- **→ Polyglot gives you:** Native FFI calls that "just work"

**🔥 "Async Coordination Nightmare"**
- Wrestling with `asyncio`, promises, futures across different languages
- Manually tracking which operations finished, which are pending
- Writing custom queue/retry logic for failed async operations
- **→ Polyglot gives you:** Automatic async coordination via state system

**🔥 "Cron Job Chaos"**
- Dozens of cron entries scattered across servers
- No visibility into which jobs ran, failed, or are running now
- Manual error handling and alerting for each job
- **→ Polyglot gives you:** Built-in trigger system with centralized monitoring

**🔥 "The 200-Line Glue Code Problem"**
- More code coordinating languages than actual business logic
- Fragile integration that breaks when any dependency updates
- Can't test the glue code without spinning up all services
- **→ Polyglot gives you:** Declarative orchestration that's testable and maintainable

**🔥 "ETL Pipeline Maintenance Hell"**
- Airflow DAGs that are 500+ lines of Python
- Can't use the best tool for each step (stuck in Python land)
- Manual state management and error recovery
- **→ Polyglot gives you:** Clean pipelines that use the right language per step

### Don't Use Polyglot If You're:

**✋ Writing pure algorithms or business logic**
→ Use Python/Rust/Go directly. Polyglot orchestrates, it doesn't implement.

**✋ Building a web application**
→ Use Flask/Express/Actix. Polyglot can *call* your API, but it's not a web framework.

**✋ Solving a problem with a single language**
→ If Python alone works, use Python. Don't add complexity unnecessarily.

**✋ Building real-time systems (<1ms latency)**
→ Use Rust/C++. Polyglot's queue-based execution has overhead.

**✋ Creating user interfaces**
→ Use React/Flutter/etc. Polyglot has no GUI primitives.

## Architecture Overview

### Compile-Time

```
.pg source file
      ↓
  [Compiler]
      ↓
SQLite database (IR + metadata
```

### Runtime

```
Trigger fires → Queue Manager → Runner executes → Results stored
                      ↑
                  [Retries]
```

### CLI Workflow

```bash
# 1. Compile .pg file to database
polyglot compile workflow.pg

# 2. Register pipeline with service
polyglot register workflow.pg my_pipeline

# 3. Activate triggers
polyglot activate my_pipeline

# 4. Pipeline runs automatically or on-demand
polyglot run my_pipeline --input '{"data": "value"'
```

## Learning Path

1. **Start Here:**
   - [Core Philosophy](core-philosophy.md - Understand the "why"
   - [Getting Started](getting-started.md - Build your first pipeline

2. **Master Async:**
   - [Async-Centric Language](async-centric-language.md - Mental model shift
   - [Variable State System](variable-state-system.md - Deep dive

3. **Learn Syntax:**
   - [syntax/overview.md](syntax/overview.md - Complete syntax guide
   - [syntax/operators.md](syntax/operators.md - All operators explained

4. **Advanced Topics:**
   - [advanced/parallel-execution.md](advanced/parallel-execution.md - Parallelism
   - [advanced/datetime-system.md](advanced/datetime-system.md - Multi-calendar support

5. **Real Examples:**
   - [examples/cross-language-integration.md](examples/cross-language-integration.md
   - [examples/automation-workflows.md](examples/automation-workflows.md

## Quick Example: Why Polyglot?

**Task:** Every hour, fetch data from an API, process it with a Rust library (for speed, generate a chart with Python (for matplotlib, and send it via Slack.

**Without Polyglot (Python + subprocess hell:**

```python
import schedule
import subprocess
import requests
import json
import tempfile

def hourly_job(:
    # Fetch data
    data = requests.get("https://api.example.com/metrics".json(

    # Call Rust binary (awkward!
    with tempfile.NamedTemporaryFile(mode='w' as f:
        json.dump(data, f
        f.flush(
        result = subprocess.run(['./rust_processor', f.name], capture_output=True

    processed = json.loads(result.stdout

    # Generate chart
    import matplotlib.pyplot as plt
    plt.plot(processed
    plt.savefig('chart.png'

    # Send to Slack (another subprocess for Node CLI tool
    subprocess.run(['slack-cli', 'upload', 'chart.png']

schedule.every(.hour.do(hourly_job
while True:
    schedule.run_pending(
    time.sleep(60
```

**With Polyglot:**

```polyglot
[|Pipeline] .hourly_metrics
[t] .trigger: #T.Hourly(.minute: 0
[o] .sent: Boolean

.data << py\requests.get("https://api.example.com/metrics" >> json(
.processed << rs\processor::transform(.data
.chart << py\matplotlib.plot_and_save(.processed
.sent << js\slack.upload(.chart
```

**Difference:**
- 35 lines → 8 lines
- Manual scheduling → Built-in trigger
- Subprocess hell → Natural function calls
- Manual error handling → Automatic error propagation
- No async coordination code → State system handles it

## Next Steps

Ready to start? Head to [Getting Started](getting-started.md to install Polyglot and create your first pipeline.

Want to understand the async model first? Read [Async-Centric Language](async-centric-language.md to shift your mental model.

Curious about the philosophy? Dive into [Core Philosophy](core-philosophy.md to understand design decisions.
