---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: MASTER-INDEX.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Documentation - Master Index

**Last Updated:** 2025-12-14
**Documentation Version:** v0.0.4 Finalized
**Project Status:** Epic 1 Implementation (v0.0.3)

---

## Quick Navigation by Role

### 🎓 I want to learn Polyglot

**Start here:**
- [Getting Started Guide](user/getting-started.md) - First steps with Polyglot
- [Language Overview](user/README.md) - Core concepts and philosophy
- [Async-Centric Paradigm](user/async-centric-paradigm.md) - Understanding the execution model

**Reference:**
- [v0.0.4 Complete Specification](specifications/v0.0.4/COMPLETE-SPEC.md) - ✅ **Latest finalized syntax**
  - [Loop System](specifications/v0.0.4/features/loop-system.md) - Unpack/pack operators
  - [Reserved Indication](specifications/v0.0.4/features/reserved-indication-system.md) - Semicolon system
  - [Metadata System](specifications/v0.0.4/features/metadata-system.md) - Annotations and documentation
- [Common Mistakes & Anti-patterns](user/common-mistakes-antipatterns.md)

### 🔧 I'm implementing the Polyglot compiler/runtime

**Architecture:**
- [Complete System Architecture](technical/architecture.md) - Component design and data flow
- [Technology Stack](project/technology-stack.md) - Implementation choices
- [Architecture Patterns](project/architecture-patterns.md) - Design patterns

**Implementation Guides:**
- [Lexer Implementation Readiness](project/LEXER-IMPLEMENTATION-READINESS.md)
- [Parser Error Detection Design](technical/parser-error-detection-design.md)
- [String Literals Internals](technical/string-literals-internals.md)
- [DateTime Architecture](technical/datetime-architecture-decision-2025-12-02.md)
- [Variable States Specification](technical/variable-states-specification.md)

**Current Work:**
- [Sprint Status](project/stories/sprint-status.yaml) - Current sprint progress
- [Active Stories](project/stories/) - Implementation tasks

### 🎨 I'm designing language features

**Language Design:**
- [v0.0.4 Specification](specifications/v0.0.4/COMPLETE-SPEC.md) - ✅ **Current finalized design**
- [Version Roadmap](specifications/version-roadmap.md) - Multi-version planning
- [Brainstorming Sessions](specifications/brainstorming/) - Design exploration

**Design Decisions:**
- [Design Specifications Catalog](DESIGN-SPECIFICATIONS-CATALOG.md) - All 23+ specifications
- [Syntax Corrections Log](specifications/SYNTAX-CORRECTIONS-2025-12-12.md)
- [Technical Decisions](technical/decisions/)

**Future Versions:**
- [v0.0.5 Proposals](specifications/v0.0.5/) - Type system enhancements

### 🤖 I'm building AI tools for Polyglot

**Machine-Readable Specs:**
- [AI Context Packages](ai-context/) - EBNF, JSON, YAML specifications
  - [v0.0.2 Package](ai-context/v0.0.2/) - Historical (13 files)
  - 🚧 v0.0.3 - To be created
  - 💡 v0.0.4 - Planned

**Technical References:**
- [Grammar EBNF](ai-context/v0.0.2/grammar.ebnf) - Complete syntactic grammar (v0.0.2)
- [Type System JSON](ai-context/v0.0.2/type-system.json) - Type rules
- [Operators JSON](ai-context/v0.0.2/operators.json) - Operator reference

### 📋 I'm managing the project

**Planning:**
- [Product Requirements (PRD)](project/prd.md) - 120 functional requirements
- [Epic Breakdown](project/epics.md) - Major feature sets
- [Product Brief](project/product-brief-Polyglot-2025-12-14.md) - Strategic direction

**Tracking:**
- [Sprint Status](project/stories/sprint-status.yaml) - Current sprint
- [Project TODO](project/project-todo.yaml) - Active tasks
- [ITIL Tickets](project/tickets/) - Issue tracking

**Collaboration:**
- [Meetings](project/meetings/) - Meeting minutes
- [Agent Sessions](project/agent-sessions/) - Design sessions
- [Brainstorming Backlog](project/brainstorming-backlog.md)

---

## Documentation Structure

### 📖 [User Documentation](user/)

**Purpose:** Language guides, tutorials, examples, and reference

**Core Guides:**
- [README](user/README.md) - User documentation index
- [Getting Started](user/getting-started.md) - Quick start guide
- [Async-Centric Language](user/async-centric-language.md) - Core paradigm
- [Async-Centric Paradigm](user/async-centric-paradigm.md) - Execution model
- [Variable State System](user/variable-state-system.md) - State management
- [Packages](user/packages.md) - Package system
- [Polyglot Service](user/polyglot-service.md) - Service architecture
- [Common Mistakes](user/common-mistakes-antipatterns.md) - Best practices

