# Roadmap: Polyglot

## Overview
Building an async programming language that can asynchronously compile other programming languages. Includes language specification, compiler infrastructure, and multi-language service layer (listener, queue manager, executioner).

## Milestone: v0.1 Language Specification & Research
**v0.1 Language Specification & Research** (v0.1.0)
Status: Complete
Phases: 2 of 2 complete

## Phases

| Phase | Name | Plans | Status | Completed |
|-------|------|-------|--------|-----------|
| 1 | Documentation Audit & Consolidation | 1 | Complete | 2026-03-12 |
| 2 | Clean Slate Reset | 1 | Complete | 2026-03-12 |

## Phase Details

### Phase 1: Documentation Audit & Consolidation

**Goal:** Audit existing documentation, remove outdated content, consolidate into a single coherent structure

**Depends on:** Nothing (first phase)

**Research:** Unlikely

**Scope:**
- Inventory all existing docs under docs/
- Identify outdated, duplicate, or contradictory content
- Consolidate into a clean documentation structure
- Establish documentation standards for the project

**Plans:** TBD (during phase planning)

### Phase 2: Clean Slate Reset

**Goal:** Commit all pending changes from Phase 1, remove archive tarballs, and reach a clean git state — true clean slate before spec writing begins

**Depends on:** Phase 1 (cleanup of Phase 1's uncommitted artifacts)

**Research:** None

**Scope:**
- Stage and commit all Phase 1 deletions (docs/Agile/, docs/v0.0.5/, docs/_graph.yaml)
- Remove archive tarballs from repo root
- Stage and commit all new/modified files (conventions, templates, indexes, scripts, pyproject)
- Remove stale handoff file
- Single clean commit for full reset

**Plans:** 1 (single-plan phase)

---

## Milestone: v0.2 Language Specification

**v0.2 Language Specification** (v0.2.0)
Status: Complete
Phases: 4 of 4 complete

Theme: Draft the complete Polyglot language specification from scratch using `/paul:draft` loop. Covers variables & lifecycle, operators, pipelines, fork patterns, error handling, loops, enums, type system, package spec, and block markers.

### Phases

| Phase | Name | Plans | Status | Completed |
|-------|------|-------|--------|-----------|
| 9 | Core Language & Type System | 1 | Complete | 2026-03-24 |
| 10 | Operators & Control Flow | 1 | Complete | 2026-03-24 |
| 11 | Pipelines & Concurrency | 1 | Complete | 2026-03-24 |
| 12 | Package System & Stdlib | 1 | Complete | 2026-03-24 |

### Phase 9: Core Language & Type System

**Goal:** Draft specification for variables, lifecycle, data definitions, type system, enums, type identity, block markers, and package declarations

**Depends on:** Nothing (first phase of v0.2)

**Research:** Likely — existing EBNF, EDGE-CASES, and brainstorming docs inform spec

**Scope:**
- Variables & lifecycle (declaration, states, push/pull semantics)
- Data definitions and schema matching
- Type system and type identity
- Enums and enum fields
- Block markers ({@}, {#}, {=}, {M})

**Plans:** TBD (during /paul:plan)

### Phase 10: Operators & Control Flow

**Goal:** Draft specification for operators, arithmetic, ranges, conditionals, exhaustiveness, loops, and string interpolation

**Depends on:** Phase 9 (type system needed for operator type rules)

**Research:** Likely — COMPILE-RULES PGE-4xx, PGE-6xx inform spec

**Scope:**
- Operators (comparison, negation, arithmetic)
- Range syntax and bounds
- Conditionals and exhaustiveness rules
- Loops
- String interpolation

**Plans:** TBD (during /paul:plan)

### Phase 11: Pipelines & Concurrency

**Goal:** Draft specification for pipeline structure, triggers, queues, wrappers, chains, expand/collect, parallel execution, fork patterns, race collectors, and error handling

**Depends on:** Phase 9 (type system), Phase 10 (operators for conditionals in pipelines)

**Research:** Likely — EDGE-CASES, COMPILE-RULES PGE-1xx/3xx/7xx/8xx inform spec

**Scope:**
- Pipeline structure ([t], [Q], [W], execution body)
- Chains and auto-wiring
- Expand/collect and parallel execution
- Fork patterns and race collectors
- Error handling and recovery ([!] blocks, chain errors, fallbacks)

**Plans:** TBD (during /paul:plan)

### Phase 12: Package System & Stdlib

**Goal:** Draft specification for packages, imports, multi-file projects, stdlib confirmations, and canonical examples

**Depends on:** Phase 11 (pipeline spec needed for stdlib pipeline definitions)

**Research:** Likely — stdlib confirmations (issues #26, #28) and COMPILE-RULES PGE-9xx

**Scope:**
- Package declarations and imports
- Multi-file project rules
- Stdlib confirmation (=T.*, =W.* speculative pipelines)
- Canonical examples for all language features

**Plans:** TBD (during /paul:plan)

---

## Milestone: v0.1.1 Doc Audit Infrastructure

**v0.1.1 Doc Audit Infrastructure** (v0.1.1)
Status: Complete
Phases: 3 of 3 complete

Theme: Establish Claude's ground-truth reference hub (docs/audit/) for structured, audience-aware documentation writing.

### Phases

| Phase | Name | Plans | Status | Completed |
|-------|------|-------|--------|-----------|
| 6 | Audit Structure & Core | 1 | Complete | 2026-03-14 |
| 7 | Audience Ground Rules | 1 | Complete | 2026-03-14 |
| 8 | Rules, Reference & Integration | 1 | Complete | 2026-03-14 |

### Phase 6: Audit Structure & Core

**Goal:** Create docs/audit/ nested structure, write README.md entry point with authority chain, and edit vision.md to remove AI meta-instructions

**Depends on:** Phase 2 (or can run independently — no code dependency, only needs vision.md stable)

**Research:** None

**Scope:**
- Create docs/audit/ directory tree (audiences/, rules/, reference/)
- Write docs/audit/README.md — file index, authority chain, smart reference guide
- Edit docs/vision.md — remove "Who It's For" section (lines 54-65)

**Plans:** TBD (during /paul:plan)

### Phase 7: Audience Ground Rules

**Goal:** Create per-audience ground rules files that instruct Claude how to write documentation for each audience

**Depends on:** Phase 6 (directory structure exists)

**Research:** None

**Scope:**
- Create docs/audit/audiences/user.md — tone, format, assumptions for user-facing docs
- Create docs/audit/audiences/developer.md — tone, format, assumptions for developer docs
- Create docs/audit/audiences/ai.md — tone, format, assumptions for AI-facing docs

**Plans:** TBD (during /paul:plan)

### Phase 8: Rules, Reference & Integration

**Goal:** Create writing conventions, quality checklist, glossary, and integrate audit system via project-level CLAUDE.md

**Depends on:** Phase 6 (directory structure), Phase 7 (audiences defined for cross-references)

**Research:** None

**Scope:**
- Create docs/audit/rules/conventions.md — style, structure, formatting rules
- Create docs/audit/rules/checklist.md — pre-publish quality checks
- Create docs/audit/reference/glossary.md — authoritative term definitions from vision.md
- Create CLAUDE.md at project root — points Claude to audit/ before doc writing
- Update .paul/STATE.md — record decisions

**Plans:** TBD (during /paul:plan)

---

## Issue Batch: Docs-Inconsistency Audit (#141-#159)

**Theme:** Resolve 19 documentation inconsistencies flagged by audit
**Created:** 2026-04-06
**Status:** Briefs prepared, 0 of 19 resolved

### Group Order

| Group | Theme | Issues | P2 | Status |
|-------|-------|--------|----|--------|
| 1 | EBNF / Compiler Rule Gaps | #158, #149, #146, #144, #150 | 3 | Pending |
| 2 | Syntax Documentation Gaps | #142, #155, #153, #156, #145 | 2 | Pending |
| 3 | Naming & Terminology | #151, #148, #143, #154, #141, #152 | 0 | Pending |
| 4 | Cross-Reference Errors | #159, #157 | 0 | Pending |
| 5 | Stdlib Classification | #147 | 0 | Pending |

### Issue Index

| # | Title | Group | Priority | Status |
|---|-------|-------|----------|--------|
| 158 | ## schema property prefix missing from EBNF Section 05 | 1 | P2-high | Done |
| 149 | PGE01001/PGE01002 pipeline ordering paradox | 1 | P2-high | Done |
| 146 | Semicolon (;) type annotation prefix missing | 1 | P2-high | Done |
| 144 | PGE05001 separator homogeneity contradicts flexible-fields | 1 | P2-high | Done |
| 150 | PGW01002/PGE01021 duplicate — warning vs error | 1 | P3-medium | Done |
| 142 | Queue definition syntax ambiguous (#Queue: vs {Q}) | 2 | P2-high | Done |
| 155 | Metadata path syntax gap user vs technical | 2 | P2-high | Done |
| 153 | =#.Column undocumented prefix collision | 2 | P3-medium | Pending |
| 156 | Array dimension :2D vs < access unmapped | 2 | P3-medium | Pending |
| 145 | Wait/collect markers [<]/[>] vs [*] undocumented | 2 | P3-medium | Pending |
| 151 | Scalar subtypes ##Int/##Float vs int/float naming | 3 | P3-medium | Pending |
| 148 | #Array vs #array case inconsistency | 3 | P3-medium | Pending |
| 143 | #Boolean enum struct vs enum classification | 3 | P4-low | Pending |
| 154 | *Agg shorthand not in glossary or vision | 3 | P4-low | Pending |
| 141 | Trigger Monitor role contradicts glossary vs vision | 3 | P3-medium | Pending |
| 152 | *Continue collector vs error recovery classification | 3 | P3-medium | Pending |
| 159 | Audience routing sends wrong groups to wrong docs | 4 | P3-medium | Pending |
| 157 | PGE04009 / *Continue / conversions.md circular ref | 4 | P3-medium | Pending |
| 147 | ForEach.Level ~~ double-prefix violates single ~ rule | 5 | P3-medium | Pending |

---
*Roadmap created: 2026-02-27*
*Last updated: 2026-04-06*
