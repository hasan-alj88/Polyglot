---
audience: [automation-builder, integrator, architect, designer]
type: reference
updated: 2026-04-20
---

<!-- @c:vision -->
<!-- @u:concepts/errors -->
<!-- @u:concepts/conditionals -->
<!-- @u:concepts/variable-lifecycle -->
# Developer Experience

> Polyglot's developer experience is built around one loop: write, compile, fix, deploy. The compiler is not a gatekeeper — it is a collaborator that catches what you missed. This page explains why that trade-off pays off. See [[vision]] for the broader project context.

## The Compile Loop

The Polyglot development workflow is deliberately iterative:

1. **Write** — Express your intent in `.pg` files. Define pipelines, triggers, permissions, error handling. Focus on *what* should happen, not *how*.
2. **Compile** — The compiler analyses your code and reports every gap it finds. Missing error handlers, unsatisfied permissions, race conditions in parallel blocks, type mismatches across language boundaries.
3. **Fix** — Address each compiler error. Add the missing error path. Grant the required permission. Resolve the type mismatch. Each fix makes the pipeline more complete.
4. **Compile again** — When the compiler reports no errors, the pipeline is ready. Every scenario is accounted for.
5. **Deploy** — The compiled Behavioral Contract is registered with the Polyglot Service. What you deploy is exactly what the compiler verified.

This loop may feel unfamiliar to developers used to "write and run" workflows. But it is the same principle that makes Rust's borrow checker valuable, or that makes TypeScript's type system worth the annotation overhead. The investment is upfront; the payoff is in production.

## The Compiler as Collaborator

The compiler is not trying to stop you from shipping. It is trying to stop you from shipping something that will break at 3 AM on a Saturday.

Every error the compiler raises represents a scenario that *will* happen in production if left unhandled. A missing `[!]` error handler means a pipeline that crashes silently when the external API returns an unexpected response. An unsatisfied `{_}` permission means a job that fails at runtime because it cannot access the file it needs. A parallel write conflict means data corruption that only surfaces under load.

The compiler finds these problems in seconds. Finding them in production takes hours of debugging, incident response, and postmortem analysis. The compiler is doing the hard work so you do not have to.

## The Exhaustive Trade-Off

Polyglot demands exhaustive coverage. Every conditional must account for every possible case. Every parallel job must have its output collected. Every error must be handled or explicitly acknowledged. There is no "happy path only" code.

This can feel tiresome. Writing an `[!]` handler for an error you think will never happen feels like busywork. Adding a `*?` wildcard catch-all to a conditional that "obviously" covers all cases feels redundant.

But this is precisely the point. The errors you think will never happen are the ones that crash your system in production. The cases you think are "obviously" covered are the ones that produce undefined behaviour when an upstream API changes its response format. Polyglot does not allow you to express certainty about things that are inherently uncertain.

The exhaustive logic is offloading work. Not adding work — *moving* it. The debugging, the incident response, the "why did this pipeline fail at 3 AM" investigation — that work exists regardless. The question is whether you do it proactively at compile time, in a controlled environment with clear error messages, or reactively at runtime, in production, under pressure.

## Fix Before Deploy, Not Debug in Production

Traditional automation development follows a pattern: write code, deploy it, discover failures in production, debug under time pressure, patch, redeploy. Each cycle costs time, trust, and sometimes data.

Polyglot inverts this. Every failure mode the compiler catches is a failure that will never reach production. The compile loop is longer than "write and run," but the deploy-to-incident loop is shorter — often zero, because the incidents were prevented at compile time.

This is not a theoretical benefit. It is a structural property of the system. The Behavioral Contract that the Service executes is the same contract the compiler verified. There is no gap between "what the compiler checked" and "what runs in production." The deployed behaviour is exactly the verified behaviour.

See [[error-philosophy]] for the Murphy's Law principle that drives exhaustive error handling, and [[behavioral-contract]] for what the compiler produces.
