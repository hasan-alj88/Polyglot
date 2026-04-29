---
audience: developer
type: guide
updated: 2026-04-23
---

# Development Guide

<!-- @c:vision -->
<!-- @c:audit/README -->
<!-- @c:audit/reference/glossary -->
This guide orients new contributors to the Aljam3 repository. Aljam3 is currently in its Design & Architecture Spec milestone, so the contribution surface is documentation authorship, design decisions, and PAUL-managed issue work — the compiler implementation has not begun. Authority chain: [[vision|c:vision]] > [[audit/README|c:audit/README]] > `.paul/`. All terms match [[audit/reference/glossary|c:glossary]] exactly.

## Overview

| Dimension | Current state |
|-----------|---------------|
| Language specification | Complete for v0.2 — see [[user/SPEC-INDEX\|u:SPEC-INDEX]] |
| Compile-rule catalog | 188 PGE, 31 PGW, 3 algorithms — see [[technical/COMPILE-RULES\|u:COMPILE-RULES]] |
| Service architecture | Specified but not implemented — see [[architecture\|u:architecture]] |
| Compiler code | Not present — all Rust was removed in the 2026-03-12 reset |
| Tooling | Python 3.12+ under `uv`; PDF/audit scripts in `scripts/` |

Contribution today means: extend the language spec, resolve audit inconsistencies, capture design decisions, or author philosophy pages.

## Prerequisites

<!-- @u:README -->
Required:

| Tool | Purpose |
|------|---------|
| `git` | Version control |
| `uv` (Python package manager) | Managed Python environment for audit and PDF scripts |
| Python 3.12+ | Runtime for tooling (declared in `pyproject.toml`) |

Recommended:

| Tool | Purpose |
|------|---------|
| Obsidian | `docs/` is an Obsidian vault; wikilinks, graph view, and backlinks work out of the box |
| A Markdown-aware IDE | For direct YAML frontmatter and wikilink editing |

## Setting Up the Repo

```sh
git clone <repo-url> Aljam3
cd Aljam3
uv sync
```

Directory orientation: see [[source-tree-analysis|u:source-tree-analysis]] for the full repo shape map. Key locations:

| Path | Role |
|------|------|
| `docs/vision.md` | Authoritative product vision |
| `docs/audit/README.md` | Documentation ground truth — read before writing |
| `docs/user/` | External-audience spec (automation-builder, integrator) |
| `docs/technical/` | Internal-audience spec (design, developer) |
| `.paul/STATE.md` | Live project state — active issue, branch, handoffs |
| `.paul/phases/` | Per-issue plan directories |
| `scripts/` | Shell scripts for PDF generation and doc tooling |

## The Docs-First Mindset

<!-- @c:philosophy/core-philosophy -->
<!-- @c:.paul/PROJECT -->
The [[.paul/PROJECT|c:.paul/PROJECT]] constraint is "Documentation-first: no code until specification is complete." The reasoning appears in [[philosophy/core-philosophy|c:core-philosophy]] and [[philosophy/behavioral-contract|c:behavioral-contract]]: Aljam3 compiles `.aj3` source into a [[audit/reference/glossary|c:Behavior Contract]] — a signal-graph IR read by the Aljam3 Service. The contract shape must be fully specified before any compiler pass is written, otherwise the compiler and runtime diverge.

Contributions that precede compiler implementation are therefore specification contributions: grammar productions, compile rules, type-system descriptors, pglib operator docs, philosophy sub-pages, audit-rule refinements, and architecture specs.

## How to Make a Documentation Change

<!-- @c:audit/README -->
<!-- @c:audit/rules/conventions -->
<!-- @c:audit/rules/checklist -->
<!-- @c:audit/audiences/ai-finder -->
Every documentation change follows the same pattern:

| Step | Action | Reference |
|------|--------|-----------|
| 1 | Read [[audit/README\|c:audit/README]] in full | [[audit/README\|c:audit/README]] |
| 2 | Identify the target audience(s) from [[audit/reference/glossary\|c:Audience Tiers]] | [[audit/reference/glossary\|c:glossary]] |
| 3 | Read the audience profile in `docs/audit/audiences/` | [[audit/audiences/ai-finder\|c:ai-finder]], etc. |
| 4 | Follow writing conventions: frontmatter, headings, code fences, typed refs | [[audit/rules/conventions\|c:conventions]] |
| 5 | Use `[[wikilinks]]` for navigation and `@c:`/`@u:`/`@d:` HTML comments for typed imports | [[audit/README\|c:audit/README]] (Typed Smart Referencing) |
| 6 | Use [[audit/reference/glossary\|c:glossary]] terms verbatim — do not improvise synonyms | [[audit/reference/glossary\|c:glossary]] |
| 7 | Run the pre-publish checklist against the file | [[audit/rules/checklist\|c:checklist]] |
| 8 | Commit with a scoped message and open a PR | (standard git workflow) |

