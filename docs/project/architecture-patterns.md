# Architecture Patterns

**Generated:** 2025-12-09
**Project:** Polyglot

---

## Part 1: Compiler Toolchain - Modular Compiler + Async Runtime

### Primary Pattern: Three-Phase Compiler Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    PHASE 1: COMPILATION                      │
│  ┌──────────┐     ┌──────────┐     ┌─────────┐            │
│  │  Lexer   │ --> │  Parser  │ --> │   IR    │            │
│  │ (.pg →   │     │ (Tokens  │     │ (AST →  │            │
│  │  tokens) │     │  → AST)  │     │  JSON)  │            │
│  └──────────┘     └──────────┘     └─────────┘            │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│                   PHASE 2: REGISTRATION                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              PostgreSQL (polyglot-db)                  │  │
│  │  ┌──────────┐  ┌──────────────┐  ┌──────────────┐   │  │
│  │  │ir_nodes  │  │ir_pipelines  │  │  ir_triggers │   │  │
│  │  └──────────┘  └──────────────┘  └──────────────┘   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│                    PHASE 3: RUNTIME                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Trigger    │→ │    Queue     │→ │    Runner    │     │
│  │   Monitor    │  │   Manager    │  │              │     │
│  │              │  │              │  │   ┌───────┐  │     │
│  │ (Watches for │  │ (Schedules & │  │   │ Async │  │     │
│  │  events)     │  │  prioritizes)│  │   │ State │  │     │
│  │              │  │              │  │   │Machine│  │     │
│  └──────────────┘  └──────────────┘  │   └───────┘  │     │
│                                       │       ↓       │     │
│                                       │  ┌─────────┐ │     │
│                                       │  │ Runtime │ │     │
│                                       │  │Wrappers │ │     │
│                                       │  └─────────┘ │     │
│                                       └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Supporting Patterns

#### 1. Repository Pattern (polyglot-db)
- **Purpose:** Abstract database access
- **Implementation:** Single crate provides all DB operations
- **Benefits:** Centralized data access, easy to test with mocks

#### 2. Pipeline Pattern (Architecture)
- **Purpose:** Data flows through stages
- **Stages:** Source → Lexer → Parser → IR → Storage → Execution
- **Benefits:** Clear separation, easy to debug at each stage

#### 3. Actor Model (Runtime Services)
- **Purpose:** Independent services communicating via shared state
- **Actors:**
  - `trigger-monitor` - Event detector
  - `queue-manager` - Task scheduler
  - `runner` - Pipeline executor
- **Communication:** PostgreSQL + Redis act as message channels
- **Benefits:** Scalable, fault-tolerant, distributed

#### 4. State Machine (Variable States)
- **Purpose:** Track async variable lifecycle
- **States:** Declared → Pending → Ready → Faulted
- **Benefits:** Explicit async semantics, deterministic error handling

#### 5. Interpreter Pattern (Runner)
- **Purpose:** Execute IR nodes
- **Implementation:** Walk AST, interpret each node
- **Benefits:** Language-agnostic execution

---

## Part 2: VSCode Extension - Declarative Configuration

### Primary Pattern: VSCode Language Extension

```
┌─────────────────────────────────────────────────────┐
│                VSCode Extension Host                 │
│  ┌────────────────────────────────────────────────┐ │
│  │     polyglot-language-support Extension        │ │
│  │                                                 │ │
│  │  ┌─────────────────────────────────────────┐  │ │
│  │  │       Language Configuration            │  │ │
│  │  │  - File associations (.pg)              │  │ │
│  │  │  - Comment rules                        │  │ │
│  │  │  - Bracket matching                     │  │ │
│  │  └─────────────────────────────────────────┘  │ │
│  │                                                 │ │
│  │  ┌─────────────────────────────────────────┐  │ │
│  │  │       TextMate Grammar                  │  │ │
│  │  │  - Syntax highlighting rules            │  │ │
│  │  │  - Token scopes                         │  │ │
│  │  │  - Pattern matching                     │  │ │
│  │  └─────────────────────────────────────────┘  │ │
│  │                                                 │ │
│  │  ┌─────────────────────────────────────────┐  │ │
│  │  │          Snippets                       │  │ │
│  │  │  - Code templates                       │  │ │
│  │  │  - Tab stops                            │  │ │
│  │  └─────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

### Pattern: Declarative Configuration
- **Approach:** JSON configuration files (no code)
- **Components:**
  - `package.json` - Extension manifest
  - `language-configuration.json` - Editor behavior
  - `polyglot.tmLanguage.json` - Syntax rules
  - `polyglot.json` - Code snippets
- **Benefits:** No runtime code, pure data-driven, easy to maintain

---

## Cross-Part Patterns

### Pattern: Loose Coupling via Shared Specification

```
┌─────────────────────────────────────────────┐
│        Polyglot Language Spec               │
│    (Defined in docs/user/language/)         │
│  - Syntax rules (BNF grammar)               │
│  - Token definitions                        │
│  - Block markers                            │
│  - Operator precedence                      │
└─────────────────────────────────────────────┘
           ↙                           ↘
    ┌──────────┐                  ┌──────────────┐
    │ Compiler │                  │   Extension  │
    │ Toolchain│                  │              │
    │          │                  │  Implements  │
    │Implements│                  │  editor      │
    │language  │                  │  support     │
    └──────────┘                  └──────────────┘
