---
audience: [pg-coder, integrator, architect, designer]
type: reference
updated: 2026-04-10
---

# The Polyglot Project

Polyglot is a trigger-driven programming language and platform — async-centric and parallel-by-design, not as an afterthought — with two core pillars:

1. **Cross-Language Integration** — Enabling developers to utilise well-tested legacy code across multiple programming languages within a unified project, rather than reinventing what already works.
2. **Trigger-Driven Orchestration** — Providing first-class support for parallelism, concurrency, race condition handling, error handling, and resource management as foundational language features. Every pipeline is triggered, not called — concurrency is the starting point, not an add-on.

The project aims to provide a unified platform for developers to leverage the strengths of different programming languages while maintaining a cohesive development experience. By fostering a collaborative environment for developers, the Polyglot Project seeks to promote innovation and creativity in software development.

> **Warning:** This project is in early stages of development and is not suitable for production use. APIs, architecture, and features are subject to change. Users are encouraged to contribute to the project and provide feedback to help shape its future direction.

---

## Core Philosophy

### Language Design

- **The Right Tool for the Right Job** — All programming languages have their own strengths and weaknesses. By allowing developers to use multiple languages within a single project, we can leverage the unique capabilities of each to create more efficient and effective solutions. For example, use Python for data analysis and machine learning, JavaScript for front-end development, and Rust for performance-critical operations. By complementing the strengths of different languages, we can create more powerful applications that would be difficult to achieve with a single language alone.

- **Don't Reinvent the Wheel — Utilise Legacy Code** — Years of battle-tested, production-proven code already exist across every language ecosystem. Rewriting it introduces new bugs; reusing it leverages decades of fixes, optimisations, and real-world validation. Polyglot's cross-language integration lets developers bring their existing codebases, libraries, and expertise directly into a unified project — no rewrites, no wrappers, no starting from scratch. The safest code is the code that already works.

- **Trigger-Driven, Async-Centric, Parallel-by-Design** — Polyglot makes parallel programming **easy, bug-free, safe, and — most importantly — intentional**. Every pipeline is triggered by an event — not called imperatively — and the code behaves exactly as the developer designed it to behave, in every situation, because the platform applies Murphy's Law as a design principle: *if something can go wrong, it will go wrong* — so Polyglot proactively covers for it.

  Traditional approaches build synchronous automation first, then retrofit async handling as an afterthought — leading to complex, error-prone code where race conditions and edge cases surface only in production. Polyglot inverts this: triggers and concurrency are the starting point, not an add-on. Developers declare *what* triggers execution, *what* runs in parallel, *how* tasks interact, and *what happens* when things go wrong. The compiler and Polyglot Code enforce the rest.

  **The compiler catches problems before code ever runs.** Race conditions, resource conflicts, unhandled parallel interactions, and missing error paths are compile-time errors — not runtime surprises discovered under load at 3 AM. If a developer's concurrent design has a gap, the compiler rejects it. This means every pipeline that compiles is a pipeline where all concurrent paths are accounted for.

  **Polyglot Code makes concurrency easy by design.** Developers express parallelism through structured constructs — triggers, queues, collectors, parallel blocks, wait/collect markers — rather than low-level primitives like locks, mutexes, or manual thread management. These constructs are inherently safe by construction. Developers describe intent; the platform handles synchronization, ordering, and resource management.

  **Intentional behavior under Murphy's Law.** Every conditional must be exhaustive. Every parallel job must have its output collected. Every error path must be handled. Polyglot does not allow "happy path only" code — the developer must address what happens when tasks fail, when resources are unavailable, when race conditions arise. The result is automation that behaves predictably in all situations the developer has covered, because the compiler guarantees there are no uncovered situations.

  Key advantages of trigger-driven, async-centric design:
  - **Compiler-enforced safety** — The compiler verifies concurrent interactions at build time, catching race conditions, resource conflicts, and missing error paths before deployment.
  - **Structured concurrency primitives** — Polyglot Code provides first-class constructs for parallelism that are safe by construction. Developers describe *what* should run concurrently and *how* results combine — the platform handles the rest.
  - **Intentional task behavior** — Developers explicitly define how concurrent tasks interact and how failures are handled, rather than discovering unexpected behaviors in production.
  - **Exhaustive coverage** — The compiler enforces that all parallel paths, error conditions, and edge cases are addressed. No implicit defaults, no silent failures, no undefined behavior.
  - Better native integration between runtime and compile-time code, enabling more efficient and optimized execution.
  - Native integration of API protocols and libraries, allowing developers to easily access and utilize tools and services without writing complex wrappers or adapters.

### Evolution

- **Polyglot Optimization Evolution** — Polyglot features will start slow, but as the project evolves and more languages are added, significant improvements in performance and efficiency will follow. Polyglot will always have the objective of minimizing its footprint in codebase integration.

  The strategy is **divide and conquer**: reduce the cross-language integration problem into smaller, solvable pieces and optimize each one.

  **Today:** Integration uses established mechanisms — pybind, FFI, and similar tools — adhering to the "Don't Reinvent the Wheel" principle.

  **Tomorrow:** Integration evolves to the variable level. For example, a Python list can be passed as a sized array to Rust (possible because it's an async call with known metadata), processed, and returned as a Python list. By reducing integration to the granularity of individual variables, we divide and conquer the problem progressively.

  This approach ensures the project remains maintainable and scalable while continuously improving the developer experience.

### Project Values

- **Collaboration and Community** — The project emphasizes collaboration and community involvement from the start of its design. By fostering a strong developer community, we encourage knowledge sharing, peer review, and collective problem-solving. This collaborative approach enhances quality and accelerates growth.

- **Open Source and Transparency** — Polyglot is committed to being open source, allowing anyone to contribute, review, and use the code. This transparency promotes trust and encourages a diverse range of contributions, leading to innovative solutions and rapid development.

- **Security** — One of the dangers of automated tasks is vulnerabilities in undefined or unexpected behaviors that can be exploited by attackers. Polyglot applies Murphy's Law at the compiler level: if a concurrent interaction *can* go wrong, the compiler forces the developer to handle it. Undefined behavior is a compile error, not an exploitable vulnerability. Race conditions, unhandled error paths, and uncovered edge cases are rejected before code reaches production. As the project evolves, we will define secure interactions between automated tasks and close potential security gaps as we and the community discover them.

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

- **In-Language Async Calls** — For each supported programming language, we provide a library/package that allows developers to make async calls to Polyglot services. This enables integration of Polyglot features into existing codebases without rewriting — developers adopt the platform incrementally.

- **Polyglot Code** — The Polyglot language is purpose-built to express interactions between automated tasks and define their behaviors while running in parallel. It can define custom triggers, actions, and orchestration flows, enabling highly customized applications.

**Integration Evolution:**
- **Phase 1 (Current):** Orchestration — Polyglot coordinates tasks across languages using established tools (FFI, pybind, language-specific libraries).
- **Phase 2 (Future):** Seamless variable-level integration — Cross-language data flows become as natural as passing variables within a single language.

---

*This document is the authoritative source for Polyglot's vision and philosophy. All subsequent specifications must align with the principles defined here.*
