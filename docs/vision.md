---
audience: [pg-coder, integrator, architect, designer]
type: reference
updated: 2026-04-03
---

# The Polyglot Project

Polyglot is an async-centric programming language and platform with two core pillars:

1. **Cross-Language Integration** — Enabling developers to write code in multiple programming languages and run them together seamlessly, leveraging the strengths of each language within a unified project.
2. **Async-Centric Automation** — Providing first-class support for parallelism, concurrency, race condition handling, error handling, and resource management as foundational language features, not afterthoughts.

The project aims to provide a unified platform for developers to leverage the strengths of different programming languages while maintaining a cohesive development experience. By fostering a collaborative environment for developers, the Polyglot Project seeks to promote innovation and creativity in software development.

> **Warning:** This project is in early stages of development and is not suitable for production use. APIs, architecture, and features are subject to change. Users are encouraged to contribute to the project and provide feedback to help shape its future direction.

---

## Core Philosophy

### Language Design

- **The Right Tool for the Right Job** — All programming languages have their own strengths and weaknesses. By allowing developers to use multiple languages within a single project, we can leverage the unique capabilities of each to create more efficient and effective solutions. For example, use Python for data analysis and machine learning, JavaScript for front-end development, and Rust for performance-critical operations. By complementing the strengths of different languages, we can create more powerful applications that would be difficult to achieve with a single language alone.

- **Don't Reinvent the Wheel, Use Legacy Code!** — Many developers have existing codebases in various programming languages. By supporting multiple languages, we allow developers to reuse their existing code and libraries, reducing the need for rewriting and increasing productivity. Legacy code is well-tested and production-proven — it can be more reliable and efficient than code written from scratch. Developers can also leverage their existing knowledge and expertise, making it easier for them to contribute and collaborate.

- **Async-Centric by Design** — Polyglot is designed from the ground up with a focus on asynchronous programming, allowing developers to write code that runs concurrently and handles tasks in parallel. This is particularly beneficial for applications requiring high performance and responsiveness: web applications, real-time data processing, and distributed systems.

  Being async-centric means the language puts focus on *how* automated tasks are executed and how their interactions and race conditions are handled while running in parallel. Task behaviors are intentional and designed by the developer's instructions, rather than building synchronous automation and retrofitting async handling as an afterthought — which leads to more complex and error-prone code.

  Key advantages of async-centric design:
  - Better native integration between runtime and compile-time code, enabling more efficient and optimized execution.
  - Intentional task behavior — developers explicitly define how concurrent tasks interact, rather than discovering unexpected behaviors and applying workarounds.
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

- **Security** — One of the dangers of automated tasks is vulnerabilities in undefined or unexpected behaviors that can be exploited by attackers. Polyglot is designed from day one to handle concurrency, race conditions, error handling, and pipeline interactions intentionally. As the project evolves, we will define secure interactions between automated tasks and close potential security gaps as we and the community discover them.

- **Privacy** — The ability to run Polyglot services locally on users' machines, without the need for cloud-based services, provides greater control over data and privacy. Local execution also reduces latency and improves performance, as users access services directly from their machines without relying on internet connectivity. Users can tailor services to their specific needs and preferences.

---

## The Polyglot Ecosystem

> *These components are described here as an overview. Detailed specifications will be developed in dedicated documents.*

- **Polyglot Service** — The runtime backbone, consisting of three components:
  - **Trigger Monitor** — Monitors events (file changes, schedules, HTTP webhooks, resource availability) that initiate automated tasks.
  - **Queue Handler** — Reacts to signals from the Trigger Monitor, managing queue state and dispatching jobs to Runners. Never evaluates conditions or makes decisions.
  - **Runner** — Executes pipelines, managing the lifecycle of each task from dispatch to completion.

  The Polyglot service must be running in the background to handle execution of automated tasks and manage their interactions.

- **Polyglot Code** — Think: *what if API was a programming language?* Polyglot code is what developers write to define their automated tasks and their interactions. It is designed to be flexible and extensible, providing a unified syntax for multi-language workflow orchestration.

---

## Ways of Integration

- **In-Language Async Calls** — For each supported programming language, we provide a library/package that allows developers to make async calls to Polyglot services. This enables integration of Polyglot features into existing codebases without rewriting — developers adopt the platform incrementally.

- **Polyglot Code** — The Polyglot language is purpose-built to express interactions between automated tasks and define their behaviors while running in parallel. It can define custom triggers, actions, and orchestration flows, enabling highly customized applications.

**Integration Evolution:**
- **Phase 1 (Current):** Orchestration — Polyglot coordinates tasks across languages using established tools (FFI, pybind, language-specific libraries).
- **Phase 2 (Future):** Seamless variable-level integration — Cross-language data flows become as natural as passing variables within a single language.

---

*This document is the authoritative source for Polyglot's vision and philosophy. All subsequent specifications must align with the principles defined here.*
