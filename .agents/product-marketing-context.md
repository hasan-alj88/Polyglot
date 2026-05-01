# Product Marketing Context

*Last updated: 2026-04-14*

## Product Overview
**One-liner:** Polyglot is the first parallel programming automation language — trigger-driven, cross-language, and safe by construction.
**What it does:** Polyglot lets developers define automated workflows that span Python, Rust, JavaScript, C++, and other languages — all from a single unified syntax. Every pipeline is event-triggered with compiler-enforced safety for parallelism, error handling, and resource governance.
**Product category:** Parallel programming automation language / workflow orchestration platform
**Product type:** Open-source developer tool (language + runtime service)
**Business model:** Open-source (license TBD: Apache 2.0 or MIT). Currently pre-revenue, specification phase.

## Target Audience
**Target users:**
- **Automation Builders** — developers who write .pg files to define triggers, pipelines, and parallel workflows
- **Integrators** — developers who connect existing codebases to Polyglot via SDKs/APIs without writing .pg code
**Primary use case:** Orchestrating multi-language automated workflows with safe parallelism
**Jobs to be done:**
- "I need Python, Rust, and JS to work together without brittle glue code"
- "I need parallel/async workflows that are safe by construction, not by luck"
- "I want to reuse my battle-tested existing code, not rewrite it"
**Use cases:**
- Data Engineering — ETL pipelines combining Python preprocessing with Rust analytics
- DevOps — CI/CD orchestration across multiple tools and languages
- Machine Learning — Train in Python, serve with Rust for performance
- System Automation — Scheduled maintenance across heterogeneous systems
- Legacy Integration — Bridge old C++ systems with modern services
- IoT Processing — Real-time sensor data processing with resource limits

## Personas

| Persona | Cares about | Challenge | Value we promise |
|---------|-------------|-----------|------------------|
| Automation Builder | Writing pipelines that react to events safely | Managing concurrency across languages is error-prone | Compiler-enforced safety — race conditions and missing error paths are compile errors |
| Integrator | Connecting existing codebases without rewrites | Glue code between Python/Rust/JS is fragile and hard to maintain | Unified platform that bridges languages — reuse what already works |
| DevOps/Platform Engineer | Reliable CI/CD and system automation | Multi-tool orchestration is brittle with shell scripts | Trigger-driven pipelines with built-in queuing and resource management |

## Problems & Pain Points
**Core problem:** Modern automation requires multiple programming languages, but there's no safe, unified way to orchestrate them together with proper concurrency handling.
**Why alternatives fall short:**
- Shell scripts / glue code: brittle, no type safety, no parallelism guarantees
- Workflow engines (Airflow, Temporal): heavy infrastructure, DAG-centric not language-centric
- Single-language async (asyncio, tokio): powerful but isolated to one runtime
**What it costs them:** Bugs discovered in production instead of at compile time. Time spent debugging race conditions, unhandled errors, and cross-language data mismatches.
**Emotional tension:** Fear of deploying concurrent workflows that "work on my machine" but fail under real load. Frustration with rewriting working code just to fit a single ecosystem.

## Competitive Landscape
**Direct:** No direct competitor — the first parallel programming automation language is a new category
**Secondary:** Airflow, Temporal, Prefect — workflow orchestration engines (DAG-based, not a language; no compile-time safety)
**Indirect:** Writing everything in one language; microservices with REST/gRPC glue; shell scripting

## Differentiation
**Key differentiators:**
- First language purpose-built for parallel automation — not bolted onto a general-purpose language
- Compile-time safety for parallelism — race conditions, missing error paths, and permission gaps are compile errors
- Cross-language integration as a first-class feature, not FFI bolted on
- Exhaustive coverage — every error path, every parallel output, every conditional must be handled
- Implicit deny permissions — nothing runs without explicit grants (cybersecurity + concurrency safety)
- Trigger-driven by design — no main(), no sequential mindset
**How we do it differently:** The compiler enforces safety before code ever runs. Traditional approaches build synchronous code first, then retrofit async. Polyglot inverts this — triggers and concurrency are the starting point.
**Why that's better:** Problems caught at compile time don't become 3am production incidents.
**Why users choose us:** Reuse existing battle-tested code across languages with compiler-guaranteed safe parallelism — no rewrites, no glue code, no runtime surprises.

## Objections

| Objection | Response |
|-----------|----------|
| "It's not production-ready" | Correct — we're in specification phase, building in public. The compiler is next. |
| "I don't want to learn a new language" | Integrators don't have to — use the SDK. Automation Builders learn Polyglot Code, but your existing Python/Rust/JS code stays as-is. |
| "Workflow engines already solve this" | They orchestrate tasks but don't enforce safety. Polyglot catches concurrency bugs, missing error paths, and permission gaps at compile time. |

