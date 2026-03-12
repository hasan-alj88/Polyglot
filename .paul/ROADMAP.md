# Roadmap: Polyglot

## Overview
Building an async programming language that can asynchronously compile other programming languages. Includes language specification, compiler infrastructure, and multi-language service layer (listener, queue manager, executioner).

## Current Milestone
**v0.1 Language Specification & Research** (v0.1.0)
Status: Not started
Phases: 0 of 4 complete

## Phases

| Phase | Name | Plans | Status | Completed |
|-------|------|-------|--------|-----------|
| 1 | Documentation Audit & Consolidation | TBD | Not started | - |
| 2 | Complete Language Specification | TBD | Not started | - |
| 3 | Compiler Architecture Design | TBD | Not started | - |
| 4 | Prior Art Research & Analysis | TBD | Not started | - |

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

---
*Roadmap created: 2026-02-27*
*Last updated: 2026-03-12*
