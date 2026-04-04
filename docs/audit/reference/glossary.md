---
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
| Queue Handler | Component that reacts to signals from the Trigger Monitor, managing queue state and dispatching jobs to Runners. Never evaluates conditions or makes decisions | Not a manager or decision-maker — it only executes commands |
| Runner | Component that executes pipelines, managing task lifecycle from dispatch to completion | Not a compiler |
| Job | A single enqueued instance of a pipeline, identified by UID + hierarchy path. One pipeline can produce multiple concurrent jobs. Jobs have lifecycle state (`#QueueState`); pipelines have definitions | Not the pipeline definition itself |
| Pipeline | A chain of async tasks with defined data flow | Not a shell pipeline |
| Fork | Conditional branching within a pipeline | Not a process fork |
| Async-centric | Async as foundational design principle, not bolted on | Not "async-capable" or "async-supported" |
| Cross-Language Integration | Pillar 1: running code in multiple languages together seamlessly | Not just FFI |
| Async-Centric Automation | Pillar 2: first-class parallelism, concurrency, race condition handling | Not traditional async/await |
| Integration Evolution | Phase 1: orchestration (FFI/pybind); Phase 2: variable-level | Not a one-time migration |
| RawString | The only true primitive type — literal raw characters, no interpolation, no substitution. Compiler intrinsic | Not `string` (`#String`), which is a struct with `.string` and `.regex` subfields |
| #String | Struct type for `;string` — contains `.string;RawString` (value) and `.regex;RawString` (RE constraint; alias: `.re`). `int`/`float` are flexible subtypes of `#String` | Not the primitive — `RawString` is the primitive |
| .pg files | Polyglot source files written in Polyglot Code | Not configuration files |

## Adding Terms

Add new terms as specs are written. Keep alphabetical within sections. Every term needs:
- **Definition** — what it IS (precise, unambiguous)
- **NOT this** — what it is NOT (prevents common confusion)
