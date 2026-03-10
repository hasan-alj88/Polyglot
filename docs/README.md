# Polyglot Documentation

**Version:** v0.0.4
**Status:** ✅ Specification Finalized - December 2025
**Documentation Structure:** Scribe 3-Tier (Agile/User/Tech)
**Last Updated:** 2025-12-24

---

## 📁 Documentation Structure

Polyglot documentation is organized into three main categories based on audience:

### 📋 [Agile/](Agile/) - Project Management
*For: Product managers, architects, development team*

- **PRDs & Requirements** - Product vision and specifications
- **Epics & Stories** - Development tracking and planning
- **Architecture Decisions** - Technical design choices
- **Technology Stack** - Platform and tool decisions

**Quick Links:**
- [PRD](Agile/prds/prd.md) - Product Requirements Document
- [Epics](Agile/epics/epics.md) - Project epics and milestones
- [Technology Stack](Agile/tech-stack/technology-stack.md)

---

### 📖 [User/](User/) - Language Documentation
*For: Polyglot programmers and language users*

The complete v0.0.4 language specification and user guides.

#### Getting Started
- [Core Principles](User/getting-started/core-principles.md) - Language philosophy
- [v0.0.4 Specification](User/specifications/v0.0.4/README.md) - Complete language spec

#### Language Reference
- [Syntax](User/language/syntax/) - Prefixes, markers, operators, I/O
- [Types](User/language/types/) - Type system, enums, variables, lifecycle
- [Control Flow](User/language/control-flow/) - Pipelines, execution order
- [Advanced Features](User/language/advanced/) - Loops, metadata, serial loading

#### Standard Library
- [Standard Library Index](User/stdlib/index.md) - Complete package tree
- [Loops](User/stdlib/loops/) - Unpack/pack operations
- [Utilities](User/stdlib/utilities/) - Math, string, datetime, data parsing
- [Wrappers](User/stdlib/wrappers/) - Runtime integration

#### Examples & Reference
- [Code Examples](User/examples/) - Practical Polyglot programs
- [Reference](User/reference/) - Grammar and quick references

**Quick Links:**
- [Getting Started](User/getting-started/core-principles.md)
- [Complete Specification](User/specifications/v0.0.4/README.md)
- [Standard Library](User/stdlib/index.md)
- [Grammar Reference](User/reference/grammar.md)

---

### 🔧 [Tech/](Tech/) - Developer Documentation
*For: Polyglot language implementers and contributors*

Implementation details, architecture, and development guides.

- **Implementation** - Parser, lexer, compiler internals
- **Architecture** - System design and decisions
- **AI Context** - LLM development context
- **Development** - Contributing guides and setup

**Quick Links:**
- [Architecture](Tech/implementation/technical/architecture.md)
- [AI Context](Tech/ai-context/README.md)

---

## 🚀 Quick Start

### New to Polyglot?
1. Read [Core Principles](User/getting-started/core-principles.md)
2. Explore the [v0.0.4 Specification](User/specifications/v0.0.4/README.md)
3. Try [Code Examples](User/examples/)

### Working on Polyglot?
1. Review the [PRD](Agile/prds/prd.md)
2. Check current [Epics](Agile/epics/epics.md)
3. Read [Architecture](Tech/implementation/technical/architecture.md)

### Contributing?
1. Understand the [Architecture](Tech/implementation/technical/architecture.md)
2. Review [Tech Stack](Agile/tech-stack/technology-stack.md)
3. Check [Development Guide](Tech/development/) *(coming soon)*

---

## 📊 Key Concepts

### Core Principles
1. **No Keywords** - Only markers and operators
2. **One Line = One Marker + One Expression**
3. **Indentation for Nesting** - 3-space indentation
4. **Universal Hierarchy** - `PREFIX.identifier.path` everywhere
5. **Explicit Over Implicit** - Metadata makes intent clear
6. **Variable Prefix: `$`** - Clear, unambiguous, greppable

### Variable Lifecycle (5 States)
```
Pending → Default → Final → Released
    ↓                 ↓
  Faulted → Released
```

