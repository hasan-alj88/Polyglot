---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-22
---

<!-- @c:vision -->
<!-- @u:concepts/errors -->
<!-- @u:concepts/variable-lifecycle#Failed -->
<!-- @u:syntax/operators#Fallback -->
<!-- @c:audit/reference/glossary -->
# Error Handling Philosophy

> Aljam3 treats error handling as a first-class design concern, not an afterthought bolted onto working code. This page explains why. See [[vision]] for the broader project context.

## Murphy's Law as Design Principle

Aljam3 applies Murphy's Law structurally: *if something can go wrong, it will go wrong* — so the language demands you plan for it before it happens.

In the trigger-driven, async-centric world Aljam3 operates in, failure is not an edge case. Network calls drop. Files disappear between check and use. Parallel jobs race. External services return garbage. Foreign code segfaults. These are not hypothetical scenarios — they are the operating conditions of any non-trivial automation pipeline.

Aljam3's response is not to hope for the best. Every failable operation in the language is marked as failable, and the compiler requires the developer to handle the failure path explicitly. There is no "unwrap and pray" pattern. There is no silent null propagation. If a pipeline calls a failable operation and does not handle the error, the compiler rejects the pipeline. The developer must decide — at write time, not at 3am — what happens when things go wrong.

This is not defensive programming as a suggestion. It is defensive programming as a compile-time requirement.

## Handle It Now or Handle It at 3am

Every unhandled edge case is a future production incident. The only question is *when* you deal with it: during working hours with the compiler helping, or at 3am responding to a page.

Aljam3's compiler is tiresome on purpose. It demands `[!]` error handlers for failable calls. It demands fallback operators (`!<`, `!>`) or explicit error blocks for every path that can fail. It demands exhaustive conditionals — every `[?]` branch must account for all possibilities, with `*?` as the mandatory catch-all. It demands that every parallel fan-out has a matching collector. It demands that every race accounts for the scenario where all candidates fail.

These requirements feel burdensome when writing code. That is the point. The compiler is offloading work that would otherwise happen in production — where the cost is orders of magnitude higher, the context is lost, and the person debugging may not be the person who wrote the code.

The alternative is familiar to anyone who has operated distributed systems: a pipeline that works perfectly for months, then fails at 3am because a downstream service returned an unexpected format, and nobody wrote a handler for that case because it "never happens." In Aljam3, the compiler catches "never happens" before deployment, because the compiler does not believe in "never."

## Failed Is a State, Not a Crash

In traditional programming, an unhandled error crashes the process. The stack unwinds, context is lost, and the developer is left with a stack trace and a prayer.

In Aljam3, **Failed** is a variable lifecycle state — one of five states in the lifecycle: Declared, Default, Final, Failed, Released. When a variable enters the Failed state, the data still exists. The pipeline still knows about it. The error metadata is accessible via `$var%sourceError`. Error handlers can inspect the failure, route it, replace the value, or escalate it to a downstream consumer.

This is fundamentally different from exception-based error handling. There is no stack unwinding. There is no "where did the error come from?" mystery. The error is attached to the variable that failed, in the pipeline that produced it, with full metadata about what went wrong. The `[!]` error block scopes directly under the operation that produced the error, making the relationship between failure and handler explicit and visible.

**Failed is terminal** — a Failed variable cannot receive further pushes (see [[technical/compile-rules/PGE/PGE02005-failed-is-terminal|PGE02005]]). But terminal does not mean lost. It means the variable's value is settled as an error, and the pipeline must deal with that error through its declared handlers. To ensure errors do not stay in limbo, Aljam3 provides built-in mechanisms, including retries, to handle the `Failed` state and explicitly drive it toward a `Final` state. Errors are data flowing through the tree, not exceptions blowing up the stack.

See [[user/concepts/variable-lifecycle|Variable Lifecycle]] for the full state machine.

## No Happy Path Only Code

Aljam3 does not allow developers to write code that only handles the success case. The compiler enforces completeness across every dimension of the language:

- **Failable calls require error handling.** Every call to a failable pipeline must have an `[!]` error block, a fallback operator (`!<` or `!>`), or an explicit error suppression (`!*-`) with a compiler warning. There is no implicit error swallowing.

- **Parallel jobs require collection.** Every `[=]` expand that fans out work must have a matching collector — `*All`, `*First`, `*Nth`, or a custom collector. Orphaned parallel [[glossary#Job|jobs]] are a compile error (see [[technical/compile-rules/PGE/PGE01040-orphan-parallel-marker|PGE01040]]).

- **Conditionals require exhaustive branches.** Every `[?]` conditional must cover all possible values, with `*?` as the mandatory catch-all for any values not explicitly matched. An incomplete conditional is a compile error.

- **Race collectors require all-Failed handling.** When using `*First` or `*Nth` race collectors, the developer must account for the scenario where all competing jobs fail. A race with no all-Failed handler is a compile error.

- **Every error path must terminate.** An `[!]` block must either provide a replacement value, escalate the error, or explicitly suppress it. An error block that does nothing is not valid — the compiler needs to know what the pipeline's instruction is for every failure scenario.

This exhaustive coverage can feel tiresome. That is precisely the point. The compiler is demanding that the developer account for every scenario *before* the pipeline runs — so the Aljam3 Service has instructions for handling any situation that arises, rather than discovering gaps in production.

## The Compiler as Safety Net

The Aljam3 compiler catches an entire class of errors that traditional languages defer to runtime:

- **Race conditions** — parallel writes to the same variable are detected and rejected at compile time. The compiler analyses the signal graph to identify concurrent access patterns that would cause data corruption.

- **Resource conflicts** — the permission system (`{_}` grants) combined with the compiler's concurrency analysis prevents two parallel jobs from claiming the same exclusive resource. Conflicts surface as compile errors, not deadlocks.

- **Unhandled error paths** — every failable call, every conditional branch, every parallel fan-out is checked for completeness. Missing handlers are compile errors.

- **Missing collectors** — parallel jobs that produce output but have no collector are rejected. The compiler ensures that every piece of fanned-out work has a defined destination for its results.

- **Permission gaps** — pipelines that access resources without the required `{_}` permission grants are rejected. The compiler validates that every IO operation has explicit authorization.

The compiler is not a gatekeeper. It is a collaborator. It tells you what you missed before production tells you — and production's feedback comes as downtime, data loss, and 3am pages. The compiler's feedback comes as a compile error with a specific rule reference and a clear path to resolution.

This connects to the "building permit" analogy from the [[philosophy/core-philosophy|core philosophy]]: the Aljam3 compiler produces a [[glossary#Behavioral Contract|Behavioral Contract]] — a complete specification of how to react and behave across all possible scenarios. Compilation is a license to launch. The compiler rejects incomplete specifications not because it is pedantic, but because the Aljam3 Service needs instructions for every scenario. A pipeline with gaps is a pipeline that will eventually encounter a situation it has no instructions for — and in an async, trigger-driven system, "eventually" means "soon."

---

## Related Philosophy

- [[philosophy/core-philosophy]] — Mind-shift, values, and evolution
- [[philosophy/language-design]] — Design principles and safety model
- [[philosophy/symbology]] — Symbol design rationale
- [[philosophy/accountability]] — Human inspection and no dynamic code
- [[philosophy/cybersecurity]] — Zero trust and black box monitoring
