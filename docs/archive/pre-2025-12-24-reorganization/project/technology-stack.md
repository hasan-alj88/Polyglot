# Technology Stack Analysis

**Generated:** 2025-12-09
**Project:** Polyglot Monorepo

---

## Part 1: Compiler Toolchain

### Overview
**Language:** Rust 2021 Edition
**Architecture Pattern:** Modular Compiler + Async Runtime System
**Workspace Type:** Cargo Workspace (9 crates)

### Core Technologies

| Category | Technology | Version | Justification |
|----------|------------|---------|---------------|
| **Language** | Rust | 2021 Edition | Memory safety, performance, strong type system for compiler development |
| **Async Runtime** | Tokio | 1.48.0 | Production-grade async runtime for concurrent pipeline execution |
| **Database (Primary)** | PostgreSQL | via SQLx 0.8.6 | Relational storage for IR, strong ACID guarantees |
| **Database (Cache)** | Redis | 0.32.7 | Fast in-memory cache for runtime state and queue management |
| **Database (Metrics)** | InfluxDB2 | 0.5.2 | Time-series database for performance metrics and monitoring |
| **Serialization** | Serde | 1.0.228 | De-facto standard for Rust serialization |
| **Error Handling** | thiserror + anyhow | 2.0.17 + 1.0.100 | Ergonomic error handling (thiserror for libraries, anyhow for applications) |
| **CLI** | Clap | 4.5.51 | Derive-based CLI argument parsing |
| **Logging** | tracing + tracing-subscriber | 0.1.41 + 0.3.20 | Structured logging with async support |
| **Configuration** | config + toml | 0.14.1 + 0.8.23 | Multi-source configuration management |
| **Date/Time** | chrono | 0.4 | Date/time manipulation for trigger scheduling |
| **UUID** | uuid | 1.10 | Unique identifiers for pipelines and executions |

### Crate Dependency Graph

```
polyglot-cli (Entry Point)
├── polyglot-parser
│   ├── polyglot-lexer
│   └── polyglot-ir
├── polyglot-db
│   └── polyglot-ir
└── polyglot-runtime-wrappers

trigger-monitor (Service)
├── polyglot-db
└── queue-manager

queue-manager (Service)
├── polyglot-db
└── runner

runner (Service)
├── polyglot-db
├── polyglot-ir
└── polyglot-runtime-wrappers
```

### Crate Breakdown

#### 1. polyglot-cli
**Purpose:** Command-line interface
**Dependencies:** clap, config, tokio, tracing
**Responsibility:** User-facing CLI for compile, register, activate, test commands

#### 2. polyglot-lexer
**Purpose:** Lexical analysis (tokenization)
**Dependencies:** serde, thiserror
**Responsibility:** Tokenize `.pg` source files into token stream

#### 3. polyglot-parser
**Purpose:** Syntax analysis (AST generation)
**Dependencies:** polyglot-lexer, polyglot-ir
**Responsibility:** Parse token stream into Abstract Syntax Tree

#### 4. polyglot-ir
**Purpose:** Intermediate Representation
**Dependencies:** serde
**Responsibility:** Define AST nodes, type system, and IR structure for storage

#### 5. polyglot-db
**Purpose:** Database abstraction layer
**Dependencies:** sqlx, redis, influxdb2, tokio
**Responsibility:** Manage PostgreSQL IR storage, Redis state cache, InfluxDB metrics

#### 6. polyglot-runtime-wrappers
**Purpose:** Foreign Function Interface
**Dependencies:** async-trait, serde, tokio
**Responsibility:** Wrap Python, Rust, Go, JS, Julia runtime calls

#### 7. trigger-monitor
**Purpose:** Event monitoring service
**Dependencies:** polyglot-db, tokio, chrono
**Responsibility:** Watch for time-based, file-based, HTTP triggers

#### 8. queue-manager
**Purpose:** Task scheduling service
**Dependencies:** polyglot-db, tokio
**Responsibility:** Prioritize and schedule pipeline executions

#### 9. runner
**Purpose:** Pipeline execution engine
**Dependencies:** polyglot-db, polyglot-ir, polyglot-runtime-wrappers, tokio
**Responsibility:** Execute pipelines, manage async state machine, handle checkpointing

### Architecture Pattern: Modular Compiler

**Phase 1: Compilation (polyglot-cli)**
```
.pg source → polyglot-lexer → tokens → polyglot-parser → AST → polyglot-ir
```

**Phase 2: Registration (polyglot-db)**
```
IR → PostgreSQL storage (ir_nodes, ir_pipelines, ir_triggers tables)
```

