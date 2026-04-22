---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-20
---

<!-- @c:vision -->
<!-- @u:technical/spec/behavior-contract -->
<!-- @u:technical/spec/compiler-floor -->
<!-- @u:concepts/pipelines/INDEX -->
# The Behavioral Contract

> Polyglot does not compile to a binary. It compiles to a complete specification of how the system should behave — the Behavioral Contract. This page explains what that means and why it matters. See [[vision]] for the broader project context.

## The Building Permit

Think of the Polyglot compiler as a building inspector, and the Behavioral Contract as a building permit.

Before any construction can start, the permit must account for every disaster scenario and every building code requirement. What happens during an earthquake? Where are the fire exits? What is the load capacity of each floor? How does the electrical system handle a surge? The inspector does not approve construction until every scenario is addressed. There is no "we'll figure it out when it happens."

The Polyglot compiler works the same way. Before a pipeline can run, the compiler must verify that every possible execution path is accounted for. What happens when this trigger fires? What if that input is missing? What if two parallel jobs race to the same resource? What if the foreign code throws an exception? The compiler produces the Behavioral Contract only when every scenario has an answer. Compilation is a license to launch.

## Single Source of Truth

The Behavioral Contract is the single, authoritative specification of runtime behaviour. It captures everything the Polyglot Service needs to orchestrate execution:

- **Signal routing** — when one job finishes, which jobs receive the trigger signal next
- **Parallelism** — which jobs run concurrently, which run sequentially, and how their outputs are collected
- **Conditional branching** — which branch receives the "go" signal based on runtime conditions, and what happens to branches that do not fire
- **Error routing** — the same mechanism as conditional branching, but the condition is the error state of the preceding job
- **Permission grants** — what resources each job is allowed to access, and at what capacity
- **Dispatch rules** — concurrency limits, queue strategy, wrapper lifecycle, host selection

This is not a sequential step list. It is a signal map — a complete description of all possible execution states and transitions. The Trigger Monitor reads this map and orchestrates execution in an async environment.

## Properties

The Behavioral Contract has four properties that distinguish it from traditional compilation output:

**Deterministic.** Given the same source code, the compiler produces the same contract. There is no randomness, no platform-dependent variation, no "it depends on the runtime." If the contract says job A triggers job B, that relationship is fixed.

**Inspectable.** The contract can be read and understood before any code runs. Developers, auditors, and monitoring tools can examine the signal map, verify the error handling paths, and confirm the permission grants — all without executing a single pipeline. What you see is what will run.

**Versionable.** The contract is a serialized artifact. It can be stored, versioned, diffed, and rolled back. When a pipeline changes, the new contract can be compared against the old one to see exactly what changed in the runtime behaviour — not just what changed in the source code.

**Auditable.** Because the contract captures every possible execution path, it serves as a complete audit trail of intended behaviour. Compliance questions like "can this pipeline access that database?" or "what happens if the external API is down?" can be answered by reading the contract, without running the system.

## Compiler to Service

The compilation flow is a one-way handoff:

1. The developer writes Polyglot Code (`.pg` files)
2. The compiler validates signal logic — buggy concurrency, missing error paths, and permission violations are compile errors
3. The compiler produces a Behavioral Contract — a serialized signal map for each pipeline
4. The contract is registered in the Polyglot Service's database
5. The developer enables the pipeline
6. The Trigger Monitor reads the contract and orchestrates execution

The compiler's job ends at step 3. It has no role at runtime. The Service trusts the contract because the compiler already proved it is complete. The Service's job is faithful execution — doing exactly what the contract specifies, nothing more, nothing less.

This separation means the compiler and the Service can evolve independently. The compiler can add new validations without changing the Service. The Service can optimise execution without changing the compiler. The contract is the stable interface between them.

See [[core-philosophy]] for the mind-shift that makes this model possible, and [[error-philosophy]] for how exhaustive error handling feeds into the contract.
