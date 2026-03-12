# Polyglot Project Structure

**Generated:** 2025-12-09
**Repository Type:** Monorepo (Cargo Workspace)

## Overview

Polyglot is a modular compiler and runtime system implemented as a Rust workspace with 9 crates, plus tooling support through a VSCode extension.

## Parts

### Part 1: Compiler Toolchain (Main)

**Type:** Backend/Library
**Location:** Root + workspace crates
**Project Type ID:** backend
**Language:** Rust 2021
**Architecture:** Modular compiler with async runtime

**Workspace Members:**

1. **polyglot-cli** - Command-line interface for compiling and running Polyglot programs
2. **polyglot-lexer** - Lexical analyzer (tokenization)
3. **polyglot-parser** - Recursive descent parser (AST generation)
4. **polyglot-ir** - Intermediate representation layer
5. **polyglot-db** - Database abstraction layer (PostgreSQL, Redis, InfluxDB)
6. **polyglot-runtime-wrappers** - Foreign function interface wrappers (Python, Rust, Go, JS, Julia)
7. **trigger-monitor** - Event trigger monitoring system
8. **queue-manager** - Task queue management and scheduling
9. **runner** - Pipeline execution engine

**Key Technologies:**
- Rust 2021 Edition
- Tokio async runtime
- SQLx (PostgreSQL)
- Redis client
- InfluxDB2 client
- Serde (serialization)
- Clap (CLI)

**Architecture Pattern:**
- Compiler frontend: lexer → parser → IR
- Runtime backend: trigger-monitor → queue-manager → runner
- Storage layer: polyglot-db (PostgreSQL for IR, Redis for state, InfluxDB for metrics)

### Part 2: VSCode Extension

**Type:** Extension
**Location:** `syntax-highlighting/vscode-extension/`
**Project Type ID:** extension
**Language:** TypeScript/Node.js
**Purpose:** Syntax highlighting and language support for `.pg` files

**Key Features:**
- Syntax highlighting for Polyglot language
- TextMate grammar definitions
- VSCode language configuration

## Repository Structure

```
polyglot/
├── polyglot-cli/           # CLI tool (Part 1)
├── polyglot-lexer/         # Lexer (Part 1)
├── polyglot-parser/        # Parser (Part 1)
├── polyglot-ir/            # IR layer (Part 1)
├── polyglot-db/            # Database layer (Part 1)
├── polyglot-runtime-wrappers/  # FFI wrappers (Part 1)
├── trigger-monitor/        # Trigger system (Part 1)
├── queue-manager/          # Queue system (Part 1)
├── runner/                 # Execution engine (Part 1)
├── syntax-highlighting/
│   └── vscode-extension/   # VSCode extension (Part 2)
├── docs/                   # Documentation
│   ├── project/            # Project planning docs
│   ├── technical/          # Technical architecture
│   └── user/               # User documentation
├── bmad/                   # BMAD workflow system
├── Cargo.toml              # Workspace manifest
└── README.md
```

## Integration Points

The two parts are largely independent:

1. **Compiler Toolchain** → Produces `.pg` language and runtime
2. **VSCode Extension** → Provides editor support for `.pg` files

The extension depends on the language definition but doesn't directly integrate with the runtime.
