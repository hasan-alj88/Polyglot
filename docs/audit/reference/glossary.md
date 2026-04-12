---
audience: ai
type: audit-reference
scope: glossary
updated: 2026-04-12
---

# Glossary — Authoritative Definitions

<!-- @c:vision:The Polyglot Ecosystem -->
<!-- @c:vision:Ways of Integration -->
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
| Instance | A composite job that runs sub-jobs in accordance with a pipeline definition. The Nth concurrent run is numbered sequentially (`:0`, `:1`, `:2`) at `%-:Pipeline:N`. Each instance contains its own jobs, IO state, and metadata values. An instance IS a job — specifically the root composite job created when a pipeline is triggered | Not a definition — an Instance is a running composite job; the Pipeline is the definition |
| Job | A task queued for execution. Two kinds: **atomic** (`{N}` native — single unit, no sub-jobs) and **composite** (instance of a `{-}` pipeline — runs sub-jobs). IO inputs must be Final to start (pulling Default promotes to Final); IO outputs Final signals completion. Identified by UID, lives at `%-:Pipeline:N.jobs:UID` | Not the pipeline definition — a Job is an executable unit; the Pipeline defines how jobs relate |
| Pipeline | The `{-}` DEFINITION of how a series and parallel combination of jobs inter-relate — triggers, queue configuration, setup, execution body with concurrency description, and cleanup. A pipeline is not a running thing; it produces Instances (composite jobs) when triggered | Not a running process — it is the definition; Instances are the running jobs |
| Fork | Conditional branching within a pipeline | Not a process fork |
| Async-centric | Async as foundational design principle, not bolted on | Not "async-capable" or "async-supported" |
| Cross-Language Integration | Pillar 1: running code in multiple languages together seamlessly | Not just FFI |
| Async-Centric Automation | Pillar 2: first-class parallelism, concurrency, race condition handling | Not traditional async/await |
| Integration Evolution | Phase 1: orchestration (FFI/pybind); Phase 2: variable-level | Not a one-time migration |
| RawString | The only true primitive type — literal raw characters, no interpolation, no substitution. Compiler intrinsic | Not `string` (`#String`), which is a struct with `.string` and `.regex` subfields |
| #String | Struct type for `#string` — contains `.string#RawString` (value) and `.regex#RawString` (RE constraint; alias: `.re`). `int`/`float` are flexible subtypes of `#String` | Not the primitive — `RawString` is the primitive |
| *Agg | Canonical shorthand alias for `*Aggregate`. Aggregation collectors that reduce expand outputs to a single value (sum, count, max, etc.). Always use `*Agg` in Polyglot Code | Not `*Aggregate` in code — `*Aggregate` is the valid long form but `*Agg` is the standard namespace used in pglib and EBNF |
| Reconciliation | The process by which parallel job outputs are collected and parallel jobs are terminated according to specified collector strategies (aggregation, selection, barrier, discard). Every parallel job must be reconciled — this is what makes Polyglot's parallelism safe by construction. The collector determines both output handling (what happens to the data) and job lifecycle (when the job is cancelled or allowed to complete) | Not just "collecting output" — reconciliation includes the job termination decision. Not a runtime concept alone — the compiler validates reconciliation completeness (PGE03002) |
| Operation Label | A `($)` IO line inside a pipeline call that names the call's IO for downstream access via `$Label>outputParam`. Labels become Final when the operation completes and are read-only. See [[operation-labels]] | Not a variable declaration — labels are auto-populated from the operation's IO, not user-assigned |
| Chain Step Label | A `(.)` IO line indented under `($)` in a chain call that names an individual chain step by position. Access via `>$Label.param` in chain IO lines. See [[operation-labels#Chain Step Labels]] | Not a chain-level label — `($)` labels the whole chain, `(.)` labels steps within it |
| IO Comment | A `( )` bracket (parentheses with space) introducing an inline comment within `(X)` IO blocks. The IO-family equivalent of `[ ]` block comments. See [[comments]] | Not `[ ]` — `( )` is scoped to IO context only |
| Data Tree | A tree of serialized strings — the universal data representation in Polyglot. "Data tree" and "serialized data" are synonyms: all Polyglot objects are trees, all tree leaves are `RawString`, and the serialized (JSON) form IS the data. See [[data-is-trees]] | Not a separate concept from serialized data — they are the same thing viewed from different angles (structure vs format) |
| #serial | The untyped serialized data tree type — a `#` struct with no schema constraints. Accepts any tree topology. Used when a pipeline returns structured data whose shape is not known at definition time (e.g., `-RT.*.Function.*` return values) | Not a primitive — `#serial` is a `{#}` struct with no `##` schema, not a `RawString`. Not "JSON" — it is a Polyglot data tree that happens to serialize as JSON |
| .pg files | Polyglot source files written in Polyglot Code | Not configuration files |
| ##Record | Parameterized schema for enum-keyed flat collections. Takes `(#) <#Fields << ##Enum` and `(#) <#ValueType <~ #`. Replaces ##Map (#275) | Not #Map — ##Record uses enum keys, not arbitrary string keys |
| #FieldsDescriptor | Enum type with `.Range` (alias `#Range`) and `.Enum` (alias `#Enum`). Used by `%##Fields` property to describe how a type's children are keyed | Not `#FlexKind` (retired #275) |
| %##Fields | Branch-level schema property taking `#FieldsDescriptor` or `##Enum` ref. `#Range` = integer-indexed; enum ref = stamp children from variants. Replaces `%##Key`, `%##Range`, `%##Flexible` (#275) | Not `%##Key` (retired #275) |
| %##Count | Branch-level schema property taking `#Bound`. Maximum number of children (`#Inf` = unlimited). Replaces `%##Range` for bounds (#275) | Not `%##Range` (retired #275) |
| #Map | *(Retired #275)* — replaced by ##Record (enum-keyed) or custom `{#}` types | Not active — use ##Record |
| #Set | *(Retired #275)* — replaced by `#Array` + `%###Unique << #True` | Not active — use #Array with uniqueness constraint |

## Adding Terms

Add new terms as specs are written. Keep alphabetical within sections. Every term needs:
- **Definition** — what it IS (precise, unambiguous)
- **NOT this** — what it is NOT (prevents common confusion)