**Phase 3: Runtime (Services)**
```
trigger-monitor → detects events → queue-manager → schedules → runner → executes
                                                                    ↓
                                                            polyglot-runtime-wrappers
                                                                    ↓
                                                        Python/Rust/Go/JS/Julia FFI
```

### Key Design Decisions

1. **Separation of Concerns:** Lexer, parser, IR are independent crates for modularity
2. **Database-Centric:** IR stored in PostgreSQL enables distributed runtime
3. **Async-First:** Tokio throughout enables concurrent pipeline execution
4. **Type Safety:** Rust's type system prevents common compiler bugs
5. **Multi-Language:** Runtime wrappers support polyglot execution (hence the name!)

---

## Part 2: VSCode Extension

### Overview
**Name:** polyglot-language-support
**Version:** 0.1.0
**Type:** VSCode Extension (Language Support)
**Minimum VSCode:** 1.60.0

### Technologies

| Category | Technology | Version | Purpose |
|----------|------------|---------|---------|
| **Extension API** | VSCode Extension API | 1.60.0+ | Language integration |
| **Grammar** | TextMate Grammar | JSON | Syntax highlighting rules |
| **Packaging** | @vscode/vsce | 2.19.0 | Extension packaging and publishing |

### Features Provided

1. **Language Definition**
   - File extension: `.pg`
   - Language ID: `polyglot`
   - Configuration: `language-configuration.json`

2. **Syntax Highlighting**
   - Grammar: `syntaxes/polyglot.tmLanguage.json`
   - Markdown injection for code blocks

3. **Snippets**
   - Code snippets: `snippets/polyglot.json`

4. **Editor Integration**
   - Bracket matching
   - Comment toggling
   - Auto-indentation

### Architecture Pattern: VSCode Language Extension

**Structure:**
```
vscode-extension/
├── package.json                          # Extension manifest
├── language-configuration.json           # Language config
├── syntaxes/
│   ├── polyglot.tmLanguage.json         # Main grammar
│   └── polyglot.markdown.injection.json # Markdown support
└── snippets/
    └── polyglot.json                    # Code snippets
```

**Integration Flow:**
```
VSCode Editor
    ↓
polyglot-language-support extension
    ↓
TextMate grammar rules
    ↓
Syntax highlighted .pg files
```

---

## Inter-Part Integration

### Integration Type: Loose Coupling

The two parts are **independent** with **indirect integration**:

1. **Compiler Toolchain** → Defines the `.pg` language specification
2. **VSCode Extension** → Implements editor support for `.pg` files

**No direct code dependency:**
- Extension doesn't call compiler
- Compiler doesn't depend on extension
- Integration is via the **language definition** (syntax rules, file format)

**Deployment:**
- Compiler: Distributed as `polyglot` CLI binary
- Extension: Distributed via VSCode Marketplace

---

## Technology Rationale Summary

### Why Rust?
- **Memory safety:** Critical for compiler correctness
- **Performance:** Near C++ speed for compilation and runtime
- **Concurrency:** Tokio enables efficient async pipeline execution
- **Type system:** Catches bugs at compile time

### Why Tokio?
- **Industry standard:** Most mature Rust async runtime
- **Ecosystem:** Works with SQLx, Redis, InfluxDB clients
- **Performance:** Efficient scheduling for concurrent pipelines

### Why PostgreSQL + Redis + InfluxDB?
- **PostgreSQL:** ACID guarantees for IR storage, relational queries
- **Redis:** Fast cache for runtime state (pending/ready variables)
- **InfluxDB:** Optimized for time-series metrics (execution duration, queue depth)

### Why Multiple Databases?
- **Right tool for the job:** Each database optimized for its use case
- **Separation of concerns:** IR ≠ state ≠ metrics
- **Scalability:** Can scale databases independently

---

## Development Environment

### Prerequisites
- Rust 1.70+ (with Cargo)
- PostgreSQL 14+
- Redis 6+
- InfluxDB 2.0+
- Node.js 16+ (for VSCode extension packaging)

### Build Commands
```bash
# Compiler toolchain
cargo build --release

# VSCode extension
cd syntax-highlighting/vscode-extension
npm run package
```

### Test Commands
```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p polyglot-lexer
cargo test -p polyglot-parser
```

---

## Version Information

- **Workspace Version:** 0.1.0
- **Rust Edition:** 2021
- **License:** MIT OR Apache-2.0
- **Repository:** https://github.com/polyglot-lang/polyglot
