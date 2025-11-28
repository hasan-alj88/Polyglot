# Polyglot Documentation

This directory contains all documentation for the Polyglot automation programming language.

---

## 📁 Documentation Structure

### [📖 User Documentation](user/)

**Audience:** End users, developers learning Polyglot
**Purpose:** Language guides, tutorials, examples, and reference documentation

**Contents:**
- **Language Specification** - Complete syntax reference, type system, operators, block markers
- **Standard Library** - Utilities, triggers, runtime wrappers, queue operations
- **Architecture Guides** - Queue system, runtime execution, trigger monitoring
- **Code Examples** - Hello world, data processing, error handling, workflows
- **CLI Reference** - Compile, register, activate commands
- **Package Management** - Creating and importing packages
- **Audit Reports** - Design decisions, compliance reports

**Current Version:** 0.0.2 (In Development)

**Source of Truth for:** Language syntax, semantics, and user-facing features

---

### [🔧 Technical Documentation](technical/)

**Audience:** Developers, architects, contributors
**Purpose:** System architecture, technical decisions, design rationale

- **[Architecture](technical/architecture.md)** - Complete system architecture
  - Component design and data flow
  - Architecture Decision Records (ADRs)
  - Technology stack and implementation patterns
  - Lexer, parser, runtime, queue system designs

- **[Decisions](technical/decisions/)** - Technical decisions and approvals
  - [Approved Decisions](technical/decisions/approved.md) - Finalized architectural decisions
  - [Pending Decisions](technical/decisions/pending.md) - Active conflicts, enhancement requests

**Source of Truth for:** Technical architecture and design decisions

---

### [📋 Project Documentation](project/)

**Audience:** Project team, stakeholders, contributors
**Purpose:** Project management, planning, tracking, and collaboration

**Planning Documents:**
- [Product Requirements (PRD)](project/prd.md) - Product vision, 120 FRs, success criteria
- [Product Briefs](project/product-brief-Polyglot-2025-11-15.md) - Strategic direction
- [Epic Breakdown](project/epics.md) - Major feature sets (Epic 1, 2, ...)
- [Stories](project/stories/) - User stories with acceptance criteria
- [Sprint Status](project/stories/sprint-status.yaml) - Current sprint tracking

**Tracking & Management:**
- [Project TODO](project/project-todo.yaml) - Active todos, assignments, deadlines
- [ITIL Tickets](project/tickets/) - Structured ticket system
  - [Incidents](project/tickets/incidents/) - Unplanned disruptions (INC)
  - [Problems](project/tickets/problems/) - Root cause investigations (PRB)
  - [Changes](project/tickets/changes/) - Planned modifications (CHG)
  - [Service Requests](project/tickets/service-requests/) - Standard requests (SR)
- [ITIL Configuration](project/itil-config.yaml) - Priority matrix, SLA, workflows
- [Agent Registry](project/agent-registry.yaml) - Agent roles and responsibilities

**Collaboration:**
- [Meetings](project/meetings/) - Meeting minutes and action items
- [Agent Sessions](project/agent-sessions/) - Brainstorming and design exploration
- [Brainstorming Backlog](project/brainstorming-backlog.md) - Topics for exploration
- [Brainstorming Results](project/brainstorming-session-results-2025-11-19.md) - Session outputs

**Source of Truth for:** Project planning, implementation scope, and team coordination

---

## 🚀 Quick Navigation

**Writing `.pg` files?**
→ See [User Documentation](user/) for language syntax and examples

**Implementing Polyglot compiler/runtime?**
→ See [Technical Documentation](technical/architecture.md) for architecture patterns

**Understanding MVP scope and priorities?**
→ See [PRD](project/prd.md) and [Epic Breakdown](project/epics.md)

**Working on a specific story?**
→ See [Stories](project/stories/) directory

**Looking for design decisions?**
→ See [Technical Decisions](technical/decisions/)

**Tracking tickets or tasks?**
→ See [ITIL Tickets](project/tickets/) and [Project TODO](project/project-todo.yaml)

**Finding meeting notes or brainstorming sessions?**
→ See [Meetings](project/meetings/) and [Agent Sessions](project/agent-sessions/)

---

## 🔗 Documentation Relationships

```
User Docs (user/)
    ↓ defines
Language Syntax & Semantics
    ↓ implemented by
Technical Architecture (technical/)
    ↓ planned in
Project Management (project/)
    ↓ executes via
Stories & Epics
    ↓ tracked by
ITIL Tickets & Sprint Status
```

**Key Principle:**
- **User docs** define WHAT Polyglot IS (complete language)
- **Technical docs** define HOW it's built (architecture)
- **Project docs** define WHEN features are built (MVP scope, priorities)

---

## 📝 Contributing

When adding or updating documentation:

1. **Language syntax changes** → Update [User Documentation](user/)
2. **Architectural decisions** → Update [Technical Documentation](technical/)
3. **Implementation planning** → Update [Project Documentation](project/)
4. **Cross-references** → Use relative paths from docs/ root

**Documentation Standards:**
- All documents follow BMAD methodology conventions
- Markdown format (GitHub-flavored)
- Cross-references use relative paths
- Each section has dedicated README for navigation

---

## 📊 Current Status

**Phase:** 4 - Implementation (Epic 1: Lexer & Parser Foundation)
**Active Sprint:** See [sprint-status.yaml](project/stories/sprint-status.yaml)
**Recent Updates:**
- ✅ [s] Serial Load Block specification completed (2025-11-19)
- ✅ Documentation reorganization (2025-11-19)
- ✅ ITIL ticket system established (2025-11-18)
- 🔄 Story 1.2 - Lexer Token Definitions (IN PROGRESS)

---

**Documentation Structure Updated:** 2025-11-19
**Maintained By:** Polyglot Documentation Team
