# Polyglot Documentation Cleanup Plan

**Date:** 2025-12-14
**Based on:** [AUDIT-REPORT.md](./AUDIT-REPORT.md)
**Execution Mode:** Phased approach (4 phases)

---

## Execution Strategy

**Principle:** Work incrementally to avoid breaking existing references

**Phases:**
1. **Archive Cleanup** - Resolve duplication, free up space
2. **Critical Navigation** - Add essential READMEs for discoverability
3. **Master Indexing** - Create top-level navigation system
4. **Polish & Completion** - Fill remaining gaps, proofread

**Estimated Total Effort:** 6-8 hours across 4 work sessions

---

## Phase 1: Archive Cleanup
**Goal:** Resolve 2.7MB of duplicate archived content
**Priority:** CRITICAL
**Estimated Time:** 2 hours

### Actions

#### 1.1 Audit Old User Docs Duplication
**Task:** Determine if `user-old/` and `user-old-original/` are duplicates

**Steps:**
```bash
# Compare file lists
diff <(find archive/old-user-docs/user-old -type f | sort) \
     <(find archive/old-user-docs/user-old-original -type f | sort)

# If structure identical, compare actual content
diff -r archive/old-user-docs/user-old/ archive/old-user-docs/user-old-original/
```

**Decision Matrix:**
- **If identical:** Delete `user-old/`, keep `user-old-original/`, rename to clearer date
- **If different:** Add README explaining the difference with timeline context
- **If mostly same with minor diffs:** Create diff summary, merge or document why both exist

#### 1.2 Consolidate Archive Folders
**If files are duplicates:**

**Actions:**
```bash
# Rename to clearer structure
mv archive/old-user-docs/user-old-original archive/user-docs-2025-11-backup

# Delete duplicate
rm -rf archive/old-user-docs/user-old

# Update archive/README.md to reference new path
```

**Create:** `archive/user-docs-2025-11-backup/README.md`
```markdown
# Archived User Documentation (Pre-December 2025)

**Archive Date:** 2025-11-30
**Reason:** Complete rewrite for v0.0.4 syntax finalization
**Replacement:** See [current user docs](../user/)

**Contents:** 73 files documenting v0.0.2 and early v0.0.3 syntax

**Historical Value:** Reference for syntax evolution, old examples

**Note:** These docs use outdated syntax:
- `,` variable prefix (now `$`)
- `[|]` pipeline blocks (now `{|}`)
- `\~\` backslash markers (now indentation-based)

**Access:** Files retained for historical reference but not actively maintained
```

#### 1.3 Optional: Compress Archive for Space Savings
**If archive exceeds 2MB after cleanup:**

```bash
cd archive/
tar -czf user-docs-2025-11-backup.tar.gz user-docs-2025-11-backup/
# Verify archive
tar -tzf user-docs-2025-11-backup.tar.gz | head -20
# Delete uncompressed if verified
rm -rf user-docs-2025-11-backup/
```

**Update archive/README.md** to explain compressed format and extraction

---

## Phase 2: Critical Navigation (READMEs)
**Goal:** Add missing READMEs to high-traffic folders
**Priority:** CRITICAL → HIGH
**Estimated Time:** 2-3 hours

### Actions

#### 2.1 User Documentation READMEs (11 folders)
**Priority Order:** CRITICAL folders first

##### 2.1.1 user/language/README.md (CRITICAL)
**Content:**
```markdown
# Polyglot Language Reference

**Current Version:** v0.0.4 (finalized December 2025)
**Implementation Status:** Epic 1 implements v0.0.3, migration to v0.0.4 planned Q2 2026

## Language Specifications

### Core Syntax
- [Operators](./operators.md) - Complete operator reference
- [Block Markers](./block-markers.md) - Pipeline, enum, error blocks
- [Variables & Types](./variables-and-types.md) - Variable syntax and type system
- [Metadata System](./metadata.md) - Doc, Author, Deprecated annotations

### Language Features
- [Pipelines](./pipelines.md) - Pipeline composition and invocation
- [Enums](./enums.md) - Enumeration types and reserved enums
- [Errors](./errors.md) - Error handling and propagation
- [Packages](./packages.md) - Package system and imports

### Grammar
- [BNF Grammar](./bnf/) - Formal grammar specification
- [EBNF](./ebnf.md) - Extended BNF notation

## Quick Links
- [Syntax Comparison](./syntax-comparison-v0.0.3-v0.0.4.md) - Changes from v0.0.3 to v0.0.4
- [Reserved Keywords](./reserved.md) - Reserved identifiers
- [Syntax Quick Reference](../quick-reference/) - Cheat sheets

## See Also
- [Specifications](../../specifications/) - Detailed version specs and design decisions
- [Examples](../examples/) - Annotated code examples
```