```

**No direct dependency** - both parts independently implement the same specification.

---

## Architecture Quality Attributes

### Compiler Toolchain

| Attribute | Rating | Implementation |
|-----------|--------|----------------|
| **Modularity** | ⭐⭐⭐⭐⭐ | 9 independent crates with clear boundaries |
| **Scalability** | ⭐⭐⭐⭐ | Distributed services (trigger-monitor, queue-manager, runner) |
| **Maintainability** | ⭐⭐⭐⭐⭐ | Small focused crates, strong typing |
| **Performance** | ⭐⭐⭐⭐⭐ | Rust + Tokio async, compiled IR |
| **Reliability** | ⭐⭐⭐⭐ | ACID DB, error handling with thiserror |
| **Testability** | ⭐⭐⭐⭐⭐ | Each crate independently testable |

### VSCode Extension

| Attribute | Rating | Implementation |
|-----------|--------|----------------|
| **Simplicity** | ⭐⭐⭐⭐⭐ | Declarative JSON configuration |
| **Maintainability** | ⭐⭐⭐⭐⭐ | No code, just data files |
| **Extensibility** | ⭐⭐⭐ | Limited to TextMate grammar capabilities |

---

## Design Principles Applied

### SOLID Principles (Compiler)

1. **Single Responsibility:** Each crate has one job
   - `polyglot-lexer` - only tokenization
   - `polyglot-parser` - only parsing
   - `polyglot-db` - only data access

2. **Open/Closed:** Can extend without modifying
   - New triggers added to `trigger-monitor` without changing core
   - New runtime wrappers added to `polyglot-runtime-wrappers`

3. **Liskov Substitution:** Can swap implementations
   - Database layer could swap PostgreSQL for another SQL DB

4. **Interface Segregation:** Small, focused interfaces
   - Each crate exposes minimal public API

5. **Dependency Inversion:** Depend on abstractions
   - Services depend on `polyglot-db` interface, not DB details

### DRY (Don't Repeat Yourself)

- **IR Definition:** Single source of truth in `polyglot-ir` crate
- **Database Layer:** All DB code in `polyglot-db`
- **Common Utilities:** Workspace-level dependencies in root `Cargo.toml`

### Separation of Concerns

- **Compilation ≠ Execution:** Compiler crates don't know about runtime
- **Storage ≠ Logic:** `polyglot-db` is pure data access, no business logic
- **Services:** Each service (monitor, queue, runner) has distinct concern

---

## Patterns NOT Used (and Why)

### Microservices (with REST APIs)
**Why Not:** Overhead of HTTP, serialization. Services communicate via DB.
**Alternative:** Shared database as integration point (simpler for v0.1.0)

### Monolithic Architecture
**Why Not:** Would make testing, scaling difficult
**Alternative:** Modular workspace with clear boundaries

### Traditional Compiler Backend (LLVM)
**Why Not:** Polyglot doesn't compile to machine code, it orchestrates scripts
**Alternative:** Interpreter pattern with FFI to existing runtimes

---

## Future Architecture Evolution

### Potential Changes for v1.0+

1. **gRPC Between Services**
   - Replace DB-based communication with gRPC
   - Better observability, explicit contracts

2. **Plugin System**
   - Allow third-party runtime wrappers
   - Dynamic loading of language integrations

3. **Language Server Protocol (LSP)**
   - Replace TextMate grammar with full LSP
   - IDE-quality features (autocomplete, go-to-definition)

4. **JIT Compilation**
   - Compile hot paths to native code
   - Significant performance boost for compute-heavy pipelines

---

## Summary

**Compiler Toolchain:** Modular, async-first, database-centric compiler and runtime system using industry-standard Rust patterns.

**VSCode Extension:** Simple, declarative language support extension with no custom code.

**Integration:** Loose coupling via shared language specification - both parts independently implement the same standard.
