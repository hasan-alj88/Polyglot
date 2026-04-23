# Agile Tracking Framework

**Audience**: Internal Agile Personas (Product Owner, Scrum Master, Development Team)

This directory serves as the memory and tracking backbone for Polyglot's AI Agile Team. It operates as the source of truth for planning, defining, and coordinating agile development work, which maps directly to corresponding GitHub Issues.

## Hierarchical Structure

- `PRD/`: Product Requirement Documents defining the "What" and "Why", maintained by the **Product Owner**.
- `architecture/`: Documentation on the system design, consumed and managed by the **Development Team**.
- `epics/`: Large-scale objectives and product areas.
  - `features/`: Specific capabilities driving value under an epic, broken down by the **Scrum Master**.
    - `user-stories/`: Scenarios framing the exact user flows and requirements.
      - `tasks/`: Executable units of work assigned to the **Development Team**.

## How to Use these Files
1. **GitHub Linkage:** Ensure all item files (epic, feature, user-story, task) contain a YAML frontmatter mapping it to the corresponding GitHub Issue.
2. **State & Dependencies:** Keep track of blockers, dependencies, and `status` natively within the frontmatter to give agents contextual memory of progressing workflows.
