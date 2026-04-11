---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/technical/brainstorming/marker-declarations.md
---

# System Architecture Overview

**Document Type:** Architecture Design Document
**Version:** 0.0.2
**Last Updated:** 2025-11-14
**Status:** Active Development - Design Phase

---

## Overview

High-level system architecture, component interaction, and design philosophy

**Language Specification:** See [BNF Grammar](../language/bnf/polyglot grammer.md) for complete syntax reference.

---

## Related Documents

### Language Specification
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Authoritative grammar
- [Complete Syntax](../language/01-syntax-complete.md) - Full syntax reference
- [Type System](../language/02-type-system.md) - Type specification

### Other Architecture Documents
- [Database Schema](./01-database-schema.md) - IR storage design
- [IR Representation](./02-ir-representation.md) - Type system & IR structure

---

## Executive Summary

Polyglot is a background service daemon written in Rust that orchestrates multi-language workflow automation. The system compiles `.pg` files into Intermediate Representations (IR) stored in PostgreSQL and executes workflows through three core components: **Trigger Monitor**, **Queue Manager**, and **Runner Pool**.

**Key Architectural Decisions:**
- **Single Binary, Multi-Threaded**: All components run in one process using Tokio async runtime
- **Database-Centric IR Storage**: Normalized PostgreSQL schema stores AST-like IR as relational data
- **Hybrid Communication**: Components use combination of database queries + in-memory channels
- **Process Isolation**: Python (via `uv`) and Rust execute in separate processes
- **User-Defined Behavior**: Queue priority, resource limits, and execution flow configured via Polyglot syntax
- **Strongly Typed**: Type system with deterministic IDE inference and explicit type conversion

**Core Innovation:**
Unlike traditional workflow engines that use YAML/JSON configuration, Polyglot provides a domain-specific programming language with native multi-language orchestration, resource-aware execution, and sophisticated pause/resume capabilities.

---

## System Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Polyglot Daemon (Single Binary)              │
│                                                                   │
│  ┌──────────────────┐  ┌──────────────────┐  ┌───────────────┐ │
│  │ Trigger Monitor  │  │  Queue Manager   │  │  Runner Pool  │ │
│  │                  │  │                  │  │               │ │
│  │ ┌──────────────┐ │  │ ┌──────────────┐ │  │ ┌───────────┐ │ │
│  │ │   Resource   │ │  │ │ Pending Q    │ │  │ │  Python   │ │ │
│  │ │   Monitor    │ │  │ │ Dispatch Q   │ │  │ │  Runners  │ │ │
│  │ │  (Submodule) │ │  │ │ Pause Q      │ │  │ │  (uv)     │ │ │
│  │ └──────────────┘ │  │ └──────────────┘ │  │ └───────────┘ │ │
│  │                  │  │                  │  │               │ │
│  │  - File Watch   │  │  - Validation    │  │ ┌───────────┐ │ │
│  │  - Cron/Schedule│  │  - Priority Mgmt │  │ │   Rust    │ │ │
│  │  - Pipeline Call│  │  - Checkpointing │  │ │  Runners  │ │ │
│  │  - Webhooks     │  │  - Resource Chk  │  │ │ (Compiled)│ │ │
│  └──────────────────┘  └──────────────────┘  │ └───────────┘ │ │
│           │                     │             └───────────────┘ │
│           └─────────┬───────────┘                      │        │
│                     │                                  │        │
└─────────────────────┼──────────────────────────────────┼────────┘
                      │                                  │
                      ▼                                  ▼
            ┌──────────────────┐              ┌──────────────────┐
            │   PostgreSQL     │              │  File System     │
            │   Database       │              │  - .pg Sources   │
            │                  │              │  - Compiled Bins │
            │  - IR Tables     │              │  - Logs          │
            │  - Workflow State│              │  - Temp Files    │
            │  - Queue Entries │              └──────────────────┘
            │  - Resources     │
            └──────────────────┘
```

### Component Interaction Flow

```
User Registration Flow:
─────────────────────
.pg files → Compiler → IR Generation → PostgreSQL Storage
                          │
                          └──→ Pre-compile Rust binaries
                          └──→ Prepare Python environments (uv)

Runtime Execution Flow:
──────────────────────
Trigger Monitor → Detects Event → Creates Queue Entry (Pending)
                                          ↓
Queue Manager → Validates → Checks Resources → Moves to Dispatch
                                          ↓
Runner Pool → Executes Steps → Updates State → Checkpointing
                                          ↓
