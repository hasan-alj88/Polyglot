---
last-redoc-date: 2025-12-12
last-reorganization: 2025-12-12
---

# Design Specifications Catalog

This catalog organizes all design documents, specifications, and brainstorming sessions for the Polyglot language.

**New Organization:** As of 2025-12-12, specifications have been reorganized into version-based folders for better structure and navigation.

---

## 📁 Folder Structure

```
specifications/
├── version-roadmap.md           # Version planning and timeline
├── v0.0.4/                      # v0.0.4 Major Syntax Refinement
│   ├── loop-system/             # Loop unpack/pack system (absorbed v0.0.3.1)
│   └── syntax-refinement/       # 30 syntax features and improvements
├── v0.0.5/                      # v0.0.5 Type System (concept phase)
└── brainstorming/               # Design exploration sessions
```

**See:** [specifications/README.md](specifications/README.md) for complete navigation guide

---

## 📋 v0.0.4 - Major Syntax Refinement + Loop System

**Status:** 🔧 Design Phase - Breaking Changes + Major Features
**Target:** Q2 2026
**Location:** [specifications/v0.0.4/](specifications/v0.0.4/)

### Overview

Version 0.0.4 absorbs the loop system (originally v0.0.3.1) and introduces 30 syntax refinement features. The loop system was absorbed because it already uses v0.0.4 syntax features (`$` prefix, 3-space indentation, lowercase `[v]`).

**See:** [v0.0.4 Overview](specifications/v0.0.4/README.md) for complete details

---

### Part 1: Loop System (Absorbed from v0.0.3.1)

**Status:** ✅ Specification Complete - Ready for Implementation
**Location:** [specifications/v0.0.4/loop-system/](specifications/v0.0.4/loop-system/)

#### Core Specifications

- **[v0.0.3.1 Loop System Specification](specifications/v0.0.4/loop-system/v0.0.3.1-loop-system-specification.md)** ⭐
  - Complete specification for the loop unpack/pack system with mini-pipeline iterations
  - Three operators: `[~]` unpack, `[*]` pack, `[v]` join/sync
  - Execution modes: `[r]` sequential, `[p]` parallel, `[b]` fire-and-forget

- **[v0.0.3.1 Blind Spots Analysis](specifications/v0.0.4/loop-system/v0.0.3.1-blind-spots-analysis.md)**
  - Comprehensive review identifying and resolving 20 potential issues
  - All blockers cleared, specification validated

#### Design Evolution

- **[Loop Unpack/Pack Final Design](specifications/v0.0.4/loop-system/loop-unpack-pack-final-design.md)**
  - Final design with complete semantics for the three-operator system

- **[Loop Pack/Unpack Improvements](specifications/v0.0.4/loop-system/loop-pack-unpack-improvements.md)**
  - Design improvements, patterns, and refinements

- **[Loop I/O Mini-Pipelines](specifications/v0.0.4/loop-system/loop-io-mini-pipelines.md)**
  - Initial exploration of loop I/O concepts

#### Key Decisions

- **[Variable Reassignment & Pack/Unpack](specifications/v0.0.4/loop-system/variable-reassignment-pack-unpack.md)**
  - Exploration of mutability (rejected) and functional solutions
  - Decision: Functional pack/unpack patterns instead of mutation

- **[Pipelines as Variables](specifications/v0.0.4/loop-system/pipelines-as-variables.md)**
  - `:pg.pipeline` type proposal (⏸️ on hold for post-MVP)

**Key Innovation:** Each loop iteration is a mini-pipeline with explicit I/O using three operators

**See:** [Loop System README](specifications/v0.0.4/loop-system/README.md) for complete navigation

---

### Part 2: Syntax Refinement (30 Features)

**Status:** 🔧 Design Phase - 30 Features
**Location:** [specifications/v0.0.4/syntax-refinement/](specifications/v0.0.4/syntax-refinement/)

#### Core Specifications

- **[v0.0.4 Final Decisions](specifications/v0.0.4/syntax-refinement/v0.0.4-final-decisions.md)** ⭐
  - Comprehensive decisions document - primary reference for all 33 features

- **[v0.0.4 Complete Syntax](specifications/v0.0.4/syntax-refinement/v0.0.4-complete-syntax.md)**
  - Full syntax specification with all features

