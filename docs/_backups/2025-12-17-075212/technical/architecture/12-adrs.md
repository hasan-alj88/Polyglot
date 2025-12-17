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

**Decision:** Use PostgreSQL with JSONB column for IR, relational columns for metadata.

**Rationale:**
- Best of both worlds: document storage + relational queries
- JSONB supports indexing and querying
- One database instead of PostgreSQL + MongoDB
- ACID guarantees for state management

**Consequences:**
- IR stored as JSON (human-readable for debugging)
- Can query inside IR if needed: `WHERE ir->>'trigger_type' = 'manual'`
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
- Handlers are spawned/stopped on IR changes
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
- Failure isolation (services can restart independently)

**Consequences:**
- All services depend on PostgreSQL/Redis availability
- No need for HTTP servers in services (except future webhooks)
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
- After compile, pipelines are available for activation
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

**Decision:** Use PostgreSQL as fallback when Redis is unavailable. Queue Manager polls `pipeline_instances WHERE status='queued'`.

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

**Context:** Story 1.2 (Lexer Token Definitions) requires tokenizing 45+ distinct token types from Polyglot v0.0.2 syntax specification, including operators, block markers, comparison operators, negation operators, range operators, and pattern operators. The lexer must achieve <100ms performance for 1000-line files (NFR-P1) while maintaining high code quality and maintainability.

**Decision:** Use the `logos` crate (version 0.14) for declarative token definition and automatic lexer generation, rather than hand-writing a manual lexer.

**Rationale:**
- **"Don't Reinvent the Wheel"** - Core Polyglot philosophy applies to Polyglot's own implementation
- **Battle-tested** - Logos is used by tree-sitter, rustpython, and other production parsers (2.5k+ GitHub stars, actively maintained)
- **Performance** - Proc macro generates optimized DFA at compile time, proven to meet <100ms requirement
- **Maintainability** - 45+ token types as regex annotations vs. 500-800 lines of manual matching logic
- **Conciseness** - Token definitions are self-documenting and easier to update
- **Fast iteration** - Adding new token types requires only enum variant + regex annotation

**Alternatives Considered:**
- **Manual lexer** - Rejected due to high bug surface area with 45+ token types, slower development time, and no performance advantage over logos
- **Nom parser combinators** - Rejected as better suited for parsing than lexing, unnecessary complexity
- **LALRPOP** - Rejected as it's a parser generator, not a lexer generator

**Consequences:**
- ✅ Adds `logos = "0.14"` dependency to workspace (via workspace inheritance per ADR-001)
- ✅ Compile times increase by ~2-5 seconds for initial build due to proc macro generation
- ✅ Lexer implementation (Story 1.3) reduces from ~500-800 LOC to ~200 LOC
- ✅ Token definitions in `token.rs` use `#[derive(Logos)]` with regex annotations
- ⚠️ Logos errors wrapped in custom `LexerError` type (using `thiserror` per ADR-004) for consistent error handling
- ⚠️ Source location tracking (line, column) implemented manually on top of logos byte positions
- ⚠️ If logos proves insufficient for special cases (unlikely), fallback to manual implementation for specific token types only

