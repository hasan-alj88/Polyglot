---
audience: [automation-builder, integrator, architect, designer]
type: reference
updated: 2026-04-20
---

# The Polyglot Project

Polyglot is a trigger-driven programming language and platform — async-centric and parallel-by-design, not as an afterthought — with two core pillars:

1. **Cross-Language Integration** — Enabling developers to utilise well-tested legacy code across multiple programming languages within a unified project, rather than reinventing what already works.
2. **Trigger-Driven Orchestration** — Providing first-class support for parallelism, concurrency, race condition handling, error handling, and resource management as foundational language features. Every pipeline is triggered, not called — concurrency is the starting point, not an add-on.

The name *Polyglot* — one who speaks many languages — is the project's mission statement. Polyglot exists because every programming language brings something valuable: Python's data science ecosystem, Rust's performance guarantees, JavaScript's frontend reach, Go's networking simplicity. Rather than replacing any of them, Polyglot provides the platform that lets them work together. The best code is often the code that already exists — battle-tested, production-proven, written by experts in each language's domain. Polyglot's job is to make that code seamlessly available across language boundaries.

> **Warning:** This project is in early stages of development and is not suitable for production use. APIs, architecture, and features are subject to change. Users are encouraged to contribute to the project and provide feedback to help shape its future direction.

---

## The Problem

1. **Cross-language integration is hard and brittle.** Existing tools (FFI, pybind, gRPC stubs) are fragile, version-sensitive, and require deep knowledge of both sides of the boundary. Every language pair demands its own integration approach, and these approaches break silently when either side changes.

2. **No language exists that is made for automation — trigger-driven and parallel by design.** Existing languages bolt async and parallelism onto imperative foundations as libraries or frameworks, never as core language constructs. Automation tools inherit these limitations and pass the complexity to the developer.

3. **Security and permissions are afterthoughts in existing tools.** Automation tasks run unattended, handle sensitive data, and interact with external systems — yet the tools that orchestrate them treat permissions as configuration, not as compile-time guarantees. Undefined behaviour is an exploitable gap, not a compile error.

4. **Async runtime errors are preventable at compile time.** Developers discover race conditions, unhandled error paths, and resource conflicts in production because their tools cannot analyze concurrent interactions at compile time. Polyglot's compiler catches these before deployment.

---

## Who Is Polyglot For?

- Teams managing multi-language codebases with complex async workflows
- Data pipeline engineers who coordinate Python, Rust, Go, and JavaScript in a single data flow
- DevOps automation builders who need safe, repeatable, trigger-driven task orchestration
- Backend teams with polyglot services that need cross-language integration without brittle glue code

**Not for:** Single-language CRUD apps. Not a replacement for general-purpose programming.

---

## What Polyglot Is Not

- **Not a general-purpose language** — use Python, Rust, Go, JavaScript for that. Polyglot orchestrates them; it doesn't replace them. It's the glue, not the bricks.
- **Not a container orchestrator** — Kubernetes schedules containers. Polyglot compiles workflows and manages their execution at the pipeline level, inside or outside containers.
- **Not a replacement for any language** — the best code is the code that already exists. Polyglot makes it work together.

---

## Philosophy

For the full philosophy behind Polyglot's design decisions, see the dedicated philosophy pages:

- [[philosophy/core-philosophy]] — Mind-shift (Think with Intent), values, and evolution strategy
- [[philosophy/language-design]] — Design principles: right tool, legacy reuse, trigger-driven safety
- [[philosophy/symbology]] — Symbol design rationale
- [[philosophy/accountability]] — Human inspection and no dynamic code
- [[philosophy/cybersecurity]] — Zero trust and black box monitoring (planned — #334)
- [[philosophy/error-philosophy]] — Murphy's Law and exhaustive error handling (planned — #335)

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

*This document is the authoritative source for Polyglot's vision. For the full philosophy, see [[philosophy/core-philosophy]] and [[philosophy/language-design]]. All subsequent specifications must align with the principles defined here and in the philosophy pages.*