- **[v0.0.4 Final Syntax Decisions](specifications/v0.0.4/syntax-refinement/v0.0.4-final-syntax-decisions.md)**
  - Finalized syntax choices and rationale

- **[v0.0.4 Design Decisions Final](specifications/v0.0.4/syntax-refinement/v0.0.4-design-decisions-final.md)**
  - Design philosophy and principles

#### Feature-Specific Documents

**Reserved Enumeration System:**
- **[Reserved Enumeration Indication](specifications/v0.0.4/syntax-refinement/reserved-enum-indication.md)**
  - All 8 proposals evaluated for reserved enum marking

- **[Reserved Enum Semicolon Prefix](specifications/v0.0.4/syntax-refinement/reserved-enum-semicolon-prefix.md)** ⭐
  - Finalized `;` prefix approach for reserved segments (approved)

**Patterns and References:**
- **[v0.0.4 Hierarchy Pattern Reference](specifications/v0.0.4/syntax-refinement/v0.0.4-hierarchy-pattern-reference.md)**
  - Hierarchy notation patterns across all constructs

- **[v0.0.4 Syntax Comparison](specifications/v0.0.4/syntax-refinement/v0.0.4-syntax-comparison.md)**
  - Before/after syntax comparisons (v0.0.3 vs v0.0.4)

**Type System:**
- **[v0.0.4 Alias System](specifications/v0.0.4/syntax-refinement/v0.0.4-alias-system.md)**
  - Type and path aliasing system

#### Enhancement Proposals

- **[Additional Syntax Improvements](specifications/v0.0.4/syntax-refinement/additional-syntax-improvements.md)**
  - Proposed syntax enhancements beyond core 33 features

- **[Metadata-Driven Improvements](specifications/v0.0.4/syntax-refinement/metadata-driven-improvements.md)**
  - Metadata system proposals and patterns

- **[Pipeline Composition Examples](specifications/v0.0.4/syntax-refinement/pipeline-composition-examples.md)**
  - Pipeline chaining patterns with `|>` operator

**Major Changes:**
- Variable prefix: `,` → `$`
- Indentation-based nesting (3 spaces)
- Reserved indication: `;` prefix for enums/errors
- 30 total new features

**See:** [Syntax Refinement README](specifications/v0.0.4/syntax-refinement/README.md) for complete navigation

---

## 📋 v0.0.5 - Type System (Future)

**Status:** 💡 Concept Phase (20% complete)
**Target:** Q4 2026
**Location:** [specifications/v0.0.5/](specifications/v0.0.5/)

### Specifications

- **[v0.0.5 Improvement Proposals](specifications/v0.0.5/v0.0.5-improvement-proposals.md)**
  - Future improvements and type system enhancements overview

- **[Type Definitions System](specifications/v0.0.5/type-definitions-system.md)**
  - Type definition blocks with `{:}` syntax
  - Constrained types with validation
  - Violation handlers (clip, raise, transform, default)

**Planned Features:**
- Type definition blocks: `{:}`
- Constrained types with min/max, patterns, ranges
- Cross-language type mappings
- Type composition and conversions
- Metadata-driven type features

**See:** [v0.0.5 README](specifications/v0.0.5/README.md) for complete details

---

## 🧠 Brainstorming Sessions

**Status:** 1 Active Session
**Location:** [specifications/brainstorming/](specifications/brainstorming/)

### December 2025 Sessions

- **[Brainstorming Session - December 11, 2025](specifications/brainstorming/brainstorming-session-2025-12-11.md)** ⭐
  - **Duration:** Extended session
  - **Status:** ✅ COMPLETED - Major Decisions Finalized
  - **Topics:** Variable reassignment, reserved enums, loop system design, syntax refinement
  - **Outcome:** Finalized v0.0.4 specifications (loop system + syntax refinement)
  - **Files Created:** 11 comprehensive design documents
  - **Key Innovation:** Mini-pipeline iterations with unpack/pack operators
  - **Supersedes:** December 8 and 10 exploratory sessions (now archived)

### Archived Exploratory Sessions

**Note:** Earlier December sessions have been archived as their explorations were incorporated into the comprehensive December 11 session.

