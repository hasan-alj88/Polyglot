---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/core-philosophy.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Core Philosophy

Polyglot is built on three fundamental principles that distinguish it from traditional programming languages. Understanding these principles is essential to using Polyglot effectively.

## The Three Pillars

### 1. Don't Reinvent the Wheel

**Use existing tools, don't re-implement them.**

Polyglot is an orchestration language, not a general-purpose programming language. When you need AI modeling and Data science use Python. When you need high-performance data processing, use Rust. When you need web scraping, use Python's BeautifulSoup.

**Good Polyglot:**
```polyglot
[r] py\matrix_multiply
[i] .matrix_a: py\ndarray
[i] .matrix_b: py\ndarray
[o] .result: py\ndarray
[<] <library: "numpy"

[|Pipeline] .multiply
[i] .a: py\ndarray
[i] .b: py\ndarray
[o] .product: py\ndarray

.product << py\numpy.matmul(.a, .b
```

**Bad Polyglot:**
```polyglot
// DON'T do this - implementing matrix multiplication in Polyglot
[|] |multiply
[i] .a:pg.array{pg\serial
[i] .a:pg.array{pg\serial
...
[o] .product:pg.array{pg\serial

// Hundreds of lines of matrix multiplication logic...
// This defeats the purpose of Polyglot!
```

**Why This Matters:**

- **Leverage Expertise:** NumPy's matrix multiplication is optimized by experts over decades
- **Reduce Complexity:** Your Polyglot code stays simple and focused on orchestration
- **Maintain Quality:** Use battle-tested libraries instead of re-implementing algorithms
- **Faster Development:** Call existing code instead of writing from scratch

### 2. Right Tool for the Right Job

**Use each language for what it does best.**

