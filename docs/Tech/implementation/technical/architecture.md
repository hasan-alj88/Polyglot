# Polyglot Architecture

**Last Updated:** 2025-12-17
**Version:** 2.0 (Hybrid Pattern Architecture)
**Project:** Polyglot - Asynchronous Automation Language
**Architecture Type:** Hybrid (Event-Driven Queue + Compiler Pipeline + Plugin System)

---

## Executive Summary

Polyglot is implemented as a **hybrid architecture** combining three proven patterns to solve the cross-language FFI integration problem through runtime type resolution. The system consists of 9 Rust crates (3 backend services, 6 libraries) that communicate via PostgreSQL and Redis.

**Core Innovation:** Runtime vs. compile-time type crossing enables dynamic→static language conversions that traditional FFI tools cannot achieve.

**Architectural Patterns:**
1. **Event-Driven Job Queue** - Backend services coordination (Trigger Monitor, Queue Manager, Runner)
2. **Compiler Pipeline** - Front-end language processing (Lexer → Parser → 3-IR)
3. **Plugin System** - Multi-language runtime wrappers (Python, Node, Rust, Go)

**Technology Foundation:**
- **Language:** Rust 2021 Edition
- **Async Runtime:** Tokio 1.41
- **Databases:** PostgreSQL (state/IR), Redis (queues), InfluxDB (time-series)
- **Paradigm:** Async-first, database-driven service communication

---

## Table of Contents

