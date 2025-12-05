# Product Brief: Polyglot

**Date:** 2025-11-15
**Author:** hhj
**Context:** Software Development Language (Level 3, Greenfield)

---

## Executive Summary

Polyglot is an asynchronous automation language designed to solve two fundamental problems that plague modern software development: the complexity of cross-language integration and the inability to manage task priorities descriptively in automated workflows.

Born from a conservative coding philosophy—"Don't reinvent the wheel, use well-tested legacy code (even if it's in another language!)"—Polyglot aims to make it effortless for developers to use the "right tool for the right job" by automating the entire cross-language integration process and providing fine-grained control over resource allocation and task prioritization.

Drawing from real-world production automation experience, Polyglot addresses the persistent challenge where resource-intensive low-priority tasks (like report generation, data processing, batch jobs) sideline critical high-priority operations by locking CPU, RAM, and database connection pools. Current automation tools lack the descriptive power to prevent or handle these scenarios—Polyglot fills this gap.

---

## Core Vision

### Problem Statement

**Problem 1: Cross-Language Integration Complexity**

Integrating code across programming languages requires manual, repetitive setup work that discourages developers from choosing the best tool for each task. For example:
- Using C++ from Python requires setting up pybind, writing bridge code, and configuration
- Rust-Python integration requires different tools and setup
- Each language pair requires learning new FFI frameworks and manual wiring

This friction prevents developers from leveraging well-tested legacy code in different languages, leading to unnecessary rewrites and less robust solutions.

**Problem 2: Automated Task Priority Management**

In production automation environments, resource-intensive low-priority tasks routinely interfere with high-priority operations:
- Background jobs (report generation, data processing, batch operations) consume CPU, RAM, and database connection pools
- High-priority tasks get delayed or fail due to resource starvation
- No matter how well-designed the automation software is, resource conflicts remain unsolved
- Current automation tools cannot be instructed to pause low-priority tasks when high-priority work arrives
- No mechanism exists to reserve resources for critical operations

### Problem Impact

**Cross-Language Integration Impact:**
- Development time wasted on repetitive FFI setup instead of solving business problems
- Developers avoid using optimal tools due to integration friction
- Code quality suffers when suboptimal languages are chosen for convenience
- Technical debt accumulates from language-specific reimplementations of existing solutions

**Automation Priority Management Impact:**
- Resource-intensive background jobs (report generation, data processing, batch operations) lock critical resources:
  - CPU cycles consumed by heavy computation
  - RAM filled with intermediate results
  - Database connection pools exhausted
- High-priority operations (user requests, critical transactions, time-sensitive tasks) become blocked
- Manual intervention required to kill low-priority jobs and free resources
- No automated way to prevent the scenario or handle it gracefully
- Service degradation during peak hours when background jobs coincide with critical operations

### Why Existing Solutions Fall Short

**Cross-Language Integration Tools:**
- **pybind (C++/Python):** Requires manual setup, boilerplate code, build configuration
- **PyO3 (Rust/Python):** Different tool, different learning curve, separate setup process
- **Each language pair needs its own solution:** No unified approach to FFI
- **No automation:** Every integration is manual work

**Workflow Automation Tools:**
- **GUI automation software (e.g., RPA tools):** Cannot express priority-based resource management
- **Traditional job schedulers:** Priority queues exist, but no fine-grained control over resource pause/resume
- **Lack of descriptive power:** Cannot instruct automation to "pause reporting if high-priority work arrives" or "always reserve 20% of DB connections for urgent tasks"
- **No introspection:** Cannot inspect running job resource consumption and make decisions

### Proposed Solution

Polyglot is an **asynchronous automation language for multi-runtime workflow orchestration** that solves both problems through a unified architecture:

**For Cross-Language Integration:**
- **Automated FFI abstraction:** Polyglot handles all language integration complexity under the hood
- **Runtime wrappers:** Support for Python, Node, Rust, Go, Ruby, Deno with unified syntax
- **Zero manual setup:** Developers write descriptive Polyglot code; integration happens automatically
- **Encourage best practices:** Make it easy to choose the right tool for each task

