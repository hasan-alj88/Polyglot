# Story 1.1: Project Workspace & Build System Setup

Status: ready-for-dev

## Story

As a developer,
I want the Cargo workspace structure established,
so that all crates can be developed and built cohesively.

## Acceptance Criteria

1. **Workspace compiles successfully**
   - Running `cargo build` compiles all 9 crates without errors
   - Workspace configured with proper member crates

2. **All crate directories created**
   - `polyglot-cli` - CLI binary crate
   - `polyglot-lexer` - Lexer library crate
   - `polyglot-parser` - Parser library crate
   - `polyglot-ir` - IR generation library crate
   - `polyglot-db` - Database operations library crate
   - `polyglot-runtime-wrappers` - Runtime wrapper library crate
   - `trigger-monitor` - Trigger Monitor service binary crate
   - `queue-manager` - Queue Manager service binary crate
   - `runner` - Runner service binary crate

3. **Shared dependencies configured**
   - tokio 1.41 (async runtime)
   - sqlx 0.8.6 (database client)
   - redis 0.32.7 with tokio-comp (queue client)
   - serde 1.0.140, serde_json 1.0.140 (serialization)
   - thiserror 2.0.17 (library errors)
   - anyhow 1.0.99 (binary errors)
   - tracing 0.1.41 (logging)
   - clap 4.5 (CLI framework)

4. **Git configuration**
   - `.gitignore` configured for Rust projects
   - Cargo.lock committed for binary crates

5. **CI/CD basics**
   - `.github/workflows/ci.yml` runs `cargo test` and `cargo clippy`
   - Local development documented in `README.md`

## Tasks / Subtasks

- [ ] Initialize Cargo workspace (AC: #1)
  - [ ] Create root `Cargo.toml` with `[workspace]` section
  - [ ] Define workspace members (all 9 crates)
  - [ ] Configure workspace-level dependencies with version inheritance

- [ ] Create library crates structure (AC: #2)
  - [ ] `cargo new --lib polyglot-lexer`
  - [ ] `cargo new --lib polyglot-parser`
  - [ ] `cargo new --lib polyglot-ir`
  - [ ] `cargo new --lib polyglot-db`
  - [ ] `cargo new --lib polyglot-runtime-wrappers`

- [ ] Create binary crates structure (AC: #2)
  - [ ] `cargo new --bin polyglot-cli`
  - [ ] `cargo new --bin trigger-monitor`
  - [ ] `cargo new --bin queue-manager`
  - [ ] `cargo new --bin runner`

- [ ] Configure shared dependencies (AC: #3)
  - [ ] Add workspace dependencies to root `Cargo.toml`
  - [ ] Configure tokio with features: `full`, `macros`
  - [ ] Configure sqlx with features: `postgres`, `runtime-tokio`, `tls-rustls`
  - [ ] Configure redis with feature: `tokio-comp`
  - [ ] Configure serde with feature: `derive`
  - [ ] All errors must be `Send + Sync` for async (architecture requirement)

- [ ] Set up version control (AC: #4)
  - [ ] Create `.gitignore` with Rust patterns (target/, Cargo.lock for libs)
  - [ ] Commit Cargo.lock for binary crates (reproducible builds)
  - [ ] Initialize git repository if not already done

- [ ] Configure CI/CD (AC: #5)
  - [ ] Create `.github/workflows/ci.yml`
  - [ ] Add `cargo test` job
  - [ ] Add `cargo clippy -- -D warnings` job
  - [ ] Add `cargo fmt --check` job (optional but recommended)

- [ ] Document local development (AC: #5)
  - [ ] Update `README.md` with build instructions
  - [ ] Document prerequisites (Rust toolchain, PostgreSQL, Redis, InfluxDB)
  - [ ] Add quick start guide

- [ ] Verify workspace builds (AC: #1)
  - [ ] Run `cargo build` from workspace root
  - [ ] Run `cargo test` from workspace root
  - [ ] Run `cargo clippy` from workspace root
  - [ ] Verify all crates compile successfully

## Dev Notes

### Architecture Context

**From Architecture Document** [Source: docs/architecture.md]

- **ADR-001: Manual Cargo Workspace Over Starter Template**
  - Polyglot's specialized architecture (3 backend services, 5 libraries, 1 CLI) doesn't fit standard templates
  - Manual workspace setup provides full control over crate organization
  - Rationale: Foundation-first approach, language integration built on pipeline system

- **Rust 2021 Edition**
  - Use latest stable Rust features
  - Async/await syntax improvements
  - Better diagnostics

- **Error Handling Strategy** (ADR-004)
  - Libraries use `thiserror` for custom error types
  - Binaries use `anyhow` for error context chaining
  - **Critical**: All errors must implement `Send + Sync` for async compatibility

### Project Structure Notes

**Workspace Organization:**
```
Polyglot/
├── Cargo.toml                    # Workspace root
├── Cargo.lock                    # Workspace lock file
├── polyglot-cli/                # CLI binary
├── polyglot-lexer/              # Lexer library
├── polyglot-parser/             # Parser library
├── polyglot-ir/                 # IR generation library
├── polyglot-db/                 # Database operations library
├── polyglot-runtime-wrappers/   # Runtime wrapper library
├── trigger-monitor/             # Trigger Monitor service
├── queue-manager/               # Queue Manager service
├── runner/                      # Runner service
├── .github/workflows/           # CI/CD configuration
└── README.md                    # Project documentation
```

**Dependency Justification:**

- **tokio 1.41**: Async runtime for all 3 backend services
- **sqlx 0.8.6**: Chosen over Diesel for async-first, compile-time query verification (ADR-002)
- **redis 0.32.7**: Async Redis client with tokio-comp feature
- **serde/serde_json**: IR serialization to JSONB (ADR-003)
- **thiserror/anyhow**: Error handling strategy (ADR-004)
- **tracing 0.1.41**: Structured logging across all crates
- **clap 4.5**: CLI framework with derive API

### Testing Standards

- Unit tests in each crate's `tests/` directory
- Integration tests in workspace-level `tests/` directory
- Use `#[cfg(test)]` modules for internal tests
- Target: >80% code coverage (NFR-M2)

### References

- [Source: docs/architecture.md#Project-Structure]
- [Source: docs/architecture.md#ADR-001-Manual-Cargo-Workspace]
- [Source: docs/architecture.md#ADR-004-Error-Handling]
- [Source: docs/epics.md#Story-1.1]

## Dev Agent Record

### Context Reference

- [Story Context](./1-1-project-workspace-build-system-setup.context.xml)

### Agent Model Used

_To be filled by dev agent_

### Debug Log References

_To be filled by dev agent during implementation_

### Completion Notes List

_To be filled by dev agent upon story completion_

### File List

_To be filled by dev agent with files created/modified/deleted_
