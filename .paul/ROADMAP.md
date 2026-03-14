# Roadmap: Polyglot

## Overview
Building an async programming language that can asynchronously compile other programming languages. Includes language specification, compiler infrastructure, and multi-language service layer (listener, queue manager, executioner).

## Current Milestone
**v0.1 Language Specification & Research** (v0.1.0)
Status: In progress
Phases: 2 of 5 complete

## Phases

| Phase | Name | Plans | Status | Completed |
|-------|------|-------|--------|-----------|
| 1 | Documentation Audit & Consolidation | 1 | Complete | 2026-03-12 |
| 2 | Complete Language Specification | 2+ | Planning | - |
| 3 | Compiler Architecture Design | TBD | Not started | - |
| 4 | Prior Art Research & Analysis | TBD | Not started | - |
| 5 | Clean Slate Reset | 1 | Complete | 2026-03-12 |

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

### Phase 2: Complete Language Specification

**Goal:** Define the complete Polyglot v0.0.4+ language specification with canonical examples

**Depends on:** Phase 1 (clean documentation base)

**Research:** Unlikely (documenting existing design decisions)

**Scope:**
- Variables & lifecycle states
- Operators (<<, <~, >>, ~>, etc.)
- Pipeline structures
- Fork patterns & conditionals
- Error handling blocks
- Loop system (unpack/pack)
- Enum definitions & serial load
- Type system (pg.string, pg.int, etc.)
- Package spec format (@Registry::Path:Version.build)
- Block markers ([s], [#], [X], [@], etc.)

**Plans:** TBD (during phase planning)

### Phase 3: Compiler Architecture Design

**Goal:** Design the async compilation service infrastructure on paper

**Depends on:** Phase 2 (language features specified)

**Research:** Likely (multi-language async compilation architecture)

**Research topics:**
- Async job queue patterns for compilation
- Multi-language compiler integration strategies
- Service orchestration for polyglot compilation

**Scope:**
- Listener: Receive compilation requests
- Queue Manager: Prioritize and distribute compilation jobs
- Executioner: Coordinate language-specific compilers
- Service communication and state management
- AST design decisions

**Plans:** TBD (during phase planning)

### Phase 4: Prior Art Research & Analysis

**Goal:** Research existing tools, languages, and frameworks that inform Polyglot's design

**Depends on:** Phase 2 (spec defined to know what to compare against)

**Research:** Required

**Research topics:**
- Existing polyglot/multi-language compilation systems
- Async compilation approaches in modern compilers
- Language interop strategies (FFI, transpilation, shared runtimes)
- Pipeline-based programming languages and DSLs

**Scope:**
- Survey of existing multi-language compilation tools
- Comparative analysis of async compilation approaches
- Lessons learned from similar projects
- Technology choices and trade-offs documentation

**Plans:** TBD (during phase planning)

### Phase 5: Clean Slate Reset

**Goal:** Commit all pending changes from Phase 1, remove archive tarballs, and reach a clean git state — true clean slate before Phase 2 work begins

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
*Roadmap created: 2026-02-27*
*Last updated: 2026-03-14*
