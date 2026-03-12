## Project Initialization

**First Implementation Story: Initialize Cargo Workspace**

```bash
cargo init polyglot
```

Then configure `Cargo.toml` as a workspace with the following structure:

```toml
[workspace]
members = [
    "crates/polyglot-cli",
    "crates/polyglot-lexer",
    "crates/polyglot-parser",
    "crates/polyglot-ir",
    "crates/polyglot-trigger-monitor",
    "crates/polyglot-queue-manager",
    "crates/polyglot-runner",
    "crates/polyglot-runtime-wrappers",
    "crates/polyglot-db",
]
resolver = "2"
```

**Rationale:** Manual workspace setup provides precise control over service boundaries, shared library organization, and dependency management for Polyglot's specialized language implementation architecture.

## Decision Summary

| Category | Decision | Version | Affects FR Categories | Rationale |
| -------- | -------- | ------- | --------------------- | --------- |
| Project Setup | Manual Cargo Workspace | Rust 2021 Edition | All | Specialized architecture requires custom workspace structure |
| Database Client | SQLx | 0.8.6 | FR10-18, FR27-40, FR95-102 | Async-first design, compile-time query verification, lightweight |
| Database Migrations | sqlx-cli | (bundled) | FR10-18 | Version-controlled SQL migrations |
| Queue Client | redis | 0.32.7 | FR27-40 | Standard Rust Redis client with tokio-comp async support |
| IR Serialization | JSON (serde_json) | 1.0.140 | FR1-9 | Human-readable for MVP debugging, PostgreSQL JSONB native support |
| IR Storage | PostgreSQL JSONB | - | FR3-5, FR10-18 | Hybrid: document storage for IR + relational power for metadata |
| CLI Framework | clap (derive API) | 4.5 | FR54-74 | Standard Rust CLI framework, auto-generated help, type-safe parsing |
| Error Handling (Libs) | thiserror | 2.0.17 | All library crates | Structured error types with custom variants |
| Error Handling (Bins) | anyhow | 1.0.99 | All binary crates | Simple propagation with context chaining, async-safe (Send+Sync) |
| Logging Framework | tracing + tracing-subscriber | 0.1.41 + 0.3.19 | FR95-102 | Async-native structured logging, OpenTelemetry ready |
| Configuration Format | TOML | 0.9.8 | FR75-83 | Rust ecosystem standard, human-readable |
| Configuration Library | config | 0.15.15 | FR75-83 | Layered config (defaults → file → env vars), 12-factor app support |
| Testing Organization | Rust standard | - | All | Unit tests inline with #[cfg(test)], integration tests in tests/, E2E separate |
| IR Type Definitions | Rust structs + serde | - | FR1-9 | .pg types map to Rust enums/structs, serde for JSON serialization |
| Time-Series Database | InfluxDB | 2.x | FR19-26, FR95-102 | Stores time-based triggers, trigger results, resource metrics (CPU/RAM/GPU) |
| IR Structure | 3 Separate IRs | - | FR1-9 | Polyglot code → {Trigger IR, Queue IR, Runner IR} as separate JSONB columns |
| Lexer Generator | logos | 0.14 | FR1-2, FR6-7 | Declarative token definitions, compile-time DFA generation, 45+ token types, <100ms performance |