**For Priority Management:**
- **Multiple dispatch queues:** User-defined queues with custom priority algorithms
- **Descriptive control:** Syntax to pause, resume, and route tasks based on priority and resource availability
- **Resource isolation:** Separate queues for GPU vs CPU workloads, expensive vs cheap operations
- **Rate limiting and cost optimization:** Built-in queue characteristics for throttling

**Architecture:**
- **3 backend services:**
  1. **Trigger Monitor:** Continuously monitors pipeline triggers (synchronous component)
  2. **Queue Manager:** Manages multiple dispatch queues with configurable priority algorithms
  3. **Runner:** Executes pipeline instances across different language runtimes
- **Database-driven communication:** PostgreSQL for IR storage, registry, activation state
- **Redis for queue management:** Fast queue operations, with future scalability to Kafka
- **Observability-first:** InfluxDB for time-series logs, OpenTelemetry built-in from day one

**Key Syntax Examples:**
```polyglot
# Define custom dispatch queue
[#] #Queues.Dispatch.NewQueue

# Route pipeline to specific queue
[Q] |Q.Queue.Assign

# Use Python runtime wrapper
[W] |W.Python3.11

# Use Rust runtime wrapper
[W] |W.Rust
```

**Concrete Example: Python User Calling C++ Without FFI Hassle**

Today, a Python developer who wants C++ performance must:
1. Learn pybind11 or similar FFI framework
2. Write C++ wrapper code
3. Configure build system (CMake, setup.py)
4. Compile and link
5. Debug cross-language issues
6. Maintain the integration layer

**With Polyglot:**
```python
import polyglot as pg

# Load C++ header - Polyglot handles all FFI complexity
pg.load_cpp_header("fast_math.h")

# Call C++ function directly - no pybind setup needed
result = pg.cpp.fast_math.matrix_multiply(matrix_a, matrix_b)
```

Polyglot takes care of:
- Type conversion (Python → C++ and back)
- Memory management
- FFI bridge generation
- Error handling across language boundaries
- All the integration complexity developers currently manage manually

The developer focuses on **what** they want to do (use fast C++ math), not **how** to wire languages together.

### Key Differentiators

1. **Runtime vs Compile-Time Crossing (The Breakthrough)**
   - Polyglot's async architecture enables type resolution at **runtime** instead of compile-time
   - **Example:** Python passes a dynamic list → Polyglot determines size at runtime → Rust receives fixed-size array
   - Compile-time languages (GraalVM, traditional FFI) cannot bridge this gap—they need sizes known upfront
   - This unlocks interoperability that was previously impossible

2. **Dual-Strategy Type Conversion**
   - **Strategy 1 (Universal Translator):** All Polyglot data is serialized (pipelines, configurations, datetime, etc.)
     - Acts as intermediate representation: `py\str` → `pg\string` → `c++\std::string`
     - Unified conversion path for any language pair
   - **Strategy 2 (Direct Conversion):** Leverage existing integration tools or slight modifications
     - Direct language-to-language when optimal
     - Falls back to Strategy 1 when direct path unavailable
   - Best of both worlds: flexibility + performance

3. **Philosophy-Driven Design:** "Don't reinvent the wheel, use well-tested legacy code" is architectural, not aspirational

4. **Async-First Architecture:** Built on asynchronous foundations, enabling runtime flexibility that sync languages cannot achieve

5. **Unified FFI Abstraction:** One language for all cross-language integration, not N² tools for N languages

6. **Descriptive Priority Control:** Express resource management intent in code, not infrastructure configuration

7. **Observability Built-In:** OpenTelemetry and time-series logging from the start, not bolted on later

8. **Right Tool for Right Job:** Technical architecture specifically designed to encourage polyglot development

---

## Target Users

### Primary Users

**Conservative Polyglot Developers**