**Subsections:** (11 subdirectories)
- [language/](user/language/) - Language syntax and semantics
- [standard-library/](user/standard-library/) - Standard library reference
- [architecture/](user/architecture/) - System architecture
- [examples/](user/examples/) - Code examples
- [cli/](user/cli/) - Command-line interface
- [package-management/](user/package-management/) - Package creation
- [audit/](user/audit/) - Design decisions and compliance
- And more...

**Current Version:** v0.0.2 (implementation target for Epic 1)

---

### 📐 [Design Specifications](specifications/)

**Purpose:** Version specifications, design decisions, feature proposals

**Organization:**
- [Complete Catalog](DESIGN-SPECIFICATIONS-CATALOG.md) - All 23+ specifications
- [Version Roadmap](specifications/version-roadmap.md) - Multi-version planning
- [README](specifications/README.md) - Specifications index

**Version Structure:**

#### ✅ [v0.0.4 - Major Syntax Refinement](specifications/v0.0.4/)
**Status:** Finalized December 2025
**Implementation:** Q2 2026

**Master Document:**
- [COMPLETE-SPEC.md](specifications/v0.0.4/COMPLETE-SPEC.md) - **Navigation hub for all v0.0.4 docs**

**Core Features:**
- [Loop System](specifications/v0.0.4/features/loop-system.md) - Unpack `[~]` / Pack `[*]` operators
- [Reserved Indication](specifications/v0.0.4/features/reserved-indication-system.md) - Semicolon `;` for reserved segments
- [Metadata System](specifications/v0.0.4/features/metadata-system.md) - Full metadata tree structure

**Additional Documentation:**
- [loop-system/](specifications/v0.0.4/loop-system/) - Loop design evolution (7 files)
- [syntax-refinement/](specifications/v0.0.4/syntax-refinement/) - Syntax improvements (8+ files)
- [core-syntax/](specifications/v0.0.4/core-syntax/) - Foundational syntax elements

**Key Changes:**
- Variable prefix: `$` (not `,`)
- Block delimiters: `{|}...{x}` (not `[|]...[X]`)
- IO markers: `[|] <param` / `[|] >param`
- Type notation: `:pg.string` (colon prefix)
- Reserved indication: `#;Boolean;True` (semicolon segments)

#### 🔄 [v0.0.5 - Type System](specifications/v0.0.5/)
**Status:** Concept phase
**Scope:** Type system enhancements, advanced typing features

#### 📝 [Brainstorming Sessions](specifications/brainstorming/)
**Purpose:** Design exploration and ideation
**Contents:** Raw brainstorming outputs, design proposals

---

### 🤖 [AI Context Packages](ai-context/)

**Purpose:** Machine-parseable specifications for AI/LLM consumption

**Available Versions:**
- [v0.0.2](ai-context/v0.0.2/) - ⚠️ Historical reference (13 files)
  - grammar.ebnf - Complete EBNF grammar
  - type-system.json - Type rules
  - operators.json - Operator reference
  - state-machine.yaml - Variable lifecycle
  - datetime-system.yaml - DateTime specification
  - And 8 more specialized files

**Pending:**
- 🚧 v0.0.3 - Current stable (to be created)
- 💡 v0.0.4 - After finalization (planned)

**Information Density:** 4.3x more compact than prose documentation

---

### 🔧 [Technical Documentation](technical/)

**Purpose:** Architecture, technical decisions, implementation design

**Core Documents:**
- [README](technical/README.md) - Technical docs index
- [Architecture](technical/architecture.md) - Complete system architecture
- [Technology Stack](project/technology-stack.md) - Implementation choices
- [Architecture Patterns](project/architecture-patterns.md) - Design patterns

**Specialized Documentation:**
- [DateTime Architecture](technical/datetime-architecture-decision-2025-12-02.md)
- [DateTime String Literal Specification](technical/datetime-string-literal-specification.md)
- [Parser Error Detection Design](technical/parser-error-detection-design.md)
- [String Literals Internals](technical/string-literals-internals.md)
- [Variable States Specification](technical/variable-states-specification.md)
- [Block Hierarchy Reference](technical/block-hierarchy-reference.md)
- [Database Architecture](technical/database-architecture-explained.md)
- [Hierarchy Tree Notation](technical/hierarchy-tree-notation.md)
- [Formatting Guidelines](technical/polyglot-formatting-guidelines-v1.0.md)
- [IR Specification](technical/ir-specification-TODO.md) - In progress

**Subdirectories:**
- [decisions/](technical/decisions/) - Technical decision records

---

### 📋 [Project Documentation](project/)

