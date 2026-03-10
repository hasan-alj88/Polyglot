# Polyglot v0.0.4 Documentation Index

**Version:** v0.0.4
**Status:** ✅ Production Ready
**Last Updated:** 2025-12-30
**Total Documents:** 1,877 markdown files
**Schema:** BMAD-optimized for agent consumption

---

## 📁 Documentation Sections

This index covers all documentation across the Polyglot project:

1. **[User Documentation](#user-documentation)** - Language reference, guides, and tutorials
2. **[Agile Documentation](#agile-documentation)** - Project management, planning, and tracking
3. **[Technical Documentation](#technical-documentation)** - Implementation details and architecture
4. **[Audit Documentation](#audit-documentation)** - Quality checks and change history
5. **[Archive](#archive)** - Historical documentation and deprecated content

---

# User Documentation

> **Audience:** Developers, language users
> **Location:** `docs/User/`
> **Files:** 137 documents

## 📂 User Documentation Structure

### 🚀 Getting Started
> **Phase:** any | **Complexity:** low | **Agents:** developer, tech-writer

Start here if you're new to Polyglot.

- [Core Principles](./User/getting-started/core-principles.md) - Language philosophy and design
- **[Hello World Tutorial](./User/getting-started/hello-world.md)** - 🎯 **START HERE** - Multi-language pipeline orchestration
- **[Inline Pipelines](./User/language/advanced/inline-pipelines.md)** - 🔥 **Most common feature** - formatted string templates
- Quick Reference (TBD) - One-page cheatsheet
- Installation (TBD) - Setup guide

**Quick Example - Inline Pipeline Calls:**
```polyglot
[r] $sum :pg.int << |U.Math.Add"{$x}, {$y}"          // Math utility
[r] $upper :pg.string << |U.String.Upper"{$text}"    // String utility
[r] $now :pg.string << |DT.Now""                     // DateTime utility
```

---

### 📖 Language Reference
> **Phase:** planning, solutioning | **Complexity:** low-high | **Agents:** architect, developer

#### Syntax (Complexity: medium)
- **[Operators Reference](./User/language/syntax/operators.md)** - 📕 Complete guide: `<<`, `<~`, `>>`, `~>`, variable lifecycle
- [Markers](./User/language/syntax/markers.md) - `[r]`, `[|]`, `{|}`, etc.
- [Prefix System](./User/language/syntax/prefix-system.md) - `$`, `:`, `#`, `|`, `!`, `@`, `%`
- [I/O Operators](./User/language/syntax/io-operators.md) - Input/output wiring

#### Types (Complexity: medium)
- [Type System](./User/language/types/type-system.md) - Complete type reference
- **[Enum Syntax Guide](./User/language/types/enums.md)** - 📗 v0.0.4 syntax, aliases, custom extensions, migration
- **[Enum Definitions](./User/language/types/enum-definitions.md)** - 📗 `{#}` blocks, serial load, field accessors, configuration-driven enums (NEW!)
- [Enums & Serial](./User/language/types/enums-serial.md) - Enumerations and serial data
- [Variables Lifecycle](./User/language/types/variables-lifecycle.md) - 5-state variable system

#### Control Flow (Complexity: medium)
- [Pipeline Structure](./User/language/control-flow/pipeline-structure.md) - Execution model
- **[Fork Patterns](./User/language/control-flow/fork-patterns.md)** - 📙 Conditional execution, exhaustiveness, wildcard `[f] *?`
- **[Parallel Execution](./User/language/control-flow/parallel-execution.md)** - 📙 `[p]` marker, race conditions, performance
- **[Loops](./User/language/control-flow/loops.md)** - 📘 Unpack/pack operators, iteration, collection (NEW!)

#### Error Handling (Complexity: medium)
- **[Error Handling Basics](./User/language/error-handling/basics.md)** - 📕 Error types, faulted states, fork-based handling
- **[Error Blocks](./User/language/error-handling/error-blocks.md)** - 📕 Pattern matching, inline conversion (NEW!)

#### Triggers (Complexity: medium)
- **[Trigger I/O Wiring](./User/language/triggers/io-wiring.md)** - 📘 Trigger outputs, pipeline input wiring, reactive patterns (NEW!)

#### Advanced Features
- **[Pipeline Composition](./User/language/advanced/pipeline-composition.md)** - 📗 Chain pipelines with `|>` operator (NEW!)
- **[Inline Pipelines](./User/language/advanced/inline-pipelines.md)** - 🔥 **Most common** - Formatted string templates (Complexity: medium)
- [Loop System](./User/language/advanced/loop-system.md) - Unpack `[~]` / Pack `[*]` (Complexity: high)
- [Metadata System](./User/language/advanced/metadata-system.md) - `%` annotations (Complexity: medium)
- [Reserved Indication](./User/language/advanced/reserved-indication.md) - Semicolon `;` system (Complexity: medium)
- [Serial Load Block](./User/language/advanced/serial-load-block.md) - Parallel file loading (Complexity: high)

---

### 🔧 Standard Library
> **Phase:** implementation | **Complexity:** low-medium | **Agents:** developer

- [Overview](./User/stdlib/index.md) - Complete stdlib reference

#### Loops
- [Unpack Operators](./User/stdlib/loops/unpack/) - ForEach, Iter, Zip
- [Pack Operators](./User/stdlib/loops/pack/) - Collection, Math aggregation

#### Utilities
- [Data](./User/stdlib/utilities/data/) - JSON, YAML, TOML, XML
- [DateTime](./User/stdlib/utilities/datetime/) - Date/time manipulation
- [Math](./User/stdlib/utilities/math/) - Mathematical operations
- [String](./User/stdlib/utilities/string/) - String utilities

#### Wrappers
- [Runtime Wrappers](./User/stdlib/wrappers/) - Execution context control

---

### 📚 Guides
> **Phase:** any | **Complexity:** low-medium | **Agents:** all

Coming soon:
- Best Practices
- Common Patterns
- Migration Guide (v0.0.3 → v0.0.4)
- Troubleshooting

---

### 📑 Reference
> **Phase:** any | **Complexity:** medium | **Agents:** architect, developer

- [Grammar (EBNF)](./User/reference/grammar.md) - Complete language grammar
- [AI Context](./User/reference/ai-context.md) - AI-specific guidance
- Complete Syntax Reference (TBD)
- Changelog (TBD)

---

# Agile Documentation

> **Audience:** Product managers, scrum masters, development team
> **Location:** `docs/Agile/`
> **Files:** 65 documents

## 📋 Project Management

### Planning & Requirements
- **[Product Requirements Document (PRD)](./Agile/prd.md)** - Complete product vision and requirements for Polyglot v0.0.4
- [Product Brief](./Agile/product-brief-Polyglot-2025-11-15.md) - High-level product vision and strategy
- [Epics](./Agile/epics.md) - Major feature sets and epic breakdown
- [Sprint Plan](./Agile/sprint-plan-v0.0.4-lexer-parser.md) - Current sprint: v0.0.4 Lexer & Parser

### Stories & Tracking
- [Stories Directory](./Agile/stories/) - User stories with acceptance criteria
  - Story 13.1: Complete v0.0.4 Lexer Implementation ✅
  - Story 13.2: Parser v0.0.4 Implementation (In Progress)
  - More stories in directory
- [Sprint Status](./Agile/stories/sprint-status.yaml) - Current sprint tracking

### Architecture & Design
- [Architecture Decisions](./Agile/architecture/) - Technical design choices
- [Technology Stack](./Agile/tech-stack/) - Platform and tool decisions
- [Change Requests](./Agile/) - Syntax and feature changes
  - [CHANGE-REQUEST-001](./Agile/CHANGE-REQUEST-001.md) - v0.0.2 Syntax Compliance
  - [SPRINT-CHANGE-PROPOSAL-v0.0.4](./Agile/SPRINT-CHANGE-PROPOSAL-v0.0.4.md) - v0.0.4 syntax updates

### Project Context
- [Project Structure](./Agile/project-structure.md) - Codebase organization
- [Brainstorming Backlog](./Agile/brainstorming-backlog.md) - Future ideas and features
- [Reorganization History](./Agile/reorganization-history.md) - Documentation structure changes

---

# Technical Documentation

> **Audience:** Core developers, contributors
> **Location:** `docs/Tech/`
> **Files:** 36 documents

## 🔧 Implementation Details

### AI Context & Validation
- [AI Context Index](./Tech/ai-context/index.md) - AI-specific guidance and corrections
- [v0.0.2 AI Context](./Tech/ai-context/v0.0.2/) - Version-specific AI guidance
  - [AI Context Corrections](./Tech/ai-context/v0.0.2/AI-CONTEXT-CORRECTIONS.md)
  - [AI Context Package Fixes](./Tech/ai-context/v0.0.2/AI-CONTEXT-PACKAGE-FIXES.md)

### Architecture
- [Technical Architecture](./Tech/implementation/technical/architecture/) - System design documents
  - [Executive Summary](./Tech/implementation/technical/architecture/01-executive-summary.md)
  - [Philosophy & Concepts](./Tech/implementation/technical/architecture/02-philosophy-and-concepts.md)
  - [Technology Stack](./Tech/implementation/technical/architecture/05-technology-stack.md)
  - [Patterns](./Tech/implementation/technical/architecture/06-patterns.md)
  - [Data Architecture](./Tech/implementation/technical/architecture/07-data-architecture.md)
  - [Security](./Tech/implementation/technical/architecture/08-security.md)
  - [Performance](./Tech/implementation/technical/architecture/09-performance.md)
  - [Deployment](./Tech/implementation/technical/architecture/10-deployment.md)
  - [Development Environment](./Tech/implementation/technical/architecture/11-development-environment.md)
  - [String Literals Internals](./Tech/implementation/technical/architecture/string-literals-internals.md)

### Specifications
- [Serialization Specification](./Tech/implementation/technical/serialization-specification.md)
- [Version Roadmap](./Tech/User/specifications/version-roadmap.md)

### Automation
- [Link Validation Automation](./Tech/automation/link-validation-automation.md)

### Development
- [Development Index](./Tech/development/index.md)

---

# Audit Documentation

> **Audience:** Documentation team, quality assurance
> **Location:** `docs/Audit/`
> **Files:** 20+ documents

## 📊 Quality & History

### Documentation Management
- [Audit Index](./Audit/INDEX.md) - Audit documentation overview
- [Documentation Hierarchy](./Audit/DOCUMENTATION-HIERARCHY.md) - Structure and organization

### Change History
- [Changelog](./Audit/history/changelog.md) - Complete documentation change log
- [Reorganization Summary](./Audit/history/reorganization-summary-2025-12-23.md)
- [Reorganization Complete](./Audit/history/reorganization-complete-2025-12-24.md)
- [Migration Log](./Audit/history/migration-log-2025-12-24.md)
- [Link Fix Report](./Audit/history/link-fix-report-2025-12-24.md)
- [Automation Setup Complete](./Audit/history/automation-setup-complete-2025-12-24.md)
- [Enum & Trigger Docs Integration](./Audit/history/2025-12-29-enum-trigger-docs-integration.md)
- [Polly Session Update](./Audit/history/2025-12-28-polly-session-update.md)
- Phase 2 Link Cleanup:
  - [Phase 2A Complete](./Audit/history/phase2a-link-cleanup-complete-2025-12-24.md)
  - [Phase 2B Complete](./Audit/history/phase2b-link-cleanup-complete-2025-12-24.md)
  - [Phase 2C Complete](./Audit/history/phase2c-link-cleanup-complete-2025-12-24.md)

### Quality Checks
- [Validation Reports](./Audit/checks/) - Documentation quality audits
  - [Reorganization Plan](./Audit/checks/reorganization-plan-2025-12-24.md)
  - [V0.0.3 to V0.0.4 Syntax Correction](./Audit/checks/v0.0.3-to-v0.0.4-syntax-correction-2025-12-23.md)
  - [Missing Content Analysis](./Audit/checks/missing-content-analysis-2025-12-25.md)
  - [Phase 2 Link Cleanup Plan](./Audit/checks/phase2-link-cleanup-plan-2025-12-24.md)
  - [Audit Backlog Execution](./Audit/checks/audit-backlog-execution-2025-12-23.md)
  - [Audit Backlog Processed](./Audit/checks/audit-backlog-processed-2025-12-23.md)

---

# Archive

> **Audience:** Historical reference
> **Location:** `docs/archive/`
> **Files:** 1,500+ historical documents

## 📦 Historical Documentation

Historical design decisions and superseded documentation preserved for reference.

### Main Sections
- [Archive Index](./archive/) - Complete archive catalog
- [Agent Sessions](./archive/agent-sessions/) - Historical agent interaction logs
- [Audits](./archive/audits/) - Previous audit reports
- [Brainstorming](./archive/brainstorming/) - Design exploration sessions
- [Meetings](./archive/meetings/) - Meeting notes and decisions
- [Old Code Examples](./archive/old-code-examples/) - Superseded code samples
- [Old User Docs](./archive/old-user-docs/) - Previous documentation versions
- [Pre-2025-12-24 Reorganization](./archive/pre-2025-12-24-reorganization/) - Documentation before major restructure
- [Pre-v0.0.4 Sync](./archive/pre-v0.0.4-sync/) - Documentation before v0.0.4 updates
- [Reports](./archive/reports/) - Historical analysis reports
- [Reviews](./archive/reviews/) - Code and documentation reviews
- [Specifications](./archive/specifications/) - Superseded specifications
- [Syntax Updates](./archive/syntax-updates/) - Historical syntax evolution

---

## 🤖 BMAD Agent Quick Links

### For Developers
- **Getting Started:** [Core Principles](./User/getting-started/core-principles.md)
- **Quick Reference:** [Syntax](./User/language/syntax/), [Stdlib](./User/stdlib/index.md)
- **Implementation:** [Standard Library](./User/stdlib/index.md), [Types](./User/language/types/)
- **Current Sprint:** [Sprint Plan](./Agile/sprint-plan-v0.0.4-lexer-parser.md)

### For Architects
- **Planning:** [Language Reference](./User/language/), [Advanced Features](./User/language/advanced/)
- **Design:** [Type System](./User/language/types/type-system.md), [Pipeline Structure](./User/language/control-flow/pipeline-structure.md)
- **Reference:** [Grammar](./User/reference/grammar.md)
- **Architecture:** [Technical Architecture](./Tech/implementation/technical/architecture/)

### For Product Managers
- **Overview:** [Core Principles](./User/getting-started/core-principles.md)
- **Planning:** [PRD](./Agile/prd.md), [Epics](./Agile/epics.md)
- **Features:** [Language Overview](./User/language/), [Product Brief](./Agile/product-brief-Polyglot-2025-11-15.md)

### For Scrum Masters
- **Sprint:** [Sprint Plan](./Agile/sprint-plan-v0.0.4-lexer-parser.md), [Stories](./Agile/stories/)
- **Tracking:** [Sprint Status](./Agile/stories/sprint-status.yaml)
- **Changes:** [Change Requests](./Agile/)

---

## 📊 Navigation by Workflow

- **Greenfield Projects:** getting-started → language → stdlib
- **Bug Fixes:** stdlib → language/syntax
- **New Features:** language → stdlib
- **Refactoring:** language/advanced → guides/best-practices
- **Planning:** PRD → epics → stories
- **Architecture:** tech/architecture → agile/architecture

---

## 📈 Navigation by Phase

- **Analysis:** getting-started, reference, PRD
- **Planning:** language, guides, epics
- **Solutioning:** language/advanced, stdlib, architecture
- **Implementation:** stdlib, language/syntax, stories

---

## 📁 Root-Level Documents

These documents provide project-wide context and meta-information:

### Project Meta
- [README](./README.md) - Documentation overview and structure guide
- [Index](./index.md) - This file - complete documentation index

### Organization & Standards
- [Conventions](./_conventions.md) - Documentation standards and style guide
- [Tags](./_tags.md) - Tag registry for documentation categorization
- [Graph](./_graph.yaml) - Navigation graph for agent pathfinding
- [Changelog](./_changelog.md) - High-level documentation changes

### Quality & Audit
- [Audit Inconsistencies](./AUDIT-INCONSISTENCIES.md) - Known documentation issues
- [Audit TODO](./AUDIT-TODO.md) - Documentation improvement backlog
- [Documentation Hierarchy](./DOCUMENTATION-HIERARCHY.md) - Structure explanation

### Reports
- [BMAD Reorganization Complete](./BMAD-REORGANIZATION-COMPLETE.md) - Major restructure summary
- [Integration Report 2025-12-27](./INTEGRATION-REPORT-2025-12-27.md) - Recent integration status
- [Index-New](./INDEX-NEW.md) - Alternate index format (deprecated)
- [INDEX](./INDEX.md) - Legacy index (deprecated - use this file instead)

---

## 📊 Documentation Statistics

- **Total Files:** 1,877 markdown documents
- **User Docs:** 137 files
- **Agile Docs:** 65 files
- **Tech Docs:** 36 files
- **Audit Docs:** 20+ files
- **Archive:** 1,500+ historical files
- **Root Files:** 12 meta/organizational documents

---

**BMAD Optimized** | [Full Navigation Graph](./_graph.yaml) | [Tag Registry](./_tags.md) | [Conventions](./_conventions.md)

**Last Indexed:** 2025-12-30 by `/BMad:tasks:index-docs`
