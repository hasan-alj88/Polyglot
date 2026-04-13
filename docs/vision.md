---
audience: [pg-coder, integrator, architect, designer]
type: reference
updated: 2026-04-13
---

# The Polyglot Project

Polyglot is a trigger-driven programming language and platform — async-centric and parallel-by-design, not as an afterthought — with two core pillars:

1. **Cross-Language Integration** — Enabling developers to utilise well-tested legacy code across multiple programming languages within a unified project, rather than reinventing what already works.
2. **Trigger-Driven Orchestration** — Providing first-class support for parallelism, concurrency, race condition handling, error handling, and resource management as foundational language features. Every pipeline is triggered, not called — concurrency is the starting point, not an add-on.

The name *Polyglot* — one who speaks many languages — is the project's mission statement. Polyglot exists because every programming language brings something valuable: Python's data science ecosystem, Rust's performance guarantees, JavaScript's frontend reach, Go's networking simplicity. Rather than replacing any of them, Polyglot provides the platform that lets them work together. The best code is often the code that already exists — battle-tested, production-proven, written by experts in each language's domain. Polyglot's job is to make that code seamlessly available across language boundaries.

> **Warning:** This project is in early stages of development and is not suitable for production use. APIs, architecture, and features are subject to change. Users are encouraged to contribute to the project and provide feedback to help shape its future direction.

---

## Core Philosophy

### Mind-Shift: Think with Intent, Abstract Away the How

The overarching theme of Polyglot is **think with intent, abstract away the how**. Developers express *what* they need — a variable from C++ in a Python script, parallel tasks that combine results, a trigger that fires on a condition — and the Polyglot platform handles the implementation details. This is not laziness; it is leverage. The "how" is built from battle-tested legacy code that has already solved those technical problems. Polyglot's job is to let developers focus on outcomes, not mechanisms.

This requires a fundamental shift in how developers think about their code:

- **There Is No Main Function** — Polyglot pipelines are not synchronous programs with a `main()` entry point. Every pipeline is event-triggered: something happens, the trigger fires, the pipeline executes. There is no "run this program" — there is "when this event occurs, do this work." Developers coming from imperative languages must unlearn the sequential mindset and think in terms of triggers and reactions. Think of it this way: *what if APIs or microservices were a programming language?* Polyglot manages reactions rather than actions. You are not writing a sequence of instructions — you are defining how the system should respond to events and conditions, creating a network of interactions and dependencies that can adapt and evolve over time.

- **Compiles Intent, Not to Binary** — The Polyglot compiler does not produce a binary executable. It compiles developer intent into instructions for the Polyglot Service — a complete specification of how to react and behave across all possible scenarios. The compiled output is a behavioural contract: "when X happens, do Y; if Y fails, do Z; if A and B race, resolve with C." This is closer to compiling a state machine than compiling a program. Think of it as a building permit: before construction can start, the permit must account for every disaster scenario and every building code requirement. The Polyglot compiler is exhaustive — it accounts for every possible path the pipeline may take. Only once all scenarios are covered does it approve the instructions and compile them into a serialized form for the Polyglot Service to register. Compilation is a license to launch.

- **Compile-Time Meets Async — Data Topology Is Known** — The concept of compile-time and runtime in parallel programming is fundamentally different from traditional programming. In synchronous code, if you don't know the shape of incoming data at compile time, the compiler cannot help you. But in Polyglot's asynchronous world, data topology *can* be determined at compile time — because it flows through the Polyglot Service, which knows what each pipeline produces before the next one consumes it. Consider integrating Python with Rust: Rust's type system and borrow checker demand known data shapes at compile time. Traditionally, receiving dynamically-typed data from Python into Rust is a problem — Rust cannot compile unknown memory allocations when its types demand it. But in Polyglot, the data flows asynchronously through the platform. When Python produces output, Polyglot knows the data topology and can dispatch it to Rust with all type constraints satisfied — because the Polyglot compiler has already verified at *its* compile time that the data shapes align. Rust compiles happily with full type knowledge, and Polyglot ensures the contract is honoured at its own compilation stage. This is a powerful consequence of the async-centric model: what appears to be a runtime concern in traditional programming becomes a compile-time guarantee in Polyglot.

- **Memory Is Not RAM** — Polyglot's memory model is fundamentally different from traditional programming. There are no local variables living in process memory, no data structures allocated on a heap that you manage with bits and bytes. Instead, Polyglot state lives in the service infrastructure: objects are hosted in a NoSQL database, and queues are hosted in Redis under the Queue Handler's control. This means developers work with data at a higher level of abstraction — as objects and documents in a distributed, shared resource — rather than as memory addresses in a single process. Different components of a pipeline can access and modify data because it lives in the service, not in any one runtime's address space.