**Purpose:** Project management, planning, tracking

**Planning:**
- [README](project/README.md) - Project docs index
- [PRD - Product Requirements](project/prd.md) - 120 functional requirements
- [Product Brief](project/product-brief-Polyglot-2025-12-14.md) - Strategic direction
- [Epic Breakdown](project/epics.md) - Major feature sets
- [Stories Directory](project/stories/) - User stories with acceptance criteria

**Tracking:**
- [Sprint Status](project/stories/sprint-status.yaml) - Current sprint progress
- [Project TODO](project/project-todo.yaml) - Active tasks and assignments
- [ITIL Tickets](project/tickets/) - Structured ticket system
  - [Incidents](project/tickets/incidents/) - Unplanned disruptions
  - [Problems](project/tickets/problems/) - Root cause investigations
  - [Changes](project/tickets/changes/) - Planned modifications
  - [Service Requests](project/tickets/service-requests/) - Standard requests

**Architecture & Planning:**
- [Project Structure](project/project-structure.md)
- [Architecture Patterns](project/architecture-patterns.md)
- [Technology Stack](project/technology-stack.md)
- [Existing Documentation Inventory](project/existing-documentation-inventory.md)

**Collaboration:**
- [Meetings](project/meetings/) - Meeting minutes
- [Agent Sessions](project/agent-sessions/) - Design sessions
- [Brainstorming Backlog](project/brainstorming-backlog.md)
- [Brainstorming Results](project/brainstorming-session-results-2025-11-19.md)

**Change Management:**
- [BMAD Alignment](project/v0.0.2-bmad-alignment.md)
- [Sprint Change Proposal - v0.0.4](project/SPRINT-CHANGE-PROPOSAL-v0.0.4-syntax-updates.md)
- [Change Request 001](project/CHANGE-REQUEST-001-v0.0.2-Syntax-Compliance.md)

**Validation:**
- [Lexer Implementation Readiness](project/LEXER-IMPLEMENTATION-READINESS.md)
- [AI Codegen Validation Report](project/ai-codegen-validation-report.md)
- [User-Provided Context](project/user-provided-context.md)
- [Stdlib Implementation Notes](project/stdlib-implementation-notes.md)

---

### 🗄️ [Archive](archive/)

**Purpose:** Historical documentation, completed reports, superseded specs

**Organization:**
- [Archive README](archive/README.md) - Archive policy and organization

**Contents:**
- [old-user-docs/](archive/old-user-docs/) - v0.0.2 documentation (two iterations)
  - [user-v0.0.2-original/](archive/old-user-docs/user-v0.0.2-original/) - Earlier iteration
  - [user-v0.0.2-revised/](archive/old-user-docs/user-v0.0.2-revised/) - Later iteration
- [reports/](archive/reports/) - Completed assessment reports
- [brainstorming/](archive/brainstorming/) - Processed brainstorming sessions
- [meetings/](archive/meetings/) - Historical meeting notes
- [agent-sessions/](archive/agent-sessions/) - Historical agent work
- [audits/](archive/audits/) - Point-in-time audit snapshots
- [reviews/](archive/reviews/) - Completed structure reviews
- [syntax-updates/](archive/syntax-updates/) - Historical syntax changes
- [specifications/](archive/specifications/) - Superseded specifications

**Archive Policy:**
- Reports where findings are integrated → Archive
- Brainstorming sessions that have been processed → Archive
- Meeting notes where decisions are captured elsewhere → Archive
- Superseded specifications → Archive

---

### 🧪 [QA Documentation](qa/)

**Purpose:** Quality assurance, testing strategies, validation

**Contents:**
- Testing plans
- Validation reports
- Quality metrics

---

## 🔍 Finding Information

### By Topic

**Language Syntax:**
- **Current Stable (v0.0.3):** [user/](user/) documentation
- **Latest Finalized (v0.0.4):** [specifications/v0.0.4/COMPLETE-SPEC.md](specifications/v0.0.4/COMPLETE-SPEC.md)
- **Future (v0.0.5):** [specifications/v0.0.5/](specifications/v0.0.5/)

**Architecture:**
- **System Design:** [technical/architecture.md](technical/architecture.md)
- **Patterns:** [project/architecture-patterns.md](project/architecture-patterns.md)
- **Decisions:** [technical/decisions/](technical/decisions/)

**Implementation:**
- **Current Sprint:** [project/stories/sprint-status.yaml](project/stories/sprint-status.yaml)
- **Stories:** [project/stories/](project/stories/)
- **Epics:** [project/epics.md](project/epics.md)

