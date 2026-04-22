---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-20
---

<!-- @c:vision -->
<!-- @u:concepts/pipelines/INDEX -->
<!-- @u:concepts/conditionals -->
<!-- @u:concepts/errors -->
<!-- @u:concepts/collections/INDEX -->
# Language Design Principles

> This page expands the language design principles summarized in [[vision]]. It is the authoritative source for Polyglot's design rationale and safety model.

## The Right Tool for the Right Job

All programming languages have their own strengths and weaknesses. By allowing developers to use multiple languages within a single project, we can leverage the unique capabilities of each to create more efficient and effective solutions. For example, use Python for data analysis and machine learning, JavaScript for front-end development, and Rust for performance-critical operations.

## On the Shoulders of Giants — Utilise Legacy Code

Years of battle-tested, production-proven code already exist across every language ecosystem. Rewriting it introduces new bugs; reusing it leverages decades of fixes, optimisations, and real-world validation. Polyglot's cross-language integration lets developers bring their existing codebases, libraries, and expertise directly into a unified project — no rewrites, no wrappers, no starting from scratch. The safest code is the code that already works.

## Trigger-Driven, Async-Centric, Parallel-by-Design

Polyglot makes parallel programming **easy, bug-free, safe, and — most importantly — intentional**. The platform applies Murphy's Law as a design principle: *if something can go wrong, it will go wrong* — so Polyglot proactively covers for it.

Polyglot's concurrency model builds on the async foundations established by languages like Python (`asyncio`), Rust (`tokio`), JavaScript (Promises), and Go (goroutines). These are powerful mechanisms — Polyglot does not replace them or claim superiority over them. What Polyglot adds is a layer of abstraction that lets developers express *what* should happen concurrently without managing the underlying synchronisation primitives directly.

Traditional approaches build synchronous automation first, then retrofit async handling as an afterthought — leading to complex, error-prone code where race conditions and edge cases surface only in production. Polyglot inverts this: triggers and concurrency are the starting point, not an add-on.

- **Compiler-enforced safety** — Race conditions, resource conflicts, unhandled parallel interactions, and missing error paths are compile-time errors — not runtime surprises. If a developer's concurrent design has a gap, the compiler rejects it.
- **Structured concurrency primitives** — Developers express parallelism through triggers, queues, collectors, parallel blocks, and wait/collect markers — not locks, mutexes, or manual thread management. These constructs are safe by construction.
- **Exhaustive coverage** — Every conditional must be exhaustive. Every parallel job must have its output collected. Every error path must be handled. Polyglot does not allow "happy path only" code. The exhaustive logic can feel tiresome, but it is offloading the hard work that would inevitably happen when a pipeline crashes at runtime to compile time instead. The compiler demands that every scenario is accounted for *before* the pipeline runs — so Polyglot has instructions for handling any situation that may arise, rather than discovering gaps in production.

---

## Related Philosophy

- [[philosophy/core-philosophy]] — Mind-shift, values, and evolution
- [[philosophy/error-philosophy]] — Murphy's Law and exhaustive error handling