These are developers who value battle-tested code over language purity. They:
- **Current behavior:** Reluctantly rewrite functionality in their primary language instead of using superior implementations in other languages due to integration friction
- **Specific frustration:** "I know this C++ library is faster and more robust, but setting up pybind and maintaining the bridge code isn't worth it"
- **What they'd value most:** Seamless integration—write one line of Polyglot code to call the C++ library instead of hours of FFI setup
- **Technical comfort:** High—comfortable with multiple languages, understand trade-offs, but frustrated by integration tax

**Production Operations Engineers**

Engineers responsible for automated workflows in production environments (fintech, enterprise automation, data processing). They:
- **Current behavior:** Manually monitor resource usage, kill runaway low-priority jobs, restart high-priority tasks
- **Specific frustration:** "The nightly report generation always causes issues during business hours, but I can't tell the automation to be smart about it"
- **What they'd value most:** Descriptive control to say "pause batch processing if high-priority requests are waiting" or "always reserve resources for priority 1 tasks"
- **Technical comfort:** Medium to high—understand systems, databases, resource management, but not necessarily multi-language development

---

## MVP Scope

### Implementation Phases

Polyglot's architecture dictates a **foundation-first approach**: language integration is built on top of the automated pipeline system, not alongside it.

**Phase 1: Core Pipeline System (MVP Foundation)**
1. **Language Syntax:** Complete Polyglot syntax specification (v0.0.2)
2. **Lexer/Parser:** Transform `.pg` files into Intermediate Representation (IR)
3. **Backend Services:** Implement the 3 backend services
   - Trigger Monitor (synchronous, continuous monitoring)
   - Queue Manager (dispatch queue management)
   - Runner (pipeline execution engine)
4. **Database Integration:** PostgreSQL for IR storage, registry, activation state

**Phase 2: Language Integration Layer (Built on Foundation)**
- Runtime wrappers for Python, Node, Rust, Go
- FFI abstraction and type conversion
- Cross-language bridge generation
- The "Python calls C++" magic happens here

### Core Features (Must-Have for MVP)

To prove Polyglot's value, the MVP must demonstrate:

1. **Pipeline Definition and Execution**
   - Define pipelines using Polyglot syntax (`[|]`, `[X]`, `[r]`, `[p]`, etc.)
   - Compile `.pg` files to IR
   - Register pipelines to the registry
   - Activate/deactivate pipelines via CLI

2. **3 Backend Services Operating**
   - Trigger Monitor detecting pipeline triggers
   - Queue Manager managing pending → dispatch transitions
   - Runner executing pipeline instances

3. **Basic Queue Management**
   - Single dispatch queue (multiple queues deferred to post-MVP)
   - Pipeline instance lifecycle: Created → Queued → Running → Exited

4. **Trigger System**
   - At least one trigger type working (e.g., manual trigger via CLI or time-based)
   - Demonstrates trigger monitoring → queue → execution flow

5. **Database Persistence**
   - IR stored in PostgreSQL
   - Pipeline registry maintained
   - Activation state tracked

6. **Basic Runtime Wrapper**
   - At least one runtime wrapper functional (likely Python as proof-of-concept)
   - Demonstrates pipeline can execute code in another language
   - Foundation for future FFI abstraction

### Out of Scope for MVP

Features deferred to post-MVP releases:

**Deferred to v0.2-v0.3:**
- Multiple dispatch queues with priority algorithms
- Advanced queue routing (`[Q] |Q.Queue.Assign`)
- Pause queue functionality
- Queue-level resource isolation

**Deferred to v0.5-v1.0:**
- OpenTelemetry integration (observability-first, but not blocking MVP)
- InfluxDB for time-series logs (start with PostgreSQL logging)
- Advanced runtime wrappers (Node, Rust, Go)
- Full FFI abstraction and dual-strategy type conversion

**Deferred to v1.0+:**
- Compensation pipelines (undo system)
- Irreversibility warnings
- AI-powered queue optimization
- Code-level checkpoints

### MVP Success Criteria

