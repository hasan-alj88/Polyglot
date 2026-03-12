# Project Documentation

**Audience:** Project team, stakeholders, contributors
**Purpose:** Project management, planning, tracking, and collaboration

---

## 📋 Project Planning

### [Product Requirements Document (PRD)](prd.md)
Complete product vision, requirements, and scope definition for Polyglot v0.0.2.

### [Product Briefs](product-brief-Polyglot-2025-11-15.md)
High-level product vision and strategic direction.

### [Epics](epics.md)
Major feature sets and epic breakdown:
- Epic 1: Lexer & Parser Foundation
- Epic 2: Parser Implementation
- Future epics TBD

### [Stories](stories/)
User stories with acceptance criteria, dependencies, and implementation details:
- `stories/1-1-project-workspace-build-system-setup.md`
- `stories/1-2-lexer-token-definitions.md`
- More stories TBD

### [Sprint Status](stories/sprint-status.yaml)
Current sprint tracking, story status, and progress monitoring.

---

## 🎯 Tracking & Management

### [Project TODO](project-todo.yaml)
Active todos, assignments, priorities, and deadlines:
- **Format:** YAML with metadata
- **Maintained By:** Mai (Secretary)
- **Updated:** Daily

### [ITIL Tickets](tickets/)
Structured ticket system following ITIL best practices:

#### Ticket Types:
- **[Incidents](tickets/incidents/)** - Unplanned disruptions (INC-YYYY-NNN)
- **[Problems](tickets/problems/)** - Root cause investigations (PRB-YYYY-NNN)
- **[Changes](tickets/changes/)** - Planned modifications (CHG-YYYY-NNN)
- **[Service Requests](tickets/service-requests/)** - Standard requests (SR-YYYY-NNN)

#### [Ticket Index](tickets/index.yaml)
Master registry of all tickets with statistics and status tracking.

#### [ITIL Configuration](itil-config.yaml)
Priority matrix, SLA definitions, workflows, and assignment groups.

#### [Ticket Reports](tickets/reports/)
Daily, weekly, and monthly reports on ticket status and SLA compliance.

---

## 🤝 Collaboration

### [Agent Registry](agent-registry.yaml)
Agent roles, responsibilities, and current assignments:
- Amelia (Dev Agent)
- Winston (Architect)
- Paige (Tech Writer)
- Carson (Brainstorming Coach)
- Bob (Scrum Master)
- Murat (Test Architect)
- Mai (Secretary)

### [Meetings](meetings/)
Meeting minutes, decisions, and action items:
- Format: `meetings/YYYY-MM-DD-topic.md`
- Includes: attendees, agenda, decisions, action items
- Maintained By: Mai (Secretary)

### [Agent Sessions](agent-sessions/)
Brainstorming sessions, design explorations, and decision-making:
- Carson's brainstorming sessions (syntax, features, design)
- Mai's work tracking and session notes
- Session outputs inform technical decisions

---

## 💡 Brainstorming

### [Brainstorming Backlog](brainstorming-backlog.md)
Topics requiring brainstorming sessions:
- **Pending:** Future topics for exploration
- **Completed:** Finalized specifications with session documents

### [Brainstorming Session Results](brainstorming-session-results-2025-11-19.md)
Detailed outputs from brainstorming sessions:
- `brainstorming-session-results-2025-11-15.md` - Initial exploration
- `brainstorming-session-results-2025-11-16.md` - Follow-up session
- `brainstorming-session-results-2025-11-19.md` - [s] Serial Load Block specification

---

## 📊 Project Status

### Current Phase
**Phase 4:** Implementation (Epic 1 - Lexer & Parser Foundation)

### Active Sprint
See [stories/sprint-status.yaml](stories/sprint-status.yaml)

### Open Tickets
See [tickets/index.yaml](tickets/index.yaml)

### Recent Activity
- ✅ [s] Serial Load Block specification completed (2025-11-19)
- ✅ ITIL ticket system established (2025-11-18)
- ✅ Documentation reorganization (2025-11-19)
- 🔄 Story 1.2 - Lexer Token Definitions (IN PROGRESS)

---

## 🔄 Workflows

### Story Creation Workflow
1. Epic defined in [epics.md](epics.md)
2. Bob (Scrum Master) creates story file in `stories/`
3. Story added to [sprint-status.yaml](stories/sprint-status.yaml)
4. Amelia (Dev Agent) implements
5. Murat (Test Architect) validates
6. Story marked DONE

### Ticket Management Workflow
1. Issue identified or request submitted
2. Mai (Secretary) creates ticket in appropriate category
3. Ticket assigned to agent/group
4. Agent works on resolution
5. Resolution documented
6. Ticket closed with verification

### Brainstorming Workflow
1. Topic added to [brainstorming-backlog.md](brainstorming-backlog.md)
2. Carson (Brainstorming Coach) facilitates session
3. Session results documented in `brainstorming-session-results-*.md`
4. Decisions recorded in agent sessions
5. Actionable items converted to tickets/stories
6. Backlog item marked COMPLETED

---

## 📁 File Organization

```
project/
├── prd.md                          # Product Requirements Document
├── epics.md                        # Epic definitions
├── product-brief-*.md              # Product vision briefs
├── project-todo.yaml               # Active TODO tracking
├── itil-config.yaml                # ITIL configuration
├── agent-registry.yaml             # Agent roles and responsibilities
├── brainstorming-backlog.md        # Brainstorming topics
├── brainstorming-session-results-*.md  # Session outputs
├── stories/                        # User stories
│   ├── sprint-status.yaml         # Sprint tracking
│   └── *.md                       # Individual story files
├── tickets/                        # ITIL tickets
│   ├── index.yaml                 # Master ticket registry
│   ├── incidents/                 # INC tickets
│   ├── problems/                  # PRB tickets
│   ├── changes/                   # CHG tickets
│   ├── service-requests/          # SR tickets
│   └── reports/                   # Ticket reports
├── meetings/                       # Meeting minutes
│   └── YYYY-MM-DD-*.md           # Meeting records
└── agent-sessions/                 # Agent brainstorming sessions
    └── *.md                       # Session documents
```

---

## 🤖 Agent Responsibilities

- **Mai (Secretary):** Meeting minutes, TODO tracking, ITIL tickets, documentation coordination
- **Bob (Scrum Master):** Story creation, sprint planning, velocity tracking
- **Carson (Brainstorming Coach):** Design sessions, syntax exploration, feature brainstorming
- **Winston (Architect):** Technical decisions, architecture documentation, ADRs
- **Amelia (Dev Agent):** Story implementation, coding, development
- **Murat (Test Architect):** Testing strategy, quality gates, test design
- **Paige (Tech Writer):** User documentation, technical writing, formatting

---

**Maintained By:** Mai (Secretary), Project Team
**Last Updated:** 2025-11-19
