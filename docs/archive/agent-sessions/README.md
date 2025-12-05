# Agent Work Tracking & Session Records

**Purpose**: Comprehensive work tracking system for all agents
**Maintained By**: All agents
**Aggregated By**: Mai (Secretary)
**Expanded Scope**: Track sessions, ongoing work, backlogs, and dependencies

---

## Operational Procedure

### For All Agents (NOT Mai)

**Your agent file `{agent-name}-sessions.md` tracks:**

1. **Individual Sessions with hhj**
   - Date, topics discussed, decisions made
   - Syntax/features clarified, examples provided
   - Action items with owners

2. **Current Work In Progress**
   - What you're actively working on
   - Status updates
   - Blockers and dependencies

3. **Your Backlog**
   - Pending tasks assigned to you
   - Priority levels
   - Estimated effort/complexity

4. **Dependencies**
   - Who/what you're waiting on
   - Who's waiting on you
   - Critical path items

**Update Frequency**:
- Sessions: Immediately during/after
- Work status: Daily or at major milestones
- Backlog: When tasks assigned or completed
- Dependencies: When they change

---

### For Mai (Secretary)

**Expanded Responsibilities**:

1. **Scan & Aggregate** (daily or after each party-mode session):
   - Review all agent files for updates
   - Extract decisions → `docs/technical/decisions/approved.md` and `pending.md`
   - Consolidate session notes → `docs/project/meetings/`
   - Update central TODO → `docs/project/project-todo.yaml`

2. **Track Everyone's Work**:
   - Monitor work in progress across all agents
   - Identify bottlenecks and blockers
   - Surface dependencies and critical path items
   - Flag overload or idle capacity

3. **Manage Backlogs**:
   - Consolidate agent backlogs into project view
   - Track task assignments and ownership
   - Monitor priority shifts
   - Ensure no tasks fall through cracks

4. **Dependency Management**:
   - Map inter-agent dependencies
   - Alert agents when dependencies resolve
   - Identify circular or problematic dependencies
   - Update dependency graphs

5. **Conflict Resolution**:
   - Flag conflicts between agent work
   - Identify misalignments early
   - Escalate to hhj when needed
   - Track resolution outcomes

6. **Reporting**:
   - Generate status reports on demand
   - Provide project health metrics
   - Track velocity and throughput
   - Maintain audit trail

---

## Agent Session Files

| Agent | File | Responsibility |
|-------|------|----------------|
| Mary (Analyst) | `mary-sessions.md` | Requirements analysis, research sessions |
| Winston (Architect) | `winston-sessions.md` | Architecture decisions, technical design |
| Amelia (Developer) | `amelia-sessions.md` | Implementation discussions, coding sessions |
| John (PM) | `john-sessions.md` | Product strategy, prioritization |
| Bob (Scrum Master) | `bob-sessions.md` | Story refinement, sprint planning |
| Murat (TEA) | `murat-sessions.md` | Testing strategy, quality gates |
| Paige (Tech Writer) | `paige-sessions.md` | Documentation sessions, writing guidance |
| Sally (UX Designer) | `sally-sessions.md` | UX design, user research |
| Carson (Brainstorming) | `carson-sessions.md` | Brainstorming sessions, ideation |
| Dr. Quinn (Problem Solver) | `drquinn-sessions.md` | Problem-solving sessions |
| Maya (Design Thinking) | `maya-sessions.md` | Design thinking workshops |
| Victor (Innovation) | `victor-sessions.md` | Innovation strategy sessions |
| Sophia (Storyteller) | `sophia-sessions.md` | Narrative development sessions |

**Note**: Famous thinkers (Leonardo, Dali, de Bono, Campbell, Jobs) should also record if they have individual sessions.

---

## Agent Work File Template

```markdown
# {Agent Name} - Work Tracking

**Role**: {Agent Role/Title}
**Last Updated**: YYYY-MM-DD

---

## Current Work In Progress

### {Task/Story Name}
**Status**: Started YYYY-MM-DD | Est. Completion: YYYY-MM-DD
**Priority**: HIGH | MEDIUM | LOW
**Related**: Epic X, Story Y, Decision Z

**Progress**:
- [x] Completed step 1
- [x] Completed step 2
- [ ] In progress: Step 3
- [ ] Pending: Step 4

**Blockers**:
- Waiting on Winston for architecture decision
- Need hhj approval on syntax

**Notes**: Additional context

---

## Backlog

### High Priority
1. **Task Name** - Brief description | Assigned: YYYY-MM-DD
2. **Task Name** - Brief description | Assigned: YYYY-MM-DD

### Medium Priority
1. **Task Name** - Brief description | Assigned: YYYY-MM-DD

### Low Priority / Nice to Have
1. **Task Name** - Brief description | Assigned: YYYY-MM-DD

---

## Dependencies

### I'm Waiting On:
- **Winston**: Architecture decision for IR design (Blocks: Epic 2)
- **Amelia**: Lexer implementation complete (Blocks: Parser work)

### Waiting On Me:
- **Bob**: Need story refinement done before sprint planning
- **Paige**: Documentation blocked until feature complete

---

## Recent Sessions with hhj

### Session: {Date} - {Topic}

**Date**: YYYY-MM-DD HH:MM
**Topics**: Brief list

**Discussion Summary**:
- Key points discussed
- Clarifications provided

**Decisions Made**:
1. **Decision**: Description
   - **Rationale**: Why
   - **Impact**: Consequences

**Examples Provided**:
```polyglot
// Code examples
```

**Action Items**:
- [ ] Task 1 - Owner - Due: YYYY-MM-DD
- [ ] Task 2 - Owner - Due: YYYY-MM-DD

---

## Completed Work (Recent)

### {Task Name} - Completed YYYY-MM-DD
**Outcome**: Brief description
**Artifacts**: Links to deliverables
**Lessons Learned**: Notes for future

---
```

---

## Why This System?

**Problem**: Individual agent sessions with hhj weren't being captured, leading to lost decisions and context.

**Solution**: Distributed documentation where:
- Each agent maintains their own session log
- Mai aggregates into central decision/meeting records
- No single point of failure
- Preserves context even if Mai isn't present

**Benefits**:
- Nothing gets lost
- Distributed workload
- Agent accountability
- Complete audit trail
- Easy to reconstruct project history

---

## Guidelines

### Do:
✅ Record sessions immediately or during breaks
✅ Be specific about decisions and rationale
✅ Include code examples when discussing syntax
✅ Note action items with clear owners
✅ Cross-reference related decisions
✅ Update as session progresses for long discussions

### Don't:
❌ Wait until end of day to record (you'll forget details)
❌ Record conversations Mai was present for (she handles those)
❌ Skip "small" decisions (they matter!)
❌ Forget to commit/save your session file
❌ Use vague language ("we discussed X") - be specific!

---

**Last Updated**: 2025-11-18
**Process Owner**: Mai (Secretary)
