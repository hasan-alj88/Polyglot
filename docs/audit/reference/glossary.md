---
type: audit-reference
scope: glossary
updated: 2026-03-14
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
| Polyglot Service | Runtime backbone: Trigger Monitor + Queue Manager + Runner | Not the language itself |
| Trigger Monitor | Component that monitors events (file changes, schedules, webhooks, resource availability) that initiate automated tasks | Not a scheduler or cron |
| Queue Manager | Component that manages task queue, priority, dispatch conditions, and resource limits | Not an executor |
| Runner | Component that executes pipelines, managing task lifecycle from dispatch to completion | Not a compiler |
| Pipeline | A chain of async tasks with defined data flow | Not a shell pipeline |
| Fork | Conditional branching within a pipeline | Not a process fork |
| Async-centric | Async as foundational design principle, not bolted on | Not "async-capable" or "async-supported" |
| Cross-Language Integration | Pillar 1: running code in multiple languages together seamlessly | Not just FFI |
| Async-Centric Automation | Pillar 2: first-class parallelism, concurrency, race condition handling | Not traditional async/await |
| Integration Evolution | Phase 1: orchestration (FFI/pybind); Phase 2: variable-level | Not a one-time migration |
| .pg files | Polyglot source files written in Polyglot Code | Not configuration files |

## Adding Terms

Add new terms as specs are written. Keep alphabetical within sections. Every term needs:
- **Definition** — what it IS (precise, unambiguous)
- **NOT this** — what it is NOT (prevents common confusion)