Queue Manager ← Completion/Pause ← Results/Errors
```

---

## Core Components

### 1. Compiler (Registration Phase)

**Responsibility:** Parse `.pg` files and generate normalized IR stored in PostgreSQL

**Key Features:**
- Lexical analysis and syntax parsing (per v0.0.2 BNF grammar)
- Type checking and validation
- AST generation
- IR table population
- Rust binary pre-compilation
- Python environment preparation (uv)

**Input:**
- `.pg` source files (multi-file packages supported)
- Package manifest/metadata

**Output:**
- Normalized IR in database tables
- Compiled Rust binaries
- Configured Python environments

### 2. Trigger Monitor (Runtime Component)

**Responsibility:** Watch for events and create workflow execution requests

**Sub-Components:**

#### Resource Monitor (Submodule)
- Periodic polling (default: 5 seconds, configurable)
- Monitors: CPU, Memory, GPU, Custom metrics
- Stores resource snapshots in database
- Triggers resource-based workflows

#### Trigger Types:
1. **File Watch** (`inotify` on Linux, planned for other OS)
2. **Schedule** (Cron-like expressions)
3. **Pipeline Call** (Programmatic invocation)
4. **Webhook** (HTTP endpoints)
5. **Resource-Based** (CPU/Memory thresholds)

**Behavior:**
- Reads trigger definitions from IR
- Fires triggers as configured (no validation)
- Creates entries in Queue Manager's Pending Queue

### 3. Queue Manager (Runtime Component)

**Responsibility:** Validate, prioritize, and manage workflow execution lifecycle

**Three Queues:**

```
┌─────────────┐     Validation      ┌──────────────┐     Dispatch     ┌─────────────┐
│  Pending    │ ──────────────────> │   Dispatch   │ ───────────────> │   Runner    │
│   Queue     │     & Resource      │    Queue     │                  │    Pool     │
└─────────────┘       Check         └──────────────┘                  └─────────────┘
       ↑                                    │                                 │
       │                             Resource Wait                      Checkpoint
       │                                    │                                 │
       │                                    ▼                                 ▼
       │                             ┌─────────────┐                   ┌─────────────┐
       └─────────────────────────────│    Pause    │◄──────────────────│  Execution  │
                 Resume              │    Queue    │    Pause Signal   │   State     │
                                     └─────────────┘                   └─────────────┘
```

**Queue Actions:**
- `add` - Add to pending queue
- `kill` - Terminate execution
- `pause` - Move to pause queue (checkpointing)
- `resume` - Move from pause to pending
- `priority_bump` - Adjust priority

**Validation:**
- Check resource availability
- Validate dependencies
- Verify runtime environments
- Apply user-defined queue behavior

**Priority Management:**
- User-defined priority levels
- FIFO within priority tiers
- Resource-based re-prioritization (future)

### 4. Runner Pool (Execution Component)

**Responsibility:** Execute workflow steps in target language runtimes

**Language Runtimes:**

#### Python Runtime (via `uv`)
- Virtual environment per workflow
- Dynamic dependency management
- Process isolation
- Sandboxing capabilities (future)

#### Rust Runtime
- Pre-compiled binaries (created during registration)
- Direct process execution
- Minimal startup overhead

**Execution Model:**
- Process pool (configurable size)
- Parallel execution (`[p]` parallel blocks)
- Join/yield points (`[Y]` join operations)
- JSON-based data serialization (initial implementation)

**Checkpointing:**
- User-defined checkpoint locations
- Serializes execution state to database
- Enables pause/resume functionality
- Granularity: Configurable per pipeline

---

## Technology Stack

### Core Service
- **Language:** Rust 1.70+
- **Async Runtime:** Tokio
- **Parser:** nom or pest (TBD)
- **Serialization:** serde, bincode
- **Database:** PostgreSQL 14+

### Language Integrations
- **Python:** PyO3 for Rust-Python bindings
- **JavaScript:** napi-rs for Rust-Node.js bindings
- **C++:** cxx for Rust-C++ interop
- **FFI:** libffi for dynamic bindings

### APIs & Protocols
- **REST API:** axum or actix-web
- **WebSocket:** tungstenite
- **gRPC:** tonic (future)
- **Message Format:** Protocol Buffers or MessagePack

### Monitoring & Observability
- **Metrics:** Prometheus
- **Tracing:** OpenTelemetry
- **Logging:** tracing crate
- **Dashboards:** Grafana or custom React UI

---

## Design Principles

### 1. Explicit Over Implicit
- No type inference across language boundaries
- No implicit conversions
- All behavior explicitly declared in `.pg` files

### 2. Database as Single Source of Truth
- IR stored in normalized PostgreSQL schema
- All components query database for state
- No in-memory-only state (except caches)

### 3. Process Isolation
- Each language runtime executes in separate process
- Prevents version conflicts
- Enables clean resource limits

### 4. User-Controlled Execution
- User defines queue behavior via Polyglot syntax
- User controls pause/resume points
- User sets resource requirements

### 5. IDE-Friendly Syntax
- Deterministic parsing for autocomplete
- Clear block structure with `[markers]`
- Type annotations for IntelliSense

---

## See Also

- [Database Schema](./01-database-schema.md) - Complete IR storage design
- [IR Representation](./02-ir-representation.md) - Type system details
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Language specification
- [Standard Library](../standard-library/00-overview.md) - Built-in pipelines

---

**Note:** This architecture document is based on v0.0.1 planning with v0.0.2 syntax compliance. All code examples in this document use v0.0.2 syntax per the authoritative [BNF Grammar](../language/bnf/polyglot grammer.md).
