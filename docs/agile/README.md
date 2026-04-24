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

## How to Use these Files & Establish Workflows
1. **GitHub Issue Enforcement:** Every newly defined Epic, Feature, User Story, and Task **must** have an associated GitHub Issue created (`gh issue create`). 
2. **Dependency Linkage:** When raising the GitHub issue, the body must contain a reference to its hierarchical dependencies (e.g., `Parent #IssueNum` or `Depends on #IssueNum`). This tracks progress and guarantees the pipeline doesn't decouple.
3. **Yaml Sync:** Upon creating the GitHub Issue, ensure the local `.md` file's YAML frontmatter replaces `github-issue-link: "#"` with the live issue URL to give agents contextual memory of progressing workflows.