- **[December 10, 2025](archive/brainstorming/brainstorming-session-results-2025-12-10.md)**
  - Indentation-based nesting exploration

- **[December 8, 2025](archive/brainstorming/brainstorming-session-results-2025-12-08.md)**
  - Marker system design review

See [Archive Brainstorming](archive/brainstorming/) for complete session history from November-December 2025.

**See:** [Brainstorming README](specifications/brainstorming/README.md) for complete navigation

---

## 📋 Version Planning & Reference

### [Version Roadmap](specifications/version-roadmap.md) ⭐

Complete version planning from v0.0.3 (current stable) through v0.1.0 (first stable release).

**Contents:**
- Version progression: v0.0.3 → v0.0.4 → v0.0.5 → v0.1.0
- Feature matrices showing version differences
- Migration guides (v0.0.3 → v0.0.4)
- Implementation priorities and phases
- Timeline and targets

**Key Change:** v0.0.3.1 absorbed into v0.0.4 for cleaner version progression

### [Syntax Corrections 2025-12-12](specifications/SYNTAX-CORRECTIONS-2025-12-12.md)

Comprehensive syntax corrections and clarifications for v0.0.4 documentation.

**Contents:**
- 12 syntax patterns clarified with examples
- Boolean markers (`[&]` AND, `[|]` OR, `[^]` XOR)
- Wildcard condition syntax (`[y] *`)
- Operator negation equivalence
- Context-specific marker usage
- Package declaration rules
- Multi-line string concatenation

**Key Discoveries:**
- Boolean logic markers for complex conditions
- Trigger OR syntax for multiple trigger types
- Exhaustive matching requirement with wildcard

---

## 📊 Document Organization

### By Status

- **✅ Complete & Approved:** v0.0.4 loop system specifications, blind spots analysis
- **🔧 Design Phase:** v0.0.4 syntax refinement specifications (95% complete)
- **💡 Concept Phase:** v0.0.5 type system proposals (20% complete)
- **⏸️ On Hold:** Pipelines as variables (`:pg.pipeline` type)
- **❌ Rejected:** Mutable variables, v0.0.3.1 as standalone version

### By Category

- **Core Specifications:** Complete language version specs (v0.0.4, v0.0.5)
- **Design Evolution:** Iterative design documents showing progression
- **Feature Proposals:** Individual feature explorations and enhancements
- **Brainstorming:** Session records with comprehensive discussion and decisions
- **Analysis:** Blind spots analysis, syntax comparisons, evaluations

### By Location