**Implementation Notes:**
- Story 1.2: Define `TokenType` enum with ~60 variants + 2 compound token types
- Story 1.3: Add `#[derive(Logos)]` and regex patterns for each token type
- Custom `LexerError` wrapper maintains `Send + Sync` requirement for async compatibility
- Unit tests still verify token recognition correctness (>80% coverage per AC#4)

**Compound Token Types (Design Decision 2025-11-17):**

1. **StringLiteral** - Pipeline syntax sugar for string processing
   ```rust
   pub struct StringLiteral {
       /// Optional pipeline for string processing
       /// If None, defaults to |String.Formatted
       /// Format: String.{runtime_lang}.{type}.Format.{format_id}
       /// Example: "String.Python.int.Format.Hex"
       pipeline: Option<String>,

       /// UTF-8 string content (Unicode supported)
       content: String,

       location: Location,
   }
   ```
   - **Rationale:** Literals in Polyglot are syntax sugar for pipeline invocations
   - Bare strings `"text"` default to `|String.Formatted` with `{.var:fmt}` substitution support
   - Pipeline-prefixed strings invoke specific formatting pipelines: `String.Python.int.Format.Hex"42"`
   - Compiler validates (Epic 2): pipeline exists and yields `pg\string` type

2. **RangeCheck** - Bracket notation for range comparisons
   ```rust
   pub enum RangeValue {
       Variable(String),      // .var_name placeholder
       Value(LiteralValue),   // Evaluatable literal
   }

   pub struct RangeCheck {
       left: RangeValue,           // Variable being checked
       start: RangeValue,          // Range start
       end: RangeValue,            // Range end
       start_inclusive: bool,      // [ = true, ( = false
       end_inclusive: bool,        // ] = true, ) = false
       location: Location,
   }
   ```
   - **Rationale:** `.var ?(start, end]` is a single logical construct, not separate tokens
   - Supports inclusive/exclusive boundaries: `[` vs `(` for start, `]` vs `)` for end
   - Can contain variables or literal values for dynamic range checking

**Token Count Update:**
- 27 block markers (single-char in brackets)
- ~24 operators (including comparison, negation, range, pattern)
- Type notation tokens (`:` colon, `\` backslash in type context)
- Comments, identifiers, literals, structural tokens
- **Total: ~60 base token types + 2 compound types**

**Cross-bridges When We Come To Them:**
If logos cannot handle a specific Polyglot syntax pattern (e.g., nested block markers with complex state), implement manual tokenization for ONLY that pattern while keeping logos for the rest. No such special cases identified as of 2025-11-17.

---

### ADR-014: String Concatenation Operator Change (`+"` vs `>"`)

**Status:** Accepted

**Context:** During Story 1.4 (Parser AST Definitions) review, the string concatenation operator `>"` was identified as non-intuitive and inconsistent with industry standards. The operator is used with line continuation marker `[*]` for explicit string literal concatenation in Polyglot v0.0.2.

**Decision:** Change the string concatenation operator from `>"` to `+"` throughout the language specification, documentation, lexer implementation, and all examples.

**Rationale:**
- **Industry Standards** - `+` is universally recognized for concatenation (JavaScript, Python, Java, C#, etc.)
- **Intuitive** - `+"` visually suggests "add/append string", while `>"` looks like "greater than quote"
- **Consistency** - Aligns with Polyglot's principle of "explicit over implicit" while using familiar operators
- **Visual Clarity** - `+"` is clearer in code: `"Hello" +" " +" "World"` vs `"Hello" >" " >" "World"`
- **Low Risk** - Language not yet in production, lexer just implemented, no user code to migrate

**Alternatives Considered:**
- **Keep `>"` operator** - Rejected due to poor intuitiveness and deviation from industry standards
- **Use `++` operator** - Rejected to avoid confusion with increment operators in other languages
- **Use `&` operator** - Rejected as it's used for boolean AND block marker `[&]` in Polyglot

**Consequences:**
- ✅ Improved developer experience for new Polyglot users (familiar syntax)
- ✅ Reduced cognitive load when reading multi-line string concatenations
- ✅ Better alignment with "Don't Reinvent the Wheel" philosophy
- ✅ Documentation updated (57 occurrences in `docs/user/language/line-continuation.md`)
- ✅ Lexer updated with new `TokenKind::OpStringConcat` token type
- ✅ All 26 lexer tests still passing after change
- ⚠️ Parser must validate `+"` only used between string literals (not variables - use interpolation instead)
- ℹ️ Total implementation time: ~2 hours (Medium impact, High value)

**Implementation Summary (2025-11-27):**
1. Updated 24 documentation files (`sed` global replacement)
2. Added `TokenKind::OpStringConcat` to `polyglot-lexer/src/token.rs`
3. Implemented `+"` recognition in `polyglot-lexer/src/lexer.rs`
4. Validated with `cargo test --workspace` (all tests passing)
5. Story 1.4 drafted with correct syntax (no updates needed)

**Parser Validation Requirements:**
- `+"` operator MUST only appear between string literals
- Variables use interpolation: `"{.var1} {.var2}"` NOT `"text" +" .var +" "text"`
- Line continuation `[*]` joins token streams; `+"` is required between literals

**Cross-Reference:**
- User Documentation: `docs/user/language/line-continuation.md`
- Token Definition: `polyglot-lexer/src/token.rs:69` (`OpStringConcat`)
- Lexer Implementation: `polyglot-lexer/src/lexer.rs`

---

_Generated by BMAD Decision Architecture Workflow v1.0_
_Date: 2025-11-16_