**Anti-persona:** Developers building single-language, synchronous applications with no cross-language or concurrency needs.

## Switching Dynamics
**Push:** Fragile glue code breaks in production; debugging cross-language race conditions is painful
**Pull:** Write once in unified syntax; compiler guarantees safety; reuse existing code
**Habit:** Teams already invested in shell scripts, Makefiles, or workflow engines
**Anxiety:** "Will this language survive? Is spec-phase too early to adopt?"

## Customer Language
**How they describe the problem:**
- "We have Python for ML, Rust for performance, JS for the frontend — and shell scripts holding it all together"
- "Our async code works until it doesn't — and we only find out in production"
**How they describe us:**
- "What if your API orchestration layer was a programming language?"
- "It's like a type system for concurrency"
**Words to use:** trigger-driven, pipelines, compile-time safety, cross-language, parallel-by-design, reuse legacy code, orchestration, parallel automation, safe by construction
**Words to avoid:** "just async," "serverless," "low-code," "no-code," "framework," "async-capable"

**Glossary:**

| Term | Meaning |
|------|---------|
| Polyglot | The project and platform as a whole — not just the language |
| Polyglot Code | The .pg language for workflow orchestration |
| Polyglot Service | Runtime backbone: Trigger Monitor + Queue Handler + Runner |
| Pipeline | A definition of how jobs inter-relate — not a running process |
| Trigger-driven | Every pipeline reacts to events; there is no main() |
| Implicit deny | Nothing runs without explicit permission grants |

## Brand Voice
**Brand position:** "The serious one." Polyglot doesn't brand on speed or simplicity — it brands on conviction. The entire design is built on the premise that "happy path only" code is irresponsible.

**Brand in one sentence:** Polyglot: the trigger-driven language that makes your existing code work together — and proves it at compile time.

**Tone:** Confident and technical, never hype-driven. Let the design speak for itself. Opinionated and unapologetic — the developers who get it will self-select.
**Style:** Show, don't tell. Code examples over marketing prose. Precise terminology.

**Three messaging pillars:**
1. **"Your code already works. Let it."** — The warmest angle. Polyglot doesn't ask people to throw away their Python or Rust. It respects existing work and makes it reusable. Lead with this — it lowers the threat response.
2. **"Compile-time, not 3am."** — The sharpest differentiator. If it compiles, entire classes of bugs don't exist. Memorable and specific.
3. **"There is no main()."** — The provocative hook. Signals "this thinks differently" without needing a paragraph. The thing that makes someone stop scrolling.

**Personality:**
- **Respectful-of-craft** — "On the shoulders of giants." Polyglot doesn't replace Python, Rust, or JS — it honours the decades of work in each ecosystem by making it reusable.
- **Safety-first** — Two dimensions: (1) cybersecurity — implicit deny permissions, no dynamic code, compile-time file binding; (2) parallel programming safety — race conditions, missing error paths, and resource conflicts are compile-time errors, not runtime surprises.
- **First parallel programming automation language** — This is a new category. We don't position against workflow engines or async libraries. We are the first language where parallelism and automation are the starting point, not an afterthought.

**Brand don'ts:**
- Don't brand as a workflow engine alternative — Airflow/Temporal comparisons shrink us into their category
- Don't lead with "parallel programming" — it's accurate but intimidating; lead with the outcome (safety, reuse) and let parallelism be the mechanism people discover
- Don't soften the opinionatedness — "every error path must be handled" is the point

## Proof Points
**Metrics:** Specification phase complete (v0.2); comprehensive language spec covering syntax, type system, pipelines, concurrency, error handling, permissions, and jm3lib standard library
**Community signals:** Building in public, seeking collaborators for compiler implementation (Rust)
**Testimonials:** *(none yet — pre-launch)*
**Value themes:**

| Theme | Proof |
|-------|-------|
| Compile-time safety | Race conditions, missing error paths, permission gaps are all compile errors |
| Reuse over rewrite | Cross-language integration preserves existing battle-tested code |
| Trigger-driven design | No main() — every pipeline reacts to events |
| Exhaustive coverage | Every conditional, every parallel output, every error path must be handled |
| Security by default | Implicit deny permissions + no dynamic code execution |

## Goals
**Business goal:** Build an active open-source community around Polyglot; attract compiler contributors
**Conversion action:** Star the repo / join as a contributor
**Current metrics:** Specification phase, pre-compiler
