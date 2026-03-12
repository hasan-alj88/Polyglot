# Technical Documentation

**Audience:** Developers, architects, contributors
**Purpose:** System architecture, technical decisions, design rationale

---

## Contents

### [Architecture](architecture.md)

Complete system architecture documentation including:
- **System Overview:** High-level architecture and design philosophy
- **Component Design:** Core components and their interactions
- **Data Flow:** How data moves through the system
- **Architecture Decision Records (ADRs):** Historical design decisions with rationale
- **Technology Stack:** Languages, frameworks, and tools used

**Key Sections:**
- Lexer & Parser Architecture
- Runtime Execution Model
- Queue System Design
- Pipeline Lifecycle
- Error Handling Strategy
- Type System Implementation

---

### [Decisions](decisions/)

Technical decisions, conflicts, and pending approvals:

#### [Approved Decisions](decisions/approved.md)
- Finalized architectural decisions
- Syntax specifications
- Technology choices
- Design patterns adopted

#### [Pending Decisions](decisions/pending.md)
- Active conflicts requiring resolution
- Enhancement requests
- Open questions for approval
- Alternatives under consideration

**Decision Workflow:**
```
Issue Identified → Pending → Analysis → Recommendation → Approval → Approved
```

---

## Architecture Decision Records (ADRs)

ADRs document significant architectural and design decisions:

**Format:**
- **Context:** What is the issue we're facing?
- **Decision:** What decision did we make?
- **Consequences:** What are the implications?
- **Alternatives:** What other options were considered?
- **Status:** Proposed | Accepted | Deprecated | Superseded

**Current ADRs:**
- Located in [architecture.md](architecture.md)
- Numbered sequentially (ADR-001, ADR-002, etc.)
- Cross-referenced in decision logs

---

## Technical Standards

### Code Documentation
- Inline comments for complex logic
- Module-level documentation
- API documentation with examples

### Architecture Documentation
- Keep ADRs concise and focused
- Include diagrams for complex systems
- Reference user stories and epics
- Update when designs evolve

### Decision Logging
- All significant decisions must be documented
- Include rationale and alternatives considered
- Link to related tickets, stories, and discussions
- Maintain decision log integrity

---

## Contributing

### Proposing Technical Changes

1. **Research:** Understand current architecture
2. **Document:** Create proposal with alternatives
3. **Discuss:** Present to architecture review
4. **Decide:** Record decision in pending.md
5. **Approve:** Move to approved.md after acceptance
6. **Implement:** Execute with proper documentation

### Architecture Review Process

- **Requester:** Winston (Architect) or assigned agent
- **Review:** Technical team + hhj (Project Owner)
- **Timeline:** P1 (1 day), P2 (3 days), P3 (1 week)
- **Approval:** Requires hhj sign-off for major changes

---

## Related Documentation

- **User Documentation:** [../user/](../user/) - End-user language guides
- **Project Documentation:** [../Agile/](../Agile/) - PRD, epics, stories, tickets
- **Brainstorming Sessions:** [../Agile/agent-sessions/](../Agile/agent-sessions/) - Design exploration

---

**Maintained By:** Winston (Architect), Technical Team
**Last Updated:** 2025-11-19