- **Strings Are the Universal Interface** — Every programming language deals with strings — as code, as data, as serialization format. Strings are the common tongue between Python, Rust, JavaScript, Go, and every other language Polyglot integrates. By building cross-language communication on this universal foundation, Polyglot avoids the complexity of bespoke type marshalling between language runtimes. The Data Tree — where all leaves are `RawString` — is the structural embodiment of this principle: all Polyglot data is ultimately a tree of strings, and strings are how languages already talk to each other.

- **Implicit Deny Permission** — Nothing runs without explicit, intentional permission. Developers must grant permissions for their pipelines to access resources, interact with external systems, or perform privileged operations. Even if the platform *could* allow an action, Polyglot will not unless the developer has explicitly permitted it. Generosity in granting permissions is discouraged — the default is denial, and every grant is a conscious design decision. Permissions are not just about access-rights security — they are also a safeguard against concurrency issues and data integrity. By defining permissions for each component, the compiler can ensure that parallel jobs do not write to shared resources simultaneously, that data is not corrupted by concurrent access, and that resource conflicts are caught at compile time rather than at runtime.

  - **Permissions as Implicit Triggers** — Permissions function as a compile-time trigger gate. Just as a pipeline cannot fire until its `[T]` trigger conditions are met and its IO inputs are satisfied, a pipeline cannot fire — or even compile — without its required `_Permission` objects being granted. The compiler knows that a pipeline missing its permissions will never run regardless of the state of its other triggers, so it rejects the pipeline entirely at compile time rather than waiting for a runtime failure that is guaranteed to occur. This makes permissions part of the same "all gates must be open" model that governs IO inputs and triggers.

### Language Design

- **The Right Tool for the Right Job** — All programming languages have their own strengths and weaknesses. By allowing developers to use multiple languages within a single project, we can leverage the unique capabilities of each to create more efficient and effective solutions. For example, use Python for data analysis and machine learning, JavaScript for front-end development, and Rust for performance-critical operations.

- **On the Shoulders of Giants — Utilise Legacy Code** — Years of battle-tested, production-proven code already exist across every language ecosystem. Rewriting it introduces new bugs; reusing it leverages decades of fixes, optimisations, and real-world validation. Polyglot's cross-language integration lets developers bring their existing codebases, libraries, and expertise directly into a unified project — no rewrites, no wrappers, no starting from scratch. The safest code is the code that already works.

- **Trigger-Driven, Async-Centric, Parallel-by-Design** — Polyglot makes parallel programming **easy, bug-free, safe, and — most importantly — intentional**. The platform applies Murphy's Law as a design principle: *if something can go wrong, it will go wrong* — so Polyglot proactively covers for it.

  Polyglot's concurrency model builds on the async foundations established by languages like Python (`asyncio`), Rust (`tokio`), JavaScript (Promises), and Go (goroutines). These are powerful mechanisms — Polyglot does not replace them or claim superiority over them. What Polyglot adds is a layer of abstraction that lets developers express *what* should happen concurrently without managing the underlying synchronisation primitives directly.

  Traditional approaches build synchronous automation first, then retrofit async handling as an afterthought — leading to complex, error-prone code where race conditions and edge cases surface only in production. Polyglot inverts this: triggers and concurrency are the starting point, not an add-on.

  - **Compiler-enforced safety** — Race conditions, resource conflicts, unhandled parallel interactions, and missing error paths are compile-time errors — not runtime surprises. If a developer's concurrent design has a gap, the compiler rejects it.
  - **Structured concurrency primitives** — Developers express parallelism through triggers, queues, collectors, parallel blocks, and wait/collect markers — not locks, mutexes, or manual thread management. These constructs are safe by construction.
  - **Exhaustive coverage** — Every conditional must be exhaustive. Every parallel job must have its output collected. Every error path must be handled. Polyglot does not allow "happy path only" code. The exhaustive logic can feel tiresome, but it is offloading the hard work that would inevitably happen when a pipeline crashes at runtime to compile time instead. The compiler demands that every scenario is accounted for *before* the pipeline runs — so Polyglot has instructions for handling any situation that may arise, rather than discovering gaps in production.

### Evolution

