---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-20
---

# The Aljam3 Project

**الجامع** (Aljam3, `/ælˈdʒæːmɪʕ/`) means to collect, gather, and bring together. It is a **trigger-driven programming language and platform** designed to communicate with the computer based on *how to react* rather than just *what to execute*. It is built on two core pillars that reflect its name:

1. **Gathering Languages (Cross-Language Integration)** — Enabling developers to *use* legacy code across multiple programming languages (Python, Rust, C++, JavaScript) within a unified project, rather than reinventing what already works.
2. **Gathering Programs (Parallel Orchestration)** — Providing first-class support for parallelism with strict resource control, preventing resource starvation through dynamic allocation. Every pipeline is triggered, not called, and enforced by **exhaustive logic** so there is no possible scenario where the program doesn't know what to do.

The name *Aljam3* is the project's mission statement. Aljam3 exists because every programming language brings something valuable: Python's data science ecosystem, Rust's performance guarantees, JavaScript's frontend reach, Go's networking simplicity. Rather than replacing any of them, Aljam3 provides the platform that lets them work together. The best code is often the code that already exists — battle-tested, production-proven, written by experts in each language's domain. Aljam3's job is to make that code seamlessly available across language boundaries.

> **Warning:** This project is in early stages of development and is not suitable for production use. APIs, architecture, and features are subject to change. Users are encouraged to contribute to the project and provide feedback to help shape its future direction.

---

## The Problem

1. **Resource Starvation and Blind Allocation.** Traditional systems lack semantic control over computing resources. A low-priority background job can easily consume resources needed by mission-critical operations, leading to catastrophic starvation and huge business losses. We lack a native way to communicate job importance and dynamically allocate resources by *reacting* to current system conditions.

2. **Lack of Exhaustive Logic.** In modern orchestration, there are always edge cases where the program simply "does not know what to do," resulting in silent failures or unhandled exceptions. True reliability requires exhaustive logic—guaranteeing that there is *no possible scenario* left undefined at compile time.

3. **Programming to React, Not Just to Execute.** We need to communicate with computers based on *how to react* when situations happen. Existing languages bolt trigger-driven behavior and parallelism onto imperative foundations as an afterthought. We need a language where triggers and reactions are the core primitives.

4. **Reinventing the Wheel.** Cross-language integration is hard and brittle. The objective is to *use* legacy code, not reinvent it. We need a tool that gathers (الجامع) programming languages together with seamless integration, controlling existing codebases under dynamic conditions where a trigger-driven model is ideal.

---

## Who Is Aljam3 For?

- Teams managing multi-language codebases with complex async workflows
- Data pipeline engineers who coordinate Python, Rust, Go, and JavaScript in a single data flow
- DevOps automation builders who need safe, repeatable, trigger-driven task orchestration
- Backend teams with aljam3 services that need cross-language integration without brittle glue code

**Not for:** Single-language CRUD apps. Not a replacement for general-purpose programming.

---

## What Aljam3 Is Not

- **Not a general-purpose language** — use Python, Rust, Go, JavaScript for that. Aljam3 orchestrates them; it doesn't replace them. It's the glue, not the bricks.
- **Not a container orchestrator** — Kubernetes schedules containers. Aljam3 compiles workflows and manages their execution at the pipeline level, inside or outside containers.
- **Not a replacement for any language** — the best code is the code that already exists. Aljam3 makes it work together.

---

## Philosophy

For the full philosophy behind Aljam3's design decisions, see the dedicated philosophy pages:

- [[philosophy/core-philosophy]] — Mind-shift (Think with Intent), values, and evolution strategy
- [[philosophy/language-design]] — Design principles: right tool, legacy reuse, trigger-driven safety
- [[philosophy/symbology]] — Symbol design rationale
- [[philosophy/accountability]] — Human inspection and no dynamic code
- [[philosophy/cybersecurity]] — Zero trust and black box monitoring
- [[philosophy/error-philosophy]] — Murphy's Law and exhaustive error handling
- [[philosophy/data-trees]] — Everything is a tree: three-tier data model and universal strings
- [[philosophy/behavioral-contract]] — The "building permit" compilation model
- [[philosophy/developer-experience]] — Write, compile, fix, deploy: the iterative loop
- [[philosophy/extensibility]] — pglib, community packages, and permission ceilings
- [[philosophy/how-aljam3-differs]] — Positioning against Airflow, Temporal, Prefect, and gRPC

---

## The Aljam3 Ecosystem

> *These components are described here as an overview. Detailed specifications will be developed in dedicated documents.*

- **Aljam3 Service** — The runtime backbone, consisting of three components:
  - **Trigger Monitor** — Monitors events (file changes, schedules, HTTP webhooks, resource availability) that initiate automated tasks.
  - **Queue Handler** — Reacts to signals from the Trigger Monitor, managing queue state and dispatching jobs to Runners. Never evaluates trigger conditions or business logic — dispatches mechanically via its internal Dispatch Coordinator.
  - **Runner** — Executes pipelines, managing the lifecycle of each task from dispatch to completion.

  The Aljam3 service must be running in the background to handle execution of automated tasks and manage their interactions.

- **Aljam3 Code** — Think: *what if API was a programming language?* Aljam3 Code is what developers write to define their automated tasks and their interactions. Its structured syntax — triggers, queues, wrappers, collectors, and parallel blocks — makes concurrency the default mode of operation, not an opt-in complexity. The compiler validates every concurrent interaction, so developers get safe parallelism without writing synchronization logic. It is designed to be flexible and extensible, providing a unified syntax for multi-language workflow orchestration.

---

## Ways of Integration

- **In-Language Async Calls** — For each supported programming language, we provide a library called `aljam3-interface` (or the idiomatic variant for that ecosystem) that allows developers to make async calls to Aljam3 services. This enables integration of Aljam3 features into existing codebases without rewriting — developers adopt the platform incrementally.

- **Aljam3 Code** — The Aljam3 language is purpose-built to express interactions between automated tasks and define their behaviors while running in parallel. It can define custom triggers, actions, and orchestration flows, enabling highly customized applications.

**Integration Evolution:**
- **Phase 1 (Current):** Orchestration — Aljam3 coordinates tasks across languages using established tools (FFI, pybind, language-specific libraries).
- **Phase 2 (Future):** Seamless variable-level integration — Cross-language data flows become as natural as passing variables within a single language.

---

*This document is the authoritative source for Aljam3's vision. For the full philosophy, see [[philosophy/core-philosophy]] and [[philosophy/language-design]]. All subsequent specifications must align with the principles defined here and in the philosophy pages.*