**Design Evolution:**
- **Version Roadmap:** [specifications/version-roadmap.md](specifications/version-roadmap.md)
- **Brainstorming:** [specifications/brainstorming/](specifications/brainstorming/)
- **Design Catalog:** [DESIGN-SPECIFICATIONS-CATALOG.md](DESIGN-SPECIFICATIONS-CATALOG.md)

### By Version

**v0.0.2** - Original Design (Historical)
- [Archive: user-v0.0.2-original/](archive/old-user-docs/user-v0.0.2-original/)
- [Archive: user-v0.0.2-revised/](archive/old-user-docs/user-v0.0.2-revised/)
- [AI Context: v0.0.2/](ai-context/v0.0.2/)

**v0.0.3** - Current Stable (Epic 1 Implementation Target)
- [user/](user/) documentation
- 🚧 AI Context to be created

**v0.0.4** - Finalized December 2025 (Implementation Q2 2026)
- [specifications/v0.0.4/COMPLETE-SPEC.md](specifications/v0.0.4/COMPLETE-SPEC.md) ✅
- [Loop System](specifications/v0.0.4/features/loop-system.md)
- [Reserved Indication](specifications/v0.0.4/features/reserved-indication-system.md)
- [Metadata System](specifications/v0.0.4/features/metadata-system.md)

**v0.0.5** - Future (Concept Phase)
- [specifications/v0.0.5/](specifications/v0.0.5/)

### By File Type

**Markdown Documentation:** ~300 files across all folders
**YAML Configuration:** Sprint status, TODO lists, ITIL config
**JSON Data:** AI context type system, operators
**EBNF Grammar:** AI context grammar specification

---

## 📊 Current Project Status

**Phase:** 4 - Implementation
**Epic:** Epic 1 - Lexer & Parser Foundation
**Target Version:** v0.0.3
**Sprint:** See [sprint-status.yaml](project/stories/sprint-status.yaml)

**Recent Major Updates:**
- ✅ v0.0.4 specification finalized (December 2025)
- ✅ Documentation reorganization (December 2025)
- ✅ Modular specification structure created
- ✅ Archive cleanup and organization
- 🔄 Epic 1 implementation in progress

**Documentation Health:**
- Total directories: 50+
- Total markdown files: ~300
- Missing READMEs: Identified and tracked
- Archive size: 2.7MB (organized)

---

## 🚀 Quick Start Paths

### Path 1: Learn to Write Polyglot Code
1. [Getting Started](user/getting-started.md)
2. [Async-Centric Paradigm](user/async-centric-paradigm.md)
3. [v0.0.4 Complete Spec](specifications/v0.0.4/COMPLETE-SPEC.md)
4. [Examples](user/examples/)

### Path 2: Implement the Compiler
1. [Architecture](technical/architecture.md)
2. [Lexer Readiness](project/LEXER-IMPLEMENTATION-READINESS.md)
3. [Sprint Status](project/stories/sprint-status.yaml)
4. [Active Stories](project/stories/)

### Path 3: Design Language Features
1. [Version Roadmap](specifications/version-roadmap.md)
2. [v0.0.4 Specification](specifications/v0.0.4/COMPLETE-SPEC.md)
3. [Brainstorming Sessions](specifications/brainstorming/)
4. [Design Catalog](DESIGN-SPECIFICATIONS-CATALOG.md)

### Path 4: Build AI Tools
1. [AI Context Packages](ai-context/)
2. [v0.0.2 Grammar EBNF](ai-context/v0.0.2/grammar.ebnf)
3. [Type System JSON](ai-context/v0.0.2/type-system.json)
4. [v0.0.4 Specification](specifications/v0.0.4/COMPLETE-SPEC.md)

---

## 🔗 Related Resources

**External Links:**
- GitHub Repository: (to be added)
- Issue Tracker: (to be added)
- Community Forum: (to be added)

**Internal Cross-References:**
- [Audit Report](AUDIT-REPORT.md) - Recent documentation audit findings
- [Cleanup Plan](CLEANUP-PLAN.md) - Documentation improvement roadmap
- [Design Catalog](DESIGN-SPECIFICATIONS-CATALOG.md) - All design specifications

---

## 📝 Documentation Standards

**Format:** GitHub-flavored Markdown
**Methodology:** BMAD (Business Management Agile Development)
**Cross-References:** Relative paths from docs/ root
**Navigation:** Each section has dedicated README

**When to Update:**
- Language syntax changes → [user/](user/)
- Architectural decisions → [technical/](technical/)
- Implementation planning → [project/](project/)
- Design proposals → [specifications/](specifications/)

---

**Master Index Created:** 2025-12-14
**Documentation Team:** Polyglot Project
**Maintained By:** ReDoc Workflow
**Next Review:** After Epic 1 completion

For questions or documentation issues, see [AUDIT-REPORT.md](AUDIT-REPORT.md) or create an issue.