- **Polyglot Optimization Evolution** — Polyglot features will start slow, but as the project evolves and more languages are added, significant improvements in performance and efficiency will follow. Polyglot will always have the objective of minimizing its footprint in codebase integration.

  The strategy is **divide and conquer**: reduce the cross-language integration problem into smaller, solvable pieces and optimize each one.

  **Today:** Integration uses established mechanisms — pybind, FFI, and similar tools — adhering to the "On the Shoulders of Giants" principle.

  **Tomorrow:** Integration evolves to the variable level. For example, a Python list can be passed as a sized array to Rust (possible because it's an async call with known metadata), processed, and returned as a Python list. By reducing integration to the granularity of individual variables, we divide and conquer the problem progressively.

  This approach ensures the project remains maintainable and scalable while continuously improving the developer experience.

### Project Values

- **Collaboration and Community** — The project emphasizes collaboration and community involvement from the start of its design. By fostering a strong developer community, we encourage knowledge sharing, peer review, and collective problem-solving. This collaborative approach enhances quality and accelerates growth.

- **Open Source and Transparency** — Polyglot is committed to being open source, allowing anyone to contribute, review, and use the code. This transparency promotes trust and encourages a diverse range of contributions, leading to innovative solutions and rapid development.

- **Security** — Automated tasks are vulnerable when behaviour is undefined or unexpected. Polyglot closes this gap through its core philosophy: exhaustive coverage at compile time and implicit deny at runtime. Undefined behavior is a compile error, not an exploitable vulnerability. As the project evolves, we will define secure interactions between automated tasks and close potential security gaps as we and the community discover them.

  - **No Dynamic Code** — All code that Polyglot executes must be static and analysable at compile time. No dynamically generated code is permitted to run through the Polyglot platform at runtime. In the age of AI, where models can write and execute code on the fly, Polyglot stands firmly against this pattern — not because dynamic code is never useful, but because it is a security risk that bypasses compile-time analysis. Developers may run code at their own risk outside of Polyglot, bearing the consequences of unanalysed execution. What Polyglot guarantees is that every decision to run code — every permission grant, every file reference, every foreign code invocation — is made at compile time, where it can be inspected, validated, and approved before execution begins.

    To enforce this principle, Polyglot applies **compile-time file binding**: any pipeline input that references a source file (e.g., `<code.file` in `-Run.*` pipelines, configuration files, data files) has its permission grant bound to the file's content at compilation time. If the file changes after compilation, the Polyglot Service revokes the associated permissions and refuses to execute until the developer recompiles with the updated file. A file change watcher trigger monitors referenced file paths and notifies the developer when recompilation is required. This ensures that no code or input runs without having passed through the compiler's analysis.

  - **AI Execution Advisory** — AI systems may write Polyglot code (`.pg` files) for human review and compilation, but should never autonomously execute Polyglot CLI commands (`polyglot compile`, `polyglot run`, etc.). Developers are advised to revoke any permissions that would allow AI tools to invoke the Polyglot CLI directly. The distinction is clear: AI as author (human reviews and compiles) is acceptable; AI as autonomous executor (bypassing human oversight) is not. If a reliable technical mechanism for detecting and blocking AI invocation becomes available, Polyglot will adopt it as an enforced safeguard.

- **Privacy** — The ability to run Polyglot services locally on users' machines, without the need for cloud-based services, provides greater control over data and privacy. Local execution also reduces latency and improves performance, as users access services directly from their machines without relying on internet connectivity. Users can tailor services to their specific needs and preferences.

---

## The Polyglot Ecosystem

> *These components are described here as an overview. Detailed specifications will be developed in dedicated documents.*

- **Polyglot Service** — The runtime backbone, consisting of three components:
  - **Trigger Monitor** — Monitors events (file changes, schedules, HTTP webhooks, resource availability) that initiate automated tasks.
  - **Queue Handler** — Reacts to signals from the Trigger Monitor, managing queue state and dispatching jobs to Runners. Never evaluates trigger conditions or business logic — dispatches mechanically via its internal Dispatch Coordinator.
  - **Runner** — Executes pipelines, managing the lifecycle of each task from dispatch to completion.

  The Polyglot service must be running in the background to handle execution of automated tasks and manage their interactions.

- **Polyglot Code** — Think: *what if API was a programming language?* Polyglot Code is what developers write to define their automated tasks and their interactions. Its structured syntax — triggers, queues, wrappers, collectors, and parallel blocks — makes concurrency the default mode of operation, not an opt-in complexity. The compiler validates every concurrent interaction, so developers get safe parallelism without writing synchronization logic. It is designed to be flexible and extensible, providing a unified syntax for multi-language workflow orchestration.

---

## Ways of Integration

- **In-Language Async Calls** — For each supported programming language, we provide a library called `polyglot-interface` (or the idiomatic variant for that ecosystem) that allows developers to make async calls to Polyglot services. This enables integration of Polyglot features into existing codebases without rewriting — developers adopt the platform incrementally.

- **Polyglot Code** — The Polyglot language is purpose-built to express interactions between automated tasks and define their behaviors while running in parallel. It can define custom triggers, actions, and orchestration flows, enabling highly customized applications.

**Integration Evolution:**
- **Phase 1 (Current):** Orchestration — Polyglot coordinates tasks across languages using established tools (FFI, pybind, language-specific libraries).
- **Phase 2 (Future):** Seamless variable-level integration — Cross-language data flows become as natural as passing variables within a single language.

---

*This document is the authoritative source for Polyglot's vision and philosophy. All subsequent specifications must align with the principles defined here.*