##### 2.1.2 user/syntax/README.md (CRITICAL)
**Content:**
```markdown
# Polyglot Syntax Documentation

**Quick Access:** See [Quick Reference](../quick-reference/) for syntax cheat sheets

## Syntax Sections

### Core Syntax Elements
- [Variable Syntax](./variables.md) - `$name` prefix, declarations
- [Block Delimiters](./blocks.md) - `{|}` pipeline, `{#}` enum, `{!}` error
- [IO Markers](./io-markers.md) - `[|] <param` input, `[|] >param` output
- [Type Annotations](./types.md) - `:pg.string`, `:pg.int`, custom types

### Operators
- [Pipeline Operators](./pipeline-operators.md) - `[r]`, `[p]`, `[W]`, `[Q]`
- [Data Flow](./data-flow.md) - `<<`, `>>`, `~` unpack
- [Conditional](./conditional.md) - `[?]` switch/conditional logic

### Advanced Syntax
- [Metadata Annotations](./metadata-syntax.md) - `[%] %Doc`, `[%] %Author`
- [Reserved Enums](./reserved-enums.md) - `#;EnumName;field` semicolon syntax
- [Alias System](./aliases.md) - `[A]` type and pipeline aliases

## Syntax Version History
- [v0.0.2 → v0.0.3 Changes](./v0.0.2-to-v0.0.3.md)
- [v0.0.3 → v0.0.4 Changes](./v0.0.3-to-v0.0.4.md)

## Related Documentation
- [Language Reference](../language/) - Complete language spec
- [Specifications](../../specifications/) - Version-specific design docs
- [Examples](../examples/) - Syntax in practice
```

##### 2.1.3 Other Critical User READMEs
**Create similar navigation READMEs for:**
- `user/guides/README.md` - Tutorials and how-to guides index
- `user/examples/README.md` - Already exists, verify completeness
- `user/cli/README.md` - CLI command reference index
- `user/advanced/README.md` - Advanced topics index
- `user/architecture/README.md` - Architecture docs (or merge with technical/)
- `user/audit/README.md` - Audit and decision logs index
- `user/packages/README.md` - Package system documentation
- `user/planning/README.md` - Planning and roadmap docs
- `user/quick-reference/README.md` - Quick reference sheets catalog
- `user/standard-library/README.md` - Standard library documentation

**Template for each:**
```markdown
# [Section Name]

[1-2 sentence description of what this section contains]

## Contents

[Bullet list of key files with brief descriptions]

## Quick Links
- [Related section 1]
- [Related section 2]

## See Also
- [External reference]
```

#### 2.2 Specifications READMEs

##### 2.2.1 specifications/VERSION-INDEX.md (HIGH PRIORITY)
**Create master version index:**

```markdown
# Polyglot Language Versions Index

**Current Stable:** v0.0.3
**Latest Finalized:** v0.0.4 (December 2025)
**Implementation Status:** Epic 1 targets v0.0.3, migration to v0.0.4 planned Q2 2026

---

## Version Matrix

| Version | Status | Implementation | Complete Spec |
|---------|--------|----------------|---------------|
| v0.0.2 | Archived | Superseded | [ai-context/v0.0.2](../ai-context/v0.0.2/) |
| v0.0.3 | **Stable** | Epic 1 (in progress) | [user docs](../user/) |
| v0.0.4 | **Finalized** | Planned Q2 2026 | [v0.0.4/](./v0.0.4/) |
| v0.0.5 | Draft | Future | [v0.0.5/](./v0.0.5/) |

---

## v0.0.4 (Finalized - December 2025)

**Major Changes from v0.0.3:**
- Variables: `,name` → `$name`
- Block delimiters: `[|]...[X]` → `{|}...{x}`
- IO markers: Universal `[|]` prefix (`[|] <param`, `[|] >param`)
- Type notation: `pg\string` → `:pg.string`
- Reserved enums: `#True` → `#;Boolean;True`
- Indentation: 3 spaces (no `\~\` markers)

**Documentation:**
- [Complete Spec](./v0.0.4/README.md)
- [Syntax Refinements](./v0.0.4/syntax-refinement/)
- [Loop System](./v0.0.4/loop-system/)
- [Migration Guide](./v0.0.4/MIGRATION-GUIDE.md) *(to be created)*

---

## Version Changelog

### v0.0.4 (December 2025)
**Focus:** Syntax finalization and loop system

**Added:**
- Comprehensive loop/iteration system
- Metadata annotations (`[%]` for Doc, Author, Deprecated)
- Alias system (`[A]` for types and pipelines)
- Reserved enum field syntax (`;field` semicolon prefix)

**Changed:**
- Variable prefix: `,` → `$`
- Block delimiters: `[|]...[X]` → `{|}...{x}`
- IO syntax: `[i]`/`[o]` → `[|] <param`/`[|] >param`
- Type syntax: `pg\type` → `:pg.type`
- Reserved enum syntax: `#True` → `#;Boolean;True`
- Indentation: Removed `\~\` markers, use 3 spaces

### v0.0.3 (November 2025)
**Focus:** Core parser implementation baseline

**Added:**
- Multi-file compilation support
- Package system foundation
- Basic error handling

### v0.0.2 (Pre-November 2025)
**Focus:** Initial syntax prototype

**Status:** Archived, superseded by v0.0.3+

---

## Migration Guides

- [v0.0.2 → v0.0.3 Migration](./migrations/v0.0.2-to-v0.0.3.md) *(to be created)*
- [v0.0.3 → v0.0.4 Migration](./migrations/v0.0.3-to-v0.0.4.md) *(to be created)*

---

## See Also
- [PRD](../project/prd.md) - Product requirements and roadmap
- [Epics](../project/epics.md) - Implementation breakdown
- [User Documentation](../user/) - Current stable version docs
```

##### 2.2.2 specifications/v0.0.4/COMPLETE-SPEC.md (HIGH PRIORITY)
**Consolidate the 4 "final" documents:**

Review these files and create single authoritative spec:
- v0.0.4-final-syntax-decisions.md
- v0.0.4-final-decisions.md
- v0.0.4-complete-syntax.md
- v0.0.4-design-decisions-final.md

**Action:** Merge or clearly label which is canonical

#### 2.3 Technical Documentation READMEs

##### 2.3.1 technical/architecture/README.md (MEDIUM PRIORITY)
**Content:**
```markdown
# Polyglot Architecture Documentation

**Current Version:** Architecture for v0.0.3 implementation (Epic 1)

## Architecture Sections

### System Design
- [01-introduction.md](./01-introduction.md) - Architecture overview
- [02-philosophy-and-concepts.md](./02-philosophy-and-concepts.md) - Core principles
- [03-architecture-overview.md](./03-architecture-overview.md) - High-level system design
- [04-project-structure.md](./04-project-structure.md) - Codebase organization

### Core Systems
- [05-technology-stack.md](./05-technology-stack.md) - Technologies and dependencies
- [06-patterns.md](./06-patterns.md) - Design patterns and conventions
- [07-data-architecture.md](./07-data-architecture.md) - Data models and flow
- [08-security.md](./08-security.md) - Security architecture
- [09-performance.md](./09-performance.md) - Performance considerations

### Operations
- [10-deployment.md](./10-deployment.md) - Deployment architecture
- [11-development-environment.md](./11-development-environment.md) - Dev setup
- [12-adrs.md](./12-adrs.md) - Architecture Decision Records index
- [13-implementation-readiness.md](./13-implementation-readiness.md) - Readiness assessment

## Key Architecture Decisions

See [../decisions/](../decisions/) for complete ADR list

**Critical ADRs:**
- ADR-008: 3-IR Structure (Trigger, Queue, Runner IR)
- ADR-003: PostgreSQL JSONB for IR storage
- ADR-012: PostgreSQL fallback for Redis

## See Also
- [Technical Decisions](../decisions/) - ADR index
- [PRD](../../project/prd.md) - Product requirements
```

##### 2.3.2 technical/decisions/README.md (MEDIUM PRIORITY)
**Content:**
```markdown
# Architecture Decision Records (ADRs)

**Purpose:** Document significant architectural decisions and their rationale

## ADR Index

| # | Title | Status | Date | Impact |
|---|-------|--------|------|--------|
| ADR-001 | Manual Cargo Workspace Setup | Accepted | 2025-11 | Build System |
| ADR-003 | PostgreSQL JSONB for IR Storage | Accepted | 2025-11 | Database |
| ADR-008 | 3-IR Structure (Trigger/Queue/Runner) | Accepted | 2025-11 | Core Architecture |
| ADR-012 | PostgreSQL Fallback for Redis | Accepted | 2025-11 | Reliability |
| ... | *(Complete from 12-adrs.md)* | ... | ... | ... |

## ADR Process

1. **Proposal:** Create ADR-NNN-title.md with problem statement
2. **Discussion:** Review with team/community
3. **Decision:** Mark as Accepted/Rejected/Superseded
4. **Implementation:** Reference ADR in code and docs

## ADR Template

See [ADR-TEMPLATE.md](./ADR-TEMPLATE.md) for standard format

## See Also
- [Architecture Overview](../architecture/12-adrs.md) - ADR summary in architecture docs
- [Project Decisions](../../project/decisions/) - Project-level decisions
```

#### 2.4 Project Management READMEs

##### 2.4.1 project/stories/README.md (MEDIUM PRIORITY)
```markdown
# Polyglot User Stories

**Current Sprint:** Sprint 1 (Epic 1 - Lexer & Parser)
**Sprint Status:** [../bmm-workflow-status.yaml](../bmm-workflow-status.yaml)

## Active Stories

**In Progress:**
- [1-9-syntax-consistency-operator-prefixes.md](./1-9-syntax-consistency-operator-prefixes.md)

**Recently Completed:**
- [1-8-serial-error-handling-test-coverage.md](./1-8-serial-error-handling-test-coverage.md)
- [1-7-december-2025-syntax-updates.md](./1-7-december-2025-syntax-updates.md)

## Story Naming Convention

**Format:** `{epic}-{story}-{title}.md`

**Example:** `1-5-5-multi-file-compilation-same-package-resolution.md`
- Epic 1
- Story 5.5 (sub-story of 1.5)
- Title: Multi-file compilation with same-package resolution

## Story Lifecycle

1. **Drafted** - Created from epic breakdown
2. **Ready** - Acceptance criteria defined, dependencies met
3. **In Progress** - Active development
4. **Ready for Review** - Implementation complete, awaiting review
5. **Done** - Accepted, merged, documented

## See Also
- [Epics](../epics.md) - Epic breakdown and FR mapping
- [PRD](../prd.md) - Product requirements
- [Sprint Status](../bmm-workflow-status.yaml) - Current sprint tracking
```

##### 2.4.2 project/tickets/README.md (LOW PRIORITY)
```markdown
# ITSM Ticket Tracking

**Purpose:** Track changes, incidents, problems, service requests per ITIL framework

## Ticket Types

- **changes/** - Change requests and implementation tracking
- **incidents/** - Production incidents and resolution
- **problems/** - Root cause analysis and preventive measures
- **service-requests/** - Feature requests and enhancements
- **reports/** - Periodic status reports and metrics

## Ticket Format

Each ticket type has its own template and numbering scheme.

## See Also
- [Stories](../stories/) - User story tracking (development work)
```

---

## Phase 3: Master Indexing
**Goal:** Create top-level navigation system
**Priority:** HIGH
**Estimated Time:** 1-2 hours

### Actions

#### 3.1 Create docs/MASTER-INDEX.md

**Content:**
```markdown
# Polyglot Documentation Master Index

**Version:** v0.0.4 (finalized December 2025)
**Implementation Status:** Epic 1 implements v0.0.3, migration to v0.0.4 planned Q2 2026

---

## Quick Start

**New to Polyglot?** Start here:
1. [Quick Start Guide](./user/quick-start.md)
2. [Examples](./user/examples/) - Learn by example
3. [Language Basics](./user/language/) - Core syntax

**Implementing Polyglot?** Developer path:
1. [PRD](./project/prd.md) - Product requirements
2. [Architecture](./technical/architecture/) - System design
3. [Epics & Stories](./project/) - Implementation plan

---

## Documentation Sections

### 📚 User Documentation (`user/`)
**For:** Polyglot language users, developers writing `.pg` files

- [Language Reference](./user/language/) - Complete language specification
- [Syntax Guide](./user/syntax/) - Syntax documentation
- [Examples](./user/examples/) - Annotated code examples
- [Guides](./user/guides/) - How-to guides and tutorials
- [CLI Reference](./user/cli/) - Command-line interface
- [Quick Reference](./user/quick-reference/) - Syntax cheat sheets
- [Standard Library](./user/standard-library/) - Built-in functions and types
- [Packages](./user/packages/) - Package system documentation
- [Advanced Topics](./user/advanced/) - Advanced features
- [Architecture](./user/architecture/) - User-facing architecture concepts

### 📋 Specifications (`specifications/`)
**For:** Language designers, specification authors

- [Version Index](./specifications/VERSION-INDEX.md) - Version history and matrix
- [v0.0.4 Specification](./specifications/v0.0.4/) - Current finalized spec
- [v0.0.5 Draft](./specifications/v0.0.5/) - Future version
- [Brainstorming](./specifications/brainstorming/) - Design explorations

### 🏗️ Technical Documentation (`technical/`)
**For:** Polyglot compiler/runtime implementers

- [Architecture](./technical/architecture/) - System architecture
- [ADRs](./technical/decisions/) - Architecture decisions

### 📊 Project Management (`project/`)
**For:** Project contributors, development tracking

- [PRD](./project/prd.md) - Product requirements document
- [Epics](./project/epics.md) - Epic and story breakdown
- [Stories](./project/stories/) - User story tracking
- [Examples](./project/examples/) - Project example pipelines
- [Tickets](./project/tickets/) - ITSM tracking (changes, incidents, problems)
- [Sprint Status](./project/bmm-workflow-status.yaml) - Current sprint

### 🤖 AI Context (`ai-context/`)
**For:** AI assistants, machine-parseable specs

- [v0.0.2](./ai-context/v0.0.2/) - Archived (outdated)
- v0.0.3 - *(To be created)*
- v0.0.4 - *(To be created)*

### 🧪 Quality Assurance (`qa/`)
**For:** QA engineers, test planning

- [Assessments](./qa/assessments/) - Test design and coverage

### 📦 Archive (`archive/`)
**For:** Historical reference

- [Old User Docs](./archive/user-docs-2025-11-backup/) - Pre-v0.0.4 documentation
- [Syntax Updates](./archive/syntax-updates/) - Historical syntax changes
- [Brainstorming](./archive/brainstorming/) - Archived design sessions
- [Reports](./archive/reports/) - Historical reports
- [Agent Sessions](./archive/agent-sessions/) - AI collaboration history

---

## Version Navigation

**What version should I use?**

- **Learning Polyglot?** → Read [user/](./user/) docs (v0.0.3 current stable)
- **Implementing parser?** → Epic 1 targets v0.0.3, see [project/epics.md](./project/epics.md)
- **Planning migration?** → v0.0.4 is finalized, see [specifications/v0.0.4/](./specifications/v0.0.4/)

**Version Matrix:** See [specifications/VERSION-INDEX.md](./specifications/VERSION-INDEX.md)

---

## Finding Specific Topics

### Syntax Questions
- **Operators:** [user/language/operators.md](./user/language/operators.md)
- **Block markers:** [user/syntax/blocks.md](./user/syntax/blocks.md)
- **Variables:** [user/syntax/variables.md](./user/syntax/variables.md)
- **Metadata:** [user/language/metadata.md](./user/language/metadata.md)

### Implementation Questions
- **Lexer:** Epic 1 Stories 1.2-1.3
- **Parser:** Epic 1 Stories 1.4-1.6
- **IR Generation:** Epic 2
- **Database:** Epic 3

### Decision Context
- **Why this syntax?** See [specifications/v0.0.4/syntax-refinement/](./specifications/v0.0.4/syntax-refinement/)
- **Architecture decisions?** See [technical/decisions/](./technical/decisions/)

---

## Contributing

**Documentation Guidelines:** See [CONTRIBUTING.md](./CONTRIBUTING.md) *(to be created)*

**Reporting Issues:** Use [project/tickets/](./project/tickets/) for formal tracking

---

**Last Updated:** 2025-12-14
**Maintained By:** ReDoc autonomous documentation system
```

#### 3.2 Update Root docs/README.md

**Enhance existing README to link to MASTER-INDEX:**

Add at top:
```markdown
**📑 Complete Navigation:** See [MASTER-INDEX.md](./MASTER-INDEX.md) for full documentation map
```

#### 3.3 Create Topic Catalogs (Optional Enhancement)

**If discoverability is still challenging after READMEs:**

Consider topic-based catalogs:
- `SYNTAX-CATALOG.md` - All syntax-related docs across sections
- `ARCHITECTURE-CATALOG.md` - All architecture docs (user + technical)
- `VERSION-CATALOG.md` - All version-specific documentation

---

## Phase 4: Polish & Completion
**Goal:** Fill remaining gaps, proofread existing docs
**Priority:** MEDIUM → LOW
**Estimated Time:** 1-2 hours

### Actions

#### 4.1 Create Remaining READMEs

**Low priority folders:**
- qa/README.md
- qa/assessments/README.md
- project/tickets/changes/README.md
- project/tickets/incidents/README.md
- project/tickets/problems/README.md
- project/tickets/service-requests/README.md
- project/tickets/reports/README.md

**Use lightweight template:**
```markdown
# [Folder Name]

[1 sentence description]

**Contents:** [Brief summary or "Currently empty - to be populated"]

**See Also:** [Link to parent README]
```

#### 4.2 Proofread Existing READMEs

**Checklist for each existing README:**
- [ ] Frontmatter exists (if using last-redoc-date)
- [ ] Links are valid (not broken)
- [ ] File references are accurate
- [ ] Grammar and spelling clean
- [ ] Terminology consistent with project style

**Priority order:**
1. docs/README.md (root)
2. user/README.md
3. specifications/README.md
4. technical/README.md
5. project/README.md
6. Others

#### 4.3 Create Missing Migration Guides

**From VERSION-INDEX.md, create:**
- `specifications/migrations/v0.0.2-to-v0.0.3.md`
- `specifications/migrations/v0.0.3-to-v0.0.4.md`

**Template:**
```markdown
# Migration Guide: v0.0.X → v0.0.Y

**Date:** YYYY-MM-DD
**Impact:** Breaking changes / Non-breaking enhancements

## Breaking Changes

### Change 1: [Description]
**Old Syntax:**
```polyglot
[old code example]
```

**New Syntax:**
```polyglot
[new code example]
```

**Migration:** [Step-by-step instructions]

[Repeat for each breaking change]

## New Features

[List new features with examples]

## Deprecations

[List deprecated features and replacements]

## Migration Checklist

- [ ] Update variable syntax
- [ ] Update block delimiters
- [ ] Update type annotations
- [ ] Test compilation
- [ ] Update documentation
```

#### 4.4 Create VERSION-MATRIX.md (Root Level)

**High-level version summary at docs/VERSION-MATRIX.md:**

```markdown
# Polyglot Version Matrix

| Version | Status | Release | Implementation | Docs |
|---------|--------|---------|----------------|------|
| v0.0.2 | Archived | 2025-10 | Superseded | [ai-context/v0.0.2](./ai-context/v0.0.2/) |
| v0.0.3 | **Stable** | 2025-11 | Epic 1 (in progress) | [user/](./user/) |
| v0.0.4 | **Finalized** | 2025-12 | Planned Q2 2026 | [specifications/v0.0.4/](./specifications/v0.0.4/) |
| v0.0.5 | Draft | TBD | Future | [specifications/v0.0.5/](./specifications/v0.0.5/) |

**See:** [specifications/VERSION-INDEX.md](./specifications/VERSION-INDEX.md) for detailed changelog
```

---

## Execution Checklist

### Phase 1: Archive Cleanup
- [ ] Compare user-old vs user-old-original for duplicates
- [ ] Consolidate or document difference
- [ ] Rename to clearer structure (user-docs-2025-11-backup)
- [ ] Create archive README explaining contents
- [ ] (Optional) Compress if >2MB

### Phase 2: Critical Navigation
**User Documentation:**
- [ ] Create user/language/README.md (CRITICAL)
- [ ] Create user/syntax/README.md (CRITICAL)
- [ ] Create user/guides/README.md (HIGH)
- [ ] Create user/cli/README.md (HIGH)
- [ ] Create user/advanced/README.md (MEDIUM)
- [ ] Create user/architecture/README.md (MEDIUM)
- [ ] Create user/audit/README.md (LOW)
- [ ] Create user/packages/README.md (LOW)
- [ ] Create user/planning/README.md (LOW)
- [ ] Create user/quick-reference/README.md (LOW)
- [ ] Create user/standard-library/README.md (LOW)

**Specifications:**
- [ ] Create specifications/VERSION-INDEX.md (HIGH)
- [ ] Consolidate v0.0.4 "final" documents (HIGH)
- [ ] Create specifications/v0.0.4/COMPLETE-SPEC.md (HIGH)

**Technical:**
- [ ] Create technical/architecture/README.md (MEDIUM)
- [ ] Create technical/decisions/README.md (MEDIUM)

**Project:**
- [ ] Create project/stories/README.md (MEDIUM)
- [ ] Create project/tickets/README.md (LOW)

### Phase 3: Master Indexing
- [ ] Create docs/MASTER-INDEX.md (HIGH)
- [ ] Update docs/README.md with link to master index
- [ ] (Optional) Create topic catalogs

### Phase 4: Polish & Completion
- [ ] Create remaining low-priority READMEs (qa/, tickets/*)
- [ ] Proofread existing READMEs
- [ ] Create migration guides (v0.0.2→v0.0.3, v0.0.3→v0.0.4)
- [ ] Create root VERSION-MATRIX.md
- [ ] Final review and link validation

---

## Success Criteria

**Phase 1 Complete:**
- ✅ Archive size reduced by >50% OR duplication documented
- ✅ Clear archive structure with explanatory README

**Phase 2 Complete:**
- ✅ All 11 user/* subdirectories have READMEs
- ✅ Version navigation clear (VERSION-INDEX.md exists)
- ✅ Technical sections have navigation READMEs

**Phase 3 Complete:**
- ✅ MASTER-INDEX.md provides complete documentation map
- ✅ New users can find "getting started" in <2 clicks
- ✅ Developers can find architecture/ADRs easily

**Phase 4 Complete:**
- ✅ Zero folders lack README (except empty ticket subdirectories)
- ✅ All existing READMEs proofread and links verified
- ✅ Migration guides exist for version transitions

---

## Post-Cleanup Maintenance

**Ongoing Tasks:**
1. Update MASTER-INDEX.md when new sections added
2. Update VERSION-INDEX.md when new versions released
3. Run `/bmad:bmb:workflows:redoc` periodically to refresh READMEs
4. Validate links quarterly (use link checker tool)

**Recommended Automation:**
- Git pre-commit hook to validate README exists in new directories
- Monthly link checker CI job
- Quarterly documentation audit

---

**End of Cleanup Plan**
