# Polyglot Documentation Index

**Last Updated:** 2026-01-11
**Structure Version:** Scribe v1.1 (4-tier with v0.0.5)

---

## 📁 Documentation Structure

### 📋 Agile/ - Project Management (40 files)
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

### 📖 User/ - Language Documentation (291 files)
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

### 🚀 v0.0.5/ - Latest Version Documentation (17 files)
*Audience: Polyglot programmers (current version - v0.0.5)*

**📘 Core Documentation:**
- **language/** - Core language features
  - loop-system.md - Unpack/pack operators, iteration patterns (666 lines) ⭐⭐
  - trigger-system.md - All trigger types, I/O wiring, automation (1361 lines) ⭐⭐
  - wrapper-system.md - Resource management, DB/HTTP/runtime wrappers (900+ lines) ⭐⭐ NEW
  - variable-lifecycle.md - Immutability, default vs final states
  - error-handling.md - Error blocks, exhaustiveness, patterns
- **stdlib/** - Standard library YAML definitions (7 files)
  - reserved-enums.yaml - Stdlib enum types and values
  - standard-wrappers.yaml - Runtime, DB, HTTP, file wrappers
  - standard-pipelines.yaml - Core pipelines and utilities
  - standard-operators.yaml - Pack/unpack operators
  - standard-triggers.yaml - CLI, file, scheduled triggers
  - schemas/ - Reserved schema specifications
- **reference/** - Grammar and technical specifications
  - grammar-v0.0.5.ebnf - Formal EBNF specification (445 lines) ⭐⭐
  - grammar-reference.md - Human-readable grammar guide (706 lines) ⭐⭐
  - trigger-technical.md - Trigger runtime, lifecycle, performance (586 lines) ⭐⭐
  - trigger-dev-reference.md - Formal specs, AST, compilation (727 lines) ⭐⭐
- **quick-reference/** - Fast lookups
  - runtime-orchestration.md - Python, Rust, JavaScript integration ⭐
- **style-guide/** - Code conventions
  - field-naming-conventions.md - Underscores vs dashes, patterns
- **examples/** - Working code examples
  - hello-world-multi-runtime.pg - Python/Rust/JS orchestration ⭐
- **training-sessions/** - Learning materials
  - session-001-2026-01-02.md - 6 examples + runtime orchestration

**Key Files:**
- [v0.0.5 README](v0.0.5/README.md) - Complete documentation index ⭐
- [What's New in v0.0.5](v0.0.5/whats-new-v0.0.5.md) - Release notes and features ⭐
- [Migration Guide (v0.0.4 → v0.0.5)](v0.0.5/migration-guide-v0.0.4-to-v0.0.5.md) - Step-by-step migration ⭐
- [Loop System Guide](v0.0.5/language/loop-system.md) - Comprehensive iteration guide ⭐⭐
- [Trigger System Guide](v0.0.5/language/trigger-system.md) - Complete trigger reference ⭐⭐
- [Wrapper System Guide](v0.0.5/language/wrapper-system.md) - Resource management guide ⭐⭐ NEW
- [Grammar Reference](v0.0.5/reference/grammar-reference.md) - Complete syntax reference ⭐⭐

**Documentation Quality:** ✅ Core Complete (98%), Examples Growing (70%), Stdlib Excellent (90%)

**New Features in v0.0.5:**
- Loop system with unpack (`~`) and pack (`*`) operators
- Runtime orchestration (Python, Rust, JavaScript)
- Collection literals: arrays `( )`, sets `{ }`, serials `{:}`
- Code block marker `[c]` for cleaner multi-line code
- Reserved schemas for type-safe configuration
- Improved syntax: `-` for reserved enums, `%%` comments, `:dt` datetime type

**Documentation Gaps (In Progress):**
- ✅ ~~Trigger system guide~~ → COMPLETE (2026-01-07) - 3 docs, 2674 lines ⭐⭐
- ✅ ~~Wrapper system guide~~ → COMPLETE (2026-01-11) - Complete guide, 900+ lines ⭐⭐
- ⚠️ Enum definitions guide (planned) → NEXT PRIORITY
- ⚠️ Additional examples needed (file processing, data pipelines)

---

### 🔧 Tech/ - Developer Documentation (35 files)
*Audience: Polyglot language implementers and contributors*

- **implementation/** - Parser, lexer, compiler internals
  - technical/ - Implementation specifications
- **ai-context/** - AI development context and corrections
- **architecture/** - Internal system architecture
- **development/** - Contributing guides and setup

**Key Files:**
- [Architecture](Tech/implementation/technical/architecture.md)
- [AI Context](Tech/ai-context/README.md)
- [Implementation Notes](Tech/implementation/technical/)

---

### 📊 Audit/ - Documentation Audit (9 files)
*Audience: Documentation team*

- **history/** - Document change history
- **checks/** - Validation reports and audits

**Key Files:**
- [Latest Validation Report](Audit/checks/validate-2025-12-24.md)
- [Migration Log](Audit/history/migration-log-2025-12-24.md)
- [Changelog](Audit/history/changelog.md)

---

### 📦 archive/ - Historical Content (369 files)
*Audience: Reference only*

- **pre-2025-12-24-reorganization/** - Content before Scribe reorganization
- **pre-v0.0.4-sync/** - Pre-v0.0.4 documentation
- **old-user-docs/** - Historical user documentation

---

## 🗂️ Quick Navigation

### For Language Users (v0.0.5 - Current Version) ⭐ START HERE
1. **Quick Start:** [v0.0.5 README](v0.0.5/README.md) - Complete documentation index
2. **Learn What's New:** [What's New in v0.0.5](v0.0.5/whats-new-v0.0.5.md)
3. **Migrate from v0.0.4:** [Migration Guide](v0.0.5/migration-guide-v0.0.4-to-v0.0.5.md)
4. **Master Loops:** [Loop System Guide](v0.0.5/language/loop-system.md)
5. **Understand Wrappers:** [Wrapper System Guide](v0.0.5/language/wrapper-system.md) ⭐ NEW
6. **Try Example:** [Hello World Multi-Runtime](v0.0.5/examples/hello-world-multi-runtime.pg)
7. **Reference:** [Grammar Guide](v0.0.5/reference/grammar-reference.md)

### For Language Users (v0.0.4 - Previous Version)
1. Start here: [Getting Started](User/getting-started/core-principles.md)
2. Learn syntax: [Language Documentation](User/language/README.md)
3. Explore stdlib: [Standard Library](User/stdlib/index.md)
4. See examples: [Code Examples](User/examples/)

### For Contributors
1. Read: [Architecture](Tech/implementation/technical/architecture.md)
2. Setup: [Development Guide](Tech/development/) *(coming soon)*
3. Understand: [AI Context](Tech/ai-context/README.md)

### For Project Team
1. Review: [PRD](Agile/prds/prd.md)
2. Track: [Epics](Agile/epics/epics.md)
3. Plan: [Stories](Agile/stories/)
4. Decide: [Tech Stack](Agile/tech-stack/technology-stack.md)

---

## 📈 Documentation Statistics

- **Total Files:** ~1,900 markdown files (~25,613 lines)
- **Agile:** 40 files - Project management
- **User:** 623 files - v0.0.4 language documentation
- **v0.0.5:** 17 files - Current version (~9,124 lines) ⭐ NEW
- **Tech:** 62 files - Implementation documentation
- **Audit:** 10+ files - Documentation audit trail
- **Archived:** 369 files - Historical content

**Recent Activity:**
- 2026-01-11: Wrapper system guide added (900+ lines) ⭐⭐
- 2026-01-11: Quick fixes applied (broken symlinks, archive warnings)
- 2026-01-11: Documentation audit completed (Score: 89/100)
- 2026-01-07: Trigger system documentation added (3 docs, 2,674 lines) ⭐⭐
- 2026-01-05: v0.0.5 documentation created (14 files, 6,450 lines) ⭐
- 2026-01-04: Formal grammar specifications (EBNF + readable)

---

## 🔍 Known Issues

⚠️ **v0.0.5 Documentation Gaps (High Priority):**
- ✅ ~~Trigger system guide~~ → COMPLETE (2026-01-07)
- ✅ ~~Wrapper system guide~~ → COMPLETE (2026-01-11)
- Enum definitions guide needed → NEXT PRIORITY
- Additional practical examples needed (only 1 .pg file)

⚠️ **v0.0.4 Legacy Issues:**
- 2 CRITICAL unresolved (uppercase `[V]` marker, `#True` alias ambiguity)
- Archive files need deprecation warnings (11 files)
- 4 broken symlinks need cleanup

⚠️ **General:**
- Master index outdated (FIXED 2026-01-05) ✅
- Code blocks without language hints (ongoing improvement)

See [Latest Audit Report](Audit/checks/audit-report-2026-01-05.md) for complete details.

---

## 📝 Maintenance

**Documentation maintained by:** Scribe Documentation Architect
**Workflow location:** `.bmad-creative-writing/workflows/scribe/`
**Integration:** Uses Polly (`/polly`) for canonical Polyglot code examples

**Commands:**
- `/scribe` - Activate Scribe documentation workflows
- `/polly` - Get canonical Polyglot examples

---

**This index is auto-maintained by Scribe. Do not edit manually.**