Different languages excel at different tasks. Polyglot lets you combine language strengths seamlessly abstracting away all the integration and FFI (Foreign Function Interface glue code.

**Language Strengths:**

| Language | Best For | Examples |
|----------|----------|----------|
| Python | Data science, ML, scripting | NumPy, Pandas, TensorFlow |
| Rust | Performance, safety, systems | Parallel processing, crypto |
| Go | Networking, APIs, concurrency | HTTP servers, microservices |
| Node.js | Web, async I/O, JavaScript ecosystem | Express, React rendering |
| Julia | Scientific computing, numerical analysis | Differential equations, simulations |

**Real-World Example:**

```polyglot
// ETL Pipeline: Each language does what it does best

[|] |ETLWorkflow
[i] .source_url:pg.string
[i] .db_table:pg.string <~ "data_table"
[t] |T.Call
[o] .report_path:pg.path

// Multiple wrappers execute in order: Python → Rust → Database → Node.js
// Setup: Python env → Rust env → DB connection → Node env
// Cleanup happens in REVERSE order after execution completes

[W] |W.RT.Python3.14
[<] <requirements:pg.file << \\FileDir\\python\\requirements.txt

[W] |W.RT.Rust1.8

[W] |W.DB.Postgresql
[>] >session:pg.db >> .db

[W] |W.RT.Nodejs
[<] <package:pg.file << \\FileDir\\nodejs\\package.json

// Extract: Python for web scraping
[r] |U.HTTP.Get
[<] <url:pg.url << .source_url
[>] >response:pg.string >> .raw_response

[r] |RT.Python.Run.Function
[<] <function:pg.string << "beautifulsoup.parse"
[<] <args:pg.serial << {.raw_response
[>] >output:pg.serial >> .parse_result

[r] .parsed:pg.string << .parse_result.0

// Transform: Rust for high-performance processing
[r] |RT.Rust.Run.File
[<] <file:pg.path << \\FileDir\\rust\\process.rs
[<] <input:pg.serial << {.parsed
[>] >output:pg.serial >> .rust_output

[r] .cleaned:pg.string << .rust_output.0
[r] .aggregated:pg.string << .rust_output.1

// Load: PostgreSQL database bulk insert
[r] |DB.BulkInsert
[<] <session:pg.db << .db
[<] <table:pg.string << .db_table
[<] <data:pg.array.pg.serial << {.cleaned, .aggregated
[>] >result:pg.array.pg.serial >> .saved

// Report: Node.js for PDF generation
[r] |RT.Nodejs.Run.File
[<] <file:pg.path << \\FileDir\\nodejs\\pdfgen.js
[<] <input:pg.serial << {.aggregated
[>] >output:pg.serial >> .pdf_result

[r] .report_path:pg.path << .pdf_result.file_path

[o] .report_path:pg.path
[X]
```

**Why This Matters:**

- **Optimal Performance:** Use Rust for CPU-intensive tasks, Go for I/O-bound tasks
- **Rich Ecosystems:** Access Python's ML libraries, Node's web frameworks
- **Team Skills:** Let Python experts write Python, Rust experts write Rust
- **Future-Proof:** Swap implementations without rewriting orchestration

**When to Use What:**

- **Python:** Data analysis, machine learning, quick prototypes, glue scripts
- **Rust:** Performance-critical code, parallel processing, unsafe operations (safely
- **Go:** Web servers, REST APIs, concurrent tasks, system tools
- **Node.js:** Real-time apps, JavaScript integration, npm packages
- **Julia:** Heavy numerical computation, scientific simulations

### 3. Async-Centric by Design

**Variables have states, not just values. The system waits automatically.**

This is the most radical departure from traditional programming. In most languages, you write synchronous code and add async manually (promises, async/await, callbacks. In Polyglot, **everything is async by default**.

**Traditional Synchronous Model:**

```python
# Python: Explicit async/await
async def process_data(url:
    response = await fetch(url      # Wait here
    data = await parse(response     # Wait here
    result = await transform(data   # Wait here
    return result
```

**Polyglot Async Model:**

```polyglot
// No await keyword - automatic waiting
[|Pipeline] .process_data
[i] .url: String
[o] .result: py\DataFrame

.response << py\fetch(.url        // PUSH makes Ready when done
.data << py\parse(.response       // PULL waits if Pending
.result << py\transform(.data     // Chain naturally
```

**Variable States:**

Every variable transitions through states:

1. **Declared:** Variable exists but has no value yet
2. **Pending:** Computation started, result not ready
3. **Ready:** Value available, can be used
4. **Faulted:** Error occurred, carries error information

**The Magic:**

- **PUSH `<<`**: Starts computation, variable becomes Pending, then Ready
- **PULL `>>`**: Waits automatically if variable is Pending
- **No await keyword**: The language handles synchronization
- **Error propagation**: Faulted variables propagate automatically

**Example: Automatic Waiting**

```polyglot
[|Pipeline] .fetch_and_process
[i] .url: String
[o] .summary: String

// Step 1: Start fetch (becomes Pending immediately
.data << py\requests.get(.url

// Step 2: Start parse (waits for .data automatically
.parsed << py\json.loads(.data >> body

// Step 3: Process (waits for .parsed automatically
.summary << rs\analyzer::summarize(.parsed

// No await, no callbacks, no promises - just natural flow
```

**Why This Matters:**

- **Simpler Mental Model:** Write code that looks synchronous, runs asynchronous
- **Automatic Parallelism:** Independent operations run in parallel automatically
- **Error Safety:** Faulted state prevents using invalid data
- **Orchestration Focus:** You focus on *what* to compute, not *how* to wait

## Orchestration vs. Implementation

Polyglot is an **orchestration language**, not an **implementation language**.

**What This Means:**

| Concept | Orchestration (Polyglot | Implementation (Python/Rust/etc. |
|---------|--------------------------|-----------------------------------|
| Purpose | Coordinate existing code | Write new algorithms |
| Focus | Workflow and data flow | Business logic and algorithms |
| Typical Size | 10-200 lines | 100-10,000+ lines |
| Complexity | High-level, declarative | Low-level, imperative |
| Async | Built-in, automatic | Manual (promises, threads |

**Think of Polyglot as:**

- **Conductor:** Coordinates an orchestra, doesn't play every instrument
- **Recipe:** Combines ingredients (functions, doesn't grow them
- **Workflow Engine:** Chains steps together, doesn't implement steps
- **Glue Code:** Connects components, doesn't replace components

## Good and Bad Use Cases

### ✅ Excellent Use Cases

**1. Multi-Language Workflows**
```polyglot
// Combine Python ML with Rust performance
[|Pipeline] .ml_pipeline
.training_data << py\pandas.read_csv(.csv_path
.model << py\sklearn.train(.training_data
.optimized << rs\optimizer::quantize(.model
.deployed << go\server.Deploy(.optimized
```

**2. Scheduled Automation**
```polyglot
// Daily report generation
[|Pipeline] .daily_report
[t] .trigger: #T.Daily(.hour: 9, .minute: 0

.data << py\analytics.fetch_yesterday(
.charts << js\chartjs.render(.data
.pdf << go\pdf.generate(.charts
.sent << py\email.send(.pdf, .recipients
```

**3. Event-Driven Processing**
```polyglot
// File monitoring pipeline
[|Pipeline] .process_uploads
[t] .trigger: #T.FileCreated(.path: "/uploads/*"

.file << .trigger >> file_path
.validated << py\validator.check(.file
.processed << rs\processor::transform(.validated
.stored << go\s3.upload(.processed
```

**4. Complex ETL**
```polyglot
// Multi-source data integration
[|Pipeline] .etl
.api_data << py\requests.get(.api_url
.db_data << go\postgres.query(.sql
.file_data << rs\csv::read(.file_path
.merged << py\pandas.merge_all([.api_data, .db_data, .file_data]
.transformed << rs\transformer::process(.merged
.loaded << go\warehouse.bulk_insert(.transformed
```

**5. Cross-Service Orchestration**
```polyglot
// Microservices coordination
[|Pipeline] .order_fulfillment
.order << go\orders.create(.customer_id, .items
.payment << js\stripe.charge(.order >> total
.inventory << rs\warehouse::reserve(.items
.shipping << py\fedex.create_label(.order >> address
.notification << go\email.send_confirmation(.order
```

### ❌ Poor Use Cases

**1. Pure Algorithms**
```polyglot
// DON'T implement sorting in Polyglot
[|Pipeline] .quicksort
// ... recursive partitioning logic ...
// Use Python/Rust/Go for this instead!
```

**2. UI Applications**
```polyglot
// DON'T build UIs in Polyglot
[|Pipeline] .render_button
// Polyglot has no GUI primitives
// Use React, Flutter, etc.
```

**3. Real-Time Games**
```polyglot
// DON'T write game loops in Polyglot
[|Pipeline] .game_loop
// Too much overhead for 60 FPS
// Use Unity, Godot, Bevy, etc.
```

**4. Low-Level Systems**
```polyglot
// DON'T write device drivers in Polyglot
[|Pipeline] .usb_driver
// Use C, Rust, etc. for hardware access
```

**5. Simple Single-Language Scripts**
```polyglot
// DON'T use Polyglot for simple Python scripts
[|Pipeline] .hello_world
.message << py\print("Hello, World!"
// Just use Python directly!
```

## Decision Framework

**Use Polyglot When:**

- ✅ You need to combine multiple languages
- ✅ You have async operations (APIs, databases, file I/O
- ✅ You're building workflows, not algorithms
- ✅ You need scheduled or event-driven execution
- ✅ You want automatic parallelism and error handling

**Don't Use Polyglot When:**

- ❌ You're writing pure algorithms or business logic
- ❌ You're building user interfaces
- ❌ You need sub-millisecond latency
- ❌ You're doing low-level systems programming
- ❌ A single language already solves your problem well

## Philosophy in Practice

**Principle 1 in Action:** Use NumPy for matrices, not custom loops
**Principle 2 in Action:** Python for data prep, Rust for processing, Go for serving
**Principle 3 in Action:** Write natural pipelines, let the system handle async

**The Polyglot Way:**

> "Orchestrate existing tools asynchronously, using the right language for each task."

This philosophy makes Polyglot powerful for automation and integration while keeping it simple and focused. You're not replacing Python, Rust, or Go—you're conducting them to work together harmoniously.

## Next Steps

- Read [Introduction](introduction.md for what Polyglot is and isn't
- Understand [Async-Centric Language](async-centric-language.md for the mental model shift
- Explore [Variable State System](variable-state-system.md for deep async understanding
- Start with [Getting Started](getting-started.md to build your first pipeline