Every `@c:`/`@u:` HTML comment must be paired with a `[[wikilink]]` — `@`-imports serve AI agents, wikilinks serve Obsidian readers.

## Fix, Sweep, and Gate Workflows

<!-- @c:audit/rules/workflows -->
Three canonical documentation workflows are defined in [[audit/rules/workflows|c:workflows]]:

| Workflow | When to use |
|----------|-------------|
| Fix | Resolving a known inconsistency from [[audit/tracking/inconsistencies\|u:tracking/inconsistencies]] |
| Sweep | Proactively auditing an area marked Not Started in [[audit/tracking/progress\|u:tracking/progress]] |
| Gate | Before finalizing any new or modified documentation — runs the checklist plus a coverage-gap pass |

Every Fix, Sweep, or Gate change routes through a GitHub issue with the `docs` label.

## Issue-Driven Development (PAUL)

<!-- @c:.paul/PROJECT -->
<!-- @u:.paul/STATE -->
<!-- @u:.paul/ROADMAP -->
Contribution flow is PAUL-managed. An issue becomes a phase directory under `.paul/phases/`, containing numbered plans that traverse four loop states:

| Loop state | Meaning | Outcome |
|------------|---------|---------|
| PLAN | A plan file with tasks, boundaries, and acceptance criteria exists and awaits approval | `NN-PP-PLAN.md` |
| APPLY | Plan tasks execute in sequence; checkpoints pause for human decisions or verification | Files modified |
| UNIFY | Executed work is reconciled against the plan; deviations recorded; summary written | `NN-PP-SUMMARY.md` |
| MERGE | Branch merges to main; STATE.md updated; issue closed | Merge commit on `main` |

Active state lives in [[.paul/STATE|u:.paul/STATE]]. Milestone sequencing lives in [[.paul/ROADMAP|u:.paul/ROADMAP]]. Specialized-flow routing lives in [[.paul/SPECIAL-FLOWS|u:.paul/SPECIAL-FLOWS]].

## Decision Records

<!-- @c:audit/decisions/README -->
Design decisions that affect any of the following must be recorded under [[audit/decisions/README|c:decisions/]]:

- Syntax (operators, markers, block types, grammar productions)
- Compiler rules (new or retired PGE/PGW codes; severity changes)
- Compiler operations (phases, compilation behavior)
- Type system (types, schemas, type-relationship changes)
- Runtime (execution model, runtime constructs)
- Audience (audience definitions, documentation routing)
- Process (workflows, tooling, conventions)

File naming: `YYYY-MM-DD-short-title.md`. The PAUL UNIFY and MERGE phases check `STATE.md` for `[DR]`-flagged decisions and prompt for a record in this folder when one is missing.

## Glossary Discipline

<!-- @c:audit/reference/glossary -->
All project terminology must match [[audit/reference/glossary|c:glossary]] exactly. Examples of terms that must be used verbatim: [[audit/reference/glossary|c:Aljam3]], [[audit/reference/glossary|c:Aljam3 Code]], [[audit/reference/glossary|c:Aljam3 Service]], [[audit/reference/glossary|c:Trigger Monitor]], [[audit/reference/glossary|c:Queue Handler]], [[audit/reference/glossary|c:Dispatch Coordinator]], [[audit/reference/glossary|c:Runner]], [[audit/reference/glossary|c:Instance]], [[audit/reference/glossary|c:Job]], [[audit/reference/glossary|c:Pipeline]], [[audit/reference/glossary|c:Behavior Contract]], [[audit/reference/glossary|c:Reconciliation]], [[audit/reference/glossary|c:RawString]]. Introducing a synonym violates the checklist item `glossary-check`.

When a new term is needed, add it to the glossary first (with a `Definition` and a `NOT this` entry), then cite it from downstream docs.

## CLAUDE.md and AI-Agent Flows

<!-- @d:CLAUDE -->
The repository root contains [[CLAUDE|d:CLAUDE]], a short Claude-Code instruction file. It exists to orient AI agents (Claude Code, subagents, the `/pg:*` and `/paul:*` skills) to the authority chain. Human contributors do not need to read it before contributing. The `/pg:generate` and `/pg:train` skills are internal documentation-tooling commands, not the project's public positioning; they do not appear in the user-facing documentation.

## Getting Help

| Need | Where |
|------|-------|
| Bug or design-issue report | GitHub Issues (use the `docs` label for documentation issues) |
| Active project state | [[.paul/STATE\|u:.paul/STATE]] |
| Issue triage and milestone order | [[.paul/ROADMAP\|u:.paul/ROADMAP]] |
| Who owns a doc | git blame on the file in question |
