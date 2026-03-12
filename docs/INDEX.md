# Polyglot Documentation Index

**Last Updated:** 2026-03-12
**Total Files:** ~857

---

## Documentation Structure

### Agile/ - Project Management
*Audience: Product managers, architects, development team*

- **prds/** - Product Requirements Documents
- **epics/** - Epic definitions and tracking
- **stories/** - User stories and implementation tickets
- **tickets/** - Issue tracking and resolutions
- **tech-stack/** - Technology decisions and architecture patterns
- **reference/** - Technical reference materials (lexer, parser specs)
- **conventions/** - Project conventions and standards

**Key Files:**
- [PRD](Agile/prds/prd.md) - Product Requirements Document
- [Epics](Agile/epics/epics.md) - All project epics
- [Technology Stack](Agile/tech-stack/technology-stack.md)
- [Conventions](Agile/conventions/conventions.md)

---

### User/ - Language Documentation (v0.0.4)
*Audience: Polyglot programmers and language users*

- **getting-started/** - Tutorials and quick start guides
- **language/** - Language syntax and semantics
  - syntax/ - Operators, markers, prefixes
  - types/ - Type system, enums, variables
  - control-flow/ - Pipelines, loops, conditionals
  - advanced/ - Advanced features
- **stdlib/** - Standard library reference
  - loops/ - Unpack and pack operations
  - utilities/ - String, math, datetime, data parsing
  - wrappers/ - Runtime wrappers
- **examples/** - Code examples and tutorials
- **reference/** - Grammar and quick reference
- **specifications/** - v0.0.4 language specification

**Key Files:**
- [Getting Started](User/getting-started/core-principles.md)
- [v0.0.4 Specification](User/specifications/v0.0.4/README.md)
- [Standard Library Index](User/stdlib/index.md)
- [Grammar Reference](User/reference/grammar.md)

---

### v0.0.5/ - Next Version Documentation
*Audience: Polyglot programmers (forward-looking)*

- **language/** - Core language features (loops, triggers, wrappers)
- **stdlib/** - Standard library YAML definitions
- **reference/** - Grammar and technical specifications
- **quick-reference/** - Fast lookups
- **style-guide/** - Code conventions
- **examples/** - Working code examples
- **training-sessions/** - Learning materials

**Key Files:**
- [What's New in v0.0.5](v0.0.5/whats-new-v0.0.5.md)
- [Migration Guide (v0.0.4 to v0.0.5)](v0.0.5/migration-guide-v0.0.4-to-v0.0.5.md)
- [Grammar Reference](v0.0.5/reference/grammar-reference.md)

---

### Tech/ - Developer Documentation
*Audience: Polyglot language implementers and contributors*

- **implementation/** - Parser, lexer, compiler internals
- **ai-context/** - AI development context and corrections
- **automation/** - Automation setup
- **development/** - Contributing guides and setup

**Key Files:**
- [Architecture](Tech/implementation/technical/architecture.md)
- [AI Context](Tech/ai-context/README.md)

---

### Audit/ - Documentation Audit
*Audience: Documentation team*

- **history/** - Document change history
- **checks/** - Validation reports and audits

---

### archive/ - Historical Content
*Minimal archive — bulk historical content archived to `docs-archive-pre-audit-2026-03-12.tar.gz`*

- **agent-sessions/** - Agent work tracking templates
- **old-code-examples/** - Pre-v0.0.4 syntax examples (archived for reference)

---

## Quick Navigation

### For Language Users
1. Start here: [Getting Started](User/getting-started/core-principles.md)
2. Learn syntax: [Language Documentation](User/language/README.md)
3. Explore stdlib: [Standard Library](User/stdlib/index.md)
4. See examples: [Code Examples](User/examples/)
5. Full spec: [v0.0.4 Specification](User/specifications/v0.0.4/README.md)

### For Contributors
1. Read: [Architecture](Tech/implementation/technical/architecture.md)
2. Understand: [AI Context](Tech/ai-context/README.md)

### For Project Team
1. Review: [PRD](Agile/prds/prd.md)
2. Track: [Epics](Agile/epics/epics.md)
3. Plan: [Stories](Agile/stories/)
4. Decide: [Tech Stack](Agile/tech-stack/technology-stack.md)