- **specifications/v0.0.4/loop-system/:** 7 documents (loop system)
- **specifications/v0.0.4/syntax-refinement/:** 12 documents (syntax features)
- **specifications/v0.0.5/:** 2 documents (type system concepts)
- **specifications/brainstorming/:** 1 active session document
- **specifications/** (root): 1 version roadmap, 1 syntax corrections

**Total Documents:** 24 design specifications across all versions

---

## 🔗 Related Documentation

- **User Documentation:** [/docs/user/](user/) - Complete language guides and reference (v0.0.3 current)
- **AI Context Packages:** [/docs/ai-context/](ai-context/) - Machine-readable specifications by version
- **Technical Documentation:** [/docs/technical/](technical/) - Architecture and implementation details
- **Project Documentation:** [/docs/project/](project/) - PRD, epics, stories, sprint status

---

## 📝 Navigation Guide

### Quick Access

**Looking for current syntax?**
→ See [User Documentation](user/) for v0.0.3 reference

**Understanding v0.0.4 changes?**
→ Start with [v0.0.4 Overview](specifications/v0.0.4/README.md)

**Understanding loop system?**
→ See [Loop System Specification](specifications/v0.0.4/loop-system/v0.0.3.1-loop-system-specification.md)

**Checking syntax refinement features?**
→ See [v0.0.4 Final Decisions](specifications/v0.0.4/syntax-refinement/v0.0.4-final-decisions.md)

**Exploring type system concepts?**
→ See [v0.0.5 Overview](specifications/v0.0.5/README.md)

**Following design evolution?**
→ See [Brainstorming Session Dec 11](specifications/brainstorming/brainstorming-session-2025-12-11.md)

**Understanding version timeline?**
→ See [Version Roadmap](specifications/version-roadmap.md)

### Folder Navigation

Each specification folder includes a comprehensive README.md:

- [specifications/README.md](specifications/README.md) - Main specifications index
- [specifications/v0.0.4/README.md](specifications/v0.0.4/README.md) - v0.0.4 overview
- [specifications/v0.0.4/loop-system/README.md](specifications/v0.0.4/loop-system/README.md) - Loop system guide
- [specifications/v0.0.4/syntax-refinement/README.md](specifications/v0.0.4/syntax-refinement/README.md) - Syntax features guide
- [specifications/v0.0.5/README.md](specifications/v0.0.5/README.md) - Type system concepts
- [specifications/brainstorming/README.md](specifications/brainstorming/README.md) - Sessions guide

---

## ⚙️ Implementation Status

| Version | Status | Spec Complete | Parser Ready | Target | Notes |
|---------|--------|---------------|--------------|--------|-------|
| v0.0.3 | ✅ Current | 100% | Yes | Stable | Production version |
| v0.0.4 | 🔧 Design | 95% | Not Started | Q2 2026 | Loop + syntax refinement |
| v0.0.5 | 💡 Concept | 20% | Not Started | Q4 2026 | Type system |
| v0.1.0 | 🎯 Target | - | Not Started | Q1 2027 | First stable release |

**Key Milestone:** v0.0.4 design near completion, implementation starting Q1 2026

---

## 🔧 Catalog Maintenance

### Adding New Specifications

When creating new design documents in `/docs/specifications/`, update this catalog:

1. **Determine version and category:**
   - v0.0.4 loop system → `specifications/v0.0.4/loop-system/`
   - v0.0.4 syntax → `specifications/v0.0.4/syntax-refinement/`
   - v0.0.5 features → `specifications/v0.0.5/`
   - Brainstorming → `specifications/brainstorming/`

2. **Add entry to appropriate section:**
   - Clear document title with link to new location
   - Brief 1-sentence description
   - Status marker (✅ Complete, 🔧 Design, 💡 Concept, ⏸️ On Hold, ❌ Rejected)

3. **Update folder README:**
   - Add document to folder's README.md
   - Update reading order if applicable
   - Cross-reference related documents

4. **Update metadata:**
   - Increment document count
   - Update last-updated date in frontmatter
   - Update implementation status matrix if needed

5. **Archive superseded documents:**
   - Move to `/archive/specifications/` or `/archive/brainstorming/`
   - Update catalog to reference archived location
   - Add note about supersession

### Current Status

**Catalog Coverage:** 100% (24/24 specifications cataloged and organized)

**File Distribution:**
- v0.0.4 loop system: 7 documents
- v0.0.4 syntax refinement: 12 documents
- v0.0.5 type system: 2 documents
- Brainstorming: 1 active session
- Version planning & reference: 1 roadmap, 1 syntax corrections

**Organization:** ✅ Reorganized into version-based folders (2025-12-12)

**Last Verification:** 2025-12-12

---

## 📅 Recent Changes

### 2025-12-12 - Major Reorganization

**Changes:**
1. ✅ Created version-based folder structure (`specifications/vX.X.X/`)
2. ✅ Moved v0.0.3.1 specs to `v0.0.4/loop-system/` (absorbed into v0.0.4)
3. ✅ Moved v0.0.4 syntax specs to `v0.0.4/syntax-refinement/`
4. ✅ Moved v0.0.5 specs to `v0.0.5/`
5. ✅ Moved brainstorming session to `brainstorming/`
6. ✅ Created comprehensive README.md files for all folders
7. ✅ Updated version roadmap to reflect v0.0.3.1 absorption
8. ✅ Reorganized ai-context into versioned structure

**Rationale:**
- Better organization and navigation
- Clear version boundaries
- Easier to find related documents
- Consistent with project structure

---

**Last Updated:** 2025-12-12
**Last Reorganization:** 2025-12-12
**Maintained By:** Polyglot Documentation Team
**Document Count:** 24 design specifications (100% cataloged and organized)
**Folder Structure:** Version-based with categorical subfolders
**Recent Addition:** Syntax Corrections 2025-12-12 (boolean markers, wildcard conditions)
