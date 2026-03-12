## Project Structure

```
polyglot/
├── Cargo.toml                          # Workspace root
├── polyglot.toml.example               # Example configuration
├── README.md
├── LICENSE
│
├── crates/
│   ├── polyglot-cli/                   # CLI binary (FR54-FR74)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # CLI entry point
│   │   │   ├── commands/               # Subcommands
│   │   │   │   ├── mod.rs
│   │   │   │   ├── compile.rs
│   │   │   │   ├── register.rs
│   │   │   │   ├── activate.rs
│   │   │   │   ├── trigger.rs
│   │   │   │   ├── status.rs
│   │   │   │   └── services.rs
│   │   │   └── config.rs               # Config loading
│   │   └── tests/
│   │
│   ├── polyglot-lexer/                 # Lexer library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── token.rs                # Token types
│   │   │   ├── lexer.rs                # Lexer implementation
│   │   │   └── error.rs                # LexerError (thiserror)
│   │   └── tests/
│   │       └── lexer_tests.rs
│   │
│   ├── polyglot-parser/                # Parser library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs               # Parser implementation
│   │   │   ├── ast.rs                  # AST types
│   │   │   └── error.rs                # ParserError (thiserror)
│   │   └── tests/
│   │       └── parser_tests.rs
│   │
│   ├── polyglot-ir/                    # Intermediate Representation (FR3-FR5)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs                # IR type definitions
│   │   │   ├── pipeline.rs             # Pipeline IR
│   │   │   ├── trigger.rs              # Trigger IR
│   │   │   ├── validation.rs           # IR validation
│   │   │   └── error.rs                # IrError (thiserror)
│   │   └── tests/
│   │
│   ├── polyglot-db/                    # Database layer (FR10-FR18)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── models.rs               # DB models
│   │   │   ├── pipelines.rs            # Pipeline queries
│   │   │   ├── instances.rs            # Instance queries
│   │   │   ├── triggers.rs             # Trigger queries
│   │   │   └── error.rs                # DbError (thiserror)
│   │   ├── migrations/                 # sqlx migrations
│   │   │   ├── 20250116_001_create_pipelines.sql
│   │   │   ├── 20250116_002_create_instances.sql
│   │   │   └── 20250116_003_create_triggers.sql
│   │   └── tests/
│   │
│   ├── polyglot-trigger-monitor/       # Service: Trigger Monitor (FR19-FR26)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # Service entry point
│   │   │   ├── monitor.rs              # TriggerMonitor struct
│   │   │   ├── handlers/               # Trigger handlers
│   │   │   │   ├── mod.rs
│   │   │   │   ├── trait.rs            # TriggerHandler trait
│   │   │   │   ├── time.rs             # TimeTrigger
│   │   │   │   ├── webhook.rs          # WebhookTrigger
│   │   │   │   ├── file_watch.rs       # FileWatchTrigger
│   │   │   │   └── manual.rs           # ManualTrigger
│   │   │   ├── registry.rs             # Dynamic trigger registry
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   ├── polyglot-queue-manager/         # Service: Queue Manager (FR27-FR40)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── manager.rs              # QueueManager struct
│   │   │   ├── queue.rs                # Queue operations
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   ├── polyglot-runner/                # Service: Runner (FR30-FR53)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── runner.rs               # Runner struct
│   │   │   ├── executor.rs             # Pipeline execution
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   └── polyglot-runtime-wrappers/      # Runtime integration (FR41-FR53)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── trait.rs                # RuntimeWrapper trait
│       │   ├── python.rs               # Python runtime wrapper
│       │   └── error.rs                # WrapperError (thiserror)
│       └── tests/
│
├── docs/                               # Documentation (FR84-FR94)
│   ├── v0.0.1/                         # Archived v0.0.1 docs
│   ├── architecture.md                 # This document
│   ├── prd.md
│   └── product-brief-Polyglot-2025-11-15.md
│
├── examples/                           # Example .pg files (FR84-FR94)
│   ├── hello_world.pg
│   ├── python_integration.pg
│   └── automation_workflow.pg
│
└── migrations/                         # Global migrations (symlink to polyglot-db/migrations)
```

## FR Category to Architecture Mapping

| FR Category | Architecture Components |
| ----------- | ----------------------- |
| Pipeline Development & Compilation (FR1-FR9) | polyglot-lexer, polyglot-parser, polyglot-ir, polyglot-cli |
| Pipeline Registry & Lifecycle (FR10-FR18) | polyglot-db, polyglot-cli |
| Trigger System (FR19-FR26) | polyglot-trigger-monitor, polyglot-db |
| Queue Management & Execution (FR27-FR40) | polyglot-queue-manager, polyglot-runner, polyglot-db |
| Runtime Integration & FFI (FR41-FR53) | polyglot-runner, polyglot-runtime-wrappers |
| CLI & Developer Tools (FR54-FR74) | polyglot-cli |
| Installation & Configuration (FR75-FR83) | All crates |
| Documentation & Examples (FR84-FR94) | docs/, examples/ |
| Observability & Monitoring (FR95-FR102) | All services (logging) |
| IDE & Tooling Integration (FR103-FR106) | Future: LSP server |
| Package Ecosystem (FR107-FR111) | Future: registry service |
| Advanced Features (FR112-FR120) | Future enhancements |