The MVP successfully proves Polyglot's core value when:

1. **A developer can write a `.pg` pipeline** that orchestrates async operations
2. **The pipeline compiles to IR** and registers successfully
3. **Trigger monitoring detects** when the pipeline should run
4. **Queue Manager queues the instance** and Runner executes it
5. **Pipeline executes code** in at least one external runtime (Python)
6. **End-to-end flow works:** Define → Compile → Register → Activate → Trigger → Queue → Execute

This proves the **foundation** works. Language integration magic comes next.

### Future Vision Features

Beyond MVP, the roadmap includes:
- **Immediate (v0.2-v0.5):** Multiple queues, priority algorithms, observability stack
- **Medium-term (v0.5-v1.0):** Full multi-language runtime wrappers, FFI abstraction, compensation pipelines
- **Long-term (v1.0+):** AI optimization, universal type bridge, live code migration

---

## Technical Preferences

### Implementation Language: Rust (Recommended)

**Rationale for Rust:**

Polyglot's architecture demands high-performance async execution, robust serialization, and safety guarantees that Rust uniquely provides:

1. **Async/Await Ecosystem:** Tokio and async-std are production-proven for concurrent backend services (exactly Polyglot's 3 services architecture)

2. **Serialization Excellence:** All Polyglot data is serialized—`serde` is world-class for this:
   - Type-safe serialization/deserialization
   - Supports all formats (JSON, MessagePack, bincode, custom)
   - Zero-copy deserialization possible
   - Perfect for IR representation

3. **Performance + Memory Safety:**
   - No garbage collection pauses (critical for Trigger Monitor's continuous operation)
   - Zero-cost abstractions—pay only for what you use
   - Memory safety without runtime overhead

4. **Database and Queue Integration:**
   - PostgreSQL: `sqlx` (compile-time checked queries) or `diesel` (type-safe ORM)
   - Redis: `redis-rs` with async support
   - InfluxDB: Official Rust client available

5. **Strong Type System for IR Design:**
   - Models Intermediate Representation precisely
   - Compile-time guarantees about IR structure
   - Prevents entire classes of bugs

6. **FFI Readiness for Phase 2:**
   - Mature FFI capabilities for calling C/C++
   - `PyO3` for Python integration
   - Foundation for runtime wrapper implementation

7. **Open-Source Community Alignment:**
   - Infrastructure projects (parsers, databases, runtimes) increasingly adopt Rust
   - Aligns with target audience (polyglot developers comfortable with systems languages)

**Trade-off:** Steeper learning curve than Go or Python, but Polyglot is a language implementation—precision, performance, and correctness outweigh rapid prototyping.

**Alternative:** Go offers simpler concurrency and good database/Redis support, but lacks Rust's serialization elegance and fine-grained performance control.

### Technology Stack Summary

- **Implementation:** Rust
- **Database:** PostgreSQL (IR storage, registry, activation state)
- **Queue System:** Redis (start), Kafka (scale later if needed)
- **Observability:** InfluxDB + OpenTelemetry (post-MVP)
- **Deployment:** TBD (likely containerized with Docker)

---

## Risks and Assumptions

### Key Uncertainties

**1. Direct Type Conversion Without Intermediate Step**
- **Risk:** Strategy 2 (direct language-to-language conversion) requires deep knowledge of each language pair's FFI
- **Uncertainty:** How reliably can we auto-detect when direct conversion is safe vs. requiring intermediate serialization?
- **Mitigation:** Start with Strategy 1 (serialized IR) for all conversions in Phase 2, add Strategy 2 optimizations incrementally
- **Assumption:** Serialization overhead is acceptable for MVP; performance optimization comes later

**2. Checkpoint Pause Functionality**
- **Risk:** Code-level checkpoints (pausing Python/Rust/Node mid-execution) require deep runtime integration
- **Uncertainty:** Can we inject cooperative multitasking hooks into arbitrary language runtimes?
- **Mitigation:** Defer to v2.0+; start with process-level pause (simpler, still valuable for resource management)
- **Assumption:** Process-level pause provides 80% of the value; code-level checkpoints are "nice-to-have"

**3. Performance at Scale**
- **Risk:** Queue Manager handling thousands of concurrent pipeline instances may hit performance bottlenecks
- **Uncertainty:** Will Redis handle the queue throughput? Will PostgreSQL IR queries become bottleneck?
- **Mitigation:**
  - Start with single-instance deployment for MVP
  - Profile early and often
  - Redis → Kafka migration path already planned
  - Database query optimization and indexing strategies
- **Assumption:** Single-instance deployment handles 100-1000 concurrent pipelines; horizontal scaling comes post-MVP

### Critical Assumptions

1. **IR Design:** Serialized IR can represent all Polyglot constructs without ambiguity
2. **Database Performance:** PostgreSQL can handle IR storage/retrieval at required throughput
3. **Runtime Wrapper Feasibility:** Wrapping Python/Node/Rust/Go is technically achievable without modifying language runtimes
4. **Adoption Hypothesis:** Developers frustrated with FFI complexity will adopt a new language to solve it
5. **Community Contribution:** Open-source community will contribute runtime wrappers and standard library utilities

---

## Success Metrics

### Long-Term Success Vision: Open-Source Community Adoption

Polyglot succeeds when it becomes the **de facto solution for cross-language automation** in the open-source ecosystem.

**Adoption Indicators:**

1. **Developer Adoption:**
   - GitHub stars and forks trending upward
   - Active community contributions (runtime wrappers, standard library utilities, example pipelines)
   - Developers blogging/tweeting about using Polyglot to solve real problems

2. **Production Usage:**
   - Companies using Polyglot for production automation workflows
   - Case studies of Polyglot replacing manual FFI setup or proprietary automation tools
   - Integration with popular tools (CI/CD pipelines, data processing frameworks)

3. **Ecosystem Growth:**
   - Third-party packages published to Polyglot registry
   - Runtime wrappers for additional languages (contributed by community)
   - IDE plugins and tooling (syntax highlighting, debuggers, LSP servers)

4. **Community Health:**
   - Active Discord/Slack community helping each other
   - Regular contributor meetings and RFC discussions
   - Diverse contributor base (not just original author)

**MVP Success Metrics (Proof of Concept):**

The MVP proves the concept when:
- ✅ End-to-end pipeline execution works (Define → Compile → Execute)
- ✅ At least 3 example pipelines demonstrate value (automation scenarios)
- ✅ Initial documentation allows developers to get started independently
- ✅ 5-10 early adopters (friends, colleagues, online community) try it and provide feedback
- ✅ No fundamental architectural blockers discovered during MVP development

**v1.0 Success Metrics (Community Launch):**

Version 1.0 earns community adoption when:
- 100+ GitHub stars within first month of announcement
- 10+ community-contributed examples or utilities
- 3+ runtime wrappers fully functional (Python, Node, Rust minimum)
- Documentation complete enough for self-service onboarding
- At least 1 production use case (can be personal project) documented

---

## Supporting Materials

This Product Brief incorporates insights from:

**Brainstorming Session (2025-11-15):**
- First Principles Thinking validated 3-backend architecture
- What If Scenarios generated compensation pipelines, AI optimization concepts
- Technology stack decisions: PostgreSQL, Redis, InfluxDB, OpenTelemetry
- Queue architecture: Multiple user-defined dispatch queues with priority algorithms
- Reference: `docs/project/brainstorming-session-results-2025-11-15.md`

**Polyglot v0.0.2 Documentation:**
- Complete language specification reviewed
- Pipeline lifecycle, syntax, type system, runtime wrappers documented
- Examples demonstrate async automation capabilities
- Reference: `docs/user/`

---

_This Product Brief captures the vision and requirements for Polyglot._

_It was created through collaborative discovery and reflects the unique needs of this software project._

_Next: The PRD workflow will transform this brief into detailed product requirements and epic breakdown for implementation._