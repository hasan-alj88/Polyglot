---
audience: product
type: reference
updated: 2026-04-23
---

# Aljam3 — Project Overview

<!-- @c:vision -->
<!-- @c:audit/reference/glossary -->
<!-- @c:audit/README -->
This document is the product-shaped entry point to the Aljam3 project. It answers "what is Aljam3, who is it for, and what do I do next" for product stakeholders, reviewers, and new contributors. For the authoritative product vision see [[vision|c:vision]]. For philosophy pages see [[philosophy/core-philosophy|c:core-philosophy]]. For documentation ground truth see [[audit/README|c:audit/README]]. All project terminology matches [[audit/reference/glossary|c:glossary]] exactly.

## What Aljam3 Is

<!-- @c:vision:The Aljam3 Ecosystem -->
<!-- @c:audit/reference/glossary -->
Aljam3 is a trigger-driven programming language and platform, async-centric and parallel-by-design. The project name refers to the whole platform — language plus runtime service — per [[audit/reference/glossary|c:Aljam3]]. Developers write [[audit/reference/glossary|c:Aljam3 Code]] (`.aj3` files) to define automated workflows; the [[audit/reference/glossary|c:Aljam3 Service]] executes them.

Two pillars define the scope (see [[vision|c:vision]]):

1. Cross-Language Integration — compose workflows that call well-tested code written in Python, Rust, Go, and JavaScript through a unified language surface.
2. Trigger-Driven Orchestration — concurrency, parallelism, race-handling, error-handling, and resource-management are first-class language constructs rather than runtime libraries.

## Who It's For

<!-- @c:vision:Who Is Aljam3 For? -->
Aljam3 targets two external audiences and four internal audiences, matching the six-tier model in [[audit/reference/glossary|c:Audience Tiers]]:

| Audience tier | Group | Who they are |
|---------------|-------|--------------|
| automation-builder | External | Developers who write `.aj3` files to orchestrate multi-language workflows |
| integrator | External | Developers who connect existing codebases via SDK/API without authoring `.aj3` files |
| product | Internal | Product managers who define requirements and acceptance criteria (this document's primary audience) |
| design | Internal | Language and architecture designers: grammar, compile rules, philosophy, service design |
| developer | Internal | Implementers: compiler passes, pglib operators, runtime services, tests |
| ai-finder | Internal | AI agents navigating the documentation: indexes, retrieval metadata, structured summaries |

Aljam3 is not a general-purpose programming language. It does not replace Python, Rust, Go, or JavaScript; it orchestrates workflows that call them.

## What's Included

<!-- @u:vision:The Aljam3 Ecosystem -->
<!-- @u:vision:Ways of Integration -->
The project spans four concerns:

| Concern | Definition | Authoritative docs |
|---------|-----------|--------------------|
| The language | `.aj3` syntax: blocks, pipelines, types, collections, errors, permissions | [[user/SPEC-INDEX\|u:SPEC-INDEX]], [[user/syntax/blocks\|u:blocks]], [[user/concepts/pipelines\|u:pipelines]] |
| The standard library (pglib) | Built-in operators: `-File.*`, `-T.*`, `-Q.*`, `-W.*`, `-Math.*`, `-Run.*`, `-DT.*`, `=ForEach.*`, `*Into.*`, `*Agg.*` | [[user/pglib/INDEX\|u:pglib/INDEX]] |
| The compiler | Grammar, compile-rule catalog, algorithms (cycle detection, overlap detection, compound exhaustiveness, reconciliation) | [[technical/INDEX\|u:technical/INDEX]], [[technical/ebnf/INDEX\|u:ebnf/INDEX]], [[technical/COMPILE-RULES\|u:COMPILE-RULES]] |
| The service | Runtime components (Trigger Monitor, Queue Handler, Runner), SDK, cross-language bridge, observability | [[technical/spec/behavior-contract\|u:behavior-contract]], [[technical/spec/aljam3-sdk\|u:aljam3-sdk]], [[technical/spec/otel-foundation\|u:otel-foundation]] |

## Project Status

<!-- @u:.paul/STATE -->
Aljam3 is in its Design & Architecture Spec milestone (M2). The authoritative record of status is [[.paul/STATE|u:.paul/STATE]] (live). Key facts:

| Dimension | State |
|-----------|-------|
| Language spec (v0.2) | Complete — grammar, type system, operators, pipelines, collections, pglib catalogued |
| Compile-rule catalog | 188 PGE error rules, 31 PGW warning rules, 3 algorithms — all documented |
| Service architecture | Designed and specified (Trigger Monitor, Queue Handler, Dispatch Coordinator, Runner, SDK, `-Run.Bridge`); implementation not started |
| Compiler implementation | Not started — all Rust code was removed in the 2026-03-12 reset; the project is documentation-first until the spec stabilises |
| Documentation framework | Audit framework live: [[audit/README\|c:audit/README]], [[audit/rules/conventions\|c:conventions]], [[audit/rules/checklist\|c:checklist]], [[audit/reference/glossary\|c:glossary]] |

The project is not suitable for production use. APIs, architecture, and syntax remain subject to change.

## Getting Started

Where to go depends on why you are here:

| Goal | Entry point |
|------|-------------|
| Read the vision | [[vision\|c:vision]] |
| Learn `.aj3` language syntax | [[user/SPEC-INDEX\|u:SPEC-INDEX]] |
| Look up a pglib operator | [[user/pglib/INDEX\|u:pglib/INDEX]] |
| Look up a compile-rule code (PGE/PGW) | [[technical/COMPILE-RULES\|u:COMPILE-RULES]] |
| Understand the service architecture | [[architecture\|u:architecture]] |
| Navigate the repository | [[source-tree-analysis\|u:source-tree-analysis]] |
| Find a component by category | [[component-inventory\|u:component-inventory]] |
| AI agent — retrieve documentation | [[ai-retrieval-index\|u:ai-retrieval-index]] |
| Contribute documentation | [[development-guide\|u:development-guide]] |
| Check documentation rules | [[audit/README\|c:audit/README]] |
| See the full master index | [[INDEX\|u:INDEX]] |

## Governance & Authority

<!-- @c:audit/README:Authority Chain -->
Authority is ordered top-to-bottom per [[audit/README|c:Authority Chain]]:

| Priority | Source | Governs |
|----------|--------|---------|
| 1 | [[vision\|c:vision]] + [[philosophy/core-philosophy\|c:core-philosophy]] | Product vision, philosophy, what Aljam3 is |
| 2 | [[audit/README\|c:audit/README]] | How to write documentation |
| 3 | [[.paul/PROJECT\|c:.paul/PROJECT]] | Project management process |

If any document contradicts [[vision|c:vision]], vision wins. If a term appears in [[audit/reference/glossary|c:glossary]], its definition there is binding.

Work is issue-driven through the PAUL workflow ([[.paul/README|c:.paul/README]] if present, else the `.paul/` directory as a whole). Each GitHub issue becomes a phase directory under `.paul/phases/`, which contains numbered plan files (`NN-PP-PLAN.md`) that go through PLAN → APPLY → UNIFY → MERGE loops. Design decisions that affect syntax, compile rules, the type system, runtime, or audience definitions are recorded under [[audit/decisions/README|c:decisions/]].
