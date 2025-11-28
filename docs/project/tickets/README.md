# ITIL Ticket System

**Maintained By:** Mai (Secretary)
**Framework:** ITIL (Information Technology Infrastructure Library)
**Last Updated:** 2025-11-18

---

## Overview

This directory contains all ITIL-compliant tickets for the Polyglot project, organized by ticket type following ITIL best practices.

---

## Directory Structure

```
tickets/
├── incidents/          # Unplanned disruptions (INC-YYYY-NNN)
├── problems/           # Root cause investigations (PRB-YYYY-NNN)
├── changes/            # Planned modifications (CHG-YYYY-NNN)
├── service-requests/   # Standard requests (SR-YYYY-NNN)
├── index.yaml         # Master ticket registry (auto-generated)
└── README.md          # This file
```

---

## Ticket Types

### 🔴 Incidents (INC)
**Purpose:** Unplanned service disruptions requiring immediate resolution

**Examples:**
- Syntax errors blocking development
- Broken documentation links
- Build failures
- Blocking inconsistencies

**Priority:** Usually P1-P2 (Critical/High)
**Workflow:** New → Assigned → In Progress → Resolved → Closed

---

### 🔍 Problems (PRB)
**Purpose:** Root cause investigations to prevent future incidents

**Examples:**
- Recurring syntax ambiguities
- Design conflicts requiring decisions
- Architectural concerns
- Pattern of related incidents

**Priority:** Usually P2-P3 (High/Medium)
**Workflow:** New → Assigned → Investigating → Root Cause Found → Resolved → Closed

---

### 📋 Changes (CHG)
**Purpose:** Planned modifications requiring approval

**Examples:**
- Syntax modifications
- Architecture changes
- Process improvements
- Tool additions

**Priority:** Usually P3-P4 (Medium/Low)
**Workflow:** New → Assessment → Approval Pending → Approved → Scheduled → In Progress → Completed → Closed

---

### 📝 Service Requests (SR)
**Purpose:** Standard requests from predefined catalog

**Examples:**
- Documentation requests
- Example creation
- Brainstorming sessions
- Code review requests

**Priority:** Usually P3-P5 (Medium/Low/Planning)
**Workflow:** New → Assigned → In Progress → Completed → Closed

---

## Priority Matrix (Impact × Urgency)

| Impact / Urgency | Immediate | High | Medium | Low |
|------------------|-----------|------|--------|-----|
| **Critical**     | P1        | P1   | P2     | P3  |
| **High**         | P1        | P2   | P2     | P3  |
| **Medium**       | P2        | P3   | P3     | P4  |
| **Low**          | P3        | P4   | P4     | P5  |

---

## Priority Definitions

- **P1 (Critical):** Blocking issue - immediate action required (SLA: 1hr response, 24hr resolution)
- **P2 (High):** Major impact - urgent resolution needed (SLA: 4hr response, 3 days resolution)
- **P3 (Medium):** Moderate impact - normal priority (SLA: 1 day response, 1 week resolution)
- **P4 (Low):** Minor impact - can be scheduled (SLA: 3 days response, 2 weeks resolution)
- **P5 (Planning):** Enhancement - future improvement (SLA: 1 week response, as scheduled)

---

## Ticket Lifecycle

### Incident Lifecycle
```
New → Assigned → In Progress → Resolved → Closed
                      ↓
                   Pending (if waiting on external input)
```

### Problem Lifecycle
```
New → Assigned → Investigating → Root Cause Found → Resolved → Closed
```

### Change Lifecycle
```
New → Assessment → Approval Pending → Approved → Scheduled → In Progress → Completed → Closed
                                          ↓
                                      Rejected → Closed
```

### Service Request Lifecycle
```
New → Assigned → In Progress → Completed → Closed
```

---

## Creating Tickets

**Format:** Each ticket is a YAML file following this naming convention:
- `incidents/INC-2025-001.yaml`
- `problems/PRB-2025-001.yaml`
- `changes/CHG-2025-001.yaml`
- `service-requests/SR-2025-001.yaml`

**Required Fields:**
- `ticket_id`
- `type`
- `title`
- `description`
- `priority`
- `status`
- `created_date`
- `created_by`

**Optional Fields:**
- `assigned_to`
- `assigned_group`
- `category`
- `impact`
- `urgency`
- `related_tickets`
- `github_issue` (for sync)
- `resolution`
- `closed_date`

See template files in each subdirectory for examples.

---

## GitHub Issues Sync (Future)

This ticket system is designed for future synchronization with GitHub Issues:

**When gh CLI is installed:**
1. Each ticket can be pushed to GitHub Issues
2. `github_issue` field stores the issue number
3. Labels automatically map (see `docs/project/itil-config.yaml`)
4. Bidirectional sync possible

**Preparation complete:** All ticket schemas include GitHub-compatible fields

---

## Querying Tickets

**Find all P1 incidents:**
```bash
grep -r "priority: P1" docs/project/tickets/incidents/
```

**Find open tickets:**
```bash
grep -r "status: in_progress" docs/project/tickets/
```

**Find tickets assigned to Amelia:**
```bash
grep -r "assigned_to: Amelia" docs/project/tickets/
```

---

## Reports

**Generated Reports Location:** `docs/project/tickets/reports/`

**Available Reports:**
- Daily: Open P1/P2 count, tickets opened/resolved today
- Weekly: Tickets by priority/status/group, SLA compliance, aging tickets
- Monthly: Trend analysis, problem patterns, backlog health

---

## See Also

- [ITIL Configuration](../itil-config.yaml) - Priority matrix, SLA definitions, workflows
- [Project TODO](../project-todo.yaml) - Legacy TODO tracking (being migrated)
- [Brainstorming Backlog](../brainstorming-backlog.md) - Syntax decision tracking

---

**For Questions:** Contact Mai (Secretary) or hhj (Project Owner)