### Critical Prefixes & Operators
- `$` - Variable (e.g., `$user`)
- `:` - Type (e.g., `:pg.string`)
- `#` - Enum/Struct (e.g., `#OrderStatus.Processing`)
- `|` - Pipeline (e.g., `|Database.Users.Find`)
- `!` - Error (e.g., `!Network.HTTP.Timeout`)
- `@` - Registry (e.g., `@Local::MyApp:1.0.0.0`)
- `%` - Metadata (e.g., `%Doc`)

### Critical Markers
- `[r]` - Sequential execution
- `[p]` - Parallel execution
- `[|]` - Pipeline I/O
- `[~]` - Unpack (main → iteration)
- `[*]` - Pack (iteration → main)
- `[s]` - Serial load block
- `[v]` - Join operation
- `[f]` - Fork/conditional

---

## 📚 Documentation Index

For a complete, searchable index of all documentation:
**→ See [INDEX.md](INDEX.md)**

The master index provides:
- File counts by category
- Navigation by audience
- Known issues tracking
- Recent activity log

---

## 🔍 Finding Information

### By Audience
- **Language Users** → Start in [User/](User/)
- **Project Team** → Start in [Agile/](Agile/)
- **Contributors** → Start in [Tech/](Tech/)

### By Topic
- **Syntax** → [User/language/syntax/](User/language/syntax/)
- **Types** → [User/language/types/](User/language/types/)
- **Standard Library** → [User/stdlib/](User/stdlib/)
- **Examples** → [User/examples/](User/examples/)
- **Architecture** → [Tech/implementation/](Tech/implementation/)
- **Project Planning** → [Agile/](Agile/)

### By Task
- **Learn Polyglot** → [User/getting-started/](User/getting-started/)
- **Understand v0.0.4 spec** → [User/specifications/v0.0.4/](User/specifications/v0.0.4/)
- **Find stdlib function** → [User/stdlib/index.md](User/stdlib/index.md)
- **Implement parser** → [Tech/implementation/](Tech/implementation/)
- **Track project** → [Agile/epics/](Agile/epics/)

---

## 📈 Documentation Statistics

- **Total Active Files:** 375
- **Agile:** 40 files
- **User:** 291 files
- **Tech:** 35 files
- **Audit:** 9 files

**Recent Updates:**
- 2025-12-24: Scribe reorganization complete (366 files migrated to 3-tier structure)
- 2025-12-24: Documentation validation completed
- 2025-12-15: v0.0.4 specification finalized

---

## 🛠️ Documentation Tools

This documentation is managed by **Scribe**, the Documentation Architect.

**Available Commands:**
- `/scribe` - Documentation management workflows
- `/polly` - Canonical Polyglot code examples

**Workflows Available:**
- `doc-validate` - Quality and link validation
- `doc-sync` - Detect drift from codebase
- `session-update` - Sync from agent sessions
- See more in Scribe workflows (`.bmad-creative-writing/workflows/scribe/`)

---

## ⚠️ Known Issues

**Critical:**
- 733 broken internal links (being fixed)
- 2 files with deprecated v0.0.3 syntax

**Warnings:**
- 535 code blocks without language hints
- 200+ files with multiple H1 headings

See [Latest Validation Report](Audit/checks/validate-2025-12-24.md) for details.

---

## 🤝 Contributing

### Documentation Contributions
- Follow Scribe 3-tier structure (Agile/User/Tech)
- Use `/polly` for canonical Polyglot examples
- Run validation before submitting

### Code Contributions
- Review [Architecture](Tech/implementation/technical/architecture.md)
- Check [Technology Stack](Agile/tech-stack/technology-stack.md)
- Follow coding standards in [Tech/](Tech/)

---

## 📝 Implementation Status

**Current Implementation:** Epic 1 (v0.0.3)
**Target Implementation:** Q2 2026 (v0.0.4)
**Specification Status:** ✅ Finalized

See [Agile/epics/](Agile/epics/) for current implementation progress.

---

## 📞 Support & Feedback

For questions, suggestions, or bug reports:
- **Issues:** GitHub Issues
- **Documentation:** This documentation is the source of truth
- **Design Decisions:** See [User/specifications/v0.0.4/](User/specifications/v0.0.4/)

---

**Maintained By:** Scribe Documentation Architect
**Integration:** Polly (Polyglot Language Expert)
**License:** See project LICENSE file

---

*This README reflects the Scribe 3-tier documentation structure (Agile/User/Tech).*
*For the complete documentation index, see [INDEX.md](INDEX.md)*
