---
audience: ai
type: audit-reference
scope: glossary
updated: 2026-04-03
---

# Glossary — Authoritative Definitions

<!-- @vision:The Polyglot Ecosystem -->
<!-- @vision:Ways of Integration -->
Source: [[vision]]

**Rule:** If a term is defined here, Claude MUST use this definition. Do not improvise or paraphrase.

| Term | Definition | NOT this |
|------|-----------|----------|
| Polyglot | The project and platform as a whole | Not just the language |
| Polyglot Code | The .pg language for workflow orchestration | Not Python/Rust/JS code used within Polyglot |
| Polyglot Service | Runtime backbone: Trigger Monitor + Queue Handler + Runner | Not the language itself |
| Trigger Monitor | Component that monitors events (file changes, schedules, webhooks, resource availability) that initiate automated tasks. Evaluates conditions and sends command signals — the decision-maker | Not a scheduler or cron |
| Dispatch Coordinator | The autonomous dispatch loop inside the Queue Handler. Reads from all Dispatch Queues simultaneously, selects candidates via two-tier round-robin, and enforces concurrency/resource constraints. Event-driven — wakes on queue state changes, sleeps when empty. Not a separate component — it is the QH's internal dispatch mechanism | Not a decision-maker for trigger logic — it only applies mechanical scheduling rules (queue order, slot limits) |
| Queue Handler | Component that reacts to signals from the Trigger Monitor, managing queue state and dispatching jobs to Runners. Never evaluates trigger conditions or business logic — dispatch scheduling is handled by its internal Dispatch Coordinator | Not a trigger-condition evaluator — it executes Trigger Monitor commands and schedules dispatch mechanically |
| Runner | Component that executes pipelines, managing task lifecycle from dispatch to completion | Not a compiler |
| Instance | The Nth concurrent run of the same pipeline definition, numbered sequentially (`:0`, `:1`, `:2`) in the metadata tree at `%-:Pipeline:N`. One definition can have many simultaneous instances; each instance contains its own jobs, IO state, and metadata values | Not a Job — an Instance is the whole pipeline run; Jobs are units of work within it |
| Job | A unit of work within a pipeline instance, created at IO boundaries. Identified by UID + hierarchy path. Sequential `[-]` jobs chain on predecessor completion; parallel `[=]` jobs fork. Jobs have lifecycle state (`#QueueState`) and live at `%-:Pipeline:N.jobs:UID` in the metadata tree. See `#Job` type | Not the pipeline definition or Instance — a Job is a sub-unit within an Instance |
| Pipeline | A chain of async tasks with defined data flow | Not a shell pipeline |
| Fork | Conditional branching within a pipeline | Not a process fork |
| Async-centric | Async as foundational design principle, not bolted on | Not "async-capable" or "async-supported" |
| Cross-Language Integration | Pillar 1: running code in multiple languages together seamlessly | Not just FFI |
| Async-Centric Automation | Pillar 2: first-class parallelism, concurrency, race condition handling | Not traditional async/await |
| Integration Evolution | Phase 1: orchestration (FFI/pybind); Phase 2: variable-level | Not a one-time migration |
| RawString | The only true primitive type — literal raw characters, no interpolation, no substitution. Compiler intrinsic | Not `string` (`#String`), which is a struct with `.string` and `.regex` subfields |
| #String | Struct type for `#string` — contains `.string#RawString` (value) and `.regex#RawString` (RE constraint; alias: `.re`). `int`/`float` are flexible subtypes of `#String` | Not the primitive — `RawString` is the primitive |
| *Agg | Canonical shorthand alias for `*Aggregate`. Aggregation collectors that reduce expand outputs to a single value (sum, count, max, etc.). Always use `*Agg` in Polyglot Code | Not `*Aggregate` in code — `*Aggregate` is the valid long form but `*Agg` is the standard namespace used in pglib and EBNF |
| .pg files | Polyglot source files written in Polyglot Code | Not configuration files |

## Adding Terms

Add new terms as specs are written. Keep alphabetical within sections. Every term needs:
- **Definition** — what it IS (precise, unambiguous)
- **NOT this** — what it is NOT (prevents common confusion)