1. [Hybrid Architecture Pattern](#hybrid-architecture-pattern)
2. [System Architecture](#system-architecture)
3. [Technology Stack](#technology-stack)
4. [Project Structure](#project-structure)
5. [Novel Architectural Patterns](#novel-architectural-patterns)
6. [Implementation Patterns](#implementation-patterns)
7. [Data Architecture](#data-architecture)
8. [Cross-Cutting Concerns](#cross-cutting-concerns)
9. [Development Environment](#development-environment)
10. [Architecture Decision Records (ADRs)](#architecture-decision-records-adrs)

---

## Hybrid Architecture Pattern

### Pattern Integration Overview

Polyglot's architecture combines three industry-proven patterns into a novel hybrid that enables runtime cross-language type resolution:

```
┌─────────────────────────────────────────────────────────────┐
│                    POLYGLOT HYBRID ARCHITECTURE              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │  PATTERN 1: Compiler Pipeline (Front-End)          │     │
│  │  Source .pg → Lexer → Parser → AST → 3-IR → DB    │     │
│  └────────────────────────────────────────────────────┘     │
│                          ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │  PATTERN 2: Event-Driven Job Queue (Backend)       │     │
│  │  Trigger Monitor → Queue Manager → Runner          │     │
│  │  (PostgreSQL + Redis coordination)                 │     │
│  └────────────────────────────────────────────────────┘     │
│                          ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │  PATTERN 3: Plugin System (Execution)              │     │
│  │  RuntimeWrapper Trait → Python/Node/Rust/Go        │     │
│  │  (Multi-language runtime abstraction)              │     │
│  └────────────────────────────────────────────────────┘     │
│                          ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │  INNOVATION: Runtime Type Resolution Layer         │     │
│  │  Dual-Strategy Type Conversion (Serialization +    │     │
│  │  Direct FFI) enables dynamic→static crossing       │     │
│  └────────────────────────────────────────────────────┘     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Pattern 1: Compiler Pipeline (Front-End)

**Borrowed From:** rustc, LLVM, traditional compilers

**Applied To:** `.pg` file compilation

**Phases:**
1. **Lexing** - Source text → Token stream (logos-based, <100ms for 1000 lines)
2. **Parsing** - Tokens → Abstract Syntax Tree (recursive descent parser)
3. **IR Generation** - AST → 3 separate IRs (Trigger, Queue, Runner)
4. **Validation** - Type checking, pipeline reference resolution
5. **Storage** - IR → PostgreSQL JSONB columns

**Why This Pattern:**
- Proven for language implementation
- Clear phase separation (single responsibility)
- Type-safe intermediate representations
- Optimizable at each stage

**Polyglot Adaptation:**
- **3-IR Structure** (ADR-008) instead of single IR - service isolation
- **Database storage** instead of file output - enables service communication
- **No code generation** - IR is directly executed by Runner

### Pattern 2: Event-Driven Job Queue (Backend)

**Borrowed From:** Fang, RSMQ, Celery, background job processors

**Applied To:** Pipeline orchestration

**Components:**
1. **Trigger Monitor** - Continuous trigger condition monitoring (synchronous loop)
2. **Queue Manager** - Redis-based dispatch queue with PostgreSQL fallback
3. **Runner** - Pipeline instance execution with runtime wrapper invocation

**Flow:**
```
Trigger Monitor (detects condition)
    ↓ Creates instance in PostgreSQL
Queue Manager (polls new instances)
    ↓ Pushes to Redis queue
Runner (pops from queue)
    ↓ Executes via runtime wrapper
    ↓ Updates instance state in PostgreSQL
```

**Why This Pattern:**
- Proven for async background processing at scale
- Natural fit for automation workflows
- Redis provides fast queue operations
- PostgreSQL provides durable state

**Polyglot Adaptation:**
- **Database-driven communication** (ADR-006) instead of HTTP/RPC
- **Dynamic trigger loading** (ADR-005) with LISTEN/NOTIFY
- **PostgreSQL fallback** (ADR-012) for high availability
- **3-IR separation** - each service reads only its relevant IR

### Pattern 3: Plugin System (Execution)

**Borrowed From:** UniFFI, WebAssembly runtimes, C ABI FFI plugins

**Applied To:** Multi-language runtime integration

**Interface:**
```rust
#[async_trait]
pub trait RuntimeWrapper: Send + Sync {
    /// Initialize runtime environment (virtualenv, package installation)
    async fn setup(&self) -> Result<()>;

    /// Execute code with type conversion
    async fn execute(
        &self,
        code: &str,
        inputs: HashMap<String, Value>
    ) -> Result<HashMap<String, Value>>;

    /// Cleanup runtime resources
    async fn close(&self) -> Result<()>;
}
```

**Implementations:**
- **Python** (MVP) - uv-based virtualenv, JSON serialization
- **Node.js** (post-MVP) - npm-based environment
- **Rust** (post-MVP) - Dynamic library loading
- **Go** (post-MVP) - CGo bridge

**Why This Pattern:**
- Proven for multi-language interop
- Plugin architecture enables ecosystem growth
- Type conversion abstraction layer
- Error isolation across language boundaries

**Polyglot Adaptation:**
- **Dual-strategy type conversion** (ADR-016) - serialization + direct FFI
- **Runtime type resolution** - inspect types at execution time (innovation)
- **Lifecycle management** - setup() → execute() → close() for resource control

### Innovation Layer: Runtime Type Resolution

**The Breakthrough:**

Traditional FFI tools (PyO3, pybind11) require compile-time type knowledge:
```
Python (dynamic) → Manual Type Annotations → C++/Rust (static) → Compile
```

Polyglot enables **runtime type resolution**:
```
Python (dynamic) → Polyglot Runtime Inspection → Type Detection → Rust (static)
```

**Example:**
```python
# Python passes list of unknown size at compile-time
def get_data():
    return [1, 2, 3]  # Size determined at runtime
```

**Traditional FFI:**
- ❌ Fails: Cannot convert `list` (runtime size) → `[i32; N]` (compile-time size)
- Requires: Manual annotation `#[pyfunction] fn process(data: Vec<i32>)`

**Polyglot Runtime Resolution:**
```rust
// Runner inspects value at execution time
let value = wrapper.execute("get_data", inputs).await?;
// Runtime: Detects list with 3 elements
// Converts: Python list[1,2,3] → Rust Vec<i32> or [i32; 3]
```

**Dual-Strategy Type Conversion (ADR-016):**

**Strategy 1: Universal Translator (Serialization)**
```
Source Type → JSON → Polyglot IR → JSON → Target Type
Python str → "\"hello\"" → pg\string → "\"hello\"" → Rust String
```
- Works for any language pair
- 5-10ms overhead (acceptable for automation)
- Guaranteed to succeed (fallback)

**Strategy 2: Direct Conversion (Optimized FFI)**
```
Source Type → Existing FFI Bridge → Target Type
Python str → PyO3 → Rust String (when available)
```
- Leverages existing FFI tools when installed
- Near-zero overhead
- Requires FFI tool availability

**Automatic Selection:**
```rust
async fn convert_type(value: Value, target: Type) -> Result<Value> {
    // Check if direct FFI bridge available
    if let Some(bridge) = detect_ffi_bridge(&value.lang, &target.lang) {
        bridge.convert(value, target).await  // Strategy 2
    } else {
        serialize_via_json(value, target).await  // Strategy 1 (fallback)
    }
}
```

**Why This Matters:**
- Unlocks type conversions that compile-time tools cannot achieve
- No manual type annotations required
- Best-of-both-worlds: flexibility + performance

---

## System Architecture

### Service Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         USER LAYER                           │
├─────────────────────────────────────────────────────────────┤
│  polyglot CLI                                                │
│  ├── compile <file>.pg   → Lexer → Parser → IR → DB         │
│  ├── activate <name>     → Update DB: activated=true        │
│  ├── trigger <name>      → Insert instance (manual trigger) │
│  ├── status <name>       → Query instance state             │
│  └── logs <instance-id>  → Query execution logs             │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                    DATABASE LAYER                            │
├─────────────────────────────────────────────────────────────┤
│  PostgreSQL                                                  │
│  ├── pipelines (trigger_ir, queue_ir, runner_ir JSONB)      │
│  ├── pipeline_instances (status, trigger_data)              │
│  ├── triggers (type, config, enabled)                       │
│  └── execution_logs (timestamp, message, level)             │
│                                                              │
│  Redis                                                       │
│  ├── dispatch_queue (pending instances)                     │
│  └── pause_queue (paused instances)                         │
│                                                              │
│  InfluxDB                                                    │
│  ├── trigger_schedules (time-based triggers)                │
│  └── resource_metrics (CPU, RAM, GPU, Network)              │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                    SERVICE LAYER                             │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Trigger Monitor (Continuous Synchronous Loop)         │  │
│  │ - Loads active pipelines via PostgreSQL LISTEN/NOTIFY│  │
│  │ - Monitors trigger conditions (time, resource, etc.)  │  │
│  │ - Creates pipeline instances in PostgreSQL            │  │
│  │ - Subservice: Resource Monitor → InfluxDB metrics    │  │
│  └───────────────────────────────────────────────────────┘  │
│                           ↓                                  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Queue Manager (Async Tokio Service)                   │  │
│  │ - Polls PostgreSQL for new instances (status=Created) │  │
│  │ - Pushes to Redis dispatch queue                      │  │
│  │ - Fallback: PostgreSQL polling if Redis unavailable   │  │
│  │ - Manages priority (single queue MVP, multi post-MVP) │  │
│  └───────────────────────────────────────────────────────┘  │
│                           ↓                                  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Runner (Async Tokio Service)                          │  │
│  │ - Pops instances from Redis queue                     │  │
│  │ - Loads runner_ir from PostgreSQL                     │  │
│  │ - Invokes runtime wrappers (Python, Node, Rust, Go)   │  │
│  │ - Updates instance state: Running → Exited/Failed     │  │
│  │ - Writes execution logs to PostgreSQL                 │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                  RUNTIME WRAPPER LAYER                       │
├─────────────────────────────────────────────────────────────┤
│  RuntimeWrapper Trait (async, Send + Sync)                  │
│  ├── Python3.11 Wrapper (MVP)                               │
│  │   ├── uv virtualenv management                           │
│  │   ├── JSON serialization (Strategy 1)                    │
│  │   └── PyO3 direct conversion when available (Strategy 2) │
│  ├── Node.js Wrapper (post-MVP)                             │
│  ├── Rust Wrapper (post-MVP)                                │
│  └── Go Wrapper (post-MVP)                                  │
└─────────────────────────────────────────────────────────────┘
```

### Compilation Flow

```
User: polyglot compile example.pg
    ↓
┌─────────────────────────────────────────────────────────┐
│ CLI (polyglot-cli crate)                                │
│ - Read .pg file                                         │
│ - Invoke compilation pipeline                           │
└─────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────┐
│ Lexer (polyglot-lexer crate)                            │
│ - Source text → Token stream                            │
│ - logos-based DFA (compile-time generation)             │
│ - <100ms for 1000-line files                            │
└─────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────┐
│ Parser (polyglot-parser crate)                          │
│ - Token stream → Abstract Syntax Tree (AST)             │
│ - Recursive descent parser                              │
│ - 3-phase resolution: current file → package → registry │
└─────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────┐
│ IR Generator (polyglot-ir crate)                        │
│ - AST → Trigger IR (when to execute)                    │
│ - AST → Queue IR (routing & priority)                   │
│ - AST → Runner IR (execution steps & runtimes)          │
│ - Validation: types, pipeline refs, trigger configs     │
└─────────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────────┐
│ Database Registry (polyglot-db crate)                   │
│ - Insert into pipelines table                           │
│ - 3 JSONB columns: trigger_ir, queue_ir, runner_ir      │
│ - activated=false (requires explicit activation)        │
│ - PostgreSQL NOTIFY triggers Trigger Monitor reload     │
└─────────────────────────────────────────────────────────┘
    ↓
User: polyglot activate example
    ↓ (Update activated=true)
    ↓ (NOTIFY triggers Trigger Monitor)
    ↓
Trigger Monitor loads new trigger configuration
```

### Execution Flow

```
Trigger Monitor (continuous loop)
    ↓ Detects: Time trigger fires (cron schedule matched)
    ↓ Loads: trigger_ir from PostgreSQL
    ↓ Creates: pipeline_instance (status=Created, trigger_data=timestamp)
    ↓
Queue Manager (polls PostgreSQL)
    ↓ Query: SELECT * FROM pipeline_instances WHERE status='Created'
    ↓ Loads: queue_ir (timing_logic, queue_selector, priority)
    ↓ Pushes: instance_id to Redis dispatch queue
    ↓ Updates: status='Queued'
    ↓
Runner (pops Redis queue)
    ↓ Pop: instance_id from Redis
    ↓ Loads: runner_ir from PostgreSQL (execution_mode, steps)
    ↓ Updates: status='Running', started_at=now()
    ↓
    ↓ For each step in runner_ir.steps:
    ↓     1. Identify runtime (e.g., "Python3.11")
    ↓     2. Get wrapper: runtime_wrappers.get("Python3.11")
    ↓     3. Setup: wrapper.setup().await (if not cached)
    ↓     4. Execute: wrapper.execute(code, inputs).await
    ↓         ├── Type Conversion (ADR-016):
    ↓         │   ├── Try Strategy 2 (Direct FFI) if available
    ↓         │   └── Fallback Strategy 1 (JSON serialization)
    ↓         ├── Error Handling: Capture Python/Node/Rust errors
    ↓         └── Return: outputs (typed values)
    ↓     5. Next step receives previous outputs as inputs
    ↓
    ↓ Updates: status='Exited', completed_at=now()
    ↓ Logs: execution_logs table (structured tracing)
```

---

## Technology Stack

### Core Technologies

| Category | Technology | Version | Purpose |
|----------|-----------|---------|---------|
| **Language** | Rust | 2021 Edition | Implementation language (all crates) |
| **Async Runtime** | Tokio | 1.41 | Async I/O, timers, spawning tasks |
| **CLI Framework** | clap | 4.5 | Command-line argument parsing (derive API) |
| **Lexer Generator** | logos | 0.14 | Declarative token definitions, DFA generation |

### Database Stack

| Technology | Version | Purpose | Features |
|-----------|---------|---------|----------|
| **PostgreSQL** | 14+ | Primary database | IR storage (JSONB), metadata, instances, triggers |
| **SQLx** | 0.8.6 | Async PG client | Compile-time query verification, connection pooling |
| **sqlx-cli** | (bundled) | Migrations | Version-controlled schema migrations |
| **Redis** | 7+ | Queue system | Dispatch queue, pause queue, fallback to PostgreSQL |
| **redis crate** | 0.32.7 | Async Redis client | tokio-comp feature for async support |
| **InfluxDB** | 2.x | Time-series | Trigger schedules, resource metrics |

### Serialization & Data

| Technology | Version | Purpose |
|-----------|---------|---------|
| **serde** | 1.0 | Rust struct serialization (derive macros) |
| **serde_json** | 1.0.140 | JSON IR format, type conversion Strategy 1 |

### Error Handling

| Technology | Version | Purpose | Used In |
|-----------|---------|---------|---------|
| **thiserror** | 2.0.17 | Structured error types | Library crates |
| **anyhow** | 1.0.99 | Simple error propagation | Binary crates (services, CLI) |

**Pattern (ADR-004):**
- Libraries define custom errors with `#[derive(Error)]`
- Binaries use `anyhow::Result<T>` and `.context()`
- All errors: `Send + Sync` for async compatibility

### Logging & Observability

| Technology | Version | Purpose |
|-----------|---------|---------|
| **tracing** | 0.1.41 | Async-native structured logging |
| **tracing-subscriber** | 0.3.19 | Log collection, formatting, JSON output |

**Pattern:**
```rust
use tracing::{info, error, instrument};

#[instrument(skip(pool))]
async fn execute_pipeline(pool: &PgPool, id: Uuid) -> Result<()> {
    info!(pipeline_id = %id, "Executing pipeline");
    // ... execution logic
    Ok(())
}
```

### Configuration

| Technology | Version | Purpose |
|-----------|---------|---------|
| **TOML** | 0.9.8 | Configuration file format |
| **config** | 0.15.15 | Layered config (defaults → file → env vars) |

**12-Factor App Support:**
```toml
# polyglot.toml
[database]
url = "postgresql://localhost/polyglot"

[redis]
url = "redis://localhost:6379"

# Override via environment variables:
# DATABASE_URL=postgresql://prod/polyglot
```

### Development Tools

| Technology | Purpose |
|-----------|---------|
| **rustfmt** | Code formatting |
| **clippy** | Linting and best practices |
| **cargo-nextest** | Fast test runner (optional) |

---

## Project Structure

### Cargo Workspace Layout

```
polyglot/
├── Cargo.toml                          # Workspace root with shared deps
├── Cargo.lock                          # Lockfile for reproducible builds
├── polyglot.toml.example               # Example configuration
├── README.md
├── LICENSE (MIT)
│
├── crates/
│   ├── polyglot-cli/                   # CLI binary (FR54-FR74)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # Entry point, clap command handling
│   │   │   ├── commands/               # Subcommand implementations
│   │   │   │   ├── mod.rs
│   │   │   │   ├── compile.rs          # polyglot compile <file>
│   │   │   │   ├── activate.rs         # polyglot activate <name>
│   │   │   │   ├── trigger.rs          # polyglot trigger <name>
│   │   │   │   ├── status.rs           # polyglot status <name>
│   │   │   │   ├── logs.rs             # polyglot logs <instance-id>
│   │   │   │   └── services.rs         # polyglot services start/stop/status
│   │   │   └── config.rs               # Config loading (TOML + env vars)
│   │   └── tests/
│   │       └── cli_tests.rs            # Integration tests for CLI commands
│   │
│   ├── polyglot-lexer/                 # Lexer library (FR1-FR9)
│   │   ├── Cargo.toml                  # deps: logos 0.14, thiserror
│   │   ├── src/
│   │   │   ├── lib.rs                  # Public API exports
│   │   │   ├── token.rs                # Token enum (60+ variants)
│   │   │   ├── lexer.rs                # Lexer impl with logos
│   │   │   └── error.rs                # LexerError (thiserror)
│   │   └── tests/
│   │       └── lexer_tests.rs          # Token recognition tests
│   │
│   ├── polyglot-parser/                # Parser library (FR1-FR9)
│   │   ├── Cargo.toml                  # deps: polyglot-lexer, thiserror
│   │   ├── src/
│   │   │   ├── lib.rs                  # Public API
│   │   │   ├── parser.rs               # Recursive descent parser
│   │   │   ├── ast.rs                  # AST node types
│   │   │   ├── import_resolver.rs      # ImportResolver trait
│   │   │   ├── file_registry_resolver.rs # FileRegistryResolver impl
│   │   │   ├── validation.rs           # Semantic validation
│   │   │   └── error.rs                # ParserError (thiserror)
│   │   └── tests/
│   │       ├── parser_tests.rs         # Single-file parsing tests
│   │       ├── multifile_tests.rs      # Multi-file compilation tests
│   │       └── fixtures/               # .pg test files
│   │           ├── multifile-forward-ref/
│   │           │   ├── file1.pg        # [#] 1
│   │           │   └── file2.pg        # [#] 2
│   │           └── ...
│   │
│   ├── polyglot-ir/                    # Intermediate Representation (FR3-FR5)
│   │   ├── Cargo.toml                  # deps: serde, serde_json, thiserror
│   │   ├── src/
│   │   │   ├── lib.rs                  # IR module exports
│   │   │   ├── types.rs                # Shared IR type definitions
│   │   │   ├── trigger_ir.rs           # Trigger IR structure
│   │   │   ├── queue_ir.rs             # Queue IR structure
│   │   │   ├── runner_ir.rs            # Runner IR structure
│   │   │   ├── compiler.rs             # AST → 3-IR compilation
│   │   │   ├── validation.rs           # IR validation rules
│   │   │   └── error.rs                # IrError (thiserror)
│   │   └── tests/
│   │       └── ir_tests.rs             # IR generation & validation tests
│   │
│   ├── polyglot-db/                    # Database layer (FR10-FR18)
│   │   ├── Cargo.toml                  # deps: sqlx, uuid, chrono, thiserror
│   │   ├── src/
│   │   │   ├── lib.rs                  # Database module exports
│   │   │   ├── models.rs               # DB model structs (FromRow derives)
│   │   │   ├── pipelines.rs            # Pipeline CRUD operations
│   │   │   ├── instances.rs            # Instance CRUD operations
│   │   │   ├── triggers.rs             # Trigger CRUD operations
│   │   │   ├── logs.rs                 # Execution log operations
│   │   │   └── error.rs                # DbError (thiserror)
│   │   ├── migrations/                 # sqlx-cli migrations
│   │   │   ├── 20250116_001_create_pipelines.sql
│   │   │   ├── 20250116_002_create_instances.sql
│   │   │   ├── 20250116_003_create_triggers.sql
│   │   │   └── 20250116_004_create_logs.sql
│   │   └── tests/
│   │       └── db_tests.rs             # Database operation tests
│   │
│   ├── polyglot-trigger-monitor/       # Service: Trigger Monitor (FR19-FR26)
│   │   ├── Cargo.toml                  # deps: tokio, polyglot-db, tracing, anyhow
│   │   ├── src/
│   │   │   ├── main.rs                 # Service entry point
│   │   │   ├── monitor.rs              # TriggerMonitor struct (continuous loop)
│   │   │   ├── handlers/               # Trigger handler implementations
│   │   │   │   ├── mod.rs              # Handler registry
│   │   │   │   ├── trait.rs            # TriggerHandler trait
│   │   │   │   ├── time.rs             # TimeTrigger (cron schedules)
│   │   │   │   ├── manual.rs           # ManualTrigger (CLI trigger)
│   │   │   │   ├── resource.rs         # ResourceTrigger (CPU/RAM thresholds)
│   │   │   │   ├── webhook.rs          # WebhookTrigger (post-MVP)
│   │   │   │   └── file_watch.rs       # FileWatchTrigger (post-MVP)
│   │   │   ├── resource_monitor.rs     # Subservice for resource metrics
│   │   │   ├── registry.rs             # Dynamic trigger loading (LISTEN/NOTIFY)
│   │   │   └── config.rs               # Service configuration
│   │   └── tests/
│   │       └── monitor_tests.rs        # Trigger handler tests
│   │
│   ├── polyglot-queue-manager/         # Service: Queue Manager (FR27-FR40)
│   │   ├── Cargo.toml                  # deps: tokio, redis, polyglot-db, anyhow
│   │   ├── src/
│   │   │   ├── main.rs                 # Service entry point
│   │   │   ├── manager.rs              # QueueManager struct
│   │   │   ├── queue.rs                # Redis queue operations
│   │   │   ├── fallback.rs             # PostgreSQL fallback (ADR-012)
│   │   │   └── config.rs               # Service configuration
│   │   └── tests/
│   │       └── queue_tests.rs          # Queue operation tests
│   │
│   ├── polyglot-runner/                # Service: Runner (FR30-FR53)
│   │   ├── Cargo.toml                  # deps: tokio, polyglot-db, polyglot-runtime-wrappers, anyhow
│   │   ├── src/
│   │   │   ├── main.rs                 # Service entry point
│   │   │   ├── runner.rs               # Runner struct (queue polling)
│   │   │   ├── executor.rs             # Pipeline execution orchestration
│   │   │   ├── type_resolver.rs        # Runtime type resolution (ADR-016)
│   │   │   └── config.rs               # Service configuration
│   │   └── tests/
│   │       └── runner_tests.rs         # Execution orchestration tests
│   │
│   └── polyglot-runtime-wrappers/      # Runtime integration (FR41-FR53)
│       ├── Cargo.toml                  # deps: async-trait, serde_json, thiserror
│       ├── src/
│       │   ├── lib.rs                  # RuntimeWrapper trait + exports
│       │   ├── trait.rs                # RuntimeWrapper trait definition
│       │   ├── python.rs               # Python3.11 wrapper (MVP)
│       │   │                           #   - uv virtualenv management
│       │   │                           #   - JSON serialization (Strategy 1)
│       │   │                           #   - PyO3 detection (Strategy 2)
│       │   ├── node.rs                 # Node.js wrapper (post-MVP)
│       │   ├── rust.rs                 # Rust wrapper (post-MVP)
│       │   ├── go.rs                   # Go wrapper (post-MVP)
│       │   ├── type_conversion.rs      # Dual-strategy type converter
│       │   └── error.rs                # WrapperError (thiserror)
│       └── tests/
│           ├── python_tests.rs         # Python wrapper tests
│           └── type_conversion_tests.rs # Type conversion tests
│
├── docs/                               # Documentation (FR84-FR94)
│   ├── technical/
│   │   ├── architecture.md             # This document (consolidated)
│   │   └── architecture/               # Sharded architecture docs
│   │       ├── index.md
│   │       ├── 01-executive-summary.md
│   │       ├── 02-philosophy-and-concepts.md
│   │       ├── ...
│   │       └── 12-adrs.md
│   ├── user/                           # User documentation (v0.0.2 syntax)
│   └── project/
│       ├── prd.md                      # Product requirements
│       └── epics.md                    # Epic breakdown
│
├── examples/                           # Example .pg files (FR84-FR94)
│   ├── 01-hello-world.pg               # Simplest pipeline
│   ├── 02-python-integration.pg        # Cross-language example
│   └── 03-automation-workflow.pg       # Real-world automation
│
└── .github/
    └── workflows/
        └── ci.yml                      # CI/CD: cargo test, clippy, rustfmt
```

### FR Category to Component Mapping

| FR Category | Crates |
|-------------|--------|
| Pipeline Development & Compilation (FR1-FR9) | polyglot-lexer, polyglot-parser, polyglot-ir, polyglot-cli |
| Pipeline Registry & Lifecycle (FR10-FR18) | polyglot-db, polyglot-cli |
| Trigger System (FR19-FR26) | polyglot-trigger-monitor, polyglot-db |
| Queue Management (FR27-FR40) | polyglot-queue-manager, polyglot-runner, polyglot-db |
| Runtime Integration (FR41-FR53) | polyglot-runner, polyglot-runtime-wrappers |
| CLI Tools (FR54-FR74) | polyglot-cli |
| Configuration (FR75-FR83) | All crates (config crate) |
| Documentation (FR84-FR94) | docs/, examples/ |
| Observability (FR95-FR102) | All services (tracing) |

---

## Novel Architectural Patterns

These patterns are unique to Polyglot and central to the hybrid architecture.

### Pattern 1: Dynamic Trigger Loading with LISTEN/NOTIFY

**Problem:** Trigger Monitor must reload trigger configurations when pipelines are activated/deactivated without service restart.

**Solution (ADR-005):** PostgreSQL LISTEN/NOTIFY + in-memory dynamic handler registry

**Architecture:**
```rust
// Trigger Monitor startup
async fn start_trigger_monitor(pool: PgPool) {
    // 1. Load all active pipelines
    let pipelines = load_active_pipelines(&pool).await?;

    // 2. Create handler registry
    let registry = Arc::new(RwLock::new(TriggerRegistry::new()));

    // 3. Spawn handlers for each trigger
    for pipeline in pipelines {
        let trigger_ir = pipeline.trigger_ir;
        for trigger in trigger_ir.triggers {
            let handler = create_handler(trigger)?;
            registry.write().await.register(pipeline.id, handler);
            tokio::spawn(async move {
                handler.run().await;
            });
        }
    }

    // 4. Listen for pipeline changes
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("pipeline_changes").await?;

    loop {
        tokio::select! {
            notification = listener.recv() => {
                // Reload triggers dynamically
                reload_triggers(&pool, &registry).await?;
            }
        }
    }
}
```

**Benefits:**
- Real-time configuration updates (no polling lag)
- No service restart required
- Hybrid trigger types (async listening + sync loops)

**Trade-offs:**
- Single Trigger Monitor instance (future: leader election for HA)
- In-memory registry (lost on crash, but reloaded on restart)

### Pattern 2: 3-IR Structure for Service Isolation

**Problem:** Each service (Trigger Monitor, Queue Manager, Runner) needs different IR information. Monolithic IR couples services and makes versioning difficult.

**Solution (ADR-008):** Compile `.pg` files into 3 separate IRs stored as separate JSONB columns

**Structure:**
```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    trigger_ir JSONB NOT NULL,    -- Trigger Monitor reads this
    queue_ir JSONB NOT NULL,       -- Queue Manager reads this
    runner_ir JSONB NOT NULL,      -- Runner reads this
    created_at TIMESTAMPTZ DEFAULT NOW(),
    activated BOOLEAN DEFAULT FALSE
);
```

**Trigger IR Schema:**
```json
{
  "triggers": [
    {
      "type": "Time",
      "schedule": "0 0 * * *",  // Cron format
      "enabled": true
    },
    {
      "type": "Resource",
      "condition": "cpu > 80",
      "enabled": false
    }
  ]
}
```

**Queue IR Schema:**
```json
{
  "timing_logic": {
    "type": "Immediate",
    "delay_ms": 0
  },
  "queue_selector": "default",
  "priority": 5,
  "rate_limit": null
}
```

**Runner IR Schema:**
```json
{
  "execution_mode": "Sequential",
  "steps": [
    {
      "id": "step1",
      "step_type": "Wrapper",
      "runtime": "Python3.11",
      "code": "print('Hello, World!')",
      "mode": "Sequential"
    }
  ]
}
```

**Benefits:**
- Clear service boundaries (each service reads only its IR)
- Independent versioning (trigger_ir v2, queue_ir v1, runner_ir v3)
- Efficient queries (no loading full IR when only trigger needed)
- PostgreSQL GIN indexes on each JSONB column

**Trade-offs:**
- Three columns instead of one (acceptable: 3x disk space is negligible)
- Compilation produces 3 outputs (complexity in compiler)

### Pattern 3: Database-Driven Service Communication

**Problem:** 3 backend services must communicate and coordinate without tight coupling.

**Solution (ADR-006):** No direct HTTP/RPC; all communication via PostgreSQL and Redis

**Flow:**
```
Trigger Monitor
    ↓ INSERT INTO pipeline_instances (pipeline_id, status, trigger_data)
    ↓ VALUES (uuid, 'Created', json_data)
    ↓
Queue Manager (polling PostgreSQL)
    ↓ SELECT * FROM pipeline_instances WHERE status = 'Created'
    ↓ RPUSH dispatch_queue instance_id (Redis)
    ↓ UPDATE pipeline_instances SET status = 'Queued'
    ↓
Runner (popping Redis)
    ↓ LPOP dispatch_queue → instance_id
    ↓ SELECT runner_ir FROM pipelines WHERE id = (SELECT pipeline_id FROM instances WHERE id = instance_id)
    ↓ UPDATE pipeline_instances SET status = 'Running', started_at = NOW()
    ↓ [Execute pipeline]
    ↓ UPDATE pipeline_instances SET status = 'Exited', completed_at = NOW()
```

**Benefits:**
- Simplified deployment (no service discovery needed)
- Strong consistency (PostgreSQL ACID guarantees)
- Fast queuing (Redis push/pop)
- Failure isolation (services restart independently)
- Visible state transitions (query database to see pipeline flow)

**Trade-offs:**
- All services depend on PostgreSQL/Redis availability
- Polling introduces latency (mitigated by LISTEN/NOTIFY and fast Redis)
- Future: May need distributed tracing for complex debugging

### Pattern 4: Dual-Strategy Type Conversion

**Problem:** Cross-language type conversion must balance flexibility (work for any type) and performance (avoid overhead).

**Solution (ADR-016):** Two conversion strategies with automatic selection

**Strategy 1: Universal Translator (Serialization)**
```rust
// polyglot-runtime-wrappers/src/type_conversion.rs
async fn serialize_strategy(
    value: Value,
    target_type: Type
) -> Result<Value> {
    // 1. Source language → JSON
    let json = serde_json::to_string(&value)?;

    // 2. JSON → pg\type (Polyglot IR representation)
    let pg_value = parse_json_to_pg_type(&json, value.pg_type)?;

    // 3. pg\type → JSON (target format)
    let target_json = pg_value_to_json(&pg_value, &target_type)?;

    // 4. JSON → Target language
    let target_value = parse_json_in_target_lang(&target_json, &target_type)?;

    Ok(target_value)
}
```

**Example:**
```
Python str "hello"
    → JSON "\"hello\""
    → pg\string
    → JSON "\"hello\""
    → Rust String "hello"
```

**Strategy 2: Direct Conversion (Optimized FFI)**
```rust
async fn direct_ffi_strategy(
    value: Value,
    target_type: Type
) -> Result<Value> {
    // Detect if FFI bridge available
    match (&value.lang, &target_type.lang) {
        ("Python", "Rust") if pyo3_available() => {
            // Use PyO3 for zero-copy conversion
            pyo3_convert(value, target_type)
        }
        ("Rust", "Python") if pyo3_available() => {
            pyo3_convert(value, target_type)
        }
        _ => {
            // No direct bridge, fallback to Strategy 1
            serialize_strategy(value, target_type).await
        }
    }
}
```

**Automatic Selection:**
```rust
pub async fn convert_type(
    value: Value,
    target_type: Type
) -> Result<Value> {
    // Try direct FFI first (faster)
    if let Some(converter) = detect_ffi_bridge(&value.lang, &target_type.lang) {
        match converter.convert(value.clone(), target_type.clone()).await {
            Ok(result) => return Ok(result),
            Err(_) => {
                // Direct FFI failed, fallback to serialization
                tracing::warn!("FFI conversion failed, falling back to serialization");
            }
        }
    }

    // Fallback: serialization (always works)
    serialize_strategy(value, target_type).await
}
```

**Performance Characteristics:**
- Strategy 1 (Serialization): 5-10ms overhead for <1MB data (acceptable for automation)
- Strategy 2 (Direct FFI): <1ms overhead (near-native performance)
- Automatic fallback ensures reliability

**Runtime Type Resolution:**
```rust
// polyglot-runner/src/type_resolver.rs
pub async fn resolve_type_at_runtime(value: &Value) -> Type {
    match value {
        Value::Python(py_obj) => {
            // Inspect Python object at runtime
            let type_name = py_obj.get_type_name();
            let size = py_obj.len().ok();

            match (type_name.as_str(), size) {
                ("list", Some(n)) => Type::Array { element_type: detect_element_type(py_obj), size: n },
                ("dict", _) => Type::HashMap { key_type: ..., value_type: ... },
                ("str", _) => Type::String,
                ("int", _) => Type::Integer,
                _ => Type::Unknown
            }
        }
        // ... other languages
    }
}
```

**Benefits:**
- Best-of-both-worlds: flexibility + performance
- No manual type annotations required (runtime inspection)
- Graceful degradation (FFI → serialization fallback)

### Pattern 5: PostgreSQL Fallback for Redis Queues

**Problem:** Redis failure would halt all pipeline execution.

**Solution (ADR-012):** Queue Manager falls back to PostgreSQL polling when Redis unavailable

**Implementation:**
```rust
// polyglot-queue-manager/src/manager.rs
async fn run_queue_manager(pool: PgPool, redis_client: RedisClient) {
    let mut redis_available = true;

    loop {
        // Check Redis health
        if let Err(_) = redis_client.ping().await {
            if redis_available {
                tracing::warn!("Redis unavailable, switching to PostgreSQL fallback");
                redis_available = false;
            }
        } else if !redis_available {
            tracing::info!("Redis recovered, switching back to Redis queuing");
            redis_available = true;
        }

        if redis_available {
            // Primary: Redis-based queuing (fast)
            queue_via_redis(&pool, &redis_client).await?;
        } else {
            // Fallback: PostgreSQL polling (slower but functional)
            queue_via_postgres(&pool).await?;
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn queue_via_redis(pool: &PgPool, redis: &RedisClient) -> Result<()> {
    // 1. Query PostgreSQL for new instances
    let instances = sqlx::query!(
        "SELECT id FROM pipeline_instances WHERE status = 'Created' LIMIT 100"
    )
    .fetch_all(pool)
    .await?;

    // 2. Push to Redis queue
    for instance in instances {
        redis.rpush("dispatch_queue", instance.id.to_string()).await?;
        sqlx::query!("UPDATE pipeline_instances SET status = 'Queued' WHERE id = $1", instance.id)
            .execute(pool)
            .await?;
    }

    Ok(())
}

async fn queue_via_postgres(pool: &PgPool) -> Result<()> {
    // Polling-based fallback (slower but no Redis dependency)
    // Runner polls PostgreSQL directly for status='Queued' instances
    // Update status to 'Running' when picked up

    // No-op here: Runner handles polling when Redis unavailable
    Ok(())
}
```

**Benefits:**
- High availability (system continues if Redis fails)
- No data loss (instances already in PostgreSQL)
- Graceful degradation (slower but functional)

**Trade-offs:**
- Performance degradation during fallback (polling vs. push/pop)
- Increased PostgreSQL load during Redis outage

---

## Implementation Patterns

These patterns ensure AI agents write compatible code across all crates.

### Naming Conventions

**Files & Modules:**
- **Snake_case:** `trigger_monitor.rs`, `queue_manager.rs`, `type_resolver.rs`
- **Match crate names:** `polyglot-trigger-monitor` → `trigger_monitor/`

**Types:**
- **PascalCase:** `TriggerMonitor`, `PipelineInstance`, `LexerError`, `RuntimeWrapper`
- **Trait names:** Describe capability: `TriggerHandler`, `ImportResolver`

**Functions:**
- **Snake_case:** `load_triggers()`, `execute_pipeline()`, `connect_db()`
- **No async prefix:** `.await` makes async clear

**Database:**
- **Tables:** Plural snake_case: `pipelines`, `pipeline_instances`, `triggers`
- **Columns:** Snake_case: `pipeline_id`, `created_at`, `activated`
- **Foreign keys:** `<table>_id` format: `pipeline_id`, `instance_id`
- **JSONB columns:** `trigger_ir`, `queue_ir`, `runner_ir`

**Error Variants:**
- **PascalCase:** `LexerError::UnexpectedChar`, `DbError::ConnectionFailed`
- **Include context:** `InvalidTrigger { trigger_id: Uuid, reason: String }`

### Structure Patterns

**Crate Organization:**
```
crate-name/
├── Cargo.toml
├── src/
│   ├── lib.rs or main.rs       # Entry point
│   ├── <domain>.rs             # Core logic (lexer.rs, monitor.rs)
│   ├── error.rs                # Error types (thiserror for libs)
│   ├── config.rs               # Configuration structs
│   └── <sub>/                  # Sub-modules if complex
│       ├── mod.rs
│       └── <feature>.rs
└── tests/                      # Integration tests
    └── <crate>_tests.rs
```

**Test Organization:**
- **Unit tests:** Inline with `#[cfg(test)] mod tests { ... }`
- **Integration tests:** `tests/` directory
- **Test file naming:** `<feature>_tests.rs`
- **Coverage target:** 80% for libraries, 60% for services

**Error Module Pattern (Library Crates):**
```rust
// error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrateNameError {
    #[error("Description: {0}")]
    SpecificVariant(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

// Ensure Send + Sync for async
static_assertions::assert_impl_all!(CrateNameError: Send, Sync);
```

**Binary Main Pattern (Services + CLI):**
```rust
// main.rs
use anyhow::{Context, Result};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize tracing
    tracing_subscriber::fmt::init();

    // 2. Load config
    let config = Config::load()
        .context("Failed to load configuration")?;

    // 3. Connect to database
    let pool = connect_db(&config.database_url).await
        .context("Failed to connect to database")?;

    // 4. Run service/CLI
    run(pool, config).await
}
```

### Format Patterns

**Database Model Serialization:**
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub trigger_ir: sqlx::types::Json<TriggerIr>,
    pub queue_ir: sqlx::types::Json<QueueIr>,
    pub runner_ir: sqlx::types::Json<RunnerIr>,
    pub created_at: DateTime<Utc>,
    pub activated: bool,
}
```

**Date/Time Format:**
- **Internal:** `chrono::DateTime<Utc>`
- **Database:** PostgreSQL `TIMESTAMPTZ`
- **Logs:** ISO 8601 via tracing JSON formatter
- **User-facing:** ISO 8601 strings

**Logging Format:**
```rust
use tracing::{info, error, warn, debug, instrument};

#[instrument(skip(pool))]  // Auto-trace function entry/exit
async fn load_pipelines(pool: &PgPool) -> Result<Vec<Pipeline>> {
    info!("Loading active pipelines");

    match query_pipelines(pool).await {
        Ok(pipelines) => {
            info!(count = pipelines.len(), "Loaded pipelines");
            Ok(pipelines)
        }
        Err(e) => {
            error!(error = %e, "Failed to load pipelines");
            Err(e.into())
        }
    }
}
```

### Cross-Cutting Patterns

**Error Handling:**
- **Library crates:** `thiserror` for domain-specific errors
- **Binary crates:** `anyhow` for easy propagation
- **Async safety:** All errors must be `Send + Sync`
- **Context:** Use `.context()` for meaningful messages

**Logging:**
- **Structured logging:** tracing with structured fields
- **Log levels:** `error!`, `warn!`, `info!`, `debug!`, `trace!`
- **Instrumentation:** `#[instrument]` for automatic span tracking
- **Format:** JSON for production, pretty-print for development

**Date/Time:**
- **Library:** `chrono` for date/time operations
- **Timezone:** Always UTC internally
- **Format:** ISO 8601 for all external representations

---

## Data Architecture

### Database Schema

#### pipelines Table
```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    trigger_ir JSONB NOT NULL,
    queue_ir JSONB NOT NULL,
    runner_ir JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    activated BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_pipelines_name ON pipelines(name);
CREATE INDEX idx_pipelines_activated ON pipelines(activated);
CREATE INDEX idx_pipelines_trigger_ir ON pipelines USING GIN(trigger_ir);
CREATE INDEX idx_pipelines_queue_ir ON pipelines USING GIN(queue_ir);
CREATE INDEX idx_pipelines_runner_ir ON pipelines USING GIN(runner_ir);
```

#### pipeline_instances Table
```sql
CREATE TABLE pipeline_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL, -- Created, Queued, Running, Exited, Failed
    trigger_data JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT
);

CREATE INDEX idx_instances_pipeline ON pipeline_instances(pipeline_id);
CREATE INDEX idx_instances_status ON pipeline_instances(status);
CREATE INDEX idx_instances_created ON pipeline_instances(created_at DESC);
```

#### triggers Table
```sql
CREATE TABLE triggers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    trigger_type VARCHAR(50) NOT NULL, -- Manual, Time, Webhook, FileWatch, Resource
    trigger_config JSONB NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_triggers_pipeline ON triggers(pipeline_id);
CREATE INDEX idx_triggers_type ON triggers(trigger_type);
CREATE INDEX idx_triggers_enabled ON triggers(enabled);
```

#### execution_logs Table
```sql
CREATE TABLE execution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ DEFAULT NOW(),
    level VARCHAR(20) NOT NULL, -- ERROR, WARN, INFO, DEBUG, TRACE
    message TEXT NOT NULL,
    metadata JSONB
);

CREATE INDEX idx_logs_instance ON execution_logs(instance_id);
CREATE INDEX idx_logs_timestamp ON execution_logs(timestamp DESC);
CREATE INDEX idx_logs_level ON execution_logs(level);
```

### IR Schemas

See [Novel Architectural Patterns](#pattern-2-3-ir-structure-for-service-isolation) for detailed IR schemas.

---

## Cross-Cutting Concerns

### Security

**Database Security:**
- Connection strings in environment variables (never in code)
- PostgreSQL user with least privilege (no superuser)
- Prepared statements via SQLx (SQL injection prevention)

**Service Security:**
- No HTTP endpoints in MVP (database-driven only)
- Shared database credentials (trusted network)
- Future: Mutual TLS for service communication (post-MVP)

**Configuration Security:**
- Sensitive values in environment variables
- `.env` file in `.gitignore`
- Example config with placeholders

### Performance

**Async-First Design:**
- All I/O operations use Tokio async runtime
- No blocking operations in hot paths
- Database queries use connection pooling

**Lexer Performance:**
- Target: <100ms for 1000-line files (NFR-P1)
- logos compile-time DFA generation
- Zero-copy string handling where possible

**Database Performance:**
- JSONB GIN indexing for IR queries
- Connection pooling (SQLx default)
- Query optimization via compile-time verification

### Reliability

**Service Availability:**
- Services restart automatically on crash (systemd/Docker)
- Health checks expose readiness status
- PostgreSQL fallback for Redis (ADR-012)

**Data Durability:**
- All state persisted to PostgreSQL before acknowledgment
- Pipeline instance state recoverable after service restart
- No data loss on graceful shutdown

**Error Handling:**
- All errors logged with context (stack trace, instance ID, timestamp)
- Errors isolated per instance
- Cross-language error propagation (RuntimeWrapper trait)

---

## Development Environment

### Prerequisites

- Rust 1.75+ (2021 edition)
- PostgreSQL 14+
- Redis 7+
- InfluxDB 2.x

### Setup Commands

```bash
# 1. Clone repository
git clone <repo-url>
cd polyglot

# 2. Install Rust toolchain
rustup update stable

# 3. Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# 4. Set up environment
cp .env.example .env
# Edit .env with database URLs:
# DATABASE_URL=postgresql://localhost/polyglot
# REDIS_URL=redis://localhost:6379
# INFLUXDB_URL=http://localhost:8086

# 5. Create database
createdb polyglot

# 6. Run migrations
sqlx migrate run --database-url $DATABASE_URL

# 7. Build workspace
cargo build

# 8. Run tests
cargo test

# 9. Run CLI
cargo run --bin polyglot-cli -- --help
```

---

## Architecture Decision Records (ADRs)

### ADR-001: Manual Cargo Workspace Over Starter Template

**Status:** Accepted

**Context:** Polyglot's architecture (3 backend services + CLI + shared libraries) is specialized for a language implementation.

**Decision:** Use manual Cargo workspace setup instead of generic Rust starter templates.

**Rationale:**
- Starter templates don't support multi-service architecture
- Custom workspace gives precise control over crate boundaries
- Polyglot's dependencies (PostgreSQL, Redis, Tokio, serde) are specific

**Consequences:**
- More initial setup work
- Full control over project structure
- Clear separation of concerns

---

### ADR-002: SQLx Over Diesel

**Status:** Accepted

**Context:** Need async-first database client for PostgreSQL.

**Decision:** Use SQLx 0.8.6 with `tokio-comp` feature.

**Rationale:**
- Async-first design (Diesel added async later)
- Compile-time query verification against actual schema
- Lightweight, no heavy ORM abstraction
- Direct SQL queries clearer for complex IR storage

**Consequences:**
- More SQL knowledge required (less abstraction than ORM)
- Better performance for async workloads
- sqlx-cli provides migration tooling

---

### ADR-003: PostgreSQL JSONB for IR Storage

**Status:** Accepted

**Context:** IR is serialized data, but metadata is relational.

**Decision:** Use PostgreSQL with JSONB columns for IR, relational columns for metadata.

**Rationale:**
- Best of both worlds: document storage + relational queries
- JSONB supports indexing and querying
- One database instead of PostgreSQL + MongoDB
- ACID guarantees for state management

**Consequences:**
- IR stored as JSON (human-readable for debugging)
- Can query inside IR: `WHERE trigger_ir->>'trigger_type' = 'manual'`
- PostgreSQL remains single source of truth

---

### ADR-004: thiserror + anyhow Error Handling

**Status:** Accepted

**Context:** Need error handling strategy for libraries + binaries in async context.

**Decision:**
- thiserror 2.0.17 for library error types
- anyhow 1.0.99 for binary error propagation
- All errors must be `Send + Sync` for async

**Rationale:**
- Rust community standard pattern
- Structured errors in libraries enable caller matching
- Anyhow simplifies context chaining in binaries
- Both support async error propagation

**Consequences:**
- Library errors define custom types with variants
- Binaries use `anyhow::Result<T>` and `.context()`
- Errors cross async boundaries safely

---

### ADR-005: Dynamic Trigger Loading with PostgreSQL LISTEN/NOTIFY

**Status:** Accepted

**Context:** Triggers must reload when pipeline IR changes without service restarts.

**Decision:** Use PostgreSQL LISTEN/NOTIFY for trigger updates, dynamic handler registry in Trigger Monitor.

**Rationale:**
- PostgreSQL NOTIFY is real-time (no polling lag)
- Dynamic loading enables runtime configuration
- Hybrid trigger types (async listening + sync loop) support diverse use cases

**Consequences:**
- Novel architectural pattern (documented in this architecture)
- Trigger Monitor maintains in-memory registry
- Handlers spawned/stopped on IR changes
- Scalability limited to single Trigger Monitor instance (future: leader election)

---

### ADR-006: Database-Driven Service Communication

**Status:** Accepted

**Context:** 3 backend services need to communicate and share state.

**Decision:** Services communicate via PostgreSQL (state) and Redis (queues), no direct HTTP/RPC.

**Rationale:**
- Simplifies deployment (no service discovery needed)
- Database provides strong consistency
- Redis provides fast queue operations
- Failure isolation (services restart independently)

**Consequences:**
- All services depend on PostgreSQL/Redis availability
- No HTTP servers in services (except future webhooks)
- State transitions visible in database
- Future: May need distributed tracing for debugging

---

### ADR-007: InfluxDB for Time-Series Data

**Status:** Accepted

**Context:** Time-based triggers, trigger execution results, and resource metrics are time-series data.

**Decision:** Add InfluxDB 2.x as MVP requirement for time-series storage.

**Rationale:**
- Time-series database optimized for temporal data
- Efficient storage for resource metrics (CPU, RAM, GPU, Network)
- Better query performance for time-based trigger schedules
- Separate concern from relational data (PostgreSQL)

**Consequences:**
- Additional dependency for MVP (InfluxDB required)
- Trigger Monitor reads from both PostgreSQL and InfluxDB
- Resource Monitor writes metrics to InfluxDB at fixed intervals
- More complex deployment (3 databases: PostgreSQL, InfluxDB, Redis)

---

### ADR-008: 3-IR Structure (Trigger, Queue, Runner)

**Status:** Accepted

**Context:** Pipeline IR has distinct concerns: triggering, queuing, and execution.

**Decision:** Split compiled IR into 3 separate IRs stored as separate JSONB columns.

**Rationale:**
- Separation of concerns (trigger logic ≠ queue logic ≠ execution logic)
- Each service reads only its relevant IR (Trigger Monitor → trigger_ir, etc.)
- Easier to query and index (GIN indexes on each IR column)
- Polyglot syntax `[t]` and `[Q]` map cleanly to Queue IR

**Consequences:**
- Database schema has 3 JSONB columns instead of 1
- Compilation produces 3 IRs from single .pg file
- Each IR has its own schema and validation rules
- Clear boundaries for AI agents implementing services

---

### ADR-009: Resource Monitor as Trigger Monitor Subservice

**Status:** Accepted

**Context:** Resource monitoring (CPU, RAM, GPU, Network) is needed for resource-based triggers.

**Decision:** Implement Resource Monitor as a subservice of Trigger Monitor, not a 4th separate service.

**Rationale:**
- Resource Monitor is triggered by Trigger Monitor
- Tight coupling: Trigger Monitor activates resource monitoring based on pipeline needs
- Reduces complexity (3 services instead of 4)
- Selective monitoring: Only monitor resources specified in pipeline IRs

**Consequences:**
- Trigger Monitor crate has resource monitoring submodule
- Resource metrics written to InfluxDB
- Continuous polling at fixed interval (configurable)
- Single service failure affects both trigger checking and resource monitoring

---

### ADR-010: Compile = Validate + Convert + Register

**Status:** Accepted

**Context:** User workflow for getting .pg files into the system.

**Decision:** `polyglot compile <file>.pg` performs validation, IR conversion, AND database registration in one command.

**Rationale:**
- Simplifies user workflow (one command instead of two)
- Compilation without storage is rarely useful (can compile but not use)
- After compile, pipelines available for activation
- Matches user mental model: "compile" means "make it ready"

**Consequences:**
- No separate `polyglot register` command needed
- Users must have database connection to compile
- "Compile without register" means validation only (future flag: `--dry-run`)
- After compile, users run `polyglot activate <name>` to enable pipelines

---

### ADR-011: Pause Types (Process Pause + Checkpoints)

**Status:** Accepted

**Context:** Pipelines may need to pause execution for various reasons.

**Decision:** Support 2 pause types:
1. **Process Pause** (OS-level): Freeze memory or cache RAM in Redis
2. **Checkpoints** (Programmatic): User-defined pause points in Polyglot code

**Rationale:**
- Process pause handles resource constraints (reduce CPU load, free RAM)
- Checkpoints enable user-defined pause/resume logic based on conditions
- Pause queue separates paused instances from active dispatch queue
- Flexibility for different pause scenarios (system resources vs. business logic)

**Consequences:**
- Pause queue added to Redis (MVP)
- Runner must support pause/resume operations
- Process pause: OS-level signals (SIGSTOP/SIGCONT) or Redis caching
- Checkpoint pause: Save execution state, resume from checkpoint

---

### ADR-012: PostgreSQL Fallback for Redis Queues

**Status:** Accepted

**Context:** Redis failure would halt all pipeline queuing and execution.

**Decision:** Use PostgreSQL as fallback when Redis unavailable. Queue Manager polls `pipeline_instances WHERE status='queued'`.

**Rationale:**
- High availability: System continues functioning if Redis fails
- PostgreSQL already stores instance state
- No data loss (instances already in database)
- Graceful degradation (slower but functional)

**Consequences:**
- Queue Manager checks Redis health
- On Redis failure, switch to PostgreSQL polling
- Performance degradation during fallback (polling vs. push/pop)
- When Redis recovers, switch back to Redis-based queuing

---

### ADR-013: Logos Lexer Generator

**Status:** Accepted

**Context:** Lexer must tokenize 60+ distinct token types with <100ms performance (NFR-P1).

**Decision:** Use `logos` crate (0.14) for declarative token definition and automatic lexer generation.

**Rationale:**
- **Don't Reinvent the Wheel** - Core Polyglot philosophy
- **Battle-tested** - Used by tree-sitter, rustpython (2.5k+ GitHub stars)
- **Performance** - Compile-time DFA generation meets <100ms requirement
- **Maintainability** - 60+ token types as regex annotations vs. 500-800 LOC manual logic

**Consequences:**
- Adds `logos = "0.14"` dependency
- Compile times increase ~2-5 seconds (proc macro generation)
- Lexer implementation reduces from ~500-800 LOC to ~200 LOC
- Source location tracking implemented manually on top of logos byte positions

---

### ADR-014: String Concatenation Operator (`+"` vs `>"`)

**Status:** Accepted

**Context:** String concatenation operator `>"` was non-intuitive and inconsistent with industry standards.

**Decision:** Change operator from `>"` to `+"` throughout language specification and implementation.

**Rationale:**
- **Industry Standards** - `+` universally recognized for concatenation
- **Intuitive** - `+"` visually suggests "add/append string"
- **Low Risk** - Language not yet in production, no user code to migrate

**Consequences:**
- Documentation updated (57 occurrences)
- Lexer updated with `TokenKind::OpStringConcat`
- Parser validates `+"` only between string literals (not variables)

---

### ADR-015: Hybrid Architecture Pattern

**Status:** Accepted (NEW - 2025-12-17)

**Context:** Polyglot requires capabilities from three different architectural patterns: language compilation, async job orchestration, and multi-language runtime integration. No single existing pattern provides all required functionality.

**Decision:** Combine three proven architectural patterns into a novel hybrid:
1. **Compiler Pipeline Pattern** (front-end) - Lexer → Parser → AST → 3-IR
2. **Event-Driven Job Queue Pattern** (backend) - Trigger Monitor → Queue Manager → Runner
3. **Plugin System Pattern** (execution) - RuntimeWrapper trait for multi-language support

**Rationale:**
- **Compiler Pipeline** alone lacks execution orchestration
- **Job Queue** alone lacks language processing and multi-runtime support
- **Plugin System** alone lacks compilation and queue management
- Hybrid combines strengths while maintaining clear boundaries between patterns

**Consequences:**
- Novel architecture with no existing template to reference
- Clear separation between compilation (stateless) and execution (stateful)
- Each pattern can evolve independently
- Documentation must explicitly explain pattern integration
- Higher initial complexity offset by long-term maintainability

**Pattern Boundaries:**
```
Compilation Layer (Pattern 1):
  CLI → Lexer → Parser → IR Generator → Database

Service Layer (Pattern 2):
  Trigger Monitor → Queue Manager → Runner

Execution Layer (Pattern 3):
  Runner → RuntimeWrapper → Python/Node/Rust/Go

Innovation Layer (Cross-cutting):
  Runtime Type Resolution (spans Execution + Service layers)
```

---

### ADR-016: Runtime Type Resolution Strategy

**Status:** Accepted (NEW - 2025-12-17)

**Context:** Traditional FFI tools (PyO3, pybind11) require compile-time type knowledge, which prevents dynamic-to-static language conversions (e.g., Python list of runtime-determined size → Rust fixed-size array). This is Polyglot's core innovation.

**Decision:** Implement dual-strategy type conversion with runtime type inspection:
- **Strategy 1 (Universal Translator):** JSON serialization through Polyglot IR (guaranteed to work, 5-10ms overhead)
- **Strategy 2 (Direct FFI):** Leverage existing FFI tools when available (near-zero overhead)
- **Automatic Selection:** Try Strategy 2 first, fallback to Strategy 1

**Rationale:**
- Async architecture enables type inspection at execution time (not compile time)
- Dual-strategy provides flexibility (any type pair works) + performance (optimized when possible)
- Runtime inspection unlocks conversions impossible with compile-time tools
- Example: Python function returns `[1, 2, 3]` → Runtime detects 3 elements → Rust receives `Vec<i32>` or `[i32; 3]`

**Consequences:**
- RuntimeWrapper trait must support type inspection: `resolve_type_at_runtime(value) -> Type`
- Type conversion module in polyglot-runtime-wrappers crate
- Strategy 1 always works (fallback safety)
- Strategy 2 provides performance when FFI bridge detected (PyO3, N-API, etc.)
- No manual type annotations required (innovation vs. traditional FFI)

**Implementation:**
```rust
// polyglot-runner/src/type_resolver.rs
pub async fn convert_type(value: Value, target: Type) -> Result<Value> {
    // Try Strategy 2: Direct FFI (fast)
    if let Some(bridge) = detect_ffi_bridge(&value.lang, &target.lang) {
        if let Ok(result) = bridge.convert(value.clone(), target.clone()).await {
            return Ok(result);
        }
    }

    // Fallback Strategy 1: Serialization (always works)
    serialize_via_json(value, target).await
}
```

---

### ADR-017: RuntimeWrapper Trait Design

**Status:** Accepted (NEW - 2025-12-17)

**Context:** Multi-language runtime support requires abstraction over diverse language runtimes (Python, Node.js, Rust, Go) with different setup requirements, execution models, and error handling.

**Decision:** Define `RuntimeWrapper` trait with lifecycle methods (setup, execute, close) and async support:

```rust
#[async_trait]
pub trait RuntimeWrapper: Send + Sync {
    async fn setup(&self) -> Result<()>;
    async fn execute(&self, code: &str, inputs: HashMap<String, Value>) -> Result<HashMap<String, Value>>;
    async fn close(&self) -> Result<()>;
}
```

**Rationale:**
- **Lifecycle Management:** setup() for environment initialization (virtualenv, package install), close() for cleanup
- **Async Support:** All methods async to integrate with Tokio runtime (non-blocking I/O)
- **Type Conversion Abstraction:** Inputs/outputs as `HashMap<String, Value>` with runtime-specific conversion
- **Error Isolation:** Errors in one runtime don't crash Runner service (trait boundary provides isolation)
- **Send + Sync:** Required for sharing across async tasks

**Consequences:**
- Each language runtime implements this trait (Python, Node, Rust, Go)
- Runner service doesn't know implementation details (abstraction)
- Python wrapper uses `uv` for virtualenv management (MVP)
- Node wrapper uses `npm` for package management (post-MVP)
- Runtime wrapper errors wrapped in `WrapperError` (thiserror)
- Enables ecosystem growth (community can add new language wrappers)

**Python Implementation (MVP):**
```rust
pub struct PythonWrapper {
    venv_path: PathBuf,
    interpreter: String,
}

#[async_trait]
impl RuntimeWrapper for PythonWrapper {
    async fn setup(&self) -> Result<()> {
        // Use uv to create virtualenv
        Command::new("uv")
            .args(["venv", self.venv_path.to_str().unwrap()])
            .output()
            .await?;
        Ok(())
    }

    async fn execute(&self, code: &str, inputs: HashMap<String, Value>) -> Result<HashMap<String, Value>> {
        // Convert inputs: Rust Value → JSON → Python
        let inputs_json = serde_json::to_string(&inputs)?;

        // Execute Python code with inputs
        let output = Command::new(&self.interpreter)
            .arg("-c")
            .arg(code)
            .env("INPUTS", inputs_json)
            .output()
            .await?;

        // Convert outputs: Python → JSON → Rust Value
        let outputs_json = String::from_utf8(output.stdout)?;
        let outputs = serde_json::from_str(&outputs_json)?;
        Ok(outputs)
    }

    async fn close(&self) -> Result<()> {
        // Cleanup virtualenv if needed
        Ok(())
    }
}
```

---

### ADR-018: Compilation Pipeline Integration

**Status:** Accepted (NEW - 2025-12-17)

**Context:** Compiler front-end (Lexer → Parser → IR) and backend services (Trigger Monitor → Queue Manager → Runner) must integrate seamlessly. The bridge between compilation and execution must be clear and stateless.

**Decision:** CLI orchestrates compilation pipeline with PostgreSQL as integration point:

```
CLI Command: polyglot compile example.pg
    ↓
1. Lexer: .pg source → Token stream
2. Parser: Tokens → AST
3. IR Generator: AST → 3-IR (Trigger, Queue, Runner)
4. Database: INSERT into pipelines (trigger_ir, queue_ir, runner_ir JSONB)
5. PostgreSQL NOTIFY → Trigger Monitor reloads triggers
    ↓
CLI Response: "Pipeline 'example' compiled and registered"
```

**Rationale:**
- **Stateless Compilation:** Lexer, Parser, IR Generator have no side effects (pure functions)
- **Database as Bridge:** PostgreSQL connects compilation (write IR) to execution (read IR)
- **Clear Boundaries:** Front-end writes, backend reads, no tight coupling
- **LISTEN/NOTIFY Integration:** PostgreSQL NOTIFY triggers service reload (ADR-005)

**Consequences:**
- CLI binary (polyglot-cli) orchestrates entire compilation flow
- Compilation can run offline (doesn't need services running)
- Services only need database connection (no direct CLI dependency)
- IR validation happens before database write (fail fast)
- Future: CLI could batch-compile multiple files before single database transaction

**Integration Flow:**
```rust
// polyglot-cli/src/commands/compile.rs
pub async fn compile_command(file_path: &Path, pool: &PgPool) -> Result<()> {
    // 1. Lexer
    let source = fs::read_to_string(file_path)?;
    let tokens = Lexer::new(&source).tokenize()?;

    // 2. Parser
    let ast = Parser::new(tokens).parse()?;

    // 3. IR Generation
    let trigger_ir = generate_trigger_ir(&ast)?;
    let queue_ir = generate_queue_ir(&ast)?;
    let runner_ir = generate_runner_ir(&ast)?;

    // 4. Database Registration
    sqlx::query!(
        "INSERT INTO pipelines (name, trigger_ir, queue_ir, runner_ir)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (name) DO UPDATE SET
            trigger_ir = $2, queue_ir = $3, runner_ir = $4, updated_at = NOW()",
        ast.pipeline_name,
        Json(&trigger_ir),
        Json(&queue_ir),
        Json(&runner_ir)
    )
    .execute(pool)
    .await?;

    // 5. Notify services (triggers LISTEN/NOTIFY in Trigger Monitor)
    sqlx::query!("NOTIFY pipeline_changes, $1", ast.pipeline_name)
        .execute(pool)
        .await?;

    Ok(())
}
```

---

## Summary: Hybrid Architecture Coherence

The hybrid architecture achieves Polyglot's goals through pattern composition:

**Compiler Pipeline** (Pattern 1) provides:
- Language syntax processing (`.pg` → AST → 3-IR)
- Type-safe intermediate representations
- Validation before execution

**Event-Driven Job Queue** (Pattern 2) provides:
- Async service orchestration (Trigger → Queue → Runner)
- Database-driven coordination (PostgreSQL + Redis)
- Scalable background processing

**Plugin System** (Pattern 3) provides:
- Multi-language runtime abstraction (RuntimeWrapper trait)
- Language-agnostic interfaces
- Ecosystem extensibility

**Runtime Type Resolution** (Innovation Layer) provides:
- Dynamic → static type conversions (impossible with compile-time FFI)
- Dual-strategy conversion (flexibility + performance)
- Zero manual type annotations

**Integration Points:**
1. **Compilation → Execution:** PostgreSQL stores IRs, services read on-demand
2. **Services → Runtimes:** Runner invokes RuntimeWrapper implementations
3. **Type Conversion:** Spans service layer (Runner) and execution layer (RuntimeWrapper)

**AI Agent Consistency:**
- Clear crate boundaries (polyglot-lexer, polyglot-parser, etc.)
- Consistent error handling (thiserror for libs, anyhow for bins)
- Structured logging (tracing with instrumentation)
- Naming conventions (snake_case, PascalCase, specific patterns)
- Test organization (unit inline, integration in tests/)

---

_Generated by BMAD Decision Architecture Workflow v2.0_
_Date: 2025-12-17_
_For: hhj_
_Architecture Type: Hybrid (Event-Driven Queue + Compiler Pipeline + Plugin System)_
