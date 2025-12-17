# Polyglot Architecture Documentation

**Implementation details and system architecture**

---

## Overview

This directory contains technical documentation about Polyglot's internal architecture, runtime systems, and implementation details. This is primarily for contributors, advanced users, and those interested in understanding how Polyglot works under the hood.

**Audience:** Language implementers, advanced users, contributors

---

## Architecture Documents

### 📐 [00 - Architecture Overview](./00-overview.md)
High-level system architecture and design philosophy.

**Topics:**
- System components
- Architecture layers
- Design principles
- Component interactions
- Technology stack

**Covers:**
- Compiler architecture
- Runtime system
- Type checker
- Code generator
- Standard library organization

---

### 🗄 [01 - Database Schema](./01-database-schema.md)
Internal database schema for pipeline registry and state management.

**Topics:**
- Registry database structure
- Pipeline metadata storage
- State persistence
- Indexing strategy
- Query patterns

**Tables:**
- Pipelines registry
- Execution state
- Queue entries
- Trigger configurations
- Error logs

---

### 🔧 [02 - IR Representation](./02-ir-representation.md)
Intermediate Representation (IR) used by the compiler.

**Topics:**
- IR design and structure
- AST to IR transformation
- IR optimization passes
- Type information in IR
- IR serialization format

**Use Cases:**
- Compiler optimization
- Cross-platform code generation
- Static analysis tools
- Debugging utilities

---

### 📬 [03 - Queue System](./03-queue-system.md)
Queue implementation for pipeline execution.

**Topics:**
- Queue architecture
- Queue types (Serial, Parallel, Priority)
- Message format
- Delivery guarantees
- Backpressure handling
- Dead letter queues

**Queue Types:**
- `|Q.Serial` - Sequential execution
- `|Q.Parallel` - Concurrent execution
- `|Q.Priority` - Priority-based scheduling

---

### 📡 [04 - Trigger Monitoring](./04-trigger-monitoring.md)
Trigger system for pipeline activation.

**Topics:**
- Trigger types implementation
- Event monitoring
- Schedule system
- Stream processing
- Trigger state management

**Trigger Types:**
- `|T.Call` - Direct invocation
- `|T.Schedule` - Time-based (cron)
- `|T.Event` - Event-driven
- `|T.Stream` - Stream processing

---

### ⚙️ [05 - Runtime Execution](./05-runtime-execution.md)
Runtime execution engine and lifecycle management.

**Topics:**
- Execution model
- Pipeline lifecycle
- Resource management
- Error propagation
- Wrapper implementation
- State isolation

**Execution Phases:**
1. Pipeline lookup
2. Input validation
3. Queue enqueue
4. Wrapper initialization
5. Pipeline execution
6. Output capture
7. Cleanup

---

## Architecture Diagrams

Each document includes architecture diagrams showing:
- Component relationships
- Data flow
- State transitions
- Interaction patterns

---

## Key Concepts

### Compilation Pipeline

```
Source Code (.pg)
    ↓
Lexer → Tokens
    ↓
Parser → AST
    ↓
Type Checker → Typed AST
    ↓
IR Generator → IR
    ↓
Optimizer → Optimized IR
    ↓
Code Generator → Bytecode (.pgc)
```

### Runtime Architecture

```
┌─────────────────────────────────────┐
│         Application Layer           │
│    (Pipelines, Enums, Errors)       │
└────────────────┬────────────────────┘
                 │
┌────────────────┴────────────────────┐
│         Runtime Engine              │
│  (Execution, Queue, Triggers)       │
└────────────────┬────────────────────┘
                 │
┌────────────────┴────────────────────┐
│       Standard Library              │
│  (Utilities, Wrappers, Operators)   │
└────────────────┬────────────────────┘
                 │
┌────────────────┴────────────────────┐
│         Core Runtime                │
│   (Memory, Types, Primitives)       │
└─────────────────────────────────────┘
```

---

## Implementation Technologies

**Compiler:**
- Written in Rust
- LLVM backend for optimization
- Incremental compilation support

**Runtime:**
- Async/await for concurrency
- Message-passing for queues
- Copy-on-write for data structures

**Storage:**
- SQLite for registry
- Memory-mapped files for IR cache
- JSON for configuration

---

## Performance Characteristics

### Compilation

- **Cold compilation:** ~100ms per 1000 LOC
- **Incremental compilation:** ~10ms per changed function
- **Type checking:** ~50ms per 1000 LOC
- **Code generation:** ~30ms per 1000 LOC

### Runtime

- **Pipeline invocation overhead:** ~10μs
- **Queue latency:** ~100μs (Serial), ~50μs (Parallel)
- **Trigger monitoring:** ~1ms polling interval
- **Memory overhead:** ~500KB per active pipeline

---

## Design Principles

### Compiler Design

1. **Fast feedback** - Syntax errors reported immediately
2. **Incremental** - Only recompile what changed
3. **Helpful errors** - Clear, actionable error messages
4. **Type safety** - Catch errors at compile time

### Runtime Design

1. **Isolation** - Pipelines don't interfere with each other
2. **Predictability** - Deterministic execution order
3. **Observability** - Rich debugging and monitoring
4. **Resource efficiency** - Minimal overhead

---

## Contributing to Architecture

When contributing changes:

1. **Update relevant architecture docs** - Keep docs in sync
2. **Add diagrams** - Visual explanations help
3. **Document trade-offs** - Explain why decisions were made
4. **Benchmark changes** - Measure performance impact
5. **Update IR version** - If IR format changes

---

## Related Documentation

**User-Facing:**
- [Language Documentation](../language/) - What users see
- [CLI Documentation](../cli/) - User-facing tools

**Implementation:**
- [Planning](../planning/) - Future architecture plans
- [Specification](../../specifications/) - Formal specs

**Development:**
- [Contributing Guide](../../../CONTRIBUTING.md) - How to contribute
- [Development Setup](../../../DEV-SETUP.md) - Getting started

---

## Further Reading

**External Resources:**
- [LLVM Documentation](https://llvm.org/docs/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)

**Academic Papers:**
- Type system design
- IR optimization techniques
- Message queue architecture

---

**Last Updated:** 2025-12-15
**Version:** Current
**Maintained by:** Polyglot Core Team
